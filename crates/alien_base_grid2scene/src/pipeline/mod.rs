use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{anyhow, Result};

use crate::model::{Entities, Grid, Tileset};

pub fn validate_map(grid: &Grid, entities: &Entities, tileset: &Tileset) -> Result<()> {
    validate_tiles_against_tileset(grid, tileset)?;
    validate_entities_against_grid_and_tileset(grid, entities, tileset)?;
    Ok(())
}

fn validate_tiles_against_tileset(grid: &Grid, tileset: &Tileset) -> Result<()> {
    let mut known_tiles: HashSet<&str> = HashSet::new();
    for m in &tileset.tile_mappings {
        known_tiles.insert(m.tile_type.as_str());
    }

    let mut unknown: HashSet<String> = HashSet::new();
    for cell in &grid.cells {
        if !known_tiles.contains(cell.tile_type.as_str()) {
            unknown.insert(cell.tile_type.clone());
        }
    }

    if !unknown.is_empty() {
        return Err(anyhow!(
            "Grid references tile_type(s) not present in tileset: {:?}",
            unknown
        ));
    }

    Ok(())
}

fn validate_entities_against_grid_and_tileset(
    grid: &Grid,
    entities: &Entities,
    tileset: &Tileset,
) -> Result<()> {
    let mut grid_roles: HashSet<&str> = HashSet::new();
    for c in &grid.cells {
        for tag in &c.role_tags {
            grid_roles.insert(tag.as_str());
        }
    }

    // Build a quick lookup for (col,row) presence.
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    for c in &grid.cells {
        occupied.insert((c.col, c.row));
    }

    // Collect all entity-related role_tags so we can report tags that don't appear in grid cells.
    let mut entity_tags: HashSet<&str> = HashSet::new();

    for sp in &entities.spawn_points {
        if !occupied.contains(&(sp.col, sp.row)) {
            return Err(anyhow!(
                "SpawnPoint '{}' refers to empty cell ({}, {})",
                sp.id,
                sp.col,
                sp.row
            ));
        }
        for tag in &sp.role_tags {
            entity_tags.insert(tag.as_str());
        }
    }

    for w in &entities.weapon_pickups {
        if !occupied.contains(&(w.col, w.row)) {
            return Err(anyhow!(
                "WeaponPickup '{}' refers to empty cell ({}, {})",
                w.id,
                w.col,
                w.row
            ));
        }
        for tag in &w.role_tags {
            entity_tags.insert(tag.as_str());
        }
    }

    for o in &entities.objective_pickups {
        if !occupied.contains(&(o.col, o.row)) {
            return Err(anyhow!(
                "ObjectivePickup '{}' refers to empty cell ({}, {})",
                o.id,
                o.col,
                o.row
            ));
        }
        for tag in &o.role_tags {
            entity_tags.insert(tag.as_str());
        }
    }

    for o in &entities.objectives {
        if !occupied.contains(&(o.col, o.row)) {
            return Err(anyhow!(
                "Objective '{}' refers to empty cell ({}, {})",
                o.id,
                o.col,
                o.row
            ));
        }
        for tag in &o.role_tags {
            entity_tags.insert(tag.as_str());
        }
    }

    for h in &entities.hazard_volumes {
        for tag in &h.role_tags {
            entity_tags.insert(tag.as_str());
        }
    }

    for n in &entities.npc_spawners {
        for tag in &n.role_tags {
            entity_tags.insert(tag.as_str());
        }
    }

    // Check that every entity role_tag appears somewhere in the grid's role_tags vocabulary.
    let mut unknown_tags: HashSet<String> = HashSet::new();
    for tag in entity_tags {
        if !grid_roles.contains(tag) {
            unknown_tags.insert(tag.to_string());
        }
    }

    if !unknown_tags.is_empty() {
        return Err(anyhow!(
            "Entity role_tags not present in any grid cell: {:?}",
            unknown_tags
        ));
    }

    // Optionally: verify that weapon_type / hazard_type / objective_type names exist in tileset entity mappings.
    let em = &tileset.entity_mappings;

    // weapon pickup classes.
    let weapon_map: HashMap<&str, &str> = em.weapon_pickup.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let mut unknown_weapons: HashSet<String> = HashSet::new();
    for w in &entities.weapon_pickups {
        if !weapon_map.contains_key(w.weapon_type.as_str()) {
            unknown_weapons.insert(w.weapon_type.clone());
        }
    }
    if !unknown_weapons.is_empty() {
        return Err(anyhow!(
            "weapon_pickups reference weapon_type(s) not present in tileset entity_mappings.weapon_pickup: {:?}",
            unknown_weapons
        ));
    }

    // hazard types.
    let hazard_map: HashMap<&str, &str> = em.hazard_volume.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let mut unknown_hazards: HashSet<String> = HashSet::new();
    for h in &entities.hazard_volumes {
        if !hazard_map.contains_key(h.hazard_type.as_str()) {
            unknown_hazards.insert(h.hazard_type.clone());
        }
    }
    if !unknown_hazards.is_empty() {
        return Err(anyhow!(
            "hazard_volumes reference hazard_type(s) not present in tileset entity_mappings.hazard_volume: {:?}",
            unknown_hazards
        ));
    }

    // objectives.
    let objective_map: HashMap<&str, &str> =
        em.objective.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let mut unknown_objectives: HashSet<String> = HashSet::new();
    for o in &entities.objectives {
        if !objective_map.contains_key(o.objective_type.as_str()) {
            unknown_objectives.insert(o.objective_type.clone());
        }
    }
    if !unknown_objectives.is_empty() {
        return Err(anyhow!(
            "objectives reference objective_type(s) not present in tileset entity_mappings.objective: {:?}",
            unknown_objectives
        ));
    }

    Ok(())
}

// Stubs for engine emitters: you can keep or expand your existing implementations.
pub fn emit_unreal(grid: &Grid, entities: &Entities, tileset: &Tileset, out_dir: &Path) -> Result<()> {
    super::emitter_unreal::emit(grid, entities, tileset, out_dir)
}

pub fn emit_unity(grid: &Grid, entities: &Entities, tileset: &Tileset, out_dir: &Path) -> Result<()> {
    super::emitter_unity::emit(grid, entities, tileset, out_dir)
}

pub fn emit_godot(grid: &Grid, entities: &Entities, tileset: &Tileset, out_dir: &Path) -> Result<()> {
    super::emitter_godot::emit(grid, entities, tileset, out_dir)
}
