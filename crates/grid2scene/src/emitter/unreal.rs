//! Unreal Engine emitter for grid2scene.

use crate::model::{entities::HazardVolumeEntity, grid::Grid};
use crate::schema::hazard::HazardProfile;
use std::io::Write;

/// Emits a Python script line to spawn an AHazardVolume actor and set its profile.
/// 
/// Given a HazardVolumeEntity and a loaded HazardProfile, this generates:
/// `hazard_vol = level.spawn_actor("AHazardVolume", location=(x,y,z)); hazard_vol.SetProfileID("hub_gas_v1")`
pub fn emit_hazard_volume<W: Write>(
    entity: &HazardVolumeEntity,
    profile: &HazardProfile,
    grid: &Grid,
    out: &mut W,
) -> std::io::Result<()> {
    let (cx, cy, cz) = grid.cell_to_world(entity.center_col, entity.center_row, 0.0);
    
    writeln!(
        out,
        "hazard_vol = level.spawn_actor(\"AHazardVolume\", location=({}, {}, {})); hazard_vol.SetProfileID(\"{}\")",
        cx, cy, cz, profile.id
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::grid::{Grid, GridCell, GridSize, Origin};

    #[test]
    fn test_emit_hazard_volume() {
        let grid = Grid {
            version: "v1".to_string(),
            grid_size: GridSize { cols: 10, rows: 10 },
            cell_size: 100.0,
            y_level: 0.0,
            origin: Origin { x: 0.0, z: 0.0 },
            cells: vec![],
        };

        let entity = HazardVolumeEntity {
            id: "gas_zone_1".to_string(),
            rtype: "hazard_volume".to_string(),
            hazard_profile_id: "hub_gas_v1".to_string(),
            center_col: 5,
            center_row: 5,
            radius_cells: 3,
            y_min_offset: 0.0,
            y_max_offset: 200.0,
        };

        let profile = HazardProfile {
            id: "hub_gas_v1".to_string(),
            damage_per_second: 10.0,
            immunity_asids: vec![1, 2],
            vfx_asset: None,
            sfx_asset: None,
        };

        let mut output = Vec::new();
        emit_hazard_volume(&entity, &profile, &grid, &mut output).unwrap();
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("AHazardVolume"));
        assert!(result.contains("SetProfileID(\"hub_gas_v1\")"));
    }
}
