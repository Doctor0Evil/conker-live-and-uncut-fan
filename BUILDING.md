# BUILDING and Running Nintendoor64 Tooling

This document is the build entrypoint for the Nintendoor64 and GAMEMODE.ai workspace. It explains how to set up the toolchain, run the N64 vertical slice locally, build the NES retro mini‑engine, and invoke the core CLIs that AI‑chat and humans share. It also enumerates the major JSON Schemas that form the contract surface for the repo.

---

## 1. Prerequisites

This workspace assumes a Rust‑first, JSON‑schema‑driven toolchain.

Rust toolchain:

- Rust stable (via rustup)
- Cargo, rustfmt, clippy

System tools:

- Git
- CMake and a C compiler (for some emulator/tool dependencies)
- Python 3 (optional, for auxiliary scripts)

Optional but recommended:

- A headless N64 emulator CLI (e.g., conk64-cli built from `crates/conk64-cli`)
- A NES/SNES emulator CLI for smoke tests
- Node or a JSON CLI for quick schema validation when hacking

Environment:

- Clone the repo and ensure submodules (if any) are initialized.
- All commands below assume repository root as the working directory.

```bash
git clone https://github.com/Doctor0Evil/Nintendoor64.git
cd Nintendoor64
rustup default stable
cargo --version
```

---

## 2. Workspace Layout and Major Contracts

At a high level, the workspace is organized as a Rust workspace plus schemas and knowledge graph:

- `crates/` – Core Rust crates (Sonia, Starzip, N64 layout, retro NES core, KG tools, sessions)
- `schemas/` – Generated JSON Schemas for all AI‑visible contracts
- `knowledgegraph/` – Knowledge graph JSON (systems, features) that AI uses for navigation
- `docs/` – Architecture, AI model, and vertical slice documents
- `examples/` – Example JSON payloads used in tests and CI
- `.github/workflows/` – CI pipelines (including the N64 vertical slice workflow)
- `artifacts/`, `target/`, `dist/` – Build outputs (ignored by Git)

### 2.1 Core schemas by family

All schemas are generated from Rust types via `schemars` and written into `schemas/`. The main families:

Gameplay:

- `schemas/stealth-params.schema.json`
- `schemas/lockon-config.schema.json`
- `schemas/enemy-stat-block.schema.json`
- `schemas/mission-dag.schema.json`
- `schemas/narrative-graph.schema.json`
- `schemas/reputation-model.schema.json`

Hardware and build:

- `schemas/nes-constraints.schema.json`
- `schemas/snes-constraints.schema.json`
- `schemas/n64-constraints.schema.json`
- `schemas/ps1-constraints.schema.json`
- `schemas/romlayout.schema.json`
- `schemas/budget-profile.schema.json`

Orchestration and AI:

- `schemas/artifact-spec.schema.json`
- `schemas/session.session.schema.json`
- `schemas/build-recipe.schema.json`
- `schemas/ci-digest.schema.json`
- `schemas/system-node.schema.json`
- `schemas/feature-entry.schema.json`
- `schemas/scenario-spec.schema.json`

N64 patching and layout:

- `schemas/n64-romlayout.schema.json`
- `schemas/n64-constraints.schema.json`
- `schemas/n64-patchspec.schema.json`
- `schemas/patchspec.schema.json`
- `schemas/budget-report.schema.json`

Retro recipes:

- `schemas/retro-recipe.schema.json`
- `schemas/retro-build-target.schema.json`

Each schema has one or more example instances under `examples/` that are used for CI validation.

---

## 3. Schema Generation and Validation

Before any substantial work, ensure schemas are up to date and JSON examples validate.

### 3.1 Generate all schemas

The `schema-gen` tool walks Rust types and regenerates JSON Schemas:

```bash
cargo run -p schema-gen --bin schema-gen -- --out-dir schemas
```

This will overwrite files under `schemas/` with the latest shapes from Rust. Run this whenever you change core types in:

- `crates/sonia-core`
- `crates/gamemodeai-session`
- `crates/n64-layout`
- `crates/starzip-core`
- `crates/retro-schemas`
- `crates/retro-nes-core`
- `crates/gamemodeai-kg`
- Other crates exposing AI‑visible types

### 3.2 Validate JSON against schemas

The `schema-guard` CLI validates JSON instances against schemas:

```bash
cargo run -p schema-guard -- \
  --schema schemas/artifact-spec.schema.json \
  --instances examples/artifacts/*.json
```

Typical validation runs:

```bash
# Core orchestration contracts
cargo run -p schema-guard -- \
  --schema schemas/artifact-spec.schema.json \
  --instances examples/artifacts/*.json

cargo run -p schema-guard -- \
  --schema schemas/session.session.schema.json \
  --instances examples/session/*.json

cargo run -p schema-guard -- \
  --schema schemas/system-node.schema.json \
  --instances knowledgegraph/systems.json

cargo run -p schema-guard -- \
  --schema schemas/feature-entry.schema.json \
  --instances knowledgegraph/features.sonia.json

# N64 layout and patch
cargo run -p schema-guard -- \
  --schema schemas/romlayout.schema.json \
  --instances examples/n64/layouts/*.json

cargo run -p schema-guard -- \
  --schema schemas/patchspec.schema.json \
  --instances examples/n64/patches/*.json

cargo run -p schema-guard -- \
  --schema schemas/n64-constraints.schema.json \
  --instances examples/n64/constraints/*.json

# Retro recipes
cargo run -p schema-guard -- \
  --schema schemas/retro-recipe.schema.json \
  --instances examples/retro/recipes/*.json
```

CI runs these checks automatically; running them locally before pushing is strongly encouraged.

---

## 4. N64 Vertical Slice: Local CI Loop

The N64 vertical slice is the reference path from AI‑generated JSON to a patched, emulated ROM for an N64 test game. It is wired through Sonia, Starzip, N64 layout, and a headless emulator.

### 4.1 Components and contracts

Key crates and files:

- `crates/n64-layout/` – `RomLayout`, `Segment`, `FileEntry` types, plus `soniabridge.rs`
- `crates/starzip-core/` and `crates/starzip-cli/` – ROM Layout Oracle and Safe Patch Synthesizer
- `crates/sonia-core/` – ArtifactSpec sink, JSON‑in/JSON‑out CLI
- `crates/gamemodeai-session/` – SessionProfile and CI digest integration
- `crates/schema-guard/` – Generic JSON Schema validator
- `schemas/` – `romlayout`, `patchspec`, `artifact-spec`, `session`, `budget-profile`, `ci-digest`
- `examples/n64/` – Example ROM layouts, constraints, patch specs, and artifact specs
- `.github/workflows/sonia-ai-n64-slice.yml` – CI workflow for the N64 slice

The main JSON contracts involved:

- `romlayout.schema.json` – Structure of the ROM (segments, files, entrypoint)
- `patchspec.schema.json` – High‑level patch edits (ReplaceFile, BootHook, etc.)
- `artifact-spec.schema.json` – Encoded payloads (Text/Hex/Base64) and metadata
- `n64-constraints.schema.json` – Budgets for ROM, segments, RAM
- `ci-digest.schema.json` – Structured CI results for feeding into SessionProfile

### 4.2 Building required tools

From repo root:

```bash
cargo build \
  -p n64-layout \
  -p starzip-cli \
  -p sonia-core \
  -p schema-guard \
  -p gamemodeai-session \
  -p n64-ai-checklist \
  -p schema-gen
```

If you have a local N64 emulator harness crate (e.g., `conk64-cli`), build it as well:

```bash
cargo build -p conk64-cli
```

### 4.3 Regenerate schemas for N64 slice

```bash
cargo run -p n64-ai-checklist --bin n64-ai-gen-schemas
```

This writes at least:

- `schemas/n64-romlayout.schema.json`
- `schemas/n64-constraints.schema.json`
- `schemas/n64-patchspec.schema.json`

alongside the existing core schemas.

### 4.4 Validate N64 example JSON

Validate layout, constraints, and patch spec:

```bash
cargo run -p schema-guard -- \
  --schema schemas/n64-romlayout.schema.json \
  --instances examples/n64/layouts/conker-mini.layout.json

cargo run -p schema-guard -- \
  --schema schemas/n64-constraints.schema.json \
  --instances examples/n64/constraints/conker-mini.constraints.json

cargo run -p schema-guard -- \
  --schema schemas/n64-patchspec.schema.json \
  --instances examples/n64/patches/conker-mini-title.patch.json
```

Validate ArtifactSpecs used in the slice:

```bash
cargo run -p schema-guard -- \
  --schema schemas/artifact-spec.schema.json \
  --instances examples/n64/artifacts/*.json
```

### 4.5 Run the N64 vertical slice pipeline locally

The CI workflow `.github/workflows/sonia-ai-n64-slice.yml` encodes the steps. To reproduce:

1. **Validate schemas and examples**

   Use the commands from sections 3 and 4.4.

2. **Materialize artifacts via Sonia**

   For each ArtifactSpec JSON used in the slice:

   ```bash
   cargo run -p sonia-core -- \
     --request '{
       "version": 1,
       "command": "write",
       "params": { "specPath": "examples/n64/artifacts/test-texture-payload.json" }
     }'
   ```

   Sonia decodes the `content` according to `encoding` and writes files into `artifacts/` under the filename specified.

3. **Run N64 layout and patch validation**

   Use `n64-layout` bridge and Starzip to validate patch specs against layout:

   ```bash
   # Optionally: a small CLI wrapper that calls validatepatchspecwithreport
   cargo run -p n64-layout --bin n64-bridge-validate -- \
     --layout examples/n64/layouts/conker-mini.layout.json \
     --patch  examples/n64/patches/conker-mini-title.patch.json \
     --artifacts-dir artifacts/
   ```

   This should emit a `PatchImpactReport` describing per‑segment impact.

4. **Apply patch via Starzip**

   Assuming you have a base ROM at `examples/n64/roms/conker-mini-base.z64`:

   ```bash
   cargo run -p starzip-cli -- \
     rom-query \
     --rom examples/n64/roms/conker-mini-base.z64 \
     --layout examples/n64/layouts/conker-mini.layout.json

   cargo run -p starzip-cli -- \
     validate-patch \
     --rom    examples/n64/roms/conker-mini-base.z64 \
     --layout examples/n64/layouts/conker-mini.layout.json \
     --spec   examples/n64/patches/conker-mini-title.patch.json

   cargo run -p starzip-cli -- \
     patch \
     --rom    examples/n64/roms/conker-mini-base.z64 \
     --layout examples/n64/layouts/conker-mini.layout.json \
     --spec   examples/n64/patches/conker-mini-title.patch.json \
     --out    target/n64/conker-mini-patched.z64
   ```

5. **Run headless emulator smoke test**

   With `conk64-cli` built:

   ```bash
   cargo run -p conk64-cli -- \
     run-headless \
     --rom target/n64/conker-mini-patched.z64 \
     --frames 300 \
     --assert-vram-write 0x80300000
   ```

6. **Generate CI digest and update session**

   First, write a CI digest JSON such as:

   ```json
   {
     "job": "sonia-ai-n64-slice",
     "status": "ok",
     "summary": "N64 vertical slice patched ROM boots and passes smoke test.",
     "timestamp": "2026-04-13T12:00:00Z",
     "failures": []
   }
   ```

   Save as `target/n64/sonia-n64-slice-ci-digest.json`, then call:

   ```bash
   cargo run -p gamemodeai-session -- \
     --request '{
       "version": 1,
       "command": "update-ci-status",
       "params": { "digestPath": "target/n64/sonia-n64-slice-ci-digest.json" }
     }'
   ```

This closes the loop that CI uses: from JSON schemas and examples through Sonia and Starzip into an emulated ROM and back into SessionProfile.

---

## 5. NES Retro Mini‑Engine: Local Build

The NES retro mini‑engine demonstrates the dual‑layer model: RetroRecipe schemas on top, NES constraints and packers underneath.

### 5.1 Components and contracts

Crates:

- `crates/retro-schemas/` – `RetroRecipe`, `Map`, `Tileset`
- `crates/retro-nes-core/` – NES constraints, CHR/nametable packer, ROM builder
- `crates/retro-cli/` – CLI for validate/build/pack/test
- Optionally `crates/retro-godot2d/` – Godot backend using the same recipes

Schemas:

- `schemas/retro-recipe.schema.json` – Structure of recipes
- `schemas/nes-constraints.schema.json` – NROM128 constraints
- `schemas/retro-build-target.schema.json` – Build targets (NES, SNES, Godot, etc.)

Examples:

- `examples/retro/recipes/*.json` – Example recipes (e.g., an “Underworld” test map)
- `examples/retro/constraints/*.toml` – Constraint profiles for NES/SNES

### 5.2 Build retro NES core and CLI

```bash
cargo build \
  -p retro-schemas \
  -p retro-nes-core \
  -p retro-cli
```

### 5.3 Generate and validate schemas

Regenerate retro schemas:

```bash
cargo run -p schema-gen --bin schema-gen -- --out-dir schemas
```

Validate recipes:

```bash
cargo run -p schema-guard -- \
  --schema schemas/retro-recipe.schema.json \
  --instances examples/retro/recipes/*.json
```

Validate NES constraints:

```bash
cargo run -p schema-guard -- \
  --schema schemas/nes-constraints.schema.json \
  --instances examples/retro/constraints/*.json
```

### 5.4 Validate and build a NES ROM

Assume you have:

- `examples/retro/recipes/underworld.recipe.json`
- `examples/retro/constraints/nes-nrom128.toml`

Validate recipe semantics against built‑in or external constraints:

```bash
cargo run -p retro-cli -- \
  validate \
  --recipe examples/retro/recipes/underworld.recipe.json \
  --target nes
```

This call parses the recipe, checks it against `RetroRecipe` and the `NesConstraints` default profile, and ensures tile counts and map sizes are legal.

Build a ROM:

```bash
cargo run -p retro-cli -- \
  build-rom \
  --recipe     examples/retro/recipes/underworld.recipe.json \
  --target     nes \
  --constraints examples/retro/constraints/nes-nrom128.toml \
  --out        dist/underworld.nes
```

This pipeline:

- Packs tiles into an 8 KiB CHR bank
- Builds a 32×30 nametable for one screen
- Combines CHR, nametable, and a small PRG stub into a valid `.nes` image

Run a quick emulator smoke test (using your preferred NES emulator CLI):

```bash
nes-emulator-cli dist/underworld.nes
```

Optional: run `retro-cli test` with a simple test suite:

```bash
cargo run -p retro-cli -- \
  test \
  --recipe    examples/retro/recipes/underworld.recipe.json \
  --target    nes \
  --test-suite examples/retro/tests/nes-smoke.json
```

---

## 6. Knowledge Graph and Session CLIs

AI‑chat and advanced tooling rely on a structured knowledge graph and session profiles.

### 6.1 Knowledge graph (systems and features)

The `gamemodeai-kg` CLI provides JSON‑in/JSON‑out queries over `knowledgegraph/systems.json` and `knowledgegraph/features.sonia.json`.

Example: resolve a system by ID:

```bash
cargo run -p gamemodeai-kg -- \
  --request '{
    "version": 1,
    "command": "get-system",
    "params": { "id": "systems.nintendoor64.starzip" }
  }'
```

List systems by tag:

```bash
cargo run -p gamemodeai-kg -- \
  --request '{
    "version": 1,
    "command": "list-systems-by-tag",
    "params": { "tag": "N64" }
  }'
```

List dependents of a system:

```bash
cargo run -p gamemodeai-kg -- \
  --request '{
    "version": 1,
    "command": "list-dependents",
    "params": { "id": "systems.nintendoor64.starzip" }
  }'
```

### 6.2 Session profiles

The `gamemodeai-session` CLI manages per‑repo, per‑branch `SessionProfile` JSON files (usually under `.gamemodeai-session.*.json`).

Get a session snapshot:

```bash
cargo run -p gamemodeai-session -- \
  --request '{
    "version": 1,
    "command": "get-session",
    "params": { "branch": "main" }
  }'
```

Update session invariants or active crate:

```bash
cargo run -p gamemodeai-session -- \
  --request '{
    "version": 1,
    "command": "update-session",
    "params": {
      "branch": "main",
      "activeCrate": "arenashooter-core",
      "featureFlags": ["netcode", "luascripting"]
    }
  }'
```

Update CI status after a run (as in the N64 slice example):

```bash
cargo run -p gamemodeai-session -- \
  --request '{
    "version": 1,
    "command": "update-ci-status",
    "params": {
      "digestPath": "target/n64/sonia-n64-slice-ci-digest.json"
    }
  }'
```

The `session.session.schema.json` schema defines:

- `repo`, `branch`, `activeCrate`
- `featureFlags`
- `invariants.rules` (e.g., determinism and ABI rules)
- `recentTodos` (TODOs with file/line)
- `ciStatus` (last CI run ID, summary, structured failures)

---

## 7. Sonia and Starzip CLI Surfaces

Sonia and Starzip are the primary CLI surfaces AI‑chat and tools use.

### 7.1 Sonia core

Sonia operates over `ArtifactSpec` JSON and a JSON RPC‑like envelope.

Validate an artifact without writing:

```bash
cargo run -p sonia-core -- \
  --request '{
    "version": 1,
    "command": "validate",
    "params": { "specPath": "examples/n64/artifacts/test-texture-payload.json" }
  }'
```

Write an artifact into `artifacts/`:

```bash
cargo run -p sonia-core -- \
  --request '{
    "version": 1,
    "command": "write",
    "params": { "specPath": "examples/n64/artifacts/test-texture-payload.json" }
  }'
```

List artifacts:

```bash
cargo run -p sonia-core -- \
  --request '{
    "version": 1,
    "command": "list",
    "params": { "root": "artifacts" }
  }'
```

The `artifact-spec.schema.json` schema defines:

- `kind` (e.g., N64RomPatch, Ps1IsoPatch, LuaScript)
- `filename` (repo‑relative path under `artifacts/`)
- `encoding` (Text, Hex, Base64)
- `content` (encoded payload as string)

### 7.2 Starzip CLI

Starzip is the ROM Layout Oracle and Safe Patch Synthesizer.

Inspect layout:

```bash
cargo run -p starzip-cli -- \
  rom-query \
  --rom    examples/n64/roms/conker-mini-base.z64 \
  --layout examples/n64/layouts/conker-mini.layout.json
```

Validate a patch:

```bash
cargo run -p starzip-cli -- \
  validate-patch \
  --rom    examples/n64/roms/conker-mini-base.z64 \
  --layout examples/n64/layouts/conker-mini.layout.json \
  --spec   examples/n64/patches/conker-mini-title.patch.json
```

Apply a patch:

```bash
cargo run -p starzip-cli -- \
  patch \
  --rom    examples/n64/roms/conker-mini-base.z64 \
  --layout examples/n64/layouts/conker-mini.layout.json \
  --spec   examples/n64/patches/conker-mini-title.patch.json \
  --out    target/n64/conker-mini-patched.z64
```

The patch spec schema:

- `patchspec.schema.json` defines `PatchSpec` with:
  - `version`
  - `baseRomId`
  - `layoutId`
  - `edits[]` (ReplaceFile, BootHook, JsonPatch, RawIntervalPatch etc.)

The ROM layout schema:

- `romlayout.schema.json` defines:
  - `entrypoint`
  - `segments[]` (name, romOffset, romSize, vramStart, compression, mutable)
  - `files[]` (path, segment, offsetInSegment, length, contentType)

---

## 8. Recommended Local Workflow

A typical local workflow for contributors:

1. **Install toolchain**

   - Install Rust stable.
   - Build core crates: `cargo build`.

2. **Regenerate schemas**

   - `cargo run -p schema-gen --bin schema-gen -- --out-dir schemas`

3. **Run schema validation**

   - Use `schema-guard` on key schema/JSON pairs.

4. **Run unit tests**

   - `cargo test` (or scoped packages: `-p n64-layout`, `-p retro-nes-core`, etc.)

5. **Run N64 vertical slice**

   - Follow section 4 to validate, patch, emulate, and update CI status.

6. **Run NES mini‑engine**

   - Follow section 5 to validate recipes and build a `.nes` ROM.

7. **Update documentation**

   - If schemas or CLIs change, update:
     - `docs/` (Sonia AI model, N64 vertical slice)
     - `knowledgegraph/` entries
     - This `BUILDING.md` and `README.md`

By keeping schemas, CLIs, and build guides in sync, the workspace remains predictable for both humans and AI‑driven tooling, and every change can be tested via repeatable, contract‑first pipelines.
