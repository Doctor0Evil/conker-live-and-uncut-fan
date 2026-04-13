# Knowledge Graph Index – Conker: Live & Uncut (GAMEMODE.ai)

This document is the human‑readable front door into the Conker: Live & Uncut knowledge graph. Every entry maps a stable **Node ID** to one or more files, engines, tools, and roles so that AI‑chat and humans can navigate the codebase as a coherent graph instead of a loose set of files.

The matching machine‑readable schema for these nodes is defined in `schemas/knowledge_graph_systems.schema.json`. Any new system, tool, or document in this repository should be added to both the JSON index and, when appropriate, this Markdown overview.

---

## 1. Node Types and Naming

The knowledge graph is organized around a small, composable set of node types:

- **SystemNode**: A gameplay or engine system (character, game mode, net model, asset service, etc.).
- **ToolNode**: A CLI or service used by AI‑chat or developers (indexers, asset converters, scenario runners).
- **DocNode**: A document that defines behavior, constraints, lore, or workflows.
- **PromptNode**: A system prompt or prompt block used to guide AI‑chat behavior for specific tasks or modes.

Node IDs follow a reverse‑DNS‑style dotted naming convention:

- `systems.conker.core.character_base`
- `systems.conker.multiplayer.heist`
- `tools.conker.index.repo_index_generator`
- `docs.conker.gdd.heist`
- `prompts.conker.ai_chat.heist_generation`

Each node may have multiple file paths across engines and languages, but it must have exactly one canonical `node_id`.

---

## 2. Core Gameplay Systems (SystemNodes)

### 2.1 CLU Character and Player Core

**Node ID:** `systems.conker.core.character_base`  
**Type:** SystemNode  
**Role:** Base third‑person character foundation for Conker in UE5 and parallel engines.

**Files (UE5 C++):**

- `Engine/Unreal/Source/Public/Core/CLUCharacterBase.h`
- `Engine/Unreal/Source/Private/Core/CLUCharacterBase.cpp`

**Responsibilities:**

- Character movement, jumping, and basic camera follow/orbit behavior.
- Replicated health, simple damage events, and death/respawn hooks.
- Emote/voice trigger interface for voice lines and animations.
- Engine‑agnostic movement/physics parameters mirrored into Rust ECS and Godot/Unity frontends.

**Related Nodes:**

- `systems.conker.core.player_controller`
- `systems.conker.core.game_instance`
- `docs.conker.gdd.game_overview`
- `docs.conker.tech.engine_choice`

---

**Node ID:** `systems.conker.core.player_controller`  
**Type:** SystemNode  
**Role:** Player input and camera controller that possesses `CLUCharacterBase`.

**Files (UE5 C++):**

- `Engine/Unreal/Source/Public/Core/CLUPlayerController.h`
- `Engine/Unreal/Source/Private/Core/CLUPlayerController.cpp`

**Responsibilities:**

- Mapping input to movement, camera rotation, and interaction.
- Handling respawn, spectate, and team‑switch logic.
- Providing hooks for UI and HUD updates.

**Related Nodes:**

- `systems.conker.core.character_base`
- `systems.conker.core.game_instance`
- `docs.conker.gdd.game_overview`

---

**Node ID:** `systems.conker.core.game_instance`  
**Type:** SystemNode  
**Role:** Global Conker game state across maps, modes, and sessions.

**Files (UE5 C++):**

- `Engine/Unreal/Source/Public/Core/CLUGameInstance.h`
- `Engine/Unreal/Source/Private/Core/CLUGameInstance.cpp`

**Responsibilities:**

- Bootstrapping the correct mode (SinglePlayer, Heist, War, Alien Base).
- Holding session data (player profiles, matchmaking, progression).
- Coordinating high‑level transitions between maps and modes.

**Related Nodes:**

- `systems.conker.multiplayer.heist`
- `systems.conker.multiplayer.war`
- `systems.conker.multiplayer.alien_base`
- `docs.conker.tech.networking_model`

---

### 2.2 Multiplayer Game Modes

**Node ID:** `systems.conker.multiplayer.heist`  
**Type:** SystemNode  
**Role:** Heist mode ruleset and game flow for UE5 and parallel simulations.

**Files (UE5 C++):**

- `Engine/Unreal/Source/Public/Multiplayer/Heist/CLUHeistGameMode.h`
- `Engine/Unreal/Source/Private/Multiplayer/Heist/CLUHeistGameMode.cpp`

**Responsibilities:**

- Team and class setup, spawn logic, and round lifecycle.
- Vault door state machine (setup, breach, escape).
- Replicated scoring and objective state for clients.
- Integration point for mission DAG definitions and scenario tests.

**Related Nodes:**

- `docs.conker.gdd.heist`
- `docs.conker.tech.networking_model`
- `prompts.conker.ai_chat.heist_generation`
- `tools.conker.index.repo_index_generator`

---

**Node ID:** `systems.conker.multiplayer.war`  
**Type:** SystemNode  
**Role:** War/Blitzkrieg mode rules and round flow.

**Files (UE5 C++):**

- `Engine/Unreal/Source/Public/Multiplayer/War/CLUWarGameMode.h`
- `Engine/Unreal/Source/Private/Multiplayer/War/CLUWarGameMode.cpp`

**Responsibilities:**

- Phase‑based objectives and capture/defend logic.
- Team scoring and sudden‑death conditions.
- Replication of match state and time remaining.

**Related Nodes:**

- `docs.conker.gdd.war`
- `docs.conker.tech.networking_model`
- `prompts.conker.ai_chat.war_generation`

---

**Node ID:** `systems.conker.multiplayer.alien_base`  
**Type:** SystemNode  
**Role:** Alien Base survival/horde ruleset.

**Files (UE5 C++):**

- `Engine/Unreal/Source/Public/Multiplayer/AlienBase/CLUAlienBaseGameMode.h`
- `Engine/Unreal/Source/Private/Multiplayer/AlienBase/CLUAlienBaseGameMode.cpp`

**Responsibilities:**

- Wave progression and enemy spawn logic.
- Cooperative survival objectives and difficulty scaling.
- Integration with scenario tests for wave compositions.

**Related Nodes:**

- `docs.conker.gdd.alien_base`
- `docs.conker.tech.networking_model`
- `prompts.conker.ai_chat.alien_base_generation`

---

### 2.3 Asset Lookup and Indexing

**Node ID:** `systems.conker.utils.asset_lookup`  
**Type:** SystemNode  
**Role:** Symbolic asset ID to engine asset path mapping for Conker content.

**Files (UE5 C++):**

- `Engine/Unreal/Source/Public/Utils/CLUAssetLookup.h`
- `Engine/Unreal/Source/Private/Utils/CLUAssetLookup.cpp`

**Responsibilities:**

- Map symbolic IDs (e.g., `char.conker.default`, `fx.muzzle_flash.heist`) to UE5 SoftObjectPaths or data table rows.
- Provide a stable interface for game code to request assets without hard‑coding content paths.
- Coordinate with Rust indexing tools to auto‑generate asset indices from `Assets_Source` and engine content.

**Related Nodes:**

- `tools.conker.index.repo_index_generator`
- `docs.conker.tech.engine_choice`
- `docs.conker.tech.ai_chat_workflow`

---

## 3. Engine and Scripting Prototypes

### 3.1 Godot

**Node ID:** `systems.conker.godot.character`  
**Type:** SystemNode  
**Role:** Godot prototype for Conker‑style character control.

**Files (Godot):**

- `Engine/Godot/src/core/conker_character.gd`

**Related Nodes:**

- `systems.conker.core.character_base`
- `docs.conker.tech.engine_choice`

---

**Node ID:** `systems.conker.godot.heist`  
**Type:** SystemNode  
**Role:** Godot prototype of Heist rules and flow.

**Files:**

- `Engine/Godot/src/multiplayer/heist_mode.gd`

**Related Nodes:**

- `systems.conker.multiplayer.heist`
- `docs.conker.gdd.heist`

---

**Node ID:** `systems.conker.godot.war`  
**Type:** SystemNode  
**Role:** Godot prototype of War rules and flow.

**Files:**

- `Engine/Godot/src/multiplayer/war_mode.gd`

**Related Nodes:**

- `systems.conker.multiplayer.war`
- `docs.conker.gdd.war`

---

### 3.2 Unity

**Node ID:** `systems.conker.unity.core`  
**Type:** SystemNode  
**Role:** Unity‑side scripts for core Conker mechanics (placeholder until implemented).

**Files (planned):**

- `Engine/Unity/Assets/Scripts/Core/`

**Related Nodes:**

- `systems.conker.core.character_base`
- `docs.conker.tech.engine_choice`

---

**Node ID:** `systems.conker.unity.multiplayer`  
**Type:** SystemNode  
**Role:** Unity‑side prototypes for multiplayer modes.

**Files (planned):**

- `Engine/Unity/Assets/Scripts/Multiplayer/`

**Related Nodes:**

- `systems.conker.multiplayer.heist`
- `systems.conker.multiplayer.war`
- `systems.conker.multiplayer.alien_base`

---

## 4. Tooling (ToolNodes)

### 4.1 Rust Asset and Index Tools

**Node ID:** `tools.conker.rust.clr_unpack_rs`  
**Type:** ToolNode  
**Role:** Rust CLI for unpacking Xbox/Conker‑style archives for legal research and asset conversion.

**Files:**

- `Tools/rust/clr_unpack_rs/Cargo.toml`
- `Tools/rust/clr_unpack_rs/src/main.rs`

**Responsibilities:**

- Provide a CLI skeleton for parsing and unpacking archive formats.
- Emit structured JSON manifests describing discovered files and structures.

---

**Node ID:** `tools.conker.rust.n64_asset_converter`  
**Type:** ToolNode  
**Role:** Library for converting N64‑era assets into modern engine‑friendly formats.

**Files:**

- `Tools/rust/n64_asset_converter/Cargo.toml`
- `Tools/rust/n64_asset_converter/src/lib.rs`

---

**Node ID:** `tools.conker.index.repo_index_generator`  
**Type:** ToolNode  
**Role:** Repository index generator for AI‑chat lookup and navigation.

**Files:**

- `Tools/indexing/repo_index_generator.rs`

**Responsibilities:**

- Crawl the repo and emit a JSON index of files, languages, node IDs, and roles.
- Keep the machine‑readable KG index (`build/knowledge_graph_systems.json`) in sync with the layout.

---

### 4.2 Scripting and Analysis Tools

**Node ID:** `tools.conker.lua.heist_round_logic`  
**Type:** ToolNode  
**Role:** Lua prototype of Heist round logic for rapid iteration.

**Files:**

- `Tools/lua/gameplay_prototyping/heist_round_logic.lua`

---

**Node ID:** `tools.conker.matlab.net_latency_analysis`  
**Type:** ToolNode  
**Role:** MATLAB analysis of network latency, jitter, and tick‑rate effects.

**Files:**

- `Tools/matlab/net_latency_analysis.m`

---

## 5. Documentation (DocNodes)

**Node ID:** `docs.conker.gdd.game_overview`  
**Type:** DocNode  
**Role:** High‑level game design and pillars.

**Files:**

- `Docs/GDD/01_Game_Overview.md`

---

**Node ID:** `docs.conker.gdd.heist`  
**Type:** DocNode  
**Role:** Heist mode design and mission structure.

**Files:**

- `Docs/GDD/02_Multiplayer_Heist.md`

---

**Node ID:** `docs.conker.gdd.war`  
**Type:** DocNode  
**Role:** War mode design and mission structure.

**Files:**

- `Docs/GDD/03_Multiplayer_War.md`

---

**Node ID:** `docs.conker.gdd.alien_base`  
**Type:** DocNode  
**Role:** Alien Base mode design and wave structure.

**Files:**

- `Docs/GDD/04_Multiplayer_Alien_Base.md`

---

**Node ID:** `docs.conker.tech.engine_choice`  
**Type:** DocNode  
**Role:** Engine choice rationale (UE5, Unity, Godot).

**Files:**

- `Docs/TechDesign/01_Engine_Choice_UE5_Unity_Godot.md`

---

**Node ID:** `docs.conker.tech.networking_model`  
**Type:** DocNode  
**Role:** Networking model and constraints.

**Files:**

- `Docs/TechDesign/02_Networking_Model.md`

---

**Node ID:** `docs.conker.tech.ai_chat_workflow`  
**Type:** DocNode  
**Role:** AI‑chat workflow and tool usage.

**Files:**

- `Docs/TechDesign/03_AI_Chat_Workflow.md`

---

**Node ID:** `docs.conker.ai.lore_base`  
**Type:** DocNode  
**Role:** Lore and narrative baseline.

**Files:**

- `Docs/AI_Chat_Context/Conker_Lore_Base.txt`

---

**Node ID:** `docs.conker.ai.system_prompts`  
**Type:** DocNode  
**Role:** System prompts and rules for GAMEMODE.ai.

**Files:**

- `Docs/AI_Chat_Context/System_Prompts_GAMEMODE_ai.md`

---

**Node ID:** `docs.conker.ai.file_generation_guidelines`  
**Type:** DocNode  
**Role:** File generation rules and constraints for AI‑chat.

**Files:**

- `Docs/AI_Chat_Context/File_Generation_Guidelines.md`

---

## 6. Prompt Nodes

**Node ID:** `prompts.conker.ai_chat.global`  
**Type:** PromptNode  
**Role:** Global system rules for Conker GAMEMODE.ai.

**Source:**

- `Docs/AI_Chat_Context/System_Prompts_GAMEMODE_ai.md` (Global section)

---

**Node ID:** `prompts.conker.ai_chat.heist_generation`  
**Type:** PromptNode  
**Role:** Specialized prompt block for Heist mode code/doc generation.

**Source:**

- `Docs/AI_Chat_Context/System_Prompts_GAMEMODE_ai.md` (Heist block, to be added)

---

**Node ID:** `prompts.conker.ai_chat.war_generation`  
**Type:** PromptNode  
**Role:** Specialized prompt block for War mode generation.

---

**Node ID:** `prompts.conker.ai_chat.alien_base_generation`  
**Type:** PromptNode  
**Role:** Specialized prompt block for Alien Base mode generation.

---

## 7. Next Objectives

1. **Machine Index:** Implement `build/knowledge_graph_systems.json` that conforms to `schemas/knowledge_graph_systems.schema.json`, seeded with the nodes above.  
2. **Repo Index Tool:** Extend `Tools/indexing/repo_index_generator.rs` to keep the JSON index in sync with the actual file tree and validate node references.  
3. **Scenario Nodes:** Add Scenario/Test nodes (e.g., `systems.conker.tests.heist_scenarios`) once deterministic test harnesses are introduced for multiplayer rules.
