# Conker: Live & Uncut – Development Environment Setup

This document describes the full coding environment for the Conker: Live & Uncut fan project, covering toolchain prerequisites, repo layout, build commands, and engine integration for UE5, Unity, and Godot.

The goal is:
- One Rust CLI (`grid2scene`) that reads JSON grids/entities/manifests and emits engine‑native level builders.
- Consistent JSON Schemas for all maps and mode profiles.
- A predictable folder layout for AI‑Chat codegen and contributors.

---

## 1. Host OS and Global Prerequisites

### 1.1. Supported platforms

- Windows 10/11 (recommended for UE5 + Xbox controller testing).
- Linux (Ubuntu 22.04+).
- macOS (for Unity/Godot only; UE5 optional depending on hardware).

### 1.2. Required global tools

Install the following tools before cloning the repo:

- **Git**  
  - Windows: Install from https://git-scm.com/downloads  
  - macOS: `xcode-select --install` or standalone Git.  
  - Linux: `sudo apt-get install git` (Debian/Ubuntu).

- **Rust toolchain (stable)**  
  - Install via rustup:  
    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```
  - After installation, ensure these work:
    ```bash
    rustc --version
    cargo --version
    ```

- **Python 3.10+**  
  - Required for Unreal Editor scripting.  
  - Check:
    ```bash
    python --version
    ```

- **Node.js (optional but recommended)**  
  - For grid viewers or future web tooling.  
  - Check:
    ```bash
    node --version
    npm --version
    ```

- **CMake and build tools**  
  - Windows: install “Desktop development with C++” via Visual Studio Installer.  
  - Linux: `sudo apt-get install build-essential cmake`.  
  - macOS: `xcode-select --install` for compiler toolchain.

---

## 2. Clone the Repo and Core Layout

### 2.1. Clone

```bash
git clone https://github.com/Doctor0Evil/GAMEMODE.ai.git
cd GAMEMODE.ai
```

### 2.2. Core directories

The repo is structured around data‑driven maps and engine emitters:

- `schemas/`
  - `multiplayermapgridv1.schema.json`
  - `multiplayermapentitiesv1.schema.json`
  - `mapmanifestv1.schema.json`
  - `modeprofilesv1.schema.json` (mode configuration for each map)
- `maps/`
  - `beachdead/` – Beach Dead super‑map JSON grids & entities.
  - `alienbase/` – Alien Base hub.
  - `thebloodcount/` – Blood Count mansion/maze.
  - … remaining maps as Phase 2 fills out.
- `config/modes/`
  - `beachdead_modeprofilesv1.json` – mode profiles for Beach, Colors, Total War, DM.
- `crates/alienbasegrid2scene/`
  - Rust CLI that loads manifests, validates JSON, and emits engine scripts.
- `tools/grid2scene/`
  - Engine‑specific emitter configs and templates.

This layout is designed to let AI‑Chat discover schemas, map data, and emitters without touching engine‑specific project files directly.[file:1][file:3]

---

## 3. Rust Tooling: `grid2scene` CLI

### 3.1. Build the CLI

From the repo root:

```bash
cd crates/alienbasegrid2scene
cargo build --release
```

This produces a binary (or `.exe`) at:

- `target/release/alienbasegrid2scene`

Add it to your PATH or call with a full path.

### 3.2. Map Manifest

The CLI uses a Map Manifest to select maps by ID:

- `maps/multiplayermapmanifestv1.json`

Each entry defines:

- `id` – e.g., `beachdead`, `alienbasehub`, `thebloodcount`.
- `gridpath` – e.g., `maps/beachdead/beachdeadgridv2.json`.
- `entitiespath` – e.g., `maps/beachdead/beachdeadentitiesv2.json`.
- `tilesetpaths` – Unreal/Unity/Godot tileset mapping files.
- `supportedmodes` – gamemode IDs exposed to engine scripts.[file:1][file:3]

Example invocation:

```bash
# Validate all maps without writing any engine files
cargo run --release -- --manifest maps/multiplayermapmanifestv1.json --validate --all --dry-run

# Generate Unreal scripts for Beach Dead only
cargo run --release -- --manifest maps/multiplayermapmanifestv1.json --map beachdead --engine unreal --out-dir out/unreal/beachdead

# Generate Unity scripts for all maps
cargo run --release -- --manifest maps/multiplayermapmanifestv1.json --engine unity --all --out-dir out/unity
```

The `--dry-run` flag prints summaries (cell counts, spawns, objectives) without touching disk, ideal for CI and quick checks.[file:1]

### 3.3. JSON Validation

The CLI integrates with JSON Schemas in `schemas/`:

- `multiplayermapgridv1.schema.json`
- `multiplayermapentitiesv1.schema.json`
- `mapmanifestv1.schema.json`
- `modeprofilesv1.schema.json` (for mode configs)

To validate all grid, entities, manifest, and mode profile files:

```bash
cargo run --release -- --manifest maps/multiplayermapmanifestv1.json --validate --all
```

This ensures:

- All JSON files conform to their schemas.
- All `tiletype` and `roletags` exist in the referenced tileset.
- Mode profiles are in sync with entity roletags.[file:1][file:3]

---

## 4. Engine Integration – Unreal Engine 5

### 4.1. Unreal Project Setup

- Install Unreal Engine 5.x via Epic Games Launcher.
- Create a C++ project, e.g., `ConkerUncutUE5`.
- Add the repo as a subfolder (or as a Git submodule) under:
