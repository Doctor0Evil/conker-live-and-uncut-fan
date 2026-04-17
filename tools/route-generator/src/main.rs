//! Route Generator for grid2scene
//! 
//! Reads a map grid JSON and a list of role_tag nodes (e.g., alien_vent_spawn),
//! then outputs a patrol_routes_v1.json file with computed A* paths between nodes.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(
    name = "route-generator",
    about = "Generate patrol routes from grid-based pathfinding"
)]
struct Cli {
    /// Path to the grid JSON file.
    #[arg(long)]
    grid: PathBuf,

    /// Path to output the patrol_routes_v1.json file.
    #[arg(long, default_value = "patrol_routes_v1.json")]
    output: PathBuf,

    /// Comma-separated list of role tags to use as route nodes (e.g., "alien_vent_spawn,guard_post").
    #[arg(long, value_delimiter = ',')]
    node_tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GridSize {
    cols: i32,
    rows: i32,
}

#[derive(Debug, Deserialize)]
struct GridCell {
    col: i32,
    row: i32,
    tile_type: String,
    #[serde(default = "default_walkable")]
    walkable: bool,
    #[serde(default)]
    role_tags: Vec<String>,
}

fn default_walkable() -> bool {
    true
}

#[derive(Debug, Deserialize)]
struct Grid {
    version: String,
    grid_size: GridSize,
    cell_size: f32,
    y_level: f32,
    #[serde(default)]
    origin: Origin,
    cells: Vec<GridCell>,
}

#[derive(Debug, Deserialize, Default)]
struct Origin {
    x: f32,
    z: f32,
}

/// A node in the patrol route system.
#[derive(Debug, Clone, Serialize)]
struct RouteNode {
    id: String,
    col: i32,
    row: i32,
    role_tag: String,
}

/// A patrol route connecting two or more nodes.
#[derive(Debug, Clone, Serialize)]
struct PatrolRoute {
    id: String,
    nodes: Vec<RouteNode>,
    path: Vec<(i32, i32)>, // Sequence of (col, row) coordinates
    total_distance: f32,
}

/// Output format for patrol_routes_v1.json.
#[derive(Debug, Serialize)]
struct PatrolRoutesOutput {
    version: String,
    routes: Vec<PatrolRoute>,
    nodes: Vec<RouteNode>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load grid
    let grid_bytes = fs::read(&cli.grid)
        .with_context(|| format!("Failed to read grid file {:?}", cli.grid))?;
    let grid: Grid = serde_json::from_slice(&grid_bytes)
        .context("Failed to parse grid JSON")?;

    println!(
        "[route-generator] Loaded grid {}x{} with {} cells",
        grid.grid_size.cols, grid.grid_size.rows, grid.cells.len()
    );

    // Find all cells matching the specified role tags
    let mut nodes_by_tag: HashMap<String, Vec<RouteNode>> = HashMap::new();
    
    for cell in &grid.cells {
        for tag in &cell.role_tags {
            if cli.node_tags.iter().any(|t| tag.contains(t.as_str())) {
                let node = RouteNode {
                    id: format!("{}_{}_{}", tag, cell.col, cell.row),
                    col: cell.col,
                    row: cell.row,
                    role_tag: tag.clone(),
                };
                nodes_by_tag
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(node);
            }
        }
    }

    // Collect all nodes
    let all_nodes: Vec<RouteNode> = nodes_by_tag.values().flatten().cloned().collect();
    println!("[route-generator] Found {} route nodes", all_nodes.len());

    // Build adjacency graph and compute A* paths between connected nodes
    let mut routes = Vec::new();
    
    // For each tag group, create routes connecting nearby nodes
    for (tag, nodes) in &nodes_by_tag {
        if nodes.len() < 2 {
            continue;
        }

        // Sort nodes by position for deterministic ordering
        let mut sorted_nodes = nodes.clone();
        sorted_nodes.sort_by(|a, b| {
            a.row.cmp(&b.row).then(a.col.cmp(&b.col))
        });

        // Create routes between consecutive nodes
        for i in 0..sorted_nodes.len() - 1 {
            let start = &sorted_nodes[i];
            let end = &sorted_nodes[i + 1];

            // Compute A* path
            match astar_path(&grid, (start.col, start.row), (end.col, end.row)) {
                Some(path) => {
                    let distance = calculate_path_distance(&path, grid.cell_size);
                    let route = PatrolRoute {
                        id: format!("route_{}_{}", start.id, end.id),
                        nodes: vec![start.clone(), end.clone()],
                        path: path.clone(),
                        total_distance: distance,
                    };
                    routes.push(route);
                }
                None => {
                    eprintln!(
                        "[route-generator] Warning: No path found between {} and {}",
                        start.id, end.id
                    );
                }
            }
        }

        // Also connect last node back to first for loop patrol
        if sorted_nodes.len() >= 2 {
            let start = sorted_nodes.last().unwrap();
            let end = sorted_nodes.first().unwrap();

            if let Some(path) = astar_path(&grid, (start.col, start.row), (end.col, end.row)) {
                let distance = calculate_path_distance(&path, grid.cell_size);
                let route = PatrolRoute {
                    id: format!("route_{}_{}_loop", start.id, end.id),
                    nodes: vec![start.clone(), end.clone()],
                    path: path.clone(),
                    total_distance: distance,
                };
                routes.push(route);
            }
        }
    }

    // Create output
    let output = PatrolRoutesOutput {
        version: "v1".to_string(),
        routes,
        nodes: all_nodes,
    };

    // Write output
    let json = serde_json::to_string_pretty(&output)
        .context("Failed to serialize patrol routes")?;
    fs::write(&cli.output, &json)
        .with_context(|| format!("Failed to write output to {:?}", cli.output))?;

    println!(
        "[route-generator] Generated {} routes with {} nodes, output written to {:?}",
        output.routes.len(),
        output.nodes.len(),
        cli.output
    );

    Ok(())
}

/// Calculate total distance of a path in world units.
fn calculate_path_distance(path: &[(i32, i32)], cell_size: f32) -> f32 {
    if path.len() < 2 {
        return 0.0;
    }

    let mut distance = 0.0;
    for i in 0..path.len() - 1 {
        let (c1, r1) = path[i];
        let (c2, r2) = path[i + 1];
        
        // Euclidean distance between cell centers
        let dc = (c2 - c1) as f32;
        let dr = (r2 - r1) as f32;
        distance += (dc * dc + dr * dr).sqrt() * cell_size;
    }

    distance
}

/// A* pathfinding on a grid.
fn astar_path(grid: &Grid, start: (i32, i32), goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    use priority_queue::PriorityQueue;

    // Build cell lookup
    let cell_map: HashMap<(i32, i32), &GridCell> = grid
        .cells
        .iter()
        .map(|c| ((c.col, c.row), c))
        .collect();

    // Check if start and goal are walkable
    let start_cell = cell_map.get(&start)?;
    let goal_cell = cell_map.get(&goal)?;
    
    if !start_cell.walkable || !goal_cell.walkable {
        return None;
    }

    // Directions: N, S, E, W, NE, NW, SE, SW
    let directions = [
        (0, -1), (0, 1), (1, 0), (-1, 0),
        (1, -1), (-1, -1), (1, 1), (-1, 1),
    ];

    // Priority queue: (cost, position)
    let mut pq: PriorityQueue<(i32, i32), i32> = PriorityQueue::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut cost_so_far: HashMap<(i32, i32), i32> = HashMap::new();

    pq.push(start, 0);
    cost_so_far.insert(start, 0);

    while let Some((current, _)) = pq.pop() {
        if current == goal {
            break;
        }

        let current_cell = cell_map.get(&current)?;
        
        for &(dc, dr) in &directions {
            let next = (current.0 + dc, current.1 + dr);
            
            if let Some(next_cell) = cell_map.get(&next) {
                if !next_cell.walkable {
                    continue;
                }

                // Cost: 1 for cardinal, sqrt(2) ~ 1.414 for diagonal (using integer approx: 10 vs 14)
                let move_cost = if dc != 0 && dr != 0 { 14 } else { 10 };
                let new_cost = cost_so_far.get(&current).copied().unwrap_or(0) + move_cost;

                if new_cost < cost_so_far.get(&next).copied().unwrap_or(i32::MAX) {
                    cost_so_far.insert(next, new_cost);
                    
                    // Heuristic: Manhattan distance * 10
                    let heuristic = ((goal.0 - next.0).abs() + (goal.1 - next.1).abs()) * 10;
                    let priority = new_cost + heuristic;
                    
                    pq.push(next, -priority); // Negate for max-heap behavior
                    came_from.insert(next, current);
                }
            }
        }
    }

    // Reconstruct path
    if !came_from.contains_key(&goal) && start != goal {
        return None;
    }

    let mut path = Vec::new();
    let mut current = goal;
    path.push(current);

    while current != start {
        current = *came_from.get(&current)?;
        path.push(current);
    }

    path.reverse();
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_path_distance() {
        let path = vec![(0, 0), (1, 0), (2, 0)];
        let distance = calculate_path_distance(&path, 100.0);
        assert!((distance - 200.0).abs() < 0.001);
    }

    #[test]
    fn test_astar_simple_path() {
        let grid = Grid {
            version: "v1".to_string(),
            grid_size: GridSize { cols: 5, rows: 5 },
            cell_size: 100.0,
            y_level: 0.0,
            origin: Origin { x: 0.0, z: 0.0 },
            cells: vec![
                GridCell { col: 0, row: 0, tile_type: "floor".to_string(), walkable: true, role_tags: vec![] },
                GridCell { col: 1, row: 0, tile_type: "floor".to_string(), walkable: true, role_tags: vec![] },
                GridCell { col: 2, row: 0, tile_type: "floor".to_string(), walkable: true, role_tags: vec![] },
            ],
        };

        let path = astar_path(&grid, (0, 0), (2, 0));
        assert!(path.is_some());
        let p = path.unwrap();
        assert_eq!(p.len(), 3);
        assert_eq!(p[0], (0, 0));
        assert_eq!(p[2], (2, 0));
    }
}
