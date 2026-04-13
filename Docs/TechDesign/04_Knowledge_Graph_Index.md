# Knowledge Graph Index – Conker: Live & Uncut (GAMEMODE.ai)

This document acts as a human- and AI-readable index of the major systems, tools, and documents in this repository. Each entry summarizes the role of a file or subsystem in one sentence and notes its primary technology.

---

## 1. High-Level Design & Context

- **Docs/GDD/01_Game_Overview.md**  
  High-level vision, pillars, and scope for the Conker: Live & Uncut fan project (all engines).

- **Docs/GDD/02_Multiplayer_Heist.md**  
  Detailed design for the Heist multiplayer mode (objectives, roles, scoring).

- **Docs/GDD/03_Multiplayer_War.md**  
  Design for the War/Blitzkrieg mode including phase flow and team rules.

- **Docs/GDD/04_Multiplayer_Alien_Base.md**  
  Design for the Alien Base / horde-style mode and its wave structure.

- **Docs/TechDesign/01_Engine_Choice_UE5_Unity_Godot.md**  
  Rationale and division of responsibilities among Unreal, Unity, and Godot.

- **Docs/TechDesign/02_Networking_Model.md**  
  Networking model and replication strategy for multiplayer modes.

- **Docs/TechDesign/03_AI_Chat_Workflow.md**  
  How AI-chat systems should load context, generate files, and iterate safely.

- **Docs/AI_Chat_Context/System_Prompts_GAMEMODE_ai.md**  
  Core GAMEMODE.ai rules, style, and output constraints for AI agents.

- **Docs/AI_Chat_Context/File_Generation_Guidelines.md**  
  Rules for path-first, full-file generation with next-step suggestions.

- **Docs/AI_Chat_Context/Conker_Lore_Base.txt**  
  Lore, tone, and character references to keep writing on-brand and fan-safe.

- **Docs/TechDesign/04_Knowledge_Graph_Index.md**  
  This index – semantic map of systems and files for both humans and AI.

---

## 2. Unreal Engine – Core Systems

- **Engine/Unreal/ConkerLiveUncut.uproject**  
  Unreal project descriptor for the Conker: Live & Uncut UE5 project.

- **Engine/Unreal/Source/ConkerLiveUncut.Target.cs**  
  Build target definition for the game runtime binary.

- **Engine/Unreal/Source/ConkerLiveUncutEditor.Target.cs**  
  Build target definition for the editor tooling binary.

- **Engine/Unreal/Source/Public/Core/CLUCharacterBase.h**  
  Base character class interface (movement, camera hooks, health, emotes).

- **Engine/Unreal/Source/Private/Core/CLUCharacterBase.cpp**  
  Implementation of common player character behavior and replication.

- **Engine/Unreal/Source/Public/Core/CLUPlayerController.h**  
  Player controller interface for input, possession, and HUD interaction.

- **Engine/Unreal/Source/Private/Core/CLUPlayerController.cpp**  
  Implementation of player input handling and UI routing.

- **Engine/Unreal/Source/Public/Core/CLUGameInstance.h**  
  Custom game instance interface for global state and session management.

- **Engine/Unreal/Source/Private/Core/CLUGameInstance.cpp**  
  Implementation of initialization, mode selection, and matchmaking glue.

---

## 3. Unreal Engine – Multiplayer Modes

### 3.1 Heist

- **Engine/Unreal/Source/Public/Multiplayer/Heist/CLUHeistGameMode.h**  
  Public game mode interface for team-based Heist rules and match phases.

- **Engine/Unreal/Source/Private/Multiplayer/Heist/CLUHeistGameMode.cpp**  
  Implementation of team setup, spawns, vault logic, and scoring.

### 3.2 War

- **Engine/Unreal/Source/Public/Multiplayer/War/CLUWarGameMode.h**  
  Public game mode interface for War/Blitzkrieg phase and objective rules.

- **Engine/Unreal/Source/Private/Multiplayer/War/CLUWarGameMode.cpp**  
  Implementation of War mode phases, respawns, and victory logic.

### 3.3 Alien Base

- **Engine/Unreal/Source/Public/Multiplayer/AlienBase/CLUAlienBaseGameMode.h**  
  Public game mode interface for Alien Base horde-style survival rules.

- **Engine/Unreal/Source/Private/Multiplayer/AlienBase/CLUAlienBaseGameMode.cpp**  
  Implementation of waves, enemy spawning, and difficulty scaling.

---

## 4. Unreal Engine – Utilities & Content

- **Engine/Unreal/Source/Public/Utils/CLUAssetLookup.h**  
  Interface for mapping symbolic asset IDs to UE5 asset references.

- **Engine/Unreal/Source/Private/Utils/CLUAssetLookup.cpp**  
  Implementation of asset ID resolution and lookup caching.

- **Engine/Unreal/Content/Maps/**  
  Container for UE5 map assets (single-player and multiplayer levels).

- **Engine/Unreal/Content/Characters/**  
  UE5-ready character meshes, animations, and related assets.

- **Engine/Unreal/Content/Blueprints/**  
  Blueprint-based gameplay logic and prototypes layered over C++.

- **Engine/Unreal/Content/Audio/**  
  Engine-imported audio assets (music, VO, SFX) derived from legal sources.

---

## 5. Godot Project

- **Engine/Godot/project.godot**  
  Godot project configuration for Conker mode prototypes.

- **Engine/Godot/src/core/conker_character.gd**  
  GDScript implementation of a basic Conker-style player character.

- **Engine/Godot/src/multiplayer/heist_mode.gd**  
  Heist mode prototype logic for Godot (rules and flow).

- **Engine/Godot/src/multiplayer/war_mode.gd**  
  War mode prototype logic for Godot.

- **Engine/Godot/scenes/**  
  Godot scenes that assemble nodes and scripts into playable slices.

---

## 6. Unity Shell

- **Engine/Unity/ProjectSettings/**  
  Unity project configuration (inputs, graphics, etc.).

- **Engine/Unity/Assets/Scripts/Core/**  
  Intended location for Unity C# core scripts (character, systems).

- **Engine/Unity/Assets/Scripts/Multiplayer/**  
  Intended location for Unity C# multiplayer mode controllers.

- **Engine/Unity/Assets/Scenes/**  
  Unity scenes for levels and test harnesses.

---

## 7. Tooling – Rust, Lua, MATLAB, Indexing

### 7.1 Rust Tools

- **Tools/rust/clr_unpack_rs/Cargo.toml**  
  Manifest for the Conker Live & Reloaded-style archive unpacker.

- **Tools/rust/clr_unpack_rs/src/main.rs**  
  CLI skeleton for parsing and unpacking Xbox-style asset archives.

- **Tools/rust/n64_asset_converter/Cargo.toml**  
  Manifest for N64-era asset conversion utilities.

- **Tools/rust/n64_asset_converter/src/lib.rs**  
  Library of N64 asset conversion routines for reuse by CLIs and pipelines.

### 7.2 Lua Prototyping

- **Tools/lua/gameplay_prototyping/heist_round_logic.lua**  
  Lightweight Heist round logic prototype script for engine-agnostic testing.

### 7.3 MATLAB Analysis

- **Tools/matlab/net_latency_analysis.m**  
  MATLAB script for modeling network latency and tick-rate effects.

### 7.4 Repo Indexing

- **Tools/indexing/repo_index_generator.rs**  
  Rust CLI that crawls the repo and emits `Build/repo_index.json` for AI lookup.

---

## 8. Assets and Build Outputs

- **Assets_Source/Models/**  
  Authoring/source 3D models for characters and environments.

- **Assets_Source/Textures/**  
  Source textures and atlases before engine import and compression.

- **Assets_Source/Audio/**  
  Raw audio source material (SFX, VO, music stems).

- **Assets_Source/Animations/**  
  Source animation files for character rigs and cutscenes.

- **Build/README.md**  
  Notes on build procedures, targets, and packaging workflows.

- **Build/repo_index.json**  
  Auto-generated manifest of repository files and their inferred languages.

---

## 9. Git & Project Metadata

- **.gitignore**  
  Ignore rules for build artifacts, engine caches, and local config.

- **LICENSE**  
  License for the fan project’s original code and assets.

- **README.md**  
  Entry-point overview for the project and AI-chat usage pattern.
