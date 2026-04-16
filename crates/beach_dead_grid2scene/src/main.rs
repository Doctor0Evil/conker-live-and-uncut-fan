use clap::{Parser, ValueEnum};
use std::{fs::File, io::BufWriter, path::PathBuf};

mod model;
mod pipeline;

use model::{entities::Entities, grid::Grid, tileset::Tileset};
use pipeline::loader::{load_entities, load_grid, load_tileset};
use pipeline::{emitter_unreal, emitter_unity, emitter_godot};

#[derive(Debug, Clone, ValueEnum)]
enum Engine {
    Unreal,
    Unity,
    Godot,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Beach Dead grid → scene generator")]
struct Cli {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    entities: PathBuf,
    #[arg(long)]
    tileset: PathBuf,
    #[arg(long)]
    engine: Engine,
    #[arg(long)]
    out: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let grid: Grid = load_grid(&cli.input)?;
    let entities: Entities = load_entities(&cli.entities)?;
    let tileset: Tileset = load_tileset(&cli.tileset)?;

    let file = File::create(&cli.out)?;
    let mut writer = BufWriter::new(file);

    match cli.engine {
        Engine::Unreal => emitter_unreal::emit_unreal(&grid, &entities, &tileset, &mut writer)?,
        Engine::Unity => emitter_unity::emit_unity(&grid, &entities, &tileset, &mut writer)?,
        Engine::Godot => emitter_godot::emit_godot(&grid, &entities, &tileset, &mut writer)?,
    }

    Ok(())
}
