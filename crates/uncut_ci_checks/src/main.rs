use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

const WEAPON_DOC_PATH: &str = "docs/systems/WeaponStats_Uncut.md";
const WEAPON_JSON_PATH: &str = "data/weapons/weaponstatsv1.json";

const ASID_DOC_PATH: &str = "docs/systems/AnimationStateRegistry.md";

// Engine mapping paths – adjust to match your repo layout
const UE_ASID_MAPPING_PATH: &str = "Config/ASIDMappings.ini";
const UNITY_ASID_MAPPING_PATH: &str = "Assets/Config/ASIDMappings.json";
const GODOT_ASID_MAPPING_PATH: &str = "godot/Systems/ASID/ASIDMappings.gd";

#[derive(Debug, Error)]
enum CiError {
    #[error("IO error while reading {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("JSON parse error in {path}: {source}")]
    Json {
        path: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("Validation failed: {0}")]
    Validation(String),
}

fn read_to_string(path: &str) -> Result<String, CiError> {
    fs::read_to_string(path).map_err(|e| CiError::Io {
        path: path.to_string(),
        source: e,
    })
}

#[derive(Debug, Deserialize)]
struct WeaponStatsFile {
    version: Option<String>,
    schemaversion: Option<String>,
    weapons: Vec<WeaponJsonEntry>,
}

#[derive(Debug, Deserialize)]
struct WeaponJsonEntry {
    id: String,

    #[serde(default)]
    is_heavy_carry: bool,

    // We ignore other fields for this CI check.
}

fn parse_weapon_json(path: &str) -> Result<Vec<WeaponJsonEntry>, CiError> {
    let text = read_to_string(path)?;
    let file: WeaponStatsFile = serde_json::from_str(&text).map_err(|e| CiError::Json {
        path: path.to_string(),
        source: e,
    })?;
    Ok(file.weapons)
}

/// Very lightweight parser for WeaponStats_Uncut.md.
/// Looks for lines like:
///   - **ID:** `Bazooka`
///   - **One-Line Description:** ...
lazy_static! {
    static ref ID_LINE_RE: Regex = Regex::new(r"^- \*\*ID:\*\* `([^`]+)`").unwrap();
    static ref HEAVY_NOTE_RE: Regex =
        Regex::new(r"(?i)heavy carry|no jump|ASID050").unwrap();
}

#[derive(Debug)]
struct WeaponDocEntry {
    id: String,
    mentions_heavy: bool,
}

fn parse_weapon_doc(path: &str) -> Result<Vec<WeaponDocEntry>, CiError> {
    let text = read_to_string(path)?;
    let mut entries = Vec::new();

    let mut current_id: Option<String> = None;
    let mut mentions_heavy = false;

    for line in text.lines() {
        if let Some(caps) = ID_LINE_RE.captures(line.trim()) {
            // Flush previous
            if let Some(id) = current_id.take() {
                entries.push(WeaponDocEntry {
                    id,
                    mentions_heavy,
                });
                mentions_heavy = false;
            }
            let id = caps.get(1).unwrap().as_str().to_string();
            current_id = Some(id);
        } else if current_id.is_some() {
            if HEAVY_NOTE_RE.is_match(line) {
                mentions_heavy = true;
            }
        }
    }

    if let Some(id) = current_id {
        entries.push(WeaponDocEntry {
            id,
            mentions_heavy,
        });
    }

    Ok(entries)
}

fn validate_weapons() -> Result<(), CiError> {
    if !Path::new(WEAPON_JSON_PATH).exists() && !Path::new(WEAPON_DOC_PATH).exists() {
        // Nothing to validate; treat as success so early repo clones don't fail.
        return Ok(());
    }

    let json_entries = parse_weapon_json(WEAPON_JSON_PATH)?;
    let doc_entries = parse_weapon_doc(WEAPON_DOC_PATH)?;

    let mut json_by_id: BTreeMap<String, WeaponJsonEntry> = BTreeMap::new();
    for w in json_entries {
        json_by_id.insert(w.id.clone(), w);
    }

    let mut doc_by_id: BTreeMap<String, WeaponDocEntry> = BTreeMap::new();
    for d in doc_entries {
        doc_by_id.insert(d.id.clone(), d);
    }

    let json_ids: BTreeSet<_> = json_by_id.keys().cloned().collect();
    let doc_ids: BTreeSet<_> = doc_by_id.keys().cloned().collect();

    let only_in_json: Vec<_> = json_ids.difference(&doc_ids).cloned().collect();
    let only_in_doc: Vec<_> = doc_ids.difference(&json_ids).cloned().collect();

    let mut errors = Vec::new();

    if !only_in_json.is_empty() {
        errors.push(format!(
            "Weapons present in JSON but missing from {}: {:?}",
            WEAPON_DOC_PATH, only_in_json
        ));
    }

    if !only_in_doc.is_empty() {
        errors.push(format!(
            "Weapons present in {} but missing from JSON: {:?}",
            WEAPON_DOC_PATH, only_in_doc
        ));
    }

    // Check heavy-carry notes vs is_heavy_carry bool.
    for (id, json_entry) in &json_by_id {
        if let Some(doc_entry) = doc_by_id.get(id) {
            if json_entry.is_heavy_carry && !doc_entry.mentions_heavy {
                errors.push(format!(
                    "Weapon '{}' is_heavy_carry=true in JSON but {} does not mention heavy carry/no jump/ASID050.",
                    id, WEAPON_DOC_PATH
                ));
            }
            if !json_entry.is_heavy_carry && doc_entry.mentions_heavy {
                errors.push(format!(
                    "Weapon '{}' is_heavy_carry=false in JSON but {} implies heavy carry.",
                    id, WEAPON_DOC_PATH
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CiError::Validation(errors.join("\n")))
    }
}

/// Parse ASID codes from the registry markdown.
/// It looks for lines like:
///   ### ASID050 – Heavy Carry Walk
lazy_static! {
    static ref ASID_HEADING_RE: Regex =
        Regex::new(r"^###\s+ASID(\d{3})\s").unwrap();
}

/// Minimal Unreal INI parser for ASID mappings; it just collects any numeric 3-digit codes.
lazy_static! {
    static ref ASID_CODE_RE: Regex = Regex::new(r"\bASID(\d{3})\b").unwrap();
}

/// Unity JSON mapping shape:
/// {
///   "asids": {
///       "050": "HeavyCarry",
///       "400": "ChainsawExecV"
///   }
/// }
#[derive(Debug, Deserialize)]
struct UnityAsidMappings {
    asids: BTreeMap<String, String>,
}

/// Godot mapping file: we accept any "050" style token in the file.
fn parse_asid_doc(path: &str) -> Result<BTreeSet<String>, CiError> {
    let text = read_to_string(path)?;
    let mut codes = BTreeSet::new();

    for line in text.lines() {
        if let Some(caps) = ASID_HEADING_RE.captures(line.trim()) {
            let code = caps.get(1).unwrap().as_str().to_string();
            codes.insert(code);
        }
    }

    Ok(codes)
}

fn parse_ue_mappings(path: &str) -> Result<BTreeSet<String>, CiError> {
    if !Path::new(path).exists() {
        return Ok(BTreeSet::new());
    }

    let text = read_to_string(path)?;
    let mut codes = BTreeSet::new();

    for caps in ASID_CODE_RE.captures_iter(&text) {
        let code = caps.get(1).unwrap().as_str().to_string();
        codes.insert(code);
    }

    Ok(codes)
}

fn parse_unity_mappings(path: &str) -> Result<BTreeSet<String>, CiError> {
    if !Path::new(path).exists() {
        return Ok(BTreeSet::new());
    }

    let text = read_to_string(path)?;
    let root: serde_json::Value = serde_json::from_str(&text).map_err(|e| CiError::Json {
        path: path.to_string(),
        source: e,
    })?;

    let mut codes = BTreeSet::new();

    if let Some(asids) = root.get("asids").and_then(|v| v.as_object()) {
        for key in asids.keys() {
            if key.len() == 3 && key.chars().all(|c| c.is_ascii_digit()) {
                codes.insert(key.clone());
            }
        }
    }

    Ok(codes)
}

fn parse_godot_mappings(path: &str) -> Result<BTreeSet<String>, CiError> {
    if !Path::new(path).exists() {
        return Ok(BTreeSet::new());
    }

    let text = read_to_string(path)?;
    let mut codes = BTreeSet::new();

    for caps in ASID_CODE_RE.captures_iter(&text) {
        let code = caps.get(1).unwrap().as_str().to_string();
        codes.insert(code);
    }

    Ok(codes)
}

fn validate_asids() -> Result<(), CiError> {
    if !Path::new(ASID_DOC_PATH).exists() {
        return Ok(());
    }

    let spec_codes = parse_asid_doc(ASID_DOC_PATH)?;
    if spec_codes.is_empty() {
        return Ok(());
    }

    let ue_codes = parse_ue_mappings(UE_ASID_MAPPING_PATH)?;
    let unity_codes = parse_unity_mappings(UNITY_ASID_MAPPING_PATH)?;
    let godot_codes = parse_godot_mappings(GODOT_ASID_MAPPING_PATH)?;

    let mut errors = Vec::new();

    for code in &spec_codes {
        if !ue_codes.is_empty() && !ue_codes.contains(code) {
            errors.push(format!(
                "ASID{} defined in {} but missing from {}",
                code, ASID_DOC_PATH, UE_ASID_MAPPING_PATH
            ));
        }
        if !unity_codes.is_empty() && !unity_codes.contains(code) {
            errors.push(format!(
                "ASID{} defined in {} but missing from {}",
                code, ASID_DOC_PATH, UNITY_ASID_MAPPING_PATH
            ));
        }
        if !godot_codes.is_empty() && !godot_codes.contains(code) {
            errors.push(format!(
                "ASID{} defined in {} but missing from {}",
                code, ASID_DOC_PATH, GODOT_ASID_MAPPING_PATH
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CiError::Validation(errors.join("\n")))
    }
}

fn main() {
    let mut had_error = false;

    match validate_weapons() {
        Ok(_) => {
            println!("OK: weapon stats JSON and docs are consistent.");
        }
        Err(e) => {
            eprintln!("ERROR: weapon validation failed:\n{:#}", e);
            had_error = true;
        }
    }

    match validate_asids() {
        Ok(_) => {
            println!("OK: ASID registry and engine mappings are consistent.");
        }
        Err(e) => {
            eprintln!("ERROR: ASID validation failed:\n{:#}", e);
            had_error = true;
        }
    }

    if had_error {
        std::process::exit(1);
    }
}
