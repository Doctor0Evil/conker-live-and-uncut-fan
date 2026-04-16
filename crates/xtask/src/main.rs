use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::Command;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Project xtasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate Unreal hub scene for Alien Base
    AlienBaseHubUnreal {
        #[arg(long, default_value = "data/alien_base_hub_grid_v1.json")]
        grid: PathBuf,
        #[arg(long, default_value = "data/alien_base_hub_entities_v1.json")]
        entities: PathBuf,
        #[arg(long, default_value = "tilesets/unreal_alien_base_tiles_v1.json")]
        tileset: PathBuf,
        #[arg(long, default_value = "build/unreal/AlienBase_Hub.json")]
        out: PathBuf,
    },
    /// Generate Unity hub scene
    AlienBaseHubUnity {
        #[arg(long, default_value = "data/alien_base_hub_grid_v1.json")]
        grid: PathBuf,
        #[arg(long, default_value = "data/alien_base_hub_entities_v1.json")]
        entities: PathBuf,
        #[arg(long, default_value = "tilesets/unity_alien_base_tiles_v1.json")]
        tileset: PathBuf,
        #[arg(long, default_value = "build/unity/AlienBase_Hub.json")]
        out: PathBuf,
    },
    /// Generate Godot hub scene
    AlienBaseHubGodot {
        #[arg(long, default_value = "data/alien_base_hub_grid_v1.json")]
        grid: PathBuf,
        #[arg(long, default_value = "data/alien_base_hub_entities_v1.json")]
        entities: PathBuf,
        #[arg(long, default_value = "tilesets/godot_alien_base_tiles_v1.json")]
        tileset: PathBuf,
        #[arg(long, default_value = "build/godot/AlienBase_Hub.tscn")]
        out: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::AlienBaseHubUnreal { grid, entities, tileset, out } => {
            run_grid2scene("unreal", &grid, &entities, &tileset, &out)?;
        }
        Commands::AlienBaseHubUnity { grid, entities, tileset, out } => {
            run_grid2scene("unity", &grid, &entities, &tileset, &out)?;
        }
        Commands::AlienBaseHubGodot { grid, entities, tileset, out } => {
            run_grid2scene("godot", &grid, &entities, &tileset, &out)?;
        }
    }

    Ok(())
}

fn run_grid2scene(
    engine: &str,
    grid: &PathBuf,
    entities: &PathBuf,
    tileset: &PathBuf,
    out: &PathBuf,
) -> Result<()> {
    let status = Command::new("cargo")
        .args([
            "run",
            "--package",
            "alien_base_grid2scene",
            "--",
            "--input",
            &grid.to_string_lossy(),
            "--entities",
            &entities.to_string_lossy(),
            "--tileset",
            &tileset.to_string_lossy(),
            "--engine",
            engine,
            "--out",
            &out.to_string_lossy(),
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("grid2scene failed for engine {}", engine);
    }
    Ok(())
}
