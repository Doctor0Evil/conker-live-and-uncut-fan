use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TileMapping {
    pub tile_type: String,
    pub asset_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EntityMappings {
    pub spawn: SpawnMapping,
    pub weapon_pickup: HashMap<String, String>,
    pub hazard_volume: HashMap<String, String>,
    pub objective: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct SpawnMapping {
    pub prefab_or_class: String,
}

#[derive(Debug, Deserialize)]
pub struct Tileset {
    pub version: String,
    pub engine: String,
    pub tile_mappings: Vec<TileMapping>,
    pub entity_mappings: EntityMappings,
}

impl Tileset {
    pub fn tile_asset(&self, tile_type: &str) -> Option<&str> {
        self.tile_mappings
            .iter()
            .find(|m| m.tile_type == tile_type)
            .map(|m| m.asset_id.as_str())
    }
}
