use crate::model::{entities::Entities, grid::Grid, tileset::Tileset};
use serde_json::from_reader;
use std::{fs::File, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn load_grid<P: AsRef<Path>>(path: P) -> Result<Grid, LoadError> {
    let file = File::open(path)?;
    Ok(from_reader(file)?)
}

pub fn load_entities<P: AsRef<Path>>(path: P) -> Result<Entities, LoadError> {
    let file = File::open(path)?;
    Ok(from_reader(file)?)
}

pub fn load_tileset<P: AsRef<Path>>(path: P) -> Result<Tileset, LoadError> {
    let file = File::open(path)?;
    Ok(from_reader(file)?)
}
