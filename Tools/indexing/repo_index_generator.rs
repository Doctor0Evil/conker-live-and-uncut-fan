use std::env;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::WalkDir;

#[derive(Debug, Serialize)]
struct FileIndexEntry {
    path: String,
    language: String,
}

fn main() -> anyhow::Result<()> {
    let repo_root = env::current_dir()?;
    let out_path = repo_root.join("Build").join("repo_index.json");

    let mut entries: Vec<FileIndexEntry> = Vec::new();

    for entry in WalkDir::new(&repo_root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_ignored_dir(e.path()))
    {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("walkdir error: {err}");
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if is_ignored_file(path) {
            continue;
        }

        let rel = pathdiff::diff_paths(path, &repo_root)
            .unwrap_or_else(|| PathBuf::from(path));

        let language = infer_language(path);
        entries.push(FileIndexEntry {
            path: rel.to_string_lossy().to_string(),
            language,
        });
    }

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(&out_path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &entries)?;

    println!("Wrote index with {} entries to {}", entries.len(), out_path.display());
    Ok(())
}

fn is_ignored_dir(path: &Path) -> bool {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        match name {
            ".git"
            | ".github"
            | "Build"
            | "Binaries"
            | "Intermediate"
            | ".vs"
            | ".idea"
            | "DerivedDataCache"
            | ".godot" => return true,
            _ => {}
        }

        // Ignore hidden dirs
        if name.starts_with('.') {
            return true;
        }
    }
    false
}

fn is_ignored_file(path: &Path) -> bool {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if name.starts_with('.') {
            return true;
        }
    }
    false
}

fn infer_language(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()).unwrap_or_default() {
        "h" | "hpp" | "hh" => "cpp-header".to_string(),
        "c" | "cpp" | "cc" | "cxx" => "cpp".to_string(),
        "cs" => "csharp".to_string(),
        "gd" => "godot-gdscript".to_string(),
        "lua" => "lua".to_string(),
        "rs" => "rust".to_string(),
        "m" | "mlx" => "matlab".to_string(),
        "toml" => "toml".to_string(),
        "json" => "json".to_string(),
        "md" => "markdown".to_string(),
        "txt" => "text".to_string(),
        "uproject" => "unreal-uproject".to_string(),
        _ => "unknown".to_string(),
    }
}
