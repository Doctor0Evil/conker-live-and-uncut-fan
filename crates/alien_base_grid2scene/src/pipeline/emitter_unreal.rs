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
    tile_type: String,
    mesh_path: String,
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
    volume_class_path: String,
    center: UnrealVec3,
    radius: f32,
    y_min: f32,
    y_max: f32,
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
    tile_sets: Vec<UnrealTileSetOut>,
    spawns: Vec<UnrealSpawnOut>,
    weapon_pickups: Vec<UnrealWeaponPickupOut>,
    hazards: Vec<UnrealHazardOut>,
    objectives: Vec<UnrealObjectiveOut>,
}

pub fn emit_unreal<W: Write>(
    grid: &Grid,
    entities: &Entities,
    tileset: &Tileset,
    out: &mut W,
) -> Result<(), serde_json::Error> {
    let mut tile_sets: Vec<UnrealTileSetOut> = Vec::new();

    for tile_map in &tileset.tile_mappings {
        let mut instances = Vec::new();
        for cell in &grid.cells {
            if cell.tile_type == tile_map.tile_type {
                let (x, y, z) = grid.cell_to_world(cell.col, cell.row, 0.0);
                instances.push(UnrealVec3 { x, y, z });
            }
        }

        if !instances.is_empty() {
            tile_sets.push(UnrealTileSetOut {
                tile_type: tile_map.tile_type.clone(),
                mesh_path: tile_map.asset_id.clone(),
                instances,
            });
        }
    }

    let mut spawns = Vec::new();
    for sp in &entities.spawn_points {
        let (x, y, z) = grid.cell_to_world(sp.col, sp.row, sp.y_offset);
        spawns.push(UnrealSpawnOut {
            id: sp.id.clone(),
            zone: sp.zone.clone(),
            position: UnrealVec3 { x, y, z },
            yaw_degrees: 0.0,
        });
    }

    let mut pickups = Vec::new();
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

    let mut hazards = Vec::new();
    for hv in &entities.hazard_volumes {
        if let Some(class_path) = tileset.entity_mappings.hazard_volume.get(&hv.r#type) {
            let (cx, cy, cz) =
                grid.cell_to_world(hv.center_col, hv.center_row, 0.0);
            let radius = hv.radius_cells as f32 * grid.cell_size;
            hazards.push(UnrealHazardOut {
                id: hv.id.clone(),
                r#type: hv.r#type.clone(),
                volume_class_path: class_path.clone(),
                center: UnrealVec3 { x: cx, y: cy, z: cz },
                radius,
                y_min: grid.y_level + hv.y_min_offset,
                y_max: grid.y_level + hv.y_max_offset,
            });
        }
    }

    let mut objectives = Vec::new();
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
        level_name: "LV_AlienBase_Multi".to_string(),
        hub_y_level: grid.y_level,
        tile_sets,
        spawns,
        weapon_pickups: pickups,
        hazards,
        objectives,
    };

    serde_json::to_writer_pretty(out, &out_obj)
}
