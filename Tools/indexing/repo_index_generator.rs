// Tools/indexing/repo_index_generator.rs

use std::env;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Serialize)]
struct FileIndexEntry {
    path: String,
    language: String,
}

// Minimal mirror of schemas/knowledge_graph_systems.schema.json for Rust usage.
// This is kept simple and should track the JSON Schema shape closely.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeGraphIndex {
    pub version: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub node_id: String,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub display_name: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub engines: Vec<EngineKind>,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub related_nodes: Vec<String>,
    #[serde(default)]
    pub primary: bool,
    #[serde(default = "default_status")]
    pub status: NodeStatus,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    SystemNode,
    ToolNode,
    DocNode,
    PromptNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EngineKind {
    Unreal,
    Unity,
    Godot,
    Rust,
    Lua,
    MATLAB,
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NodeStatus {
    Planned,
    Prototype,
    InProgress,
    Complete,
    Deprecated,
}

fn default_status() -> NodeStatus {
    NodeStatus::Planned
}

#[derive(Debug)]
struct ValidationError {
    message: String,
}

fn main() -> anyhow::Result<()> {
    // Part 1: build a flat file index for quick lookup.
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
                eprintln!("[repo_index_generator] walkdir error: {err}");
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

    println!(
        "[repo_index_generator] Wrote index with {} entries to {}",
        entries.len(),
        out_path.display()
    );

    // Part 2: validate Build/knowledge_graph_systems.json against repo layout.
    validate_knowledge_graph(&repo_root);

    Ok(())
}

fn validate_knowledge_graph(repo_root: &Path) {
    use std::path::Path;

    let schema_path = Path::new("schemas/knowledge_graph_systems.schema.json");
    let index_path = Path::new("Build/knowledge_graph_systems.json");

    if !schema_path.exists() {
        eprintln!(
            "[repo_index_generator] WARNING: schema file not found at {}",
            schema_path.display()
        );
    }

    let index_data = match fs::read_to_string(index_path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!(
                "[repo_index_generator] ERROR: failed to read {}: {err}",
                index_path.display()
            );
            std::process::exit(1);
        }
    };

    let index: KnowledgeGraphIndex = match serde_json::from_str(&index_data) {
        Ok(idx) => idx,
        Err(err) => {
            eprintln!(
                "[repo_index_generator] ERROR: invalid JSON in {}: {err}",
                index_path.display()
            );
            std::process::exit(1);
        }
    };

    let mut errors: Vec<ValidationError> = Vec::new();

    validate_node_ids(&index, &mut errors);
    validate_file_paths(repo_root, &index, &mut errors);
    validate_related_nodes(&index, &mut errors);

    if errors.is_empty() {
        println!(
            "[repo_index_generator] Knowledge graph index is valid ({} nodes).",
            index.nodes.len()
        );
    } else {
        eprintln!(
            "[repo_index_generator] Knowledge graph index has {} validation error(s):",
            errors.len()
        );
        for (i, err) in errors.iter().enumerate() {
            eprintln!("  {}. {}", i + 1, err.message);
        }
        std::process::exit(1);
    }
}

fn validate_node_ids(index: &KnowledgeGraphIndex, errors: &mut Vec<ValidationError>) {
    use regex::Regex;

    let re = Regex::new(r"^[a-z0-9]+(\.[a-z0-9_]+)+$").expect("regex must compile");

    for node in &index.nodes {
        if !re.is_match(&node.node_id) {
            errors.push(ValidationError {
                message: format!(
                    "Node ID '{}' does not match required pattern.",
                    node.node_id
                ),
            });
        }
    }

    let mut seen = std::collections::HashSet::new();
    for node in &index.nodes {
        if !seen.insert(&node.node_id) {
            errors.push(ValidationError {
                message: format!("Duplicate node_id found: '{}'.", node.node_id),
            });
        }
    }
}

fn validate_file_paths(
    repo_root: &Path,
    index: &KnowledgeGraphIndex,
    errors: &mut Vec<ValidationError>,
) {
    for node in &index.nodes {
        for file in &node.files {
            let path = repo_root.join(file);
            if !path.exists() {
                errors.push(ValidationError {
                    message: format!(
                        "Node '{}' references missing file path '{}'.",
                        node.node_id, file
                    ),
                });
            }
        }
    }
}

fn validate_related_nodes(index: &KnowledgeGraphIndex, errors: &mut Vec<ValidationError>) {
    let known_ids: std::collections::HashSet<_> =
        index.nodes.iter().map(|n| n.node_id.as_str()).collect();

    for node in &index.nodes {
        for related in &node.related_nodes {
            if !known_ids.contains(related.as_str()) {
                errors.push(ValidationError {
                    message: format!(
                        "Node '{}' has related node '{}' that does not exist in the index.",
                        node.node_id, related
                    ),
                });
            }
        }
    }
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
