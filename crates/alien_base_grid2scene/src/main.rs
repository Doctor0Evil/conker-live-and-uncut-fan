use clap::{Parser, ValueEnum};
use std::{fs::File, io::BufWriter, path::PathBuf};

mod model;
mod pipeline;

use model::{entities::Entities, grid::Grid, tileset::Tileset};
use pipeline::loader::{load_entities, load_grid, load_tileset};
use pipeline::{emitter_godot, emitter_unity, emitter_unreal};

#[derive(Debug, Clone, ValueEnum)]
enum Engine {
    Unreal,
    Unity,
    Godot,
}

#[derive(Debug, Clone, ValueEnum)]
enum MapId {
    M01BeachDead,
    M02TheHeist,
    M03Fortress,
    M04AlienBase,
    M05RaptorTemple,
    M06TmsSpamono,
}

struct MapPaths {
    grid: PathBuf,
    entities: PathBuf,
    tileset_unreal: PathBuf,
    tileset_unity: PathBuf,
    tileset_godot: PathBuf,
}

fn resolve_paths(map: &MapId) -> MapPaths {
    let base = PathBuf::from("data");
    let tilesets_base = PathBuf::from("tilesets");

    let (grid_name, entities_name, tileset_stub) = match map {
        MapId::M01BeachDead => (
            "beach_dead_mid_grid_v1.json",
            "beach_dead_mid_entities_v1.json",
            "beach_dead_tiles_v1.json",
        ),
        MapId::M02TheHeist => (
            "the_heist_hub_grid_v1.json",
            "the_heist_hub_entities_v1.json",
            "heist_tiles_v1.json",
        ),
        MapId::M03Fortress => (
            "fortress_valley_grid_v1.json",
            "fortress_valley_entities_v1.json",
            "fortress_tiles_v1.json",
        ),
        MapId::M04AlienBase => (
            "alien_base_hub_grid_v1.json",
            "alien_base_hub_entities_v1.json",
            "alien_base_tiles_v1.json",
        ),
        MapId::M05RaptorTemple => (
            "raptor_temple_hub_grid_v1.json",
            "raptor_temple_hub_entities_v1.json",
            "raptor_temple_tiles_v1.json",
        ),
        MapId::M06TmsSpamono => (
            "tms_spamono_mid_grid_v1.json",
            "tms_spamono_mid_entities_v1.json",
            "tms_spamono_tiles_v1.json",
        ),
    };

    let grid = base.join(grid_name);
    let entities = base.join(entities_name);

    let tileset_unreal = tilesets_base.join("unreal").join(tileset_stub);
    let tileset_unity = tilesets_base.join("unity").join(tileset_stub);
    let tileset_godot = tilesets_base.join("godot").join(tileset_stub);

    MapPaths {
        grid,
        entities,
        tileset_unreal,
        tileset_unity,
        tileset_godot,
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Conker: Live & Uncut grid → scene generator")]
struct Cli {
    /// Target engine (unreal/unity/godot)
    #[arg(long, value_enum)]
    engine: Engine,

    /// Map id (M01BeachDead..M06TmsSpamono)
    #[arg(long, value_enum)]
    map: MapId,

    /// Optional override for grid JSON; if not set, defaults are used based on map
    #[arg(long)]
    input: Option<PathBuf>,

    /// Optional override for entities JSON; if not set, defaults are used based on map
    #[arg(long)]
    entities: Option<PathBuf>,

    /// Optional override for tileset JSON; if not set, defaults are used based on map + engine
    #[arg(long)]
    tileset: Option<PathBuf>,

    /// Output file path (engine-specific layout)
    #[arg(long)]
    out: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let defaults = resolve_paths(&cli.map);

    let grid_path = cli.input.unwrap_or(defaults.grid);
    let entities_path = cli.entities.unwrap_or(defaults.entities);

    let tileset_path = if let Some(ts) = cli.tileset {
        ts
    } else {
        match cli.engine {
            Engine::Unreal => defaults.tileset_unreal,
            Engine::Unity => defaults.tileset_unity,
            Engine::Godot => defaults.tileset_godot,
        }
    };

    let grid: Grid = load_grid(&grid_path)?;
    let entities: Entities = load_entities(&entities_path)?;
    let tileset: Tileset = load_tileset(&tileset_path)?;

    let file = File::create(&cli.out)?;
    let mut writer = BufWriter::new(file);

    match cli.engine {
        Engine::Unreal => {
            emitter_unreal::emit_unreal(&grid, &entities, &tileset, &mut writer)?;
        }
        Engine::Unity => {
            emitter_unity::emit_unity(&grid, &entities, &tileset, &mut writer)?;
        }
        Engine::Godot => {
            emitter_godot::emit_godot(&grid, &entities, &tileset, &mut writer)?;
        }
    }

    Ok(())
}
