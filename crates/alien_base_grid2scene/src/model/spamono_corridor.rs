use crate::model::grid::Grid;

/// Simple helper for T.M.S Spamono corridor navigation.
/// Treats the main play space as a set of longitudinal lanes (columns)
/// and important Z-intervals (spawn, mid-airlocks, goals).
pub struct SpamonoCorridor<'a> {
    grid: &'a Grid,
    /// World-space Z of the center, spawn, airlocks, and goals
    pub z_center: f32,
    pub z_spawn_a: f32,
    pub z_spawn_b: f32,
    pub z_airlock_a: f32,
    pub z_airlock_b: f32,
    pub z_goal_a: f32,
    pub z_goal_b: f32,
}

impl<'a> SpamonoCorridor<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        let (_, cz) = grid.center_indices();
        let center_world_z = grid.origin.z + 0.0;

        // These offsets assume your grid uses Z increasing from Team A to Team B.
        let spawn_offset = grid.cell_size * 3.0;
        let airlock_offset = grid.cell_size * 6.0;
        let goal_offset = grid.cell_size * 10.0;

        Self {
            grid,
            z_center: center_world_z,
            z_spawn_a: center_world_z - spawn_offset,
            z_spawn_b: center_world_z + spawn_offset,
            z_airlock_a: center_world_z - airlock_offset,
            z_airlock_b: center_world_z + airlock_offset,
            z_goal_a: center_world_z - goal_offset,
            z_goal_b: center_world_z + goal_offset,
        }
    }

    /// Returns a lane index (column) best suited for running the sphere
    /// based on an input world X coordinate.
    pub fn lane_from_world_x(&self, x: f32) -> i32 {
        let (cx, _) = self.grid.center_indices();
        let rel = (x - self.grid.origin.x) / self.grid.cell_size;
        (cx + rel.round()) as i32
    }

    /// Clamps a world-space Z into one of the key corridor bands (spawn, mid, goal).
    pub fn clamp_to_corridor_band(&self, z: f32) -> f32 {
        let bands = [
            self.z_spawn_a,
            self.z_airlock_a,
            self.z_center,
            self.z_airlock_b,
            self.z_spawn_b,
        ];

        let mut best = bands[0];
        let mut best_dist = (z - bands[0]).abs();

        for &b in &bands[1..] {
            let d = (z - b).abs();
            if d < best_dist {
                best = b;
                best_dist = d;
            }
        }

        best
    }
}
