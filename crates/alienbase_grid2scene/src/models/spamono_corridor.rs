use crate::grid::{GridCell, MultiplayerMapGridV1};
use serde::{Deserialize, Serialize};

/// Logical bands along the Spamono corridor for AI/director logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpamonoBand {
    TeamASpawn,
    TeamAApproach,
    AirlockA,
    Mid,
    AirlockB,
    TeamBApproach,
    TeamBSpawn,
}

/// Lateral lanes across the corridor width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpamonoLane {
    Left,
    Center,
    Right,
}

/// World‑space definitions of the Spamono corridor along the Z axis.
#[derive(Debug, Clone)]
pub struct SpamonoCorridor {
    /// Minimum and maximum world Z of the entire corridor.
    pub z_min: f32,
    pub z_max: f32,

    /// World Z bounds of team spawn bands.
    pub team_a_spawn_z_min: f32,
    pub team_a_spawn_z_max: f32,
    pub team_b_spawn_z_min: f32,
    pub team_b_spawn_z_max: f32,

    /// World Z bounds of pre‑airlock approach bands.
    pub team_a_approach_z_min: f32,
    pub team_a_approach_z_max: f32,
    pub team_b_approach_z_min: f32,
    pub team_b_approach_z_max: f32,

    /// World Z bounds of airlock bands.
    pub airlock_a_z_min: f32,
    pub airlock_a_z_max: f32,
    pub airlock_b_z_min: f32,
    pub airlock_b_z_max: f32,

    /// World Z bounds of mid band (Data Core region).
    pub mid_z_min: f32,
    pub mid_z_max: f32,

    /// World X center and half width for lane calculations.
    pub x_center: f32,
    pub half_width: f32,

    /// Raw grid metadata (optional, primarily for debugging and meta‑tools).
    pub cell_size: f32,
}

impl SpamonoCorridor {
    /// Construct a SpamonoCorridor from a MultiplayerMapGridV1 description.
    ///
    /// Assumptions:
    /// - Grid cells are defined on a shared 4‑unit cell size (but any cell_size works).
    /// - Roletags are assigned on rows as:
    ///   - "team_a_spawn"   near one end of the corridor
    ///   - "pre_airlock_a"  just forward of team A spawn
    ///   - "airlock_a"      first airlock band
    ///   - "corridor_mid"   center third, containing "data_core_spawn" tiles
    ///   - "airlock_b"      second airlock band
    ///   - "pre_airlock_b"  just before team B spawn
    ///   - "team_b_spawn"   at the opposite end of the corridor
    ///
    /// The grid can be 9–11 columns wide by 40–48 rows long; only the roletags matter.
    pub fn from_grid(grid: &MultiplayerMapGridV1) -> Option<Self> {
        let cell_size = grid.cell_size;

        let mut a_spawn_rows = Vec::new();
        let mut a_pre_airlock_rows = Vec::new();
        let mut a_airlock_rows = Vec::new();
        let mut mid_rows = Vec::new();
        let mut b_airlock_rows = Vec::new();
        let mut b_pre_airlock_rows = Vec::new();
        let mut b_spawn_rows = Vec::new();

        let mut mid_cells_for_center = Vec::new();

        for row in 0..grid.rows {
            for col in 0..grid.cols {
                let cell = &grid.cells[row as usize][col as usize];

                if cell.has_roletag("team_a_spawn") {
                    a_spawn_rows.push(row);
                }
                if cell.has_roletag("pre_airlock_a") {
                    a_pre_airlock_rows.push(row);
                }
                if cell.has_roletag("airlock_a") {
                    a_airlock_rows.push(row);
                }
                if cell.has_roletag("corridor_mid") || cell.has_roletag("data_core_spawn") {
                    mid_rows.push(row);
                    mid_cells_for_center.push((col, row));
                }
                if cell.has_roletag("airlock_b") {
                    b_airlock_rows.push(row);
                }
                if cell.has_roletag("pre_airlock_b") {
                    b_pre_airlock_rows.push(row);
                }
                if cell.has_roletag("team_b_spawn") {
                    b_spawn_rows.push(row);
                }
            }
        }

        if a_spawn_rows.is_empty()
            || a_pre_airlock_rows.is_empty()
            || a_airlock_rows.is_empty()
            || mid_rows.is_empty()
            || b_airlock_rows.is_empty()
            || b_pre_airlock_rows.is_empty()
            || b_spawn_rows.is_empty()
        {
            return None;
        }

        let (team_a_spawn_z_min, team_a_spawn_z_max) = Self::row_band_to_world_z(&a_spawn_rows, cell_size);
        let (team_a_approach_z_min, team_a_approach_z_max) =
            Self::row_band_to_world_z(&a_pre_airlock_rows, cell_size);
        let (airlock_a_z_min, airlock_a_z_max) = Self::row_band_to_world_z(&a_airlock_rows, cell_size);

        let (mid_z_min, mid_z_max) = Self::row_band_to_world_z(&mid_rows, cell_size);

        let (airlock_b_z_min, airlock_b_z_max) = Self::row_band_to_world_z(&b_airlock_rows, cell_size);
        let (team_b_approach_z_min, team_b_approach_z_max) =
            Self::row_band_to_world_z(&b_pre_airlock_rows, cell_size);
        let (team_b_spawn_z_min, team_b_spawn_z_max) = Self::row_band_to_world_z(&b_spawn_rows, cell_size);

        let z_min = team_a_spawn_z_min.min(team_b_spawn_z_min);
        let z_max = team_a_spawn_z_max.max(team_b_spawn_z_max);

        let (x_center, half_width) =
            Self::estimate_lane_center_and_half_width(grid, &mid_cells_for_center, cell_size)?;

        Some(SpamonoCorridor {
            z_min,
            z_max,
            team_a_spawn_z_min,
            team_a_spawn_z_max,
            team_b_spawn_z_min,
            team_b_spawn_z_max,
            team_a_approach_z_min,
            team_a_approach_z_max,
            team_b_approach_z_min,
            team_b_approach_z_max,
            airlock_a_z_min,
            airlock_a_z_max,
            airlock_b_z_min,
            airlock_b_z_max,
            mid_z_min,
            mid_z_max,
            x_center,
            half_width,
            cell_size,
        })
    }

    fn row_band_to_world_z(rows: &[i32], cell_size: f32) -> (f32, f32) {
        let min_row = *rows.iter().min().unwrap() as f32;
        let max_row = *rows.iter().max().unwrap() as f32;
        let z_min = min_row * cell_size;
        let z_max = (max_row + 1.0) * cell_size;
        (z_min, z_max)
    }

    fn estimate_lane_center_and_half_width(
        grid: &MultiplayerMapGridV1,
        mid_cells: &[(i32, i32)],
        cell_size: f32,
    ) -> Option<(f32, f32)> {
        if mid_cells.is_empty() {
            return None;
        }

        let mut min_col = grid.cols;
        let mut max_col = 0;
        for (col, _) in mid_cells {
            if *col < min_col {
                min_col = *col;
            }
            if *col > max_col {
                max_col = *col;
            }
        }

        let x_min = (min_col as f32) * cell_size;
        let x_max = ((max_col + 1) as f32) * cell_size;

        let x_center = (x_min + x_max) * 0.5;
        let half_width = (x_max - x_min) * 0.5;

        Some((x_center, half_width))
    }

    pub fn classify_band(&self, z: f32) -> SpamonoBand {
        if z >= self.team_a_spawn_z_min && z < self.team_a_spawn_z_max {
            SpamonoBand::TeamASpawn
        } else if z >= self.team_a_approach_z_min && z < self.team_a_approach_z_max {
            SpamonoBand::TeamAApproach
        } else if z >= self.airlock_a_z_min && z < self.airlock_a_z_max {
            SpamonoBand::AirlockA
        } else if z >= self.mid_z_min && z < self.mid_z_max {
            SpamonoBand::Mid
        } else if z >= self.airlock_b_z_min && z < self.airlock_b_z_max {
            SpamonoBand::AirlockB
        } else if z >= self.team_b_approach_z_min && z < self.team_b_approach_z_max {
            SpamonoBand::TeamBApproach
        } else {
            SpamonoBand::TeamBSpawn
        }
    }

    pub fn classify_lane(&self, x: f32) -> SpamonoLane {
        if self.half_width <= 0.0 {
            return SpamonoLane::Center;
        }

        let dx = x - self.x_center;
        let third = self.half_width / 3.0;

        if dx < -third {
            SpamonoLane::Left
        } else if dx > third {
            SpamonoLane::Right
        } else {
            SpamonoLane::Center
        }
    }

    pub fn clamp_to_corridor(&self, z: f32) -> f32 {
        if z < self.z_min {
            self.z_min
        } else if z > self.z_max {
            self.z_max
        } else {
            z
        }
    }

    pub fn band_center_z(&self, band: SpamonoBand) -> f32 {
        match band {
            SpamonoBand::TeamASpawn => Self::midpoint(self.team_a_spawn_z_min, self.team_a_spawn_z_max),
            SpamonoBand::TeamAApproach => {
                Self::midpoint(self.team_a_approach_z_min, self.team_a_approach_z_max)
            }
            SpamonoBand::AirlockA => Self::midpoint(self.airlock_a_z_min, self.airlock_a_z_max),
            SpamonoBand::Mid => Self::midpoint(self.mid_z_min, self.mid_z_max),
            SpamonoBand::AirlockB => Self::midpoint(self.airlock_b_z_min, self.airlock_b_z_max),
            SpamonoBand::TeamBApproach => {
                Self::midpoint(self.team_b_approach_z_min, self.team_b_approach_z_max)
            }
            SpamonoBand::TeamBSpawn => Self::midpoint(self.team_b_spawn_z_min, self.team_b_spawn_z_max),
        }
    }

    pub fn lane_center_x(&self, lane: SpamonoLane) -> f32 {
        match lane {
            SpamonoLane::Left => self.x_center - self.half_width * (2.0 / 3.0),
            SpamonoLane::Center => self.x_center,
            SpamonoLane::Right => self.x_center + self.half_width * (2.0 / 3.0),
        }
    }

    fn midpoint(a: f32, b: f32) -> f32 {
        (a + b) * 0.5
    }
}

pub trait SpamonoGridExt {
    fn build_spamono_corridor(&self) -> Option<SpamonoCorridor>;
}

impl SpamonoGridExt for MultiplayerMapGridV1 {
    fn build_spamono_corridor(&self) -> Option<SpamonoCorridor> {
        SpamonoCorridor::from_grid(self)
    }
}

impl GridCell {
    pub fn has_roletag(&self, tag: &str) -> bool {
        self.roletags.iter().any(|t| t == tag)
    }
}
