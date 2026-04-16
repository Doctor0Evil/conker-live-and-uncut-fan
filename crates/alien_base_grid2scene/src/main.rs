use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, ValueEnum};
use serde::Deserialize;

mod model;
mod pipeline;

use crate::model::{Entities, Grid, Tileset};

#[derive(Debug, Clone, ValueEnum)]
enum Engine {
    Unreal,
    Unity,
    Godot,
}

#[derive(Debug, Parser)]
#[command(
    name = "grid2scene",
    about = "Conker: Live & Uncut grid → scene generator (grid + entities → UE5 / Unity / Godot)."
)]
struct Cli {
    /// Map ID to process (as defined in the Map Manifest).
    #[arg(long)]
    map: Option<String>,

    /// Process all maps defined in the Map Manifest.
    #[arg(long)]
    all: bool,

    /// Target engine.
    #[arg(long, value_enum)]
    engine: Engine,

    /// Path to the Map Manifest JSON (map_manifest_v1).
    #[arg(long, default_value = "maps/multiplayer_map_manifest_v1.json")]
    manifest: PathBuf,

    /// Output directory for generated engine artifacts.
    #[arg(long, default_value = "build")]
    out_dir: PathBuf,

    /// Perform validation only (schema + tileset + tag checks); do not write any engine output.
    #[arg(long)]
    validate: bool,

    /// Perform a dry run (parse + validate + plan) but do not write any output files.
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Deserialize)]
struct MapEntry {
    id: String,
    name: String,
    grid_path: String,
    entities_path: String,
    #[serde(default)]
    tileset_paths: TilesetPaths,
    #[serde(default)]
    recommended_players: Option<u32>,
    #[serde(default)]
    supported_modes: Vec<String>,
    #[serde(default)]
    notes: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct TilesetPaths {
    #[serde(default)]
    unreal: Option<String>,
    #[serde(default)]
    unity: Option<String>,
    #[serde(default)]
    godot: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MapManifest {
    version: String,
    maps: Vec<MapEntry>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.map.is_none() && !cli.all {
        return Err(anyhow!(
            "You must provide either --map <id> or --all when invoking grid2scene."
        ));
    }

    let manifest_bytes = fs::read(&cli.manifest)
        .with_context(|| format!("Failed to read manifest {:?}", cli.manifest))?;
    let manifest: MapManifest =
        serde_json::from_slice(&manifest_bytes).context("Failed to parse Map Manifest JSON")?;

    if cli.all {
        for entry in &manifest.maps {
            process_map(entry, &cli)?;
        }
    } else if let Some(ref id) = cli.map {
        let entry = manifest
            .maps
            .iter()
            .find(|m| &m.id == id)
            .ok_or_else(|| anyhow!("Map id '{}' not found in manifest.", id))?;
        process_map(entry, &cli)?;
    }

    Ok(())
}

fn process_map(entry: &MapEntry, cli: &Cli) -> Result<()> {
    println!(
        "[grid2scene] Processing map '{}' ({}) for engine {:?}",
        entry.id, entry.name, cli.engine
    );

    let grid_path = PathBuf::from(&entry.grid_path);
    let entities_path = PathBuf::from(&entry.entities_path);

    let grid_bytes = fs::read(&grid_path)
        .with_context(|| format!("Failed to read grid file {:?}", grid_path))?;
    let entities_bytes = fs::read(&entities_path)
        .with_context(|| format!("Failed to read entities file {:?}", entities_path))?;

    let grid: Grid = serde_json::from_slice(&grid_bytes)
        .with_context(|| format!("Failed to parse grid JSON {:?}", grid_path))?;
    let entities: Entities = serde_json::from_slice(&entities_bytes)
        .with_context(|| format!("Failed to parse entities JSON {:?}", entities_path))?;

    let tileset_path = match cli.engine {
        Engine::Unreal => entry
            .tileset_paths
            .unreal
            .as_ref()
            .ok_or_else(|| anyhow!("No Unreal tileset path configured for map '{}'", entry.id))?,
        Engine::Unity => entry
            .tileset_paths
            .unity
            .as_ref()
            .ok_or_else(|| anyhow!("No Unity tileset path configured for map '{}'", entry.id))?,
        Engine::Godot => entry
            .tileset_paths
            .godot
            .as_ref()
            .ok_or_else(|| anyhow!("No Godot tileset path configured for map '{}'", entry.id))?,
    };

    let tileset_bytes = fs::read(tileset_path)
        .with_context(|| format!("Failed to read tileset file {}", tileset_path))?;
    let tileset: Tileset =
        serde_json::from_slice(&tileset_bytes).context("Failed to parse tileset JSON")?;

    // 1. Runtime validation against tileset and grid/entity relationships.
    if cli.validate || cli.dry_run {
        pipeline::validate_map(&grid, &entities, &tileset)
            .with_context(|| format!("Validation failed for map '{}'", entry.id))?;
        println!("[grid2scene] Validation OK for map '{}'", entry.id);

        if cli.dry_run {
            // If requested, print a summary instead of writing files.
            let summary = pipeline::summarize_map(&grid, &entities);
            println!("{}", summary);
            return Ok(());
        }

        if cli.validate {
            return Ok(());
        }
    }

    if cli.dry_run {
        // Dry-run without explicit --validate: still provide a summary.
        let summary = pipeline::summarize_map(&grid, &entities);
        println!("{}", summary);
        return Ok(());
    }

    // 2. Emit engine-specific artifacts.
    let out_dir = cli.out_dir.join(&entry.id);
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("Failed to create output directory {:?}", out_dir))?;

    match cli.engine {
        Engine::Unreal => pipeline::emit_unreal(&grid, &entities, &tileset, &out_dir)
            .with_context(|| "Failed to emit Unreal output")?,
        Engine::Unity => pipeline::emit_unity(&grid, &entities, &tileset, &out_dir)
            .with_context(|| "Failed to emit Unity output")?,
        Engine::Godot => pipeline::emit_godot(&grid, &entities, &tileset, &out_dir)
            .with_context(|| "Failed to emit Godot output")?,
    }

    println!(
        "[grid2scene] Finished map '{}' for engine {:?}, output at {:?}",
        entry.id, cli.engine, out_dir
    );

    Ok(())
}
