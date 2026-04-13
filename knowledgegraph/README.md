# Knowledge Graph Index

This directory contains the machine-readable maps that let AI chat systems and tools navigate the repository by **concept** rather than raw file paths. It defines two main layers:

- `knowledgegraphsystems.json`: system-level nodes for crates, tools, subsystems, and assets.
- `knowledgegraphfeatures.sonia.json`: high-level “feature” entries (superpowers, vertical slices, workflows) that bundle systems, schemas, and examples into AI-friendly surfaces.

Together they provide the semantic index for navigation, impact analysis, and schema-first editing.

---

## Files in this directory

### `knowledgegraphsystems.json`

This file stores an array of `SystemNode` records describing all significant systems in the workspace: core ECS modules, retro toolchains, CLIs, N64/PS1 tooling, schemas, and supporting utilities.

Each `SystemNode` has at least:

- `id`: stable identifier, e.g. `systems.nintendoor64.starzip.layout`.
- `title`: human-readable name, e.g. `Starzip ROM Layout Oracle`.
- `description`: short summary of purpose and behavior.
- `crate`: owning Cargo crate name, when applicable.
- `files`: repo-relative paths implementing the system.
- `related`: IDs of other systems it depends on or interacts with.
- `tags`: semantic labels such as `Nintendoor64`, `Starzip`, `RomLayout`, `PatchSynthesizer`, `Deterministic`, `PublicAbi`, `LuaFacing`, `Editor`, `TestOnly`.

A JSON Schema (usually stored as `schemas/knowledgegraph-systems.schema.json`) defines the exact structure and is enforced in CI so that every `SystemNode` is valid and queryable.

#### Typical examples

- N64 layout and patching:
  - `systems.nintendoor64.starzip.layout`
  - `systems.nintendoor64.starzip.patch`
  - `systems.nintendoor64.starzip.budget`
- Sonia orchestration:
  - `systems.nintendoor64.sonia`
  - `systems.nintendoor64.sonia.featurelayout`
- Retro mini-engine:
  - `systems.retro.schema.recipe`
  - `systems.retro.backend.nes`
  - `systems.retro.backend.snes`
- Bond FPS and narrative:
  - `systems.bondfpscore.stealthai`
  - `systems.bondfpscore.lockon`
  - `systems.bondfpscore.missions`
  - `systems.nintendoor64.narrative`

These IDs are the canonical handles that CLIs and AI-chat use when asking “what files implement this system?” or “what breaks if I change this module?”.

---

### `knowledgegraphfeatures.sonia.json`

This file stores a `FeatureLayout` document: a curated index of higher-level features and superpowers built on top of the raw `SystemNode` graph. It is designed specifically for AI-chat navigation and capability discovery.

At the top level:

- `repo`: repository identifier, e.g. `Doctor0Evil/Nintendoor64`.
- `version`: semantic version of the feature index.
- `features`: array of `FeatureEntry` objects.

Each `FeatureEntry` describes a coherent capability surface:

- `id`: feature identifier, e.g. `nintendoor64.starzip.romlayout`, `nintendoor64.narrative.cartographer`.
- `title`: short name, e.g. `ROM Layout Oracle`, `Safe Patch Synthesizer`.
- `description`: what this feature does and when to use it.
- `tags`: controlled vocabulary such as:
  - Platform: `Nintendoor64`, `Retro`, `NES`, `SNES`, `PS1`.
  - Role: `RomLayout`, `PatchSynthesizer`, `BudgetPlanner`, `ScenarioDirector`, `NarrativeCartographer`, `SchemaDesigner`, `BuildConductor`.
  - Properties: `Deterministic`, `BinarySafe`, `SchemaFirst`, `Experimental`, `Stable`.
- `systems`: list of `SystemNode.id` values that implement this feature.
- `schemas`: paths to JSON Schemas that define its contracts.
- `examples`: example JSON or config files under `examples/` that demonstrate correct usage.
- Optional:
  - `roles`: finer-grained roles inside the feature.
  - `commands`: recommended CLIs and example invocations.
  - `stability`: e.g. `internal`, `experimental`, `stable`.

A JSON Schema (e.g. `schemas/featurelayout.schema.json`) governs this structure and is validated in CI. The schema enforces the tag vocabulary and ensures that every referenced `SystemNode` and schema path actually exists.

#### Typical feature entries

Some high-value features you will see mapped here:

- `nintendoor64.starzip.romlayout`
  - Systems: `systems.nintendoor64.starzip.layout`
  - Schemas: `schemas/romlayout.schema.json`
  - Tags: `Nintendoor64`, `Starzip`, `RomLayout`, `Deterministic`
- `nintendoor64.starzip.patch-synthesizer`
  - Systems: `systems.nintendoor64.starzip.patch`, `systems.nintendoor64.sonia`
  - Schemas: `schemas/patchspec.schema.json`, `schemas/artifact-spec.schema.json`
  - Tags: `Nintendoor64`, `Starzip`, `PatchSynthesizer`, `BinarySafe`, `Deterministic`
- `nintendoor64.budget-planner`
  - Systems: `systems.nintendoor64.starzip.budget`, `systems.nintendoor64.n64-constraints`
  - Schemas: `schemas/n64-constraints.schema.json`, `schemas/n64-asset-manifest.schema.json`
  - Tags: `Nintendoor64`, `BudgetPlanner`, `HardwareConstraints`, `SchemaFirst`
- `nintendoor64.scenario-director`
  - Systems: `systems.nintendoor64.conk64.scenario`, `systems.bondfpscore.scenario`
  - Schemas: `schemas/scenario-spec.schema.json`, `schemas/bondstealth.schema.json`
  - Tags: `Nintendoor64`, `ScenarioDirector`, `Deterministic`, `TestOnly`
- `nintendoor64.narrative.cartographer`
  - Systems: `systems.nintendoor64.narrative`, `systems.bondfpscore.missions`
  - Schemas: `schemas/narrative-graph.schema.json`, `schemas/mission-dag.schema.json`
  - Tags: `Nintendoor64`, `NarrativeCartographer`, `SchemaDesigner`
- Retro slice:
  - `retro.recipes.core`, `retro.backend.nes`, `retro.backend.snes`
- Bond FPS slice:
  - `bondfps.stealth-core`, `bondfps.lockon-core`, `bondfps.mission-core`

These features are the primary entry points for AI-chat. Instead of scanning the whole repository, the model can ask “show features tagged `Nintendoor64` and `PatchSynthesizer`” and receive a small, well-documented set of capabilities.

---

## Core schemas referenced from the knowledge graph

The knowledge graph is tightly coupled to a set of JSON Schemas that define the shape of all AI-visible contracts. The most important ones are:

- Knowledge graph and sessions:
  - `schemas/knowledgegraph-systems.schema.json`
  - `schemas/featurelayout.schema.json`
  - `schemas/session.session.schema.json`
- Orchestration and artifacts:
  - `schemas/artifact-spec.schema.json`
  - `schemas/sonia-protocol.schema.json`
  - `schemas/sonia-command-descriptor.schema.json` (for CLI capabilities)
- Retro and Nintendoor64 data:
  - `schemas/romlayout.schema.json` (N64/PS1 ROM layout)
  - `schemas/patchspec.schema.json` (Safe Patch Synthesizer)
  - `schemas/n64-constraints.schema.json`
  - `schemas/n64-asset-manifest.schema.json`
  - `schemas/retrorecipe.schema.json` (portable retro game recipe)
- Gameplay and narrative:
  - `schemas/bondstealth.schema.json` (stealth parameters)
  - `schemas/bondlockon.schema.json` (lock-on profile)
  - `schemas/mission-dag.schema.json` (objective DAG)
  - `schemas/narrative-graph.schema.json` (higher-level narrative arcs)
  - `schemas/scenario-spec.schema.json` (emulator scenario director)
- Analysis and CI:
  - `schemas/budgetreport.schema.json`
  - `schemas/cidigest.schema.json`
  - Telemetry and visual-diff schemas, where present.

Every `SystemNode` and `FeatureEntry` that consumes or produces JSON should list the relevant schema paths so that tools and AI-chat can:

1. Discover which contracts apply.
2. Validate instances before use.
3. Use examples as safe starting points.

---

## CLIs that operate on the knowledge graph

Two families of CLIs provide structured access to these files: `gamemodeai-kg` for system-level navigation and `sonia-featurelayout` for feature-level discovery. Both speak JSON over stdin/stdout and follow a consistent envelope.

### `gamemodeai-kg`

Binary: `crates/gamemodeai-kg`  

Responsibilities:

- Load `knowledgegraphsystems.json` and validate it against `knowledgegraph-systems.schema.json`.
- Answer graph-oriented questions about systems and dependencies.

Typical commands:

- `getSystem`
  - Input: `{ "version": 1, "command": "getSystem", "params": { "id": "<system-id>" } }`
  - Output: system metadata with crate, files, tags, and related nodes.
- `listSystemsByTag`
  - Input: `{ "version": 1, "command": "listSystemsByTag", "params": { "tag": "<tag>" } }`
  - Output: all `SystemNode` entries that carry the given tag.
- `listDependents`
  - Input: `{ "version": 1, "command": "listDependents", "params": { "id": "<system-id>" } }`
  - Output: systems that declare the given `id` in their `related` list.

This is the primary tool for answering:

- “Which files implement `systems.bondfpscore.stealthai`?”
- “What depends on `systems.nintendoor64.starzip.layout`?”
- “Which systems are tagged `Deterministic` or `PublicAbi`?”

### `sonia-featurelayout`

Binary: `crates/sonia-featurelayout`  

Responsibilities:

- Load `knowledgegraphfeatures.sonia.json` and validate it against `featurelayout.schema.json`.
- Expose high-level features for AI navigation, with tag-based queries.

Typical commands:

- `listByTag`
  - Input: `{ "version": 1, "command": "listByTag", "params": { "tag": "<tag>" } }`
  - Output: subset of features whose tag sets include the requested tag.
- `listByTagsAll`
  - Input: `{ "version": 1, "command": "listByTagsAll", "params": { "tags": ["TagA", "TagB"] } }`
  - Output: features that include **all** queried tags.
- `listByTagsAny`
  - Input: `{ "version": 1, "command": "listByTagsAny", "params": { "tags": ["TagA", "TagB"] } }`
  - Output: features that include **any** of the queried tags.
- `get`
  - Input: `{ "version": 1, "command": "get", "params": { "id": "<feature-id>" } }`
  - Output: the full `FeatureEntry` record.

In practice:

- AI-chat uses `listByTagsAll` with tags like `["Nintendoor64", "PatchSynthesizer", "Deterministic"]` to discover the Safe Patch Synthesizer.
- After selecting a feature, it uses the `systems` and `schemas` fields to call the appropriate CLIs and validate payloads.

---

## How AI-chat and tools should use this directory

For AI systems and human tooling, the recommended flow is:

1. Use `gamemodeai-kg` to resolve system IDs, file paths, and dependency structure from `knowledgegraphsystems.json`.
2. Use `sonia-featurelayout` to discover capabilities and recommended entry points from `knowledgegraphfeatures.sonia.json`.
3. Use the schemas listed in `schemas/` to validate all JSON instances before any build or patch operation.
4. Keep both knowledge graph files under version control and subject to the same CI gates as code.

When adding new crates, tools, or data formats:

- Add or update `SystemNode` entries in `knowledgegraphsystems.json`.
- Add or update `FeatureEntry` records in `knowledgegraphfeatures.sonia.json`, or let a generator tool produce skeleton entries and refine them by hand.
- Ensure all new JSON formats have Rust types with `JsonSchema` and committed JSON Schemas under `schemas/`.
- Extend the CLIs only via their JSON envelopes, preserving deterministic, stateless behavior.

This keeps the knowledge graph as the single source of truth for navigation and contracts, making the repository easy to explore for both AI-chat systems and human developers.
