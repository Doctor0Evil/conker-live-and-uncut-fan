# Schemas Index

This directory contains the canonical JSON Schemas for all AI-visible contracts in the GAMEMODE.ai / Nintendoor64 workspace. All CLIs, CI jobs, and AI-chat flows are expected to validate against these schemas before performing any build, patch, or write operation.

The schemas are grouped by domain:

- Orchestration and sessions
- Knowledge graph and features
- Retro hardware layouts and budgets
- Patching and artifacts
- Gameplay recipes and configs
- CI digests and scenarios

Each section below lists the primary schema files, the Rust owners (via `schemars`), and the main CLIs that consume them.

---

## 1. Orchestration and Sessions

### 1.1 ArtifactSpec

- File: `schemas/artifact-spec.schema.json`
- Rust type: `crates/sonia-core/src/lib.rs::ArtifactSpec`
- Purpose: Canonical payload for any file AI or tools create. Describes what kind of artifact this is and how its content is encoded.

Key fields:

- `kind`: enum of artifact types, e.g. `N64RomPatch`, `Ps1IsoPatch`, `LuaScript`, `InputMapperConfig`, `Other`.
- `filename`: repo-relative path where Sonia will write the decoded artifact, typically under `artifacts/...`.
- `encoding`: `Text`, `Hex`, or `Base64` (default `Text`), describing how `content` is encoded.
- `content`: string-encoded payload, decoded by Sonia before writing.

Used by:

- `sonia-core` CLI (`validate`, `write`)
- AI-chat when emitting any non-source artifact (patches, configs, scripts)

### 1.2 SessionProfile

- File: `schemas/session-profile.schema.json`
- Rust type: `crates/gamemodeai-session/src/lib.rs::SessionProfile`
- Purpose: Per-repo, per-branch session state for AI and tools. Encodes the current crate, invariants, feature flags, and latest CI status.

Key fields:

- `repo`: canonical repo identifier, e.g. `Doctor0Evil/GAMEMODE.ai`.
- `branch`: current Git branch.
- `activeCrate`: primary Cargo crate for this session.
- `featureFlags`: optional feature toggles (e.g., `netcode`, `experimentalCamera`).
- `invariants.rules`: array of invariant rules (determinism, ABI, hardware budgets).
- `recentTodos`: optional TODO items with file/line references.
- `ciStatus`: latest CI digest summary and structured failures.

Used by:

- `gamemodeai-session` CLI (`get-session`, `update-session`, `update-ci-status`)
- System prompt generation for AI conditioning

### 1.3 Sonia JSON-RPC Envelope

- File: `schemas/sonia-protocol.schema.json`
- Rust type: `crates/sonia-core/src/cli.rs::ProtocolEnvelope`
- Purpose: Shared request/response envelope for Sonia-style CLIs (JSON in, JSON out).

Key fields:

- `version`: protocol version.
- `command` or `method`: logical command name (e.g., `validate`, `write`, `get-session`).
- `params`: object containing command-specific inputs.
- `status`: `ok` or `error`.
- `data` / `error`: payload or error detail object.

Used by:

- `sonia-core`, `gamemodeai-session`, `gamemodeai-kg`, and related CLIs

---

## 2. Knowledge Graph and Features

### 2.1 SystemNode (Knowledge Graph Systems)

- File: `schemas/knowledgegraph-systems.schema.json`
- Rust type: `crates/gamemodeai-kg/src/model.rs::SystemNode`
- Purpose: Describes systems, crates, and files, forming the main knowledge graph.

Key fields:

- `id`: stable system identifier (`coreecs.world`, `systems.nintendoor64.starzip`).
- `title`: human-friendly title.
- `description`: short description of the system’s role.
- `crate`: owning Cargo crate, if any.
- `files`: repo-relative file paths implementing this system.
- `related`: IDs of related systems.
- `tags`: classification such as `Deterministic`, `PublicAbi`, `LuaFacing`, `BuildRecipe`, `Retro`.

Used by:

- `gamemodeai-kg` CLI (`getSystem`, `listSystemsByTag`, `listDependents`)
- AI navigation over Rust code and tools

### 2.2 FeatureEntry (Sonia Feature Layout)

- File: `schemas/sonia-featurelayout.schema.json`
- Rust type: `crates/sonia-featurelayout/src/lib.rs::FeatureEntry`
- Purpose: Index of AI-visible features (“superpowers”) with tags and example flows.

Key fields:

- `id`: feature identifier (e.g., `n64.safePatchSynthesizer`).
- `title`: human-readable name.
- `description`: what the feature does.
- `tags`: platform, genre, and role tags (e.g., `Nintendoor64`, `BudgetPlanner`, `Stealth`).
- `systems`: related `SystemNode` IDs.
- `schemas`: referenced schema IDs (ArtifactSpec, RomLayout, PatchSpec, etc.).
- `examples`: example CLI flows and payloads.

Used by:

- `sonia-featurelayout` CLI (`list-by-tag`, `get`, multi-tag queries)
- AI feature discovery for retro tooling and workflows

---

## 3. Retro Hardware Layouts and Budgets

### 3.1 RomLayout (N64 / Platform-Agnostic Layouts)

- File: `schemas/n64-romlayout.schema.json`
- Rust type: `crates/n64-layout/src/lib.rs::RomLayout`
- Purpose: High-level view of ROM segments and files, used as the “legal space” for patching.

Key fields:

- `entrypoint`: ROM entry address.
- `romSize`: total ROM size in bytes.
- `segments`: array of segments, each with:
  - `name`, `kind` (e.g., `code`, `data`, `assets`),
  - `romOffset`, `romSize`,
  - `vramStart`,
  - `compression`,
  - `mutable` (whether patches may touch this segment).
- `files`: array of logical files, each with:
  - `path` (logical file path),
  - `segment` (segment name),
  - `offsetInSegment`, `length`,
  - `contentType`.

Used by:

- `n64-layout` crate and CLIs
- `starzip-cli` (`rom-query`, `validate-patch`, `patch`)
- Sonia N64 bridge (`soniaBridge` module)

### 3.2 N64Constraints / Hardware Budgets

- File: `schemas/n64-constraints.schema.json`
- Rust type: `crates/n64-ai-checklist/src/lib.rs::N64Constraints`
- Purpose: N64 hardware budget profile for ROM, RAM, and per-segment resource ceilings.

Key fields:

- `profileId`: identifier for constraint profile.
- `romMaxBytes`: maximum ROM size.
- `rdramBytes`: available RAM.
- `segmentBudgets`: array of `SegmentBudget`:
  - `segment`: segment name.
  - `maxBytes`: ceiling for that segment.
  - `mutable`: whether AI may change this segment.

Used by:

- `n64-ai-checklist` for constraint checks over RomLayout and PatchSpec
- Starzip budget tooling and CI budget gates

### 3.3 NesConstraints / SnesConstraints (Retro NES/SNES)

- Files:
  - `schemas/nes-constraints.schema.json`
  - `schemas/snes-constraints.schema.json`
- Rust types:
  - `crates/retro-nes-core/src/constraints.rs::NesConstraints`
  - `crates/retro-snes-core/src/constraints.rs::SnesConstraints`
- Purpose: Constraint models for NES and SNES, governing PRG/CHR/page sizes, nametable dimensions, and per-level tile budgets.

Key fields (NES):

- `prgSizeBytes`: PRG ROM size.
- `chrBankSizeBytes`: CHR bank size (bytes).
- `maxTilesPerLevel`: unique tile budget per map.
- `nametableWidth`, `nametableHeight`: tile dimensions (e.g., 32×30).

Used by:

- `retro-nes-core` CHR/nametable packers
- `retro-cli` (`validate`, `build-rom` for NES/SNES)

---

## 4. Patching and Artifacts

### 4.1 PatchSpec (Starzip Safe Patch Synthesizer)

- File: `schemas/n64-patchspec.schema.json`
- Rust type: `crates/starzip-core/src/patchspec.rs::PatchSpec`
- Purpose: High-level patch description for Starzip; compiles into safe byte-level edits.

Key fields:

- `version`: PatchSpec version.
- `baseRomId`: identifier (e.g., SHA-256) of the base ROM.
- `layoutId`: ID of the RomLayout this patch targets.
- `edits`: array of `PatchEdit` variants, such as:
  - `ReplaceFile`: `logicalPath`, `payloadRef`, `encoding`.
  - `BootHook`: `hookKind`, `params` (JSON object).
  - `JsonPatch`: `logicalPath`, `jsonPointer`, `value`.
  - `RawIntervalPatch`: `romOffset`, `maxBytes`, `payloadRef`, `encoding`.

Used by:

- `starzip-cli` (`validate-patch`, `patch`)
- Sonia N64 bridge and checklist logic

### 4.2 BudgetReport

- File: `schemas/budget-report.schema.json`
- Rust type: `crates/starzip-budget/src/lib.rs::BudgetReport` (or equivalent)
- Purpose: Output of hardware budget analysers for N64, PS1, NES, SNES.

Key fields:

- `profileId`: constraint profile used.
- `isWithinBudget`: boolean flag.
- `totals`: aggregate resource usage (ROM, VRAM, CPU).
- `slack`: per-dimension remaining headroom (negative = overflow).
- `perSegment`: usage and slack per segment or asset class.

Used by:

- CI budget steps (N64 vertical slice, retro builds)
- AI conditioning (weighting invariants based on recent budget failures)

---

## 5. Gameplay Recipes and Configs

### 5.1 RetroRecipe (Retro Game Recipe)

- File: `schemas/retro-recipe.schema.json`
- Rust type: `crates/retro-schemas/src/retrorecipe.rs::RetroRecipe`
- Purpose: Platform-agnostic recipe describing maps, tilesets, and other assets for retro games.

Key fields:

- `id`: recipe ID.
- `title`: human-readable title.
- `maps`: array of `Map`:
  - `id`, `name`, `tilesetId`.
  - `size`: `[width, height]` in tiles.
  - `layer`: includes `encoding` (e.g., `csv`) and `data` (tile grid).
- `tilesets`: array of `Tileset`:
  - `id`, `name`, `tileCount`.

Used by:

- `retro-cli` (`validate`, `build-rom`, `pack-assets`)
- NES/SNES/Godot backends sharing the same recipe

### 5.2 StealthParams / MissionGraph / NarrativeGraph

- Files:
  - `schemas/stealth-params.schema.json`
  - `schemas/mission-graph.schema.json`
  - `schemas/narrative-graph.schema.json`
- Rust types:
  - Stealth: `crates/bondfps-core/src/stealth.rs::StealthParams`
  - Missions: `crates/bondfps-core/src/missions.rs::MissionGraph`
  - Narrative: `crates/narrative-core/src/graph.rs::NarrativeGraph`
- Purpose: Core gameplay tuning/config structures for stealth behavior, mission DAGs, and narrative graphs.

StealthParams key fields:

- FOV angles, detection thresholds, awareness rates.
- Lighting and sound coefficients.
- Flags for difficulty tiers.

MissionGraph key fields:

- Nodes (missions/objectives) with IDs and descriptions.
- Edges with conditions and rewards.
- Start/end node sets and gating conditions.

NarrativeGraph key fields:

- Story nodes, branches, and arcs.
- Reputation/morality vectors attached to edges.
- Constraints such as minimum endings, reachable branches.

Used by:

- Gameplay ECS cores (Bond-style FPS, narrative systems)
- Scenario testing, visual diff, and AI-assisted tuning

---

## 6. CI Digests and Scenarios

### 6.1 CiDigest / CiFailure

- File: `schemas/ci-digest.schema.json`
- Rust type: `crates/gamemodeai-session/src/cidigest.rs::CiDigest`
- Purpose: Normalized CI output for AI and tools; feeds into `SessionProfile.ciStatus`.

Key fields:

- `job`: CI job name (e.g., `n64-conker-mini-ci`, `retro-nes-smoke`).
- `status`: `ok` or `failed`.
- `summary`: human-readable summary.
- `timestamp`: CI run timestamp or GitHub run ID.
- `failures`: array of `CiFailure`:
  - `crate`: affected crate or component.
  - `kind`: taxonomy label (e.g., `SchemaViolation`, `BudgetOverflow`, `DeterminismViolation`, `EmulatorCrash`, `TestFailure`).
  - `message`: short description.
  - `logUrl`: optional link to full logs.

Used by:

- `gamemodeai-session update-ci-status`
- AI conditioning and prompt generation

### 6.2 ScenarioSpec (Emulator Scenario Director)

- File: `schemas/scenario-spec.schema.json`
- Rust type: `crates/scenario-core/src/spec.rs::ScenarioSpec`
- Purpose: Declarative scenarios for headless emulator runs (N64, NES, PS1).

Key fields:

- `id`: scenario ID.
- `targetRom` or `buildRecipeId`: which build to run.
- `initialState`: map/level, entities, stealth parameters, seed values.
- `inputSequence`: time-indexed input bitmask frames or macros.
- `telemetrySpec`: which metrics to record.
- `assertions`: pass/fail conditions over telemetry (e.g., detection curves, lock-on behavior).

Used by:

- Conk64 and other emulator harnesses
- CI regression tests for gameplay behavior

---

## 7. Schema Governance and Generation

All schemas in this directory are generated from Rust types using `schemars` and the schema generator tool:

- Generator: `tools/schema-gen/main.rs`
- Output directory: `schemas/`

Key guarantees:

- Rust structs and enums deriving `JsonSchema` are the single source of truth.
- CI regenerates schemas and validates all JSON files (configs, KG, recipes, layouts, patches) against the corresponding schema before running any build or emulator step.
- Breaking schema changes must be accompanied by version bumps or migration paths in docs and tooling.

---

## 8. How to Work With Schemas

### 8.1 Regenerating Schemas

From the workspace root:

```bash
cargo run -p schema-gen -- --out-dir schemas
```

This will regenerate all `*.schema.json` files based on the current Rust types.

### 8.2 Validating JSON Instances

Use the schema guard CLI to validate JSON files:

```bash
cargo run -p schema-guard -- \
  --schema schemas/artifact-spec.schema.json \
  --instances examples/artifacts/example-artifact.json
```

CI workflows run similar commands for RomLayout, PatchSpec, ArtifactSpec, SessionProfile, KG files, RetroRecipe, ScenarioSpec, and other key contracts.

### 8.3 Adding a New Schema

To introduce a new schema:

1. Define or update a Rust type and derive `JsonSchema`.
2. Register the type in `schema-gen` so it is emitted to `schemas/`.
3. Add example JSON files under `examples/` or `tests/` that conform to the schema.
4. Wire a schema validation step into CI for those examples.
5. If the schema is AI-visible, add references in:
   - `knowledgegraph/systems.json` (as schema owner),
   - `features.sonia.json` (as part of a feature surface),
   - Relevant docs under `docs/`.

This keeps contracts, code, CI, and AI-chat tools aligned and versioned together.
