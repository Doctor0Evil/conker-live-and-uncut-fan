use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GridSize {
    pub cols: i32,
    pub rows: i32,
}

#[derive(Debug, Deserialize)]
pub struct GridCell {
    pub col: i32,
    pub row: i32,
    pub tile_type: String,
    #[serde(default = "default_walkable")]
    pub walkable: bool,
    #[serde(default)]
    pub role_tags: Vec<String>,
}

fn default_walkable() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct Grid {
    pub version: String,
    pub grid_size: GridSize,
    pub cell_size: f32,
    pub y_level: f32,
    #[serde(default)]
    pub origin: Origin,
    pub cells: Vec<GridCell>,
    #[serde(default)]
    pub level_name: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Origin {
    pub x: f32,
    pub z: f32,
}

impl Grid {
    pub fn center_indices(&self) -> (f32, f32) {
        let cx = (self.grid_size.cols - 1) as f32 / 2.0;
        let cz = (self.grid_size.rows - 1) as f32 / 2.0;
        (cx, cz)
    }

    pub fn cell_to_world(&self, col: i32, row: i32, y_offset: f32) -> (f32, f32, f32) {
        let (cx, cz) = self.center_indices();
        let wx = self.origin.x + ((col as f32 - cx) * self.cell_size);
        let wz = self.origin.z + ((row as f32 - cz) * self.cell_size);
        let wy = self.y_level + y_offset;
        (wx, wy, wz)
    }
}
