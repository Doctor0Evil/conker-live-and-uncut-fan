use crate::model::grid::{Grid, GridCell};

/// Simple navigation helper for Beach Dead-style trench corridors.
/// This assumes your Beach grid tags trench/fence tiles with role_tags
/// like "trench", "fence_1", "fence_2", "fence_3".
pub struct BeachDeadNav<'a> {
    pub grid: &'a Grid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub col: i32,
    pub row: i32,
}

impl<'a> BeachDeadNav<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Self { grid }
    }

    /// Returns all walkable neighbor cells (4-way) from a grid position.
    pub fn neighbors(&self, pos: GridPos) -> Vec<GridPos> {
        let mut out = Vec::with_capacity(4);
        let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (dx, dz) in deltas {
            let nc = pos.col + dx;
            let nr = pos.row + dz;
            if self.is_walkable(nc, nr) {
                out.push(GridPos { col: nc, row: nr });
            }
        }

        out
    }

    /// Returns true if this cell exists and is walkable.
    pub fn is_walkable(&self, col: i32, row: i32) -> bool {
        self.grid
            .cells
            .iter()
            .find(|c| c.col == col && c.row == row)
            .map(|c| c.walkable)
            .unwrap_or(false)
    }

    /// Find all trench cells (for SHC assault routes).
    pub fn trench_cells(&self) -> Vec<GridPos> {
        self.cells_with_tag("trench")
    }

    /// Find all cells for a specific fence stage (e.g. "fence_1").
    pub fn fence_cells(&self, fence_tag: &str) -> Vec<GridPos> {
        self.cells_with_tag(fence_tag)
    }

    fn cells_with_tag(&self, tag: &str) -> Vec<GridPos> {
        self.grid
            .cells
            .iter()
            .filter(|c| c.role_tags.iter().any(|t| t == tag))
            .map(|c| GridPos { col: c.col, row: c.row })
            .collect()
    }

    /// Convenience: convert a GridPos to world coordinates.
    pub fn grid_to_world(&self, pos: GridPos, y_offset: f32) -> (f32, f32, f32) {
        self.grid.cell_to_world(pos.col, pos.row, y_offset)
    }
}
