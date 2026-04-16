use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpawnPoint {
    pub id: String,
    pub zone: String,
    pub col: i32,
    pub row: i32,
    #[serde(default)]
    pub y_offset: f32,
}

#[derive(Debug, Deserialize)]
pub struct WeaponPickup {
    pub id: String,
    pub r#type: String,
    pub col: i32,
    pub row: i32,
    #[serde(default)]
    pub y_offset: f32,
}

#[derive(Debug, Deserialize)]
pub struct HazardVolume {
    pub id: String,
    pub r#type: String,
    pub center_col: i32,
    pub center_row: i32,
    pub radius_cells: i32,
    #[serde(default)]
    pub y_min_offset: f32,
    #[serde(default)]
    pub y_max_offset: f32,
}

#[derive(Debug, Deserialize)]
pub struct Objective {
    pub id: String,
    pub r#type: String,
    pub col: i32,
    pub row: i32,
    #[serde(default)]
    pub y_offset: f32,
}

#[derive(Debug, Deserialize)]
pub struct Entities {
    #[serde(default)]
    pub spawn_points: Vec<SpawnPoint>,
    #[serde(default)]
    pub weapon_pickups: Vec<WeaponPickup>,
    #[serde(default)]
    pub hazard_volumes: Vec<HazardVolume>,
    #[serde(default)]
    pub objectives: Vec<Objective>,
}
