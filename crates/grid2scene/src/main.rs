use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

mod model;
mod schema;
mod emitter;
mod validator;

use model::{Entities, Grid};
use schema::hazard::HazardProfile;

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
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Emit scene code for a map
    Emit {
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

        /// Mode profile JSON file for filtering entities based on role tags.
        #[arg(long)]
        mode_profile: Option<PathBuf>,

        /// Dry run: output consolidated JSON instead of engine-specific files.
        #[arg(long)]
        dry_run_json: bool,
    },
    /// Validate hazard profile JSON files
    ValidateProfiles {
        /// Directory containing hazard profile JSON files (recursive scan).
        #[arg(long)]
        profiles_dir: PathBuf,
    },
    /// Watch mode: automatically re-emit when source files change
    Watch {
        /// Map ID to watch.
        #[arg(long)]
        map: String,

        /// Target engine.
        #[arg(long, value_enum)]
        engine: Engine,

        /// Path to the Map Manifest JSON.
        #[arg(long, default_value = "maps/multiplayer_map_manifest_v1.json")]
        manifest: PathBuf,

        /// Output directory for generated engine artifacts.
        #[arg(long, default_value = "build")]
        out_dir: PathBuf,
    },
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

/// Mode profile for filtering entities based on role tags.
#[derive(Debug, Deserialize)]
struct ModeProfile {
    /// Objective types/roles that should be enabled.
    #[serde(default)]
    enabled_objective_role_tags: Vec<String>,
    /// Hazard types/roles that should be enabled.
    #[serde(default)]
    enabled_hazard_role_tags: Vec<String>,
}

impl ModeProfile {
    fn filter_entities(&self, entities: &mut Entities) {
        // Filter objectives based on enabled_objective_role_tags
        if !self.enabled_objective_role_tags.is_empty() {
            entities.objectives.retain(|obj| {
                obj.role_tags.iter().any(|tag| {
                    self.enabled_objective_role_tags.iter().any(|enabled| tag.contains(enabled))
                })
            });
        }

        // Filter hazard volumes based on enabled_hazard_role_tags
        if !self.enabled_hazard_role_tags.is_empty() {
            entities.hazard_volumes.retain(|hv| {
                hv.role_tags.iter().any(|tag| {
                    self.enabled_hazard_role_tags.iter().any(|enabled| tag.contains(enabled))
                })
            });
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Emit {
            map,
            all,
            engine,
            manifest,
            out_dir,
            mode_profile,
            dry_run_json,
        } => {
            if map.is_none() && !all {
                return Err(anyhow!(
                    "You must provide either --map <id> or --all when invoking grid2scene emit."
                ));
            }

            let manifest_bytes = fs::read(&manifest)
                .with_context(|| format!("Failed to read manifest {:?}", manifest))?;
            let manifest: MapManifest =
                serde_json::from_slice(&manifest_bytes).context("Failed to parse Map Manifest JSON")?;

            // Load mode profile if provided
            let mode_profile = if let Some(ref mp_path) = mode_profile {
                let mp_bytes = fs::read(mp_path)
                    .with_context(|| format!("Failed to read mode profile {:?}", mp_path))?;
                let mp: ModeProfile = serde_json::from_slice(&mp_bytes)
                    .context("Failed to parse mode profile JSON")?;
                Some(mp)
            } else {
                None
            };

            if all {
                for entry in &manifest.maps {
                    process_map(entry, &engine, &out_dir, &mode_profile, dry_run_json)?;
                }
            } else if let Some(ref id) = map {
                let entry = manifest
                    .maps
                    .iter()
                    .find(|m| &m.id == id)
                    .ok_or_else(|| anyhow!("Map id '{}' not found in manifest.", id))?;
                process_map(entry, &engine, &out_dir, &mode_profile, dry_run_json)?;
            }
        }
        Commands::ValidateProfiles { profiles_dir } => {
            validate_profiles(&profiles_dir)?;
        }
        Commands::Watch {
            map,
            engine,
            manifest,
            out_dir,
        } => {
            watch_mode(&map, &engine, &manifest, &out_dir)?;
        }
    }

    Ok(())
}

fn process_map(
    entry: &MapEntry,
    engine: &Engine,
    out_dir: &Path,
    mode_profile: &Option<ModeProfile>,
    dry_run_json: bool,
) -> Result<()> {
    println!(
        "[grid2scene] Processing map '{}' ({}) for engine {:?}",
        entry.id, entry.name, engine
    );

    let grid_path = PathBuf::from(&entry.grid_path);
    let entities_path = PathBuf::from(&entry.entities_path);

    let grid_bytes = fs::read(&grid_path)
        .with_context(|| format!("Failed to read grid file {:?}", grid_path))?;
    let entities_bytes = fs::read(&entities_path)
        .with_context(|| format!("Failed to read entities file {:?}", entities_path))?;

    let mut grid: Grid = serde_json::from_slice(&grid_bytes)
        .with_context(|| format!("Failed to parse grid JSON {:?}", grid_path))?;
    let mut entities: Entities = serde_json::from_slice(&entities_bytes)
        .with_context(|| format!("Failed to parse entities JSON {:?}", entities_path))?;

    // Apply mode profile filtering if provided
    if let Some(ref mp) = mode_profile {
        println!("[grid2scene] Applying mode profile filter...");
        mp.filter_entities(&mut entities);
    }

    // Run cross-reference validation
    let warnings = validator::check_baby_dino_feeder(&grid, &entities);
    for warning in &warnings {
        println!("{}", warning);
    }

    if dry_run_json {
        // Output consolidated JSON for debugging
        let consolidated = ConsolidatedOutput {
            map_id: entry.id.clone(),
            map_name: entry.name.clone(),
            grid,
            entities,
            mode_filtering_applied: mode_profile.is_some(),
        };
        
        let json = serde_json::to_string_pretty(&consolidated)
            .context("Failed to serialize consolidated output")?;
        
        let output_path = out_dir.join(format!("{}_consolidated.json", entry.id));
        fs::write(&output_path, &json)
            .with_context(|| format!("Failed to write consolidated JSON to {:?}", output_path))?;
        
        println!("[grid2scene] Dry-run JSON written to {:?}", output_path);
        return Ok(());
    }

    // TODO: Implement actual engine emission here
    let output_path = out_dir.join(&entry.id);
    fs::create_dir_all(&output_path)
        .with_context(|| format!("Failed to create output directory {:?}", output_path))?;

    println!(
        "[grid2scene] Finished map '{}' for engine {:?}, output at {:?}",
        entry.id, engine, output_path
    );

    Ok(())
}

#[derive(Debug, Serialize)]
struct ConsolidatedOutput {
    map_id: String,
    map_name: String,
    grid: Grid,
    entities: Entities,
    mode_filtering_applied: bool,
}

fn validate_profiles(profiles_dir: &Path) -> Result<()> {
    if !profiles_dir.exists() {
        return Err(anyhow!("Profiles directory does not exist: {:?}", profiles_dir));
    }

    let mut errors = Vec::new();
    let mut validated_count = 0;

    // Recursively scan for JSON files
    for entry in walkdir::WalkDir::new(profiles_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                errors.push(format!("{}: Failed to read file: {}", path.display(), e));
                continue;
            }
        };

        // Try to deserialize as HazardProfile
        match serde_json::from_str::<HazardProfile>(&content) {
            Ok(profile) => {
                if let Err(e) = profile.validate() {
                    errors.push(format!("{}: Validation error: {}", path.display(), e));
                } else {
                    validated_count += 1;
                    println!("[validate] OK: {} (profile: {})", path.display(), profile.id);
                }
            }
            Err(e) => {
                // Try to get line number from error
                let line = e.line().unwrap_or(0);
                errors.push(format!("{}:{}: Parse error: {}", path.display(), line, e));
            }
        }
    }

    println!("\n[validate-profiles] Validated {} profiles", validated_count);

    if !errors.is_empty() {
        eprintln!("\n[validate-profiles] Errors found:");
        for err in &errors {
            eprintln!("  {}", err);
        }
        return Err(anyhow!("{} validation error(s) found", errors.len()));
    }

    println!("[validate-profiles] All profiles valid!");
    Ok(())
}

fn watch_mode(map_id: &str, engine: &Engine, manifest_path: &Path, out_dir: &Path) -> Result<()> {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

    println!("[watch] Starting watch mode for map '{}' (engine: {:?})", map_id, engine);

    // Load manifest to get file paths
    let manifest_bytes = fs::read(manifest_path)
        .with_context(|| format!("Failed to read manifest {:?}", manifest_path))?;
    let manifest: MapManifest =
        serde_json::from_slice(&manifest_bytes).context("Failed to parse Map Manifest JSON")?;

    let entry = manifest
        .maps
        .iter()
        .find(|m| &m.id == map_id)
        .ok_or_else(|| anyhow!("Map id '{}' not found in manifest.", map_id))?;

    // Collect files to watch
    let mut watch_paths = vec![
        PathBuf::from(&entry.grid_path),
        PathBuf::from(&entry.entities_path),
    ];

    // Add tileset paths
    if let Some(ref p) = entry.tileset_paths.unreal {
        watch_paths.push(PathBuf::from(p));
    }
    if let Some(ref p) = entry.tileset_paths.unity {
        watch_paths.push(PathBuf::from(p));
    }
    if let Some(ref p) = entry.tileset_paths.godot {
        watch_paths.push(PathBuf::from(p));
    }

    // Create channel for file events
    let (tx, rx) = std::sync::mpsc::channel();

    // Create watcher
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default(),
    )?;

    // Watch all relevant paths
    for path in &watch_paths {
        if path.exists() {
            watcher.watch(path, RecursiveMode::NonRecursive)?;
            println!("[watch] Watching: {:?}", path);
        }
    }

    // Initial emit
    println!("[watch] Performing initial emit...");
    if let Err(e) = process_map(entry, engine, out_dir, &None, false) {
        eprintln!("[watch] Initial emit failed: {}", e);
    }

    println!("[watch] Monitoring for changes. Press Ctrl+C to stop.");

    // Event loop
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                
                for path in &event.paths {
                    println!("[{}] File changed: {:?}", timestamp, path);
                }

                // Debounce: wait a bit before re-emitting
                std::thread::sleep(Duration::from_millis(250));

                println!("[{}] Re-emitting scene...", timestamp);
                if let Err(e) = process_map(entry, engine, out_dir, &None, false) {
                    eprintln!("[{}] Emit failed: {}", timestamp, e);
                } else {
                    println!("[{}] Emit complete!", timestamp);
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // Continue waiting
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    Ok(())
}
