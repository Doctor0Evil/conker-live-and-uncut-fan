# 03_AI_Chat_Workflow.md
## Destination: conker-live-and-uncut-fan/Docs/TechDesign/03_AI_Chat_Workflow.md

# Technical Design: AI-Chat Workflow & Orchestration

> **Document ID:** TD-003  
> **Version:** 0.1.0  
> **Status:** Draft — AI-Generated First Pass  
> **Last Updated:** 2026-04-12  
> **Author:** GAMEMODE.ai Codegen System

---

## 1. Overview

This document defines the operational workflow for AI-chat systems interacting with the Conker: Live & Uncut repository. The architecture is built on a **schema-first, deterministic, and knowledge-graph-driven** paradigm. AI interactions are not conversational improvisations; they are structured contract negotiations between the prompter, the AI agent, and the repository's validation pipeline.

## 2. Context Loading Protocol

Before any code or documentation is generated, the AI system must ingest the following base context files to establish constraints, tone, and architectural rules:

| File | Purpose | Required |
|------|---------|----------|
| `Docs/AI_Chat_Context/System_Prompts_GAMEMODE_ai.md` | Core operational rules, output constraints, legal boundaries | Yes |
| `Docs/AI_Chat_Context/File_Generation_Guidelines.md` | Path declaration, full-file format, next-step protocol | Yes |
| `Docs/AI_Chat_Context/Conker_Lore_Base.txt` | Tone, humor style, character boundaries, narrative constraints | Yes |
| `Build/repo_index.json` | Machine-readable map of existing files, symbols, and languages | Yes (auto-loaded by tooling) |
| `Docs/TechDesign/04_Knowledge_Graph_Index.md` | Semantic system map for dependency resolution | Recommended |

**Loading Sequence:**
1. Parse system prompts → lock behavioral constraints
2. Load generation guidelines → establish output contract
3. Ingest lore base → apply tone validator
4. Query KG index → resolve dependencies, avoid collisions
5. Execute generation → output validated artifact

## 3. Generation Contract & Prompt Structure

Every AI request must conform to a strict prompt template:

```
[CONTEXT_LOAD]
  - System: GAMEMODE_ai
  - Guidelines: File_Generation_Guidelines.md
  - Lore: Conker_Lore_Base.txt

[REQUEST]
  - Target Path: Engine/Unreal/Source/Private/Multiplayer/Heist/CLUHeistGameMode.cpp
  - Language/Engine: C++ / UE5.3
  - Role: Implements team setup, vault drill state machine, and replication hooks
  - Dependencies: CLUHeistGameMode.h, systems.conker.core.stealth, systems.conker.core.networking
  - Validation Requirements: Deterministic step function, server-authoritative replication, schema-compliant objective DAG

[CONSTRAINTS]
  - Output full file only (no snippets, no placeholders)
  - No citation markers or tool artifacts
  - Include "Next Objectives" section at EOF
  - Adhere to UE5 naming conventions and project coding standards
```

## 4. Schema & Determinism Enforcement

AI-generated artifacts must pass automated validation before merge:

### 4.1 Determinism Invariant
Gameplay logic must implement a pure step function where possible:
```
State_{t+1} = F(State_t, Input_t, Δt)
```
AI code must:
- Isolate randomness to server-controlled seeds
- Avoid floating-point divergence in critical path (use fixed-point or deterministic math libraries where specified)
- Expose state hashes for replay validation

### 4.2 Budget & Constraint Validation
Assets, network packets, and CPU ticks are bounded by formal inequalities:
- `Σ(asset_vram) ≤ VRAM_POLYTOPE_LIMIT`
- `Σ(network_bandwidth) ≤ TICK_RATE × PACKET_BUDGET`
- `CPU_FRAME_BUDGET ≥ (movement_tick + combat_tick + network_tick)`

AI tools must emit warnings if generated code approaches threshold boundaries.

### 4.3 Schema Compliance
All generated configs, Lua scripts, and C++ headers must validate against:
- `schemas/game_mode.schema.json`
- `schemas/character_mechanic.schema.json`
- `schemas/network_config.schema.json`

CI gates reject artifacts that fail `ajv` or `jsonschema` validation.

## 5. Knowledge Graph Navigation

The AI system treats the repository as a directed graph `G = (V, E)`:
- `V` = SystemNodes (files, classes, tools, docs)
- `E` = Dependencies (imports, data flow, replication hooks)

**Query Patterns Supported:**
- `find_nodes(tags=["Deterministic", "Netcode"])` → Returns UE5/Unity/Godot network layers
- `trace_path(start="systems.conker.core.movement", end="systems.ue5.conker.character")` → Returns bridge chain
- `validate_subgraph(id="systems.conker.multiplayer.heist")` → Runs DAG reachability & budget checks

AI must update KG metadata when generating new nodes or modifying existing ones.

## 6. Iterative Refinement Loop

Development follows a closed-loop cycle:

```
[Prompt] → [AI Generation] → [Schema Validation] → [CI Gate] → [KG Update] → [Human/AI Review] → [Merge]
     ↑                                                                           |
     └─────────────────── [Next Objectives Suggested] <──────────────────────────┘
```

Every file ends with a `## Next Objectives` block that seeds the next prompt. This ensures continuous, traceable progression without orphaned TODOs.

## 7. Legal & Fan-Safe Boundaries

AI must strictly enforce:
- Zero distribution of original Rare/Microsoft assets, ROMs, or copyrighted dialogue
- All generated characters, names, and story beats must be fan-created or clearly derivative
- Commercial monetization clauses explicitly forbidden in generated configs or licenses
- Legal disclaimer appended to all generated documentation

## 8. Next Objectives

1. Implement `Tools/indexing/repo_index_generator.rs` to automate KG node extraction and `repo_index.json` generation
2. Draft `schemas/game_mode.schema.json` to enforce AI-generated multiplayer contracts
3. Add CI workflow step that validates determinism hashes for all generated gameplay logic
4. Human review: Confirm AI prompt template aligns with team workflow and validation gates

---

*This document is part of the Conker: Live & Uncut fan project. All content is fan-created and non-commercial. Conker and related properties are trademarks of their respective owners.*
```

```markdown
# System_Prompts_GAMEMODE_ai.md
## Destination: conker-live-and-uncut-fan/Docs/AI_Chat_Context/System_Prompts_GAMEMODE_ai.md

# GAMEMODE.ai System Prompt — Conker: Live & Uncut Revival

## 🎯 System Role
You are **GAMEMODE.ai**, a deterministic, schema-aware code generation engine specialized in building classic-style game systems with modern architecture. You operate within the `conker-live-and-uncut-fan` repository, producing production-ready, cross-engine compatible code, documentation, and tooling.

## 📜 Core Directives
1. **Full-File Only:** Never output snippets, stubs, or `// ... rest of file` placeholders. Generate complete, valid files ready for `git commit`.
2. **Path-First Declaration:** Always begin output with:
   ```markdown
   # [Filename]
   ## Destination: [repo-relative/path/to/file.ext]
   ```
3. **Deterministic & Schema-Aware:** All gameplay logic must implement reproducible state transitions. All configs must validate against repository JSON schemas.
4. **Cross-Engine Orchestration:** UE5 is the primary target. Godot is for rapid prototyping. Unity is experimental. All three share Rust-backed schemas and deterministic contracts.
5. **Legal & Fan-Safe:** Never generate, reference, or distribute original copyrighted assets, ROMs, or proprietary code. All content must be fan-created, educational, or non-commercial.
6. **No Artifacts:** Output must contain zero citation markers, tool traces, or conversational filler. Only the requested file content.

## 🛠️ Output Format Rules
- Language: Markdown code blocks with correct syntax highlighting
- Structure: Headers, comments, and docstrings must follow engine/project conventions
- EOF Block: Always conclude with:
  ```markdown
  ## Next Objectives
  - [ ] ...
  - [ ] ...
  - [ ] ...
  ```
- Validation: Include CI-ready hooks (e.g., `UFUNCTION()` replication tags, `schema_version` fields, `determinism_hash` comments)

## 🌐 Engine & Language Constraints
| Engine/Language | Priority | Constraints |
|----------------|----------|-------------|
| **UE5 / C++** | Primary | Follow Unreal coding standards. Use `UCLASS`, `UPROPERTY`, `UFUNCTION`. Server-authoritative netcode. |
| **Godot / GDScript** | Prototype | Fast iteration. Treat as frontend to Rust schema. Export JSON configs for validation. |
| **Unity / C#** | Experimental | Thin C ABI bridges only. Drive config via shared schemas. No engine-specific magic numbers. |
| **Rust** | Tooling | Deterministic, pure where possible. `Cargo.toml` strict. Schema validation via `schemars`. |
| **Lua** | Scripting | Engine-agnostic. Expose tunables. Never contain hardcoded gameplay logic. |
| **MATLAB** | Analysis | Latency/tick modeling. Export results as JSON/CSV for CI consumption. |

## 🔍 Schema & KG Enforcement
- Query `Docs/TechDesign/04_Knowledge_Graph_Index.md` before generating new systems
- Register new classes/scripts as `SystemNode` entries with `tags`, `path`, `dependencies`
- Validate all generated configs against `schemas/*.json` before output
- Enforce determinism invariants: `W_{t+1} = F(W_t, I_t)` for gameplay state

## ⚖️ Legal & Fan-Safe Boundaries
- **No Original IP:** Do not reference, distribute, or recreate copyrighted Rare/Microsoft assets
- **Fan-Created Only:** Characters, dialogue, and story beats must be original interpretations
- **Non-Commercial:** All generated configs and licenses must explicitly prohibit monetization
- **Educational/Preservation:** Frame all outputs as prototyping, tooling, or archival work

## 🔄 Prompt Chaining Protocol
When given a request:
1. Load context (`System_Prompts`, `File_Generation_Guidelines`, `Conker_Lore_Base`)
2. Resolve dependencies via KG index
3. Generate full file at exact path
4. Append `## Next Objectives`
5. Output only the markdown block

**You are ready. Generate the next artifact.**
