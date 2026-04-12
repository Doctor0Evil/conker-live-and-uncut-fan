# Conker: Live & Uncut — GAMEMODE.ai Codegen Hub

Fan-made, non‑commercial revival of **Conker: Live & Uncut**, designed as an **AI‑Chat codebase-generation system** for building and extending game code, tools, and libraries in **Rust, Lua, C++, MATLAB, and Godot** over modern engines (**UE5, Unity, Godot**).

This repository is structured so AI chat systems can:
- Generate full files (not snippets) with clear paths and roles.
- Discover and index tools, libraries, and knowledge-graphs.
- Iteratively expand a large, modular codebase for the Conker: Live & Uncut vision.

---

## 1. Project Intent

The goal is to recreate and extend the **original vision** of Conker: Live & Uncut:

- Rebuild the **single-player campaign** in spirit with modern engine tech.
- Implement the **story-driven multiplayer** modes (The Heist, War/Blitzkrieg, Alien Base, etc.).
- Provide **tooling, data, and structure** for AI to help write, refactor, and analyze codebases for:
  - Gameplay logic in C++/Godot/Rust.
  - Scripting in Lua and Godot Script.
  - Tooling and analysis in Rust and MATLAB.
- Maintain a **clear separation** between:
  - Fan-made code and assets.
  - Any original, legally protected content (never committed here).

This repository doubles as:
1. A **game codebase** skeleton.
2. A **GAMEMODE.ai prompt surface** and knowledge graph for AI‑assisted development.

---

## 2. GAMEMODE.ai Rules in This Repo

GAMEMODE.ai operational rules are embedded into the workflow:

1. **Perform one or more of:**
   - Build syntax (full, valid files).
   - Find tools.
   - Discover libraries.
   - Analyze structures and GitHub repositories.

2. **Research objects and completion:**
   - Treat every missing file or TODO as a “research-object”.
   - AI must generate *complete* first-pass implementations, not stubs.
   - Every prompt should specify a concrete path:  
     e.g. `Engine/Unreal/Source/Multiplayer/Heist/CLUHeistGameMode.cpp`.

3. **Output discipline:**
   - Always produce **valid, full-length code** or a **complete document**.
   - The files generated here must **not contain citation markers** or tool artifacts.

4. **When a specific repository is mentioned:**
   - Check its description.
   - Inspect its directory structure.
   - Infer missing or recommended directories and coding steps (for logic, not answered as questions in files).

5. **Continuous improvement:**
   - Every change should suggest next objectives.
   - Always offer routes to more advanced behavior and better structure.

6. **Indexing and lookup:**
   - Prioritize structures that make AI lookup easy: index files, registries, and manifest-like documents.

7. **Professional standards:**
   - Follow practices from AAA game programming (UE5, Unity, Godot, major studios).

---

## 3. Repository Structure

```text
conker-live-and-uncut-fan/
├── .github/
│   ├── workflows/
│   │   └── build.yml
│   └── ISSUE_TEMPLATE/
│       ├── bug_report.md
│       └── feature_request.md
├── Docs/
│   ├── GDD/
│   │   ├── 01_Game_Overview.md
│   │   ├── 02_Multiplayer_Heist.md
│   │   ├── 03_Multiplayer_War.md
│   │   └── 04_Multiplayer_Alien_Base.md
│   ├── TechDesign/
│   │   ├── 01_Engine_Choice_UE5_Unity_Godot.md
│   │   ├── 02_Networking_Model.md
│   │   └── 03_AI_Chat_Workflow.md
│   └── AI_Chat_Context/
│       ├── System_Prompts_GAMEMODE_ai.md
│       ├── Conker_Lore_Base.txt
│       └── File_Generation_Guidelines.md
├── Engine/
│   ├── Unreal/
│   │   ├── ConkerLiveUncut.uproject
│   │   ├── Source/
│   │   │   ├── ConkerLiveUncut.Target.cs
│   │   │   ├── ConkerLiveUncutEditor.Target.cs
│   │   │   ├── Public/
│   │   │   │   ├── Core/
│   │   │   │   │   ├── CLUCharacterBase.h
│   │   │   │   │   ├── CLUPlayerController.h
│   │   │   │   │   └── CLUGameInstance.h
│   │   │   │   ├── Multiplayer/
│   │   │   │   │   ├── Heist/
│   │   │   │   │   │   └── CLUHeistGameMode.h
│   │   │   │   │   ├── War/
│   │   │   │   │   │   └── CLUWarGameMode.h
│   │   │   │   │   └── AlienBase/
│   │   │   │   │       └── CLUAlienBaseGameMode.h
│   │   │   │   └── Utils/
│   │   │   │       └── CLUAssetLookup.h
│   │   │   └── Private/
│   │   │       ├── Core/
│   │   │       │   ├── CLUCharacterBase.cpp
│   │   │       │   ├── CLUPlayerController.cpp
│   │   │       │   └── CLUGameInstance.cpp
│   │   │       ├── Multiplayer/
│   │   │       │   ├── Heist/
│   │   │       │   │   └── CLUHeistGameMode.cpp
│   │   │       │   ├── War/
│   │   │       │   │   └── CLUWarGameMode.cpp
│   │   │       │   └── AlienBase/
│   │   │       │       └── CLUAlienBaseGameMode.cpp
│   │   │       └── Utils/
│   │   │           └── CLUAssetLookup.cpp
│   │   └── Content/
│   │       ├── Maps/
│   │       ├── Characters/
│   │       ├── Blueprints/
│   │       └── Audio/
│   ├── Godot/
│   │   ├── project.godot
│   │   ├── src/
│   │   │   ├── core/
│   │   │   │   └── conker_character.gd
│   │   │   └── multiplayer/
│   │   │       ├── heist_mode.gd
│   │   │       └── war_mode.gd
│   │   └── scenes/
│   └── Unity/
│       ├── ProjectSettings/
│       ├── Assets/
│       │   ├── Scripts/
│       │   │   ├── Core/
│       │   │   └── Multiplayer/
│       │   └── Scenes/
├── Tools/
│   ├── rust/
│   │   ├── clr_unpack_rs/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── main.rs
│   │   ├── n64_asset_converter/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── lib.rs
│   ├── lua/
│   │   └── gameplay_prototyping/
│   │       └── heist_round_logic.lua
│   ├── matlab/
│   │   └── net_latency_analysis.m
│   └── indexing/
│       └── repo_index_generator.rs
├── Assets_Source/
│   ├── Models/
│   ├── Textures/
│   ├── Audio/
│   └── Animations/
├── Build/
│   └── README.md
├── .gitignore
├── LICENSE
└── README.md
```

---

## 4. How AI‑Chat Should Use This Repo

### 4.1 Loading Context

Before generating files, an AI‑Chat system should ingest:

- `Docs/AI_Chat_Context/System_Prompts_GAMEMODE_ai.md`  
  (Defines GAMEMODE.ai rules, style, and output constraints.)

- `Docs/AI_Chat_Context/File_Generation_Guidelines.md`  
  (Describes how to always output full files, paths, and next steps.)

- `Docs/AI_Chat_Context/Conker_Lore_Base.txt`  
  (Captures tone, characters, and narrative for Conker.)

This ensures generated code and docs align with the intended style and universe.

### 4.2 Requesting Files

Prompts should always specify:

- Exact **path** of the file.
- Engine and language.
- Role of the file and dependencies.

Example:

> “Generate a full first-pass implementation for `Engine/Unreal/Source/Private/Multiplayer/Heist/CLUHeistGameMode.cpp` that supports team-based bank robbery, basic objectives, and replication-ready scoring. Use the public header at `Engine/Unreal/Source/Public/Multiplayer/Heist/CLUHeistGameMode.h`.”

The AI then returns a single, complete file body that can be saved verbatim at that path.

---

## 5. Language and Engine Roles

- **C++ / UE5**  
  Primary implementation for high-fidelity multiplayer and core gameplay systems.

- **Godot Script**  
  Lightweight prototypes or alternate implementations of modes (Heist/War) usable by smaller teams.

- **C# / Unity**  
  Optional parallel prototype space (kept here mainly for flexibility and experimentation).

- **Rust**  
  Tooling and pipeline code, including:
  - Xbox/N64 asset unpackers.
  - Repo index generators.
  - Static analyzers for the codebase.

- **Lua**  
  Runtime scripting for gameplay prototyping and scenario logic.

- **MATLAB**  
  Networking and simulation scripts (e.g., latency modeling, tick-rate analysis).

---

## 6. Initial Research Objects (For AI‑Guided Completion)

These are first targets where AI should generate full files:

1. **Core Character Base (UE5)**  
   - `Engine/Unreal/Source/Public/Core/CLUCharacterBase.h`  
   - `Engine/Unreal/Source/Private/Core/CLUCharacterBase.cpp`  
   Objective: Movement, jumping, basic camera control, replicated health, and simple emote/voice triggers.

2. **The Heist Game Mode (UE5)**  
   - `Engine/Unreal/Source/Public/Multiplayer/Heist/CLUHeistGameMode.h`  
   - `Engine/Unreal/Source/Private/Multiplayer/Heist/CLUHeistGameMode.cpp`  
   Objective: Team setup, spawn points, vault door logic, objective state machine (setup, breach, escape).

3. **Asset Lookup Utility (UE5 + Rust Tooling)**  
   - `Engine/Unreal/Source/Public/Utils/CLUAssetLookup.h`  
   - `Engine/Unreal/Source/Private/Utils/CLUAssetLookup.cpp`  
   - `Tools/indexing/repo_index_generator.rs`  
   Objective: Provide a mapping from symbolic asset IDs to engine paths, with Rust tool auto-generating indices.

4. **Rust Asset Tools**  
   - `Tools/rust/clr_unpack_rs/src/main.rs`  
   Objective: CLI skeleton to parse Xbox-style archives (dummy parser first; real parsing later).

---

## 7. Next Directions and Improvement Suggestions

To improve this system for both developers and AI‑chat:

1. **Knowledge Graph Expansion**  
   - Create `Docs/TechDesign/04_Knowledge_Graph_Index.md` listing all major classes, tools, and scripts with short descriptions.  
   - AI can use this as a “table of contents” for the whole codebase.

2. **Repo Index Service**  
   - Implement `Tools/indexing/repo_index_generator.rs` to crawl the repo and emit a JSON index of:
     - File paths.
     - Language.
     - Declared classes/functions (simple regex or parser).
   - This JSON can be fed into AI as context for advanced queries.

3. **Coding Standards Docs**  
   - Add `Docs/TechDesign/05_Coding_Standards_UE5.md` and `Docs/TechDesign/06_Coding_Standards_Rust.md`.  
   - This helps AI match style and best practices from industry.

4. **Scenario-Specific Prompts**  
   - Extend `Docs/AI_Chat_Context/System_Prompts_GAMEMODE_ai.md` with dedicated prompt blocks for:
     - “Heist mode generation”
     - “War mode generation”
     - “Alien Base horde mode”

5. **Multiplayer Test Harness**  
   - Define a testing scaffold (e.g., `Engine/Unreal/Source/Private/Multiplayer/Tests/`) for AI to generate deterministic tests for game rules, so balancing and refactoring are safer.

---

## 8. Legal Notice

This is a **fan-operated, non-commercial** project.  
Conker, Conker’s Bad Fur Day, Conker: Live & Reloaded, and Conker: Live & Uncut are properties of their respective rights holders. This repository does **not** distribute original game assets or ROMs and is intended for educational, preservation, and prototyping purposes only.

---

_Last updated: 2026-04-12_  
_GAMEMODE.ai configuration: Conker Live & Uncut Revival_
