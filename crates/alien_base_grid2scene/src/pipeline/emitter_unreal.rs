use crate::model::{entities::Entities, grid::Grid, tileset::Tileset};
use serde::Serialize;
use std::io::Write;

#[derive(Serialize)]
struct UnrealVec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize)]
struct UnrealTileSetOut {
    tiletype: String,
    meshpath: String,
    instances: Vec<UnrealVec3>,
}

#[derive(Serialize)]
struct UnrealSpawnOut {
    id: String,
    zone: String,
    position: UnrealVec3,
    yaw_degrees: f32,
}

#[derive(Serialize)]
struct UnrealWeaponPickupOut {
    id: String,
    r#type: String,
    class_path: String,
    position: UnrealVec3,
}

#[derive(Serialize)]
struct UnrealHazardOut {
    id: String,
    r#type: String,
    class_path: String,
    center: UnrealVec3,
    radius: f32,
    y_min: f32,
    y_max: f32,
    hazard_profile_id: String,
}

#[derive(Serialize)]
struct UnrealObjectiveOut {
    id: String,
    r#type: String,
    class_path: String,
    position: UnrealVec3,
}

#[derive(Serialize)]
struct UnrealLevelOut {
    level_name: String,
    hub_y_level: f32,
    tilesets: Vec<UnrealTileSetOut>,
    spawns: Vec<UnrealSpawnOut>,
    weapon_pickups: Vec<UnrealWeaponPickupOut>,
    hazards: Vec<UnrealHazardOut>,
    objectives: Vec<UnrealObjectiveOut>,
}

/// Emit a single JSON blob that Unreal editor tooling can consume to build LVAlienBaseMulti,
/// The Blood Count, etc. All hazards are emitted as BP_HazardVolume with a data-driven
/// `hazard_profile_id` which maps into data/hazards/hazard_profiles_v1.json.
pub fn emit_unreal<W: Write>(
    grid: Grid,
    entities: Entities,
    tileset: Tileset,
    mut out: W,
) -> Result<(), serde_json::Error> {
    // Tiles as instanced meshes.
    let mut tilesets: Vec<UnrealTileSetOut> = Vec::new();
    for tilemap in &tileset.tile_mappings {
        let mut instances = Vec::new();
        for cell in &grid.cells {
            if cell.tiletype == tilemap.tiletype {
                let (x, y, z) = grid.cell_to_world(cell.col, cell.row, 0.0);
                instances.push(UnrealVec3 { x, y, z });
            }
        }
        if !instances.is_empty() {
            tilesets.push(UnrealTileSetOut {
                tiletype: tilemap.tiletype.clone(),
                meshpath: tilemap.asset_id.clone(),
                instances,
            });
        }
    }

    // Player spawns.
    let mut spawns: Vec<UnrealSpawnOut> = Vec::new();
    for sp in &entities.spawn_points {
        let (x, y, z) = grid.cell_to_world(sp.col, sp.row, sp.y_offset);
        spawns.push(UnrealSpawnOut {
            id: sp.id.clone(),
            zone: sp.zone.clone(),
            position: UnrealVec3 { x, y, z },
            yaw_degrees: sp.yaw_degrees.unwrap_or(0.0),
        });
    }

    // Weapon pickups.
    let mut pickups: Vec<UnrealWeaponPickupOut> = Vec::new();
    for wp in &entities.weapon_pickups {
        if let Some(class_path) = tileset.entity_mappings.weapon_pickup.get(&wp.r#type) {
            let (x, y, z) = grid.cell_to_world(wp.col, wp.row, wp.y_offset);
            pickups.push(UnrealWeaponPickupOut {
                id: wp.id.clone(),
                r#type: wp.r#type.clone(),
                class_path: class_path.clone(),
                position: UnrealVec3 { x, y, z },
            });
        }
    }

    // Hazards, including Alien Base gas/acid and The Blood Count Grinder.
    let mut hazards: Vec<UnrealHazardOut> = Vec::new();
    for hv in &entities.hazard_volumes {
        // All hazards use the same BP, behavior is driven by `hazard_profile_id`.
        // tileset.entity_mappings.hazard_volume maps logical types (hub_gas, sublevel_acid,
        // library_grinder) to a BP_HazardVolume-derived class.
        if let Some(class_path) = tileset.entity_mappings.hazard_volume.get(&hv.r#type) {
            let (cx, cy, cz) = grid.cell_to_world(hv.center_col, hv.center_row, 0.0);
            let radius = hv.radius_cells as f32 * grid.cell_size;
            let y_min = grid.y_level + hv.y_min_offset;
            let y_max = grid.y_level + hv.y_max_offset;

            let hazard_profile_id = hv
                .hazard_profile_id
                .clone()
                .unwrap_or_else(|| "gas_standard_floor".to_string());

            hazards.push(UnrealHazardOut {
                id: hv.id.clone(),
                r#type: hv.r#type.clone(),
                class_path: class_path.clone(),
                center: UnrealVec3 { x: cx, y: cy, z: cz },
                radius,
                y_min,
                y_max,
                hazard_profile_id,
            });
        }
    }

    // Objectives.
    let mut objectives: Vec<UnrealObjectiveOut> = Vec::new();
    for obj in &entities.objectives {
        if let Some(class_path) = tileset.entity_mappings.objective.get(&obj.r#type) {
            let (x, y, z) = grid.cell_to_world(obj.col, obj.row, obj.y_offset);
            objectives.push(UnrealObjectiveOut {
                id: obj.id.clone(),
                r#type: obj.r#type.clone(),
                class_path: class_path.clone(),
                position: UnrealVec3 { x, y, z },
            });
        }
    }

    let out_obj = UnrealLevelOut {
        level_name: grid
            .level_name
            .clone()
            .unwrap_or_else(|| "LVUnknown".to_string()),
        hub_y_level: grid.y_level,
        tilesets,
        spawns,
        weapon_pickups: pickups,
        hazards,
        objectives,
    };

    serde_json::to_writer_pretty(&mut out, &out_obj)
}
