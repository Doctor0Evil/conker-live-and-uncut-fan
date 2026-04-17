//! Validator module for grid2scene.
//! 
//! Provides cross-reference checks between entities and grid cells.

use crate::model::{entities::Entities, grid::Grid};

/// Cross-reference check: for every entity with objective_type "BabyDinoFeeder",
/// verify that at least one grid cell within a 2-cell radius has a role_tag containing "baby_dino_nest".
/// Issues a warning if not found.
pub fn check_baby_dino_feeder(grid: &Grid, entities: &Entities) -> Vec<String> {
    let mut warnings = Vec::new();

    for objective in &entities.objectives {
        if objective.objective_type == "BabyDinoFeeder" {
            let feeder_col = objective.col;
            let feeder_row = objective.row;
            
            // Check all cells within 2-cell radius (Manhattan distance <= 2 for simplicity, or Euclidean)
            let mut found_nest = false;
            
            for cell in &grid.cells {
                let col_diff = (cell.col - feeder_col).abs();
                let row_diff = (cell.row - feeder_row).abs();
                
                // Using Chebyshev distance (max of col_diff, row_diff) for 2-cell radius square
                if col_diff <= 2 && row_diff <= 2 {
                    for tag in &cell.role_tags {
                        if tag.contains("baby_dino_nest") {
                            found_nest = true;
                            break;
                        }
                    }
                    if found_nest {
                        break;
                    }
                }
            }
            
            if !found_nest {
                warnings.push(format!(
                    "WARNING: Objective '{}' (BabyDinoFeeder) at ({}, {}) has no 'baby_dino_nest' role_tag within 2-cell radius",
                    objective.id, feeder_col, feeder_row
                ));
            }
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::entities::Objective;
    use crate::model::grid::{Grid, GridCell, GridSize, Origin};

    #[test]
    fn test_baby_dino_feeder_with_nest() {
        let grid = Grid {
            version: "v1".to_string(),
            grid_size: GridSize { cols: 10, rows: 10 },
            cell_size: 100.0,
            y_level: 0.0,
            origin: Origin { x: 0.0, z: 0.0 },
            cells: vec![
                GridCell {
                    col: 5,
                    row: 5,
                    tile_type: "floor".to_string(),
                    walkable: true,
                    role_tags: vec!["baby_dino_nest_alpha".to_string()],
                },
            ],
        };

        let entities = Entities {
            spawn_points: vec![],
            weapon_pickups: vec![],
            hazard_volumes: vec![],
            objectives: vec![Objective {
                id: "feeder_1".to_string(),
                objective_type: "BabyDinoFeeder".to_string(),
                col: 5,
                row: 5,
                y_offset: 0.0,
                role_tags: vec![],
            }],
        };

        let warnings = check_baby_dino_feeder(&grid, &entities);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_baby_dino_feeder_without_nest() {
        let grid = Grid {
            version: "v1".to_string(),
            grid_size: GridSize { cols: 10, rows: 10 },
            cell_size: 100.0,
            y_level: 0.0,
            origin: Origin { x: 0.0, z: 0.0 },
            cells: vec![
                GridCell {
                    col: 10,  // Too far away
                    row: 10,
                    tile_type: "floor".to_string(),
                    walkable: true,
                    role_tags: vec!["baby_dino_nest_alpha".to_string()],
                },
            ],
        };

        let entities = Entities {
            spawn_points: vec![],
            weapon_pickups: vec![],
            hazard_volumes: vec![],
            objectives: vec![Objective {
                id: "feeder_1".to_string(),
                objective_type: "BabyDinoFeeder".to_string(),
                col: 5,
                row: 5,
                y_offset: 0.0,
                role_tags: vec![],
            }],
        };

        let warnings = check_baby_dino_feeder(&grid, &entities);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("feeder_1"));
    }

    #[test]
    fn test_non_baby_dino_feeder_ignored() {
        let grid = Grid {
            version: "v1".to_string(),
            grid_size: GridSize { cols: 10, rows: 10 },
            cell_size: 100.0,
            y_level: 0.0,
            origin: Origin { x: 0.0, z: 0.0 },
            cells: vec![],
        };

        let entities = Entities {
            spawn_points: vec![],
            weapon_pickups: vec![],
            hazard_volumes: vec![],
            objectives: vec![Objective {
                id: "other_obj".to_string(),
                objective_type: "DataTerminal".to_string(),
                col: 5,
                row: 5,
                y_offset: 0.0,
                role_tags: vec![],
            }],
        };

        let warnings = check_baby_dino_feeder(&grid, &entities);
        assert!(warnings.is_empty());
    }
}
