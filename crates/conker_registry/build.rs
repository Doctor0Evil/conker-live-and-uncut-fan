use std::env;
use std::fs;
use std::path::PathBuf;

use schemars::schema_for;
use serde::Serialize;

// Bring the enums into scope for schema generation.
use conker_registry::{SfxId, VfxId};

fn write_schema<T>(name: &str)
where
    T: schemars::JsonSchema + Serialize,
{
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let mut out_dir = PathBuf::from(manifest_dir);
    out_dir.push("schemas");

    // Ensure `schemas/` exists.
    fs::create_dir_all(&out_dir)
        .expect("failed to create schemas directory");

    let schema = schema_for!(T);
    let schema_json =
        serde_json::to_string_pretty(&schema).expect("failed to serialize schema");

    let mut path = out_dir.clone();
    path.push(format!("{name}.schema.json"));

    fs::write(&path, schema_json)
        .unwrap_or_else(|e| panic!("failed to write schema {name}: {e}"));
}

fn main() {
    // Re-run if the registry enums change.
    println!("cargo:rerun-if-changed=src/sfx.rs");
    println!("cargo:rerun-if-changed=src/vfx.rs");

    // Generate schemas for SFX and VFX only (minimal smoke-test).
    write_schema::<SfxId>("sfx");
    write_schema::<VfxId>("vfx");
}
