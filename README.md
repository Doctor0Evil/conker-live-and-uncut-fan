### 🧱 Choosing the Tech Stack & Language

You’ll be building a modern game with modern tools, so it makes sense to lean on mature, well-supported technology. Here’s a practical, production-ready stack recommendation:

* **Game Engine:** Unreal Engine 5 (UE5), Unity, or Godot.  
  * **Why:** These engines are industry-standard (or rapidly emerging) with strong communities and robust systems for animation, physics, networking, and rendering. Recreating the original Xbox or N64 engine would be a huge time sink that adds little practical value to a modern fan project.  
  * **UE5 vs. Unity vs. Godot:** UE5 offers high-fidelity visuals out of the box and a strong multiplayer networking model that aligns well with the “Xbox-quality” vision for *Live & Uncut*. Unity is an excellent choice if you prefer C# and want a very flexible asset pipeline. Godot is a lightweight, open-source alternative that can be attractive for experimentation, custom tooling, and tight AI-Chat integration.

* **Primary Programming Language:**  
  * **C++** for Unreal Engine 5  
  * **C#** for Unity  
  * **GDScript/C#** for Godot  

These core languages can be complemented with **Rust** and **Lua** for tooling, scripting, and data pipelines where appropriate.

***

### 📦 Leveraging Existing Resources & Reverse-Engineering Work

You do not need to start from a blank slate. The community has already done substantial work on the original N64 version of *Conker’s Bad Fur Day*, which forms the single-player backbone of the cancelled *Live & Uncut* project.

#### 🏗️ Foundation: The N64 Decompilation Project

The `mkst/conker` repository is a work-in-progress decompilation of the N64 game into human-readable C. This is one of the most important technical references for this codebase because it allows you to:

* Understand game logic by seeing how the original handled character states, AI behavior, camera control, and physics.  
* Extract and mirror data structures so you can reason about how levels, actors, scripts, and assets are laid out in memory and on disk.  
* Port mechanics into a modern engine by using the decompiled C as a reference when re-implementing systems in C++ (UE5), C# (Unity), or GDScript/C# (Godot).

> **Important:** As with most decompilation-based projects, you are expected to supply your own legally acquired copy of the original N64 ROM for any tools or builds that depend on original game data.

#### 🛠️ Tools & Reverse Engineering

* **For Conker: Live & Reloaded (Xbox):** The `clr_unpack` tool and similar utilities can extract assets from the Xbox version. These tools are useful for studying updated models, textures, and file layouts, and for informing your own asset conversion pipeline. Modding and decensoring projects for *Live & Reloaded* have already reverse-engineered file types such as CAFF and RBM, which you can study as reference formats.  
* **For Low-Level Reverse Engineering (Advanced):** Tools like Ghidra and IDA Pro let you analyze compiled game binaries, recover control flow, and understand how engine subsystems handle things like scripting, sound triggers, and resource loading.

***

### 📁 A Suggested Project Structure

A clear, consistent project structure from day one is essential for attracting contributors and keeping AI-Chat output predictable. The following layout is tailored for GAMEMODE.ai workflows and large-codebase generation:

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
│   │   ├── 02_Multiplayer_Modes.md
│   │   └── 03_Asset_Pipeline.md
│   ├── ReverseEngineering/
│   │   ├── N64_Decomp_Notes.md
│   │   └── Xbox_File_Formats.md
│   └── AI_Chat_Context/
│       ├── Conker_Lore_Base.txt
│       └── System_Prompts.md
├── Engine/
│   ├── Unreal/
│   │   ├── ConkerLiveUncut.uproject
│   │   ├── Source/
│   │   │   ├── ConkerLiveUncut.Target.cs
│   │   │   ├── ConkerLiveUncutEditor.Target.cs
│   │   │   ├── Private/
│   │   │   │   ├── Core/
│   │   │   │   │   ├── CLUCharacterBase.cpp
│   │   │   │   │   └── CLUPlayerController.cpp
│   │   │   │   ├── Multiplayer/
│   │   │   │   │   ├── Heist/
│   │   │   │   │   │   └── CLUHeistGameMode.cpp
│   │   │   │   │   └── War/
│   │   │   │   │       └── CLUWarGameState.cpp
│   │   │   │   └── Utils/
│   │   │   │       └── CLUAssetLookup.cpp
│   │   │   └── Public/
│   │   │       ├── Core/
│   │   │       │   ├── CLUCharacterBase.h
│   │   │       │   └── CLUPlayerController.h
│   │   │       ├── Multiplayer/
│   │   │       │   ├── Heist/
│   │   │       │   │   └── CLUHeistGameMode.h
│   │   │       │   └── War/
│   │   │       │       └── CLUWarGameState.h
│   │   │       └── Utils/
│   │   │           └── CLUAssetLookup.h
│   │   └── Content/
│   │       ├── Characters/
│   │       ├── Maps/
│   │       ├── Blueprints/
│   │       └── Audio/
│   └── Tools/
│       ├── clr_unpack_rust/
│       │   ├── Cargo.toml
│       │   └── src/
│       │       └── main.rs
│       └── n64_asset_converter/
│           ├── Cargo.toml
│           └── src/
│               └── lib.rs
├── Assets_Source/
│   ├── Audio/
│   │   ├── Voice/
│   │   ├── SFX/
│   │   └── Music/
│   ├── Models/
│   │   ├── Characters/
│   │   │   ├── Conker.blend
│   │   │   └── Tediz.blend
│   │   └── Props/
│   ├── Textures/
│   │   └── UI/
│   └── Animations/
│       └── Conker_Rig.fbx
├── Build/
│   └── README.md
├── .gitignore
├── LICENSE
└── README.md
```

In this structure, `Engine/` contains the actual game project and code, which is where most contributors and AI-generated C++ will live. `Assets_Source/` separates raw art and audio files from engine-imported content, which simplifies version control and makes non-destructive iteration easier.

***

### ⚙️ The Development Workflow & Essential Tools

Building a multiplayer-heavy game demands a disciplined development workflow and the right supporting tools.

# Conker: Live & Uncut — Fan Revival Codebase

**Powered by GAMEMODE.ai**  
Professional-grade codebase generation for AI-Chat assisted development.

## 🎯 Project Intent

This repository is the foundational codebase and knowledge graph for reviving **Conker: Live & Uncut**—the uncensored, expanded Xbox remake of *Conker’s Bad Fur Day* that was shown at E3 2003 but never released. The goal is to produce a **fan-made, non-commercial** implementation that respects the original vision while taking advantage of modern tools and workflows.

The core objectives include:

- Restoring an uncut single-player experience with expanded dialogue and missing cutscenes inspired by early previews.  
- Implementing story-driven multiplayer scenarios such as **The Heist** (16-player bank robbery), **War** (Squirrels vs Tediz), and **Alien Base** co-op survival.  
- Integrating these modes into a modern engine (Unreal Engine 5, Unity, or Godot) while preserving the feel and timing of the original Xbox/N64-era gameplay.

This repository is intentionally structured for **AI-Chat assisted development**, with documentation and prompts tailored to large-language models and automated code generation.

## 🧠 GAMEMODE.ai System Rules (Context)

The project follows the **GAMEMODE.ai** methodology:

- **Supported Languages:** Rust, Lua, C++, MATLAB, Godot Script.  
- **Supported Engines:** Unreal Engine 5, Unity, Godot.  
- **Operational Directives (R):**  
  1. Build syntax, find tools, discover libraries, and analyze repository structures.  
  2. Identify research objects that require completion and produce finalized code with explicit file paths.  
  3. Generate valid, full-length code or documents without citation markers.  
  4. When a repository is mentioned, inspect its description and directory layout, then infer missing pieces and next coding steps.  
  5. Always suggest new directions, next objectives, and concrete improvement tasks.  
  6. Enhance AI-Chat codebase workflows via repo indexing, tagging, and lookup services.  
  7. Learn from top industry standards and practices in professional game development and narrative design.

## 🏗️ File Structure & Purpose

| Directory / File | Purpose | Primary Language |
| :--- | :--- | :--- |
| `.github/` | CI/CD workflows, build checks, and issue templates. | YAML, Markdown |
| `Docs/` | Game design documents, reverse-engineering notes, and AI-Chat context. | Markdown, Plaintext |
| `Engine/Unreal/` | Main UE5 project: gameplay, multiplayer modes, systems, and utilities. | C++ |
| `Engine/Tools/` | Custom tools for asset extraction and conversion from N64/Xbox builds. | Rust |
| `Assets_Source/` | Raw art and audio sources (Blender, FBX, WAV) prior to engine import. | N/A |
| `Build/` | Packaged builds and runtime artifacts (typically ignored by Git). | N/A |

### Key Files to Generate with AI

- `Engine/Unreal/Source/Private/Core/CLUCharacterBase.cpp` – Base squirrel character, locomotion, and inventory.  
- `Engine/Unreal/Source/Private/Multiplayer/Heist/CLUHeistGameMode.cpp` – Core logic for the bank heist multiplayer scenario.  
- `Engine/Tools/clr_unpack_rust/src/main.rs` – Rust CLI tool to parse Xbox CAFF/XPR archives.

## 🔗 Repository References for Reverse Engineering

These community repositories are reference material for behavior and file formats. They must not be copied verbatim into this project.

| Repository | Description | Research Objective |
| :--- | :--- | :--- |
| `mkst/conker` | N64 decompilation of *Conker’s Bad Fur Day* into C. | Recover original actor states, physics, camera, and asset structures. |
| `clr_unpack` | Asset extractor for *Conker: Live & Reloaded* (Xbox). | Study CAFF and related formats for asset conversion and tooling. |
| `ConkerModding` | Documentation and tools for decensoring *Live & Reloaded*. | Understand how censorship, triggers, and audio swaps were implemented. |

## 🛠️ Development Workflow for AI-Chat

To get the most out of AI-assisted coding:

1. **Load Context Files**  
   Start by feeding the AI the contents of `Docs/AI_Chat_Context/Conker_Lore_Base.txt` and `Docs/AI_Chat_Context/System_Prompts.md`. This anchors both tone and technical constraints before generating any code.

2. **Request Full File Implementations**  
   Use prompts that include the full file path and describe the complete behavior you want. For example:  
   “Generate the full C++ implementation for `Engine/Unreal/Source/Public/Multiplayer/War/CLUWarPlayerState.h`, including replication for team score and kill count.”

3. **Iterate and Refine**  
   After you receive a file from the AI, compile, test, and then request targeted refinements. Use the “Next Objectives” list below to advance systematically through the codebase.

***

### 🧰 Supporting Tools and Pipelines

Building a robust multiplayer game requires more than just an engine; it needs disciplined tooling around it.

| Tool Category | Recommended Tools | Why It Matters |
| :--- | :--- | :--- |
| Version Control | Git, GitHub or GitLab | Tracks changes, supports collaboration, and allows safe rollbacks. |
| Asset Creation & Audio | Blender, Maya, Photoshop, FMOD or Wwise | Provides a clean pipeline for models, textures, and sound into the engine. |
| Multiplayer & Networking | UE5 replication, Unity Netcode, or custom Godot networking | Multiplayer is central to *Live & Uncut*, especially 16-player scenarios. |
| Community & Legal | Discord server, clear non-commercial license (e.g., MIT with explicit non-commercial clause or custom fan license) | Creates a hub for contributors and clarifies how work can be used. |
| Prototyping | UE5 Blueprints, Unity Playmaker, or Godot visual scripting | Enables quick iteration on mechanics before committing to full C++ or C# implementations. |

***

### 🎯 Multiplayer Focus: Core Game Modes

The multiplayer component is the heart of *Live & Uncut*. It should guide your design and implementation priorities.

- **The Heist:** Four teams of four players infiltrate and rob a bank. You will need systems for team selection, vault cracking or drilling, money bag handling, and extraction, along with dynamic interactions between rival teams.  
- **War / Blitzkrieg:** Class-based, team-oriented combat between Squirrels and Tediz. This mode can highlight asymmetric roles, objective-based gameplay, and tightly tuned map design.  
- **Alien Base:** A co-op mode where players face waves of aliens, essentially a “horde mode” that tests your spawning systems, AI behavior, and moment-to-moment combat loop.

***

### 📋 Next Objectives & Coding Tasks

Here is a concrete starting roadmap for AI-assisted development in this repository:

1. **Core Movement and Animation**  
   - **Target:** `Engine/Unreal/Source/Private/Core/CLUCharacterBase.cpp` and its corresponding header.  
   - **Objective:** Implement a responsive movement system that evokes N64-era movement while benefiting from modern UE5 input, animation blueprints, and root motion.

2. **The Heist Game Mode Loop**  
   - **Target:** `Engine/Unreal/Source/Private/Multiplayer/Heist/CLUHeistGameMode.cpp`  
   - **Objective:** Implement team selection, objective flow (vault breach, money extraction), scoring, and network replication for key gameplay events.

3. **Asset Extraction Pipeline (Rust)**  
   - **Target:** `Engine/Tools/clr_unpack_rust/src/main.rs`  
   - **Objective:** Build a Rust CLI that reads `.xpr` and `.caff` files and converts them into standard formats like PNG and WAV for use in modern engines.

***

### ⚙️ Improvement Suggestions for AI Output

To keep AI-generated code maintainable and production-friendly:

- Emphasize strong **code quality**, following Unreal’s coding standards and using features like `UPROPERTY(Replicated)` and `UFUNCTION(Server, Reliable)` where appropriate.  
- Design for **modularity**, preferring small, focused components (such as `CLUInteractableComponent`) rather than monolithic classes that handle many responsibilities.  
- Implement **lookup services**, for example a `CLUAssetLookup` singleton that maps original asset IDs to modern engine asset paths, making it easier to recreate or script original sequences.  
- Favor **Rust** for tooling, as it provides memory safety, good performance, and cross-platform distribution with minimal friction.

***

### 🚫 Legal Disclaimer

This is a **fan-made, non-commercial** project created for educational and preservation purposes. *Conker*, *Conker’s Bad Fur Day*, *Conker: Live & Reloaded*, and related characters and assets are trademarks of Rare Ltd. and Microsoft Corporation. This repository does not distribute original game assets or ROMs. Users are responsible for owning a legal copy of the original software if they choose to use any asset extraction tools associated with this project.

***

*Last Updated: 2026-04-12*  
GAMEMODE.ai Active Session
