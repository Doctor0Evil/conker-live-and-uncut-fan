# Contributing to GAMEMODE.ai / Nintendoor64

This repository is a schema-first, CLI-driven, retro game development workspace built around deterministic Rust crates, JSON Schemas, and knowledge-graph navigation. It is designed so both humans and AI-chat systems can collaborate safely on large codebases.

This document explains how to propose changes while respecting determinism, ABI stability, hardware budgets, and CI contracts.

---

## 1. Ground rules

Contributions are welcome as issues or pull requests as long as they:

- Preserve determinism and platform constraints for retro targets (NES, SNES, N64, PS1).
- Respect the contract-first design: Rust types → JSON Schemas → validated JSON/TOML → build tools.
- Keep AI-chat and human workflows aligned: everything important must be discoverable by schemas and the knowledge graph.
- Do not add or distribute original Conker or other third-party IP; this repo hosts tooling, schemas, and fan-oriented glue only.

Please discuss large architectural changes in an issue first so we can align on schemas, invariants, and CI impact.

---

## 2. Repository layout (high level)

Key top-level areas:

- `crates/` – Rust crates (Sonia, Starzip, Conk64, retro-* cores, invariants, schema tools).
- `schemas/` – Generated and hand-authored JSON Schemas (ArtifactSpec, SessionProfile, SystemNode, RomLayout, PatchSpec, RetroRecipe, ScenarioSpec, BudgetProfile, NarrativeGraph, etc.).
- `knowledgegraph/` – JSON knowledge-graph files (e.g., `knowledgegraph/systems.json`, `knowledgegraph/features.sonia.json`).
- `docs/` – Architecture documents, AI-model specs, vertical slice walkthroughs, and build guides.
- `examples/` – Small, schema-checked examples (N64 layouts and patches, retro recipes, scenarios).
- `.github/workflows/` – CI pipelines (schema validation, checklist runs, N64 vertical slice, retro mini-engine builds).

When in doubt, prefer adding new code or data under these existing roots rather than inventing new top-level directories.

---

## 3. Determinism and invariants

This workspace treats determinism and hard invariants as first-class citizens, enforced by dedicated Rust modules and CI jobs.

### 3.1 Deterministic Rust code

When touching deterministic crates (ECS cores, core gameplay systems, invariant modules):

- Do **not** use nondeterministic APIs (e.g., wall-clock time, OS randomness, thread-local RNG).
- Avoid un-ordered collections (e.g., `HashMap`, `HashSet`) in deterministic update paths unless you impose a stable iteration order.
- Keep system execution order explicit and testable.

In code, this typically means:

- Using `IndexMap`/`IndexSet` or sorted vectors when iteration order matters.
- Avoiding hidden global state or side effects in hot loops.
- Treating each ECS “tick” as a pure function over world state.

If in doubt, assume a crate is “deterministic” unless explicitly tagged otherwise in docs or the knowledge graph.

### 3.2 Invariant modules

Invariant enforcement lives in dedicated modules, such as:

- `invariants/determinism.rs` – patterns that are forbidden in deterministic crates.
- `invariants/hardware_budget.rs` – ROM/VRAM/CPU budget checks per platform.
- `invariants/abi_guard.rs` – C ABI stability rules for engine-facing exports.

When you introduce a new rule or relax an existing one:

- Update the corresponding invariant module.
- Add or adjust tests that exercise the rule.
- Document the rule in code comments and in `docs/` as needed.
- Keep rules machine-readable whenever possible so they can be surfaced in CI and AI prompts.

---

## 4. ABI policies

C ABI surfaces (for Godot/Unreal/Unity bridges, emulator hooks, etc.) are treated as contracts.

Key guidelines:

- Public C symbols must follow the established naming conventions (for example, prefixes like `gm_` for engine exports).
- Struct layouts exposed over FFI must be `#[repr(C)]` and stable.
- Public ABI changes should be additive whenever possible; breaking changes require version bumps and migration notes.

When modifying ABI:

1. Update Rust types and regenerate headers (e.g., via `cbindgen`).
2. Run the ABI guard tools (where available) to detect breaking vs additive changes.
3. Document any breaking change in `docs/` and mention it in the PR description.

---

## 5. Hardware budgets and constraints

Hardware constraints are expressed as Rust types + JSON Schemas, then enforced by specialized tools:

- NES/SNES: CHR/nametable limits, tile budgets per map, VRAM and palette constraints.
- N64: ROM size, RDRAM usage, segment budgets, CPU budgets, safe patch regions.
- PS1 and other targets: disc layout, VRAM, audio and streaming constraints.

When adding or modifying content that affects budgets (new assets, larger maps, more complex patches):

- Update the relevant constraint type and schema (e.g., `N64Constraints`, `NesConstraints`, `BudgetProfile`).
- Ensure a budget checker exists or is extended to cover the new cases.
- Run the budget-related CI jobs locally (`cargo test`, specific `*_budget` CLIs) before pushing.

It is better to encode constraints as explicit inequalities and invariants than to rely on comments or tribal knowledge.

---

## 6. Schema-first workflow

All AI-visible and tool-visible data is governed by schemas.

### 6.1 Rust types → JSON Schemas

If you introduce or change a data structure that will be serialized (configs, layouts, recipes, scenarios, session profiles, KG nodes):

- Define a Rust type for it and derive `serde::{Serialize, Deserialize}`.
- Derive `schemars::JsonSchema` where appropriate so schema generation tools can emit a JSON Schema under `schemas/`.
- Update `schema-gen` tooling (if needed) to generate or update schema files for the new type.

Do not hand-edit generated schema files unless clearly marked; instead, update Rust types and regenerate.

### 6.2 JSON/TOML data

New JSON/TOML files should:

- Conform to an existing schema, or
- Come with a new schema and type as described above.

Before opening a PR:

- Run the schema-validation tools (e.g., `schema-guard` or equivalent) on your new/changed data.
- Ensure CI schema checks pass locally for your changes.

---

## 7. Knowledge graph and feature index

The knowledge graph (`knowledgegraph/systems.json`, `knowledgegraph/features.sonia.json`, and related files) is the primary way AI and tools discover systems, schemas, and CLIs.

When adding new systems, tools, or features:

- Add or update `SystemNode` entries in `knowledgegraph/systems.json` with:
  - A stable `id` (e.g., `systems.nintendoor64.starzip`).
  - A clear `title` and `description`.
  - Owning crate, file paths, and tags (e.g., `Deterministic`, `Retro`, `ArtifactSink`, `BuildTool`).
- If relevant, add feature entries in `knowledgegraph/features.sonia.json` to expose capabilities to AI (e.g., N64 Safe Patch Synthesizer, NES CHR packer, Scenario Director).
- Keep knowledge-graph entries in sync with actual Rust code and schemas.

Knowledge graph changes should always be validated by the same schema and CI checks as other JSON data.

---

## 8. CI expectations

CI is the hard gate for this workspace. Typical jobs include:

- Rust compilation and tests across relevant crates.
- Linting (e.g., `cargo clippy`) with strict settings in core crates.
- Schema generation and validation for all schemas and JSON/TOML data.
- Invariant/checklist runs (determinism, budgets, ABI, patch safety).
- Vertical slice pipelines (e.g., N64 slice, retro mini-engine builds).

Before opening a PR:

1. Run `cargo test` for all affected crates.
2. Run `cargo clippy` with at least the default workspace configuration.
3. Run schema generation/validation scripts if your change touches schemas or JSON.
4. For vertical slices you change (e.g., N64 Conker mini-slice, NES test ROM), run the corresponding CI workflow locally if possible (or at least the local CLI commands it runs).

If CI fails on your PR:

- Prefer minimal, targeted fixes that preserve existing invariants.
- Use the CI logs and any “digest” tooling to identify which invariant or schema was violated.
- Update tests and docs if the behavior change is intentional.

---

## 9. Working with AI-chat systems

This repo is explicitly designed to be used with AI-chat assistants that call JSON-in/JSON-out CLIs.

When authoring code or docs, keep in mind:

- CLIs should be deterministic, stateless (per call), and use structured JSON for both input and output.
- Commands should be discoverable: if you add a new CLI verb, document it in `docs/` and (where applicable) in protocol specs (e.g., JSON-RPC descriptors).
- Error messages should be structured and machine-readable when possible (error codes, fields, kinds), so AI can implement retry/fix patterns.
- Avoid “magic” side effects; prefer explicit configuration, schemas, and KG entries.

If you add new tools meant for AI:

- Provide a short doc section describing the contract (input schema, output schema, invariants).
- Add entries in the knowledge graph or feature index so AI can find and use them.

---

## 10. Style and review process

- Rust: follow standard community style (rustfmt, idiomatic patterns) and the existing patterns in this repo.
- Documentation: keep docs in `docs/` concise but precise; link to research files or design documents for deeper context where needed.
- Commits: use clear, descriptive messages; group logically related changes together.
- PRs: include a short summary, mention affected crates/schemas/CI workflows, and call out any new invariants or contracts.

We aim for a codebase that is easy to reason about by both humans and AI. Determinism, explicit contracts, and strong CI are the guiding principles.

Thank you for helping build and harden this workspace.
