use regex::Regex;
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::Path;

fn read(path: &str) -> anyhow::Result<String> {
    Ok(fs::read_to_string(path)?)
}

/// Extract all three-digit ASIDs from docs/systems/AnimationStateRegistry.md
/// Lines must contain a pattern like "ASID-400" or "ASID‑400".
fn extract_registry_asids(md: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    // Match ASID-012, ASID‑400, etc.
    let re = Regex::new(r"ASID[-\u2011\u2013\u2014](\d{3})").unwrap();

    for cap in re.captures_iter(md) {
        if let Some(m) = cap.get(1) {
            out.insert(m.as_str().to_string());
        }
    }

    out
}

/// Extract ASIDs from Unreal helper (int32) and any explicit numeric cases.
fn extract_unreal_asids(header: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let re = Regex::new(r"case\s+(\d{3})\s*:").unwrap();

    for cap in re.captures_iter(header) {
        if let Some(m) = cap.get(1) {
            out.insert(m.as_str().to_string());
        }
    }

    out
}

/// Extract ASIDs from Godot helpers / mappings (string keys like "400").
fn extract_godot_asids(script: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let re = Regex::new(r#""(\d{3})""#).unwrap();

    for cap in re.captures_iter(script) {
        if let Some(m) = cap.get(1) {
            out.insert(m.as_str().to_string());
        }
    }

    out
}

/// Extract ASIDs from Unity ASIDHelpers.cs ("case 400:" or "case 50:").
fn extract_unity_asids(cs: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let re = Regex::new(r"case\s+(\d{2,3})\s*:").unwrap();

    for cap in re.captures_iter(cs) {
        if let Some(m) = cap.get(1) {
            let s = m.as_str();
            // Normalize to three digits to match registry format.
            let normalized = if s.len() == 2 {
                format!("0{}", s)
            } else {
                s.to_string()
            };
            out.insert(normalized);
        }
    }

    out
}

/// Simple report type so the CI log is readable.
fn report_diff(
    name: &str,
    registry: &BTreeSet<String>,
    engine: &BTreeSet<String>,
) -> anyhow::Result<()> {
    let registry_only: HashSet<_> = registry.difference(engine).cloned().collect();
    let engine_only: HashSet<_> = engine.difference(registry).cloned().collect();

    if registry_only.is_empty() && engine_only.is_empty() {
        println!("[asid-ci-check] {name}: OK (no drift)");
        return Ok(());
    }

    if !registry_only.is_empty() {
        eprintln!("[asid-ci-check] {name}: missing in engine mapping:");
        for asid in &registry_only {
            eprintln!("  - ASID {}", asid);
        }
    }

    if !engine_only.is_empty() {
        eprintln!("[asid-ci-check] {name}: extra ASIDs not in registry:");
        for asid in &engine_only {
            eprintln!("  - ASID {}", asid);
        }
    }

    anyhow::bail!("ASID mapping drift detected for {name}")
}

fn main() -> anyhow::Result<()> {
    // Paths assume the crate lives under crates/asid_ci_check.
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .unwrap()
        .to_path_buf();

    let registry_md = read(
        repo_root
            .join("docs/systems/AnimationStateRegistry.md")
            .to_str()
            .unwrap(),
    )?;

    let unreal_header = read(
        repo_root
            .join("Source/UncutMultiplayer/Public/ASIDHelpers.h")
            .to_str()
            .unwrap(),
    )?;

    let godot_exec_helpers = read(
        repo_root
            .join("godot/Systems/ASID/ASIDExecutionHelpers.gd")
            .to_str()
            .unwrap(),
    )?;

    let godot_mappings = read(
        repo_root
            .join("godot/Systems/ASID/ASIDMappings.gd")
            .to_str()
            .unwrap(),
    )?;

    let unity_asid_helpers = read(
        repo_root
            .join("Unity/Assets/Scripts/Systems/ASID/ASIDHelpers.cs")
            .to_str()
            .unwrap(),
    )?;

    let registry_asids = extract_registry_asids(&registry_md);
    let unreal_asids = extract_unreal_asids(&unreal_header);

    // For Godot, merge mappings + helpers so both execution and locomotion
    // states are covered.
    let mut godot_asids = extract_godot_asids(&godot_exec_helpers);
    for asid in extract_godot_asids(&godot_mappings) {
        godot_asids.insert(asid);
    }

    let unity_asids = extract_unity_asids(&unity_asid_helpers);

    report_diff("Unreal", &registry_asids, &unreal_asids)?;
    report_diff("Godot", &registry_asids, &godot_asids)?;
    report_diff("Unity", &registry_asids, &unity_asids)?;

    Ok(())
}
