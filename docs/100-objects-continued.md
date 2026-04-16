Based on the comprehensive design documents and the data-driven pipeline established for the **Conker: Live & Uncut** fan project, the following 100 research topics, objectives, and coding directives are provided. This list is designed to fully specify the details of the seven multiplayer maps (`01_Multiplayer_Beach_Dead` through `07_Multiplayer_The_Blood_Count`) and the underlying systems, enabling AI-Chat to produce precise, schema-compliant code and assets for the repository at `github.com/Doctor0Evil/conker-live-and-uncut-fan`.

### 🏗️ Core Infrastructure & Tools (Grid2Scene, Data Schemas, Engine Integration)

1. **Objective:** Create a comprehensive JSON Schema (`map_grid_v1.schema.json`) to validate all map grid files, defining allowed `tile_type` enums, `role_tags` vocabulary, and cell properties.
2. **Objective:** Create a comprehensive JSON Schema (`map_entities_v1.schema.json`) to validate entity placement files covering spawns, pickups, hazards, NPC spawners, and objectives.
3. **Directive:** Refactor the `alien_base_grid2scene` Rust crate into a unified CLI that dynamically loads map data based on a `--map <id>` argument using a central Map Manifest.
4. **Directive:** Implement a `--validate` flag in the grid2scene tool that checks grid and entity JSON against their schemas and verifies that all `tile_type` and `role_tags` exist in the specified engine tileset.
5. **Research Question:** What is the most efficient method for handling texture atlasing and material instancing when generating levels from grid data in Unreal Engine 5, Unity 6, and Godot 4?
6. **Coding Improvement:** Modify the Unreal output emitter to generate a Python script that uses the Unreal Editor API to automate level construction (spawning ISM components, placing actors) instead of requiring manual JSON import.
7. **Coding Improvement:** Modify the Unity output emitter to generate a single C# script (`AlienBaseLevelBuilder.cs`) that builds the level in the Editor using `PrefabUtility.InstantiatePrefab` and the `Undo` system for proper integration.
8. **Objective:** Design a semantic versioning system for grid, entity, and tileset files (e.g., `schema_version` and `content_version` fields) to track changes and ensure tooling compatibility.
9. **Objective:** Create a "Map Manifest" JSON file (`map_manifest_v1.json`) that lists all available maps, their file paths, supported game modes, and recommended player counts, enabling batch processing by the grid2scene tool.
10. **Coding Improvement:** Add a `--dry-run` flag to the grid2scene tool that performs all validation and processing but writes no output files, returning only a summary report.
11. **Directive:** Ensure all generated C++/C#/GDScript code strictly adheres to the naming conventions established in `04_Multiplayer_Alien_Base_Entities.md` (e.g., `AAlienBase_Volume_HubFloorGas`, `Pickup_Chainsaw`).
12. **Objective:** For each engine, create an abstract "Base" actor class (e.g., `BP_Uncut_PickupBase`, `PickupBase.cs`, `PickupBase.gd`) that standardizes respawn timers, interaction ranges, and heavy carry status flags.
13. **Research Question:** What is the best practice for managing and versioning large binary assets (models, textures, sounds) using Git LFS in the GitHub repository to keep clone times manageable?
14. **Directive:** Create a CI/CD pipeline (e.g., GitHub Actions workflow `validate_maps.yml`) that automatically runs `grid2scene --validate --all` on every push and pull request to enforce data integrity.
15. **Objective:** Write a detailed guide (`docs/art/Tileset_Authoring_Guide.md`) for artists on creating new tileset assets that conform to the project's 4x4 unit grid scale, pivot point conventions, and naming scheme.

### 🏝️ Map-Specific Specifications (01-07)

#### 01_Multiplayer_Beach_Dead
16. **Research Question:** What is the exact geometric layout (length, angles) of the three "Fences" from the N64 Beach map, and how should they be translated into a grid of 4x4 tiles with `role_tags` `fence_1`, `fence_2`, `fence_3`?
17. **Objective:** Define the specific `role_tags` vocabulary for the Beach Dead grid, including `trench`, `attacker_spawn_band`, `defender_spawn_band`, `mg_nest`, and `bunker_interior`.
18. **Objective:** Create the `beach_dead_entities_v1.json` file, specifying exact grid coordinates for all defender MG nests, attacker spawn ships, and the final bunker objective trigger.
19. **Directive:** Implement the "fallback spawn" logic for Attackers: when a fence is destroyed, the attacker spawn band shifts forward to the next trench line.
20. **Coding Task:** Write a reusable component (C++/C#/GDScript) for the "Fence" objective that tracks health, updates visual states (intact/damaged/destroyed), and broadcasts a global event upon destruction to update spawn logic.
21. **Research Question:** What are the specific damage values, fire rates, and magazine sizes for the N64-era weapons (Pistol, Uzi, Sniper Rifle, Bazooka) to be used in Beach Dead?
22. **Objective:** Design the interior layout and `role_tags` for the fortress bunker (e.g., `commander_room`, `roof_access`) using the 4x4 grid system.

#### 02_Multiplayer_The_Heist
23. **Research Question:** How was the "gas chamber" instant-win mechanic triggered in the N64 Heist map (button, timer, tripwire), and what were the exact parameters of its countdown and area of effect?
24. **Objective:** Create the grid for The Heist, defining the central vault (`role_tag: vault`), the four team corridors (`team_red_spawn`, etc.), and the gas chamber area (`gas_chamber`).
25. **Objective:** Define the logic for the "Money Bag" pickup: its spawn location, the movement speed penalty (Heavy Carry), and the scoring condition when returned to a team base.
26. **Directive:** Design the environmental hazard trigger for the gas chamber as an interactable actor (`trigger_gas_chamber`) with a clear interaction prompt.
27. **Coding Task:** Implement the "Heist" game mode logic that tracks which team holds the money bag, awards points upon delivery, and triggers the gas chamber win condition.
28. **Research Question:** Are there any unique architectural features (e.g., specific vault door mechanisms, teller windows) from the Feral Reserve Bank in the single-player game that should be incorporated into the multiplayer map's visual design?

#### 03_Multiplayer_Fortress
29. **Research Question:** How does the "Total War" game mode's gas canister mechanic translate to a larger 16-player environment? Should it be retained as an instant-win or adapted as a temporary area denial hazard?
30. **Objective:** Define the tile palette for Fortress, focusing on industrial and war-torn variants as specified in `03_Multiplayer_Fortress_Tile_Palette.md`.
31. **Objective:** Create the grid for Fortress, defining the central valley/bridge, the two main bases (`shc_base`, `tediz_base`), and the network of trenches (`trench`).
32. **Directive:** Design the capture point logic for Fortress. Should it be a single central bridge control point, multiple towers, or a "tug-of-war" style front line?
33. **Objective:** Specify the placement of heavy weapon pickups (Bazooka, Chaingun) in the Fortress map, ensuring they are in high-risk, exposed positions.
34. **Coding Task:** Implement the gas canister logic (if retained): player pickup with Heavy Carry penalty, and the ability to "arm" it at an enemy base to trigger a map-wide hazard countdown.

#### 04_Multiplayer_Alien_Base
35. **Research Question:** What are the specific geometric bounds (radius, height) for the `hazard_hub_floor_gas` and `hazard_sublevel_acid` volumes in world units to ensure they cover the intended grid cells?
36. **Objective:** Finalize the `alien_base_hub_grid_v1.json` to include all necessary tiles for a complete greybox, including the outer wall ring, catwalk tiles, and all corridor entrances.
37. **Coding Task:** Implement the `AlienBaseAirlockController` state machine (Idle, Arming, Active, Cooldown) in C++ (Unreal), C# (Unity), and GDScript (Godot) based on `04_Multiplayer_Alien_Base_Triggers.md`.
38. **Directive:** Create the "Alien Egg" objective actor. It should have health, visual stages (cracking, pulsing), and trigger the airlock hazard or alien spawns when damaged or destroyed.
39. **Objective:** Define the spawn points and patrol routes (using `role_tags` like `alien_vent_spawn`) for the Alien NPCs in the "Invasion" game mode variant.
40. **Coding Task:** Implement the damage-over-time logic for the hazard volumes, ensuring it correctly checks for and respects the execution state immunity defined in the ASID system.

#### 05_Multiplayer_Raptor_Temple
41. **Research Question:** What were the original N64 Raptor mode's specific mechanics for Raptors feeding cavemen to the baby dino (how many required?) and Cavemen cooking eggs?
42. **Objective:** Translate the N64 Raptor mode mechanics into a 16-player Uncut environment. Should it remain asymmetrical (Raptors vs. Cavemen) or be adapted into a team-based objective mode where both sides can do both?
43. **Objective:** Create the grid for Raptor Temple, defining the S-shaped valley, the central two-story temple (`temple_ground`, `temple_upper`), and the bases (`uga_base`, `raptor_nest`).
44. **Directive:** Design the "Egg" and "Baby Dino" objectives. How are they represented in the world? How do players interact with them to score?
45. **Coding Task:** Implement the "Raptor Pounce" and "Caveman" abilities as distinct character states or temporary power-ups, adhering to the Uncut "no classes" philosophy.
46. **Research Question:** What is the intended layout of the central temple's interior? Should it have multiple floors, traps, or specific chokepoints like the N64 Temple deathmatch map?

#### 06_Multiplayer_TMS_Spamono
47. **Research Question:** Based on the Live & Reloaded implementation and cut content, what are the exact dimensions and layout of the T-shaped corridor in TMS Spamono?
48. **Objective:** Create the grid for TMS Spamono, defining the long central corridor (`corridor_main`), the "airlock" sections (`airlock`), and the team bases at each end (`team_a_spawn`, `team_b_goal`).
49. **Directive:** Design the objective for TMS Spamono. Adapt the "code capture" concept to a non-class-based mode, perhaps requiring a player to carry a "Data Core" pickup to the enemy goal.
50. **Coding Task:** Implement the "airlock" door logic for the central corridor. When should it open or close? Is it tied to an objective timer or a player-activated trigger?
51. **Objective:** Define the placement of weapon pickups along the central corridor to encourage strategic pushes and defense, avoiding a static stalemate.
52. **Coding Task:** Implement the `SpamonoCorridor` helper struct in Rust as defined in the crates to assist with AI navigation and scripting for bots or future AI director logic.

#### 07_Multiplayer_The_Blood_Count
53. **Research Question:** What are the exact canonical names and relative positions of the four wings off the central hall in Count Batula's Mansion (e.g., Library, Crypt, etc.) to correctly assign team spawn themes?
54. **Objective:** Create the grid for The Blood Count, defining the mansion interior (`central_hall`), the four team wings (`team_red_spawn`, etc.), the exterior hedge maze (`hedge_maze`), and the library (`library`).
55. **Coding Task:** Implement the "Panther King's Blood-Vial" pickup logic: applying the `ASID_050` Heavy Carry state, dropping on death, and triggering the Fire Imp spawn.
56. **Directive:** Design the spawning and respawning logic for the blood-vial. What are its possible spawn locations (e.g., `blood_vial_spawn_1`)? What is the decay timer, and how is it communicated to players (e.g., particle effects, HUD timer)?
57. **Coding Task:** Implement the "Fire Imp" AI behavior. It should be inactive until the vial is picked up, then relentlessly hunt the carrier using NavMesh pathfinding.
58. **Objective:** Define the zombie spawn points, density caps, and patrol areas for The Blood Count, using `npc_spawner` entities with `max_alive` parameters.
59. **Coding Task:** Implement the zombie "crawl" state logic: when limb/body damage exceeds a threshold, transition to a crawling animation and movement mode, but only allow death from headshots or close-range shotgun blasts.
60. **Research Question:** What is the ideal respawn time for the Fire Imp after being killed (e.g., 15 seconds) to provide a balanced "breathing room" for the carrier?

### 🧠 Systems, Mechanics & ASID Registry

61. **Objective:** Create a master `docs/systems/Animation_State_Registry.md` file that lists all ASIDs, their properties (Interrupt Priority, Lock Type, Gore Trigger Frame), and associated SFX/VFX.
62. **Coding Task:** Implement the core ASID state machine for player characters in all three engines. It must enforce "Hard Lock" rules, preventing movement, jumping, or other actions during `FIN_` execution animations.
63. **Coding Task:** Implement the "Heavy Carry" state (`ASID_050`) that overrides movement speed, disables jumping, and prevents weapon use.
64. **Directive:** Create a "Gore Manager" system responsible for spawning the correct gib meshes (`OBJ_GORE_MUSH`, etc.) and particle effects (`VFX_010`, `VFX_015`) at specific bone locations based on the ASID and Gore Trigger frame.
65. **Research Question:** How should the "Stun Lock" state (`ASID_012`) be implemented to balance melee combat and heavy weapon hits, ensuring it's impactful but not frustrating (e.g., 1.5-second duration with reduced turn speed)?
66. **Objective:** Define a complete table of weapon statistics for all Uncut weapons (Chainsaw, Katana, Crossbow, Shotgun, Flamethrower, Hunting Revolver, etc.) in `weapon_stats_v1.json`.
67. **Coding Task:** Implement the weapon pickup system. Ensure it correctly handles ammo, weapon switching, and toggles the "no jump" flag for heavy weapons.
68. **Objective:** Design the "Profanity Toggle" system. How will the game load either uncensored (`VO_*`) or bleeped audio files based on a user setting, defaulting to "Uncut"?
69. **Coding Task:** Implement the "Line-of-Sight (LoS)" spawning logic for 16-player matches to prevent spawn camping, using a configurable radius (e.g., 20.0 units) and fallback spawn nodes.
70. **Directive:** Create a unified "Damageable" interface (e.g., `IDamageable` in C#, an Unreal `UInterface`, a Godot `Node` with a `take_damage` method) for all actors that can receive damage.
71. **Objective:** Define the rules for "Friendly Fire" in team-based modes. Should it be enabled or disabled? How does it interact with execution animations (e.g., can you accidentally kill a teammate with a Katana sweep)?
72. **Research Question:** What is the best way to implement the "context-sensitive" action button (B-button) from the N64 original in a modern multi-platform context (e.g., using an `Interactable` interface)?

### 👾 AI & NPC Behavior (Zombies, Aliens, Fire Imp)

73. **Coding Task:** Implement the base Zombie AI as a simple state machine (Idle, Patrol, Chase, Attack, Crawl) using each engine's built-in pathfinding (NavMesh).
74. **Directive:** Implement the Zombie damage model: they only take lethal damage from headshots or point-blank shotgun blasts to the head/upper torso. Body shots cause them to enter the "Crawl" state.
75. **Objective:** Create a spawn manager for zombies that can activate spawn points based on player proximity and maintain a target density in different map zones, using data from `npc_spawner` entities.
76. **Coding Task:** Implement the Alien NPC AI for Alien Base: focus on "pounce" (`ASID_900`) and "facebite" (`ASID_901`) execution attacks.
77. **Directive:** Design the Alien's pathfinding to utilize vents and ceilings, using special NavMesh areas or off-mesh links to make them a threat from multiple angles.
78. **Coding Task:** Implement the Fire Imp's unique targeting logic: it should only become active and target players when the blood-vial is held, and prioritize the carrier.
79. **Objective:** Create a "threat assessment" system for AI that allows them to prioritize targets (e.g., the blood-vial carrier, a player performing an execution, the nearest attacker).
80. **Research Question:** How can AI behavior be optimized for 16-player matches with dozens of active NPCs to maintain stable performance (e.g., using LOD for AI updates, spatial partitioning)?
81. **Coding Task:** Implement a "horde mode" director AI for Invasion-style games that dynamically adjusts alien spawn rates and types based on player performance metrics (kills, deaths, objective progress).

### 🔊 Audio-Visual & VFX Registry

82. **Objective:** Create a master `docs/systems/Audio_Visual_Registry.md` file that catalogs all SFX IDs (e.g., `SFX_001`, `SFX_800`) and VFX IDs (e.g., `VFX_010`, `VFX_020`) with their descriptions and intended use.
83. **Coding Task:** Implement an "Audio Manager" that can play sounds with correct 3D spatialization, attenuation, and random pitch variation as specified in the registry.
84. **Directive:** Create a "Decal Manager" that handles the infinite lifetime and culling (FIFO, max count 200) for gore decals like `PRT_GORE_LIME` to maintain performance.
85. **Objective:** Define the visual language for the "Airlock Hazard" telegraph: the sequence of sirens, flashing lights, and venting steam (`VFX_080`) that precede the gas release in Alien Base.
86. **Research Question:** What modern rendering techniques (e.g., bloom, ambient occlusion, volumetric fog) can be used to enhance the "N64 but up-rezzed" aesthetic without betraying the original visual style's high-contrast, saturated look?
87. **Coding Task:** Implement the "Chainsaw Sparks" logic: when the chainsaw hits a surface with the `MAT_METAL_BASE` tag, spawn `VFX_020` instead of gore.

### ⚡ Performance & Optimization for 16 Players + Split-Screen

88. **Objective:** Establish performance budgets for each target platform (PC, Xbox, potentially Switch-like handhelds) for CPU (AI, physics), GPU (draw calls, shaders), and memory.
89. **Directive:** Implement a Level of Detail (LOD) system for all map tiles and character models to reduce polycount at a distance, using engine-specific LOD groups.
90. **Coding Task:** Implement the foliage/entity pooling system as described in the C++ snippets to minimize instantiation overhead for common effects (gore, bullet impacts) and NPCs (zombies).
91. **Research Question:** How can the grid2scene tool be used to pre-compute occlusion culling data (e.g., a simple PVS or portal system) to improve rendering performance in dense indoor maps like Alien Base and The Blood Count?
92. **Objective:** Create a performance test map (`perf_test_grid_v1.json`) that can spawn 16 players and hundreds of zombies to stress-test and profile the game's systems.
93. **Directive:** Optimize network code for online play. Use relevancy and prioritization to only send critical updates for distant NPCs and players, reducing bandwidth usage.

### 📚 Documentation & Tooling for AI-Chat

94. **Objective:** Create a `CONTRIBUTING.md` file that explains the data-driven pipeline (JSON schemas, grid2scene tool) and provides a step-by-step guide on how to add a new map or modify an existing one.
95. **Directive:** Add extensive `"description"` fields and comments to all JSON schemas and example files, explaining the purpose and expected values of each field for AI-Chat context.
96. **Objective:** Create a template file (`map_template.md`) that can be copied and filled out to create a design document for any new map, including standard sections for layout, modes, and mechanics.
97. **Directive:** Write a "Glossary of Terms" document (`docs/glossary.md`) defining all project-specific jargon (ASID, `role_tag`, Heavy Carry, Uncut philosophy, etc.) for AI-Chat context.
98. **Objective:** Create a simple web-based viewer for the map grid JSON files that renders a 2D top-down view of the tiles, color-coded by `tile_type`, to aid in design and debugging.

### 🧪 Community & Playtesting

99. **Directive:** Design a system for toggling game rules and variables (e.g., zombie density, hazard timers, friendly fire) via a configuration file (`gameplay_config.json`) or console commands to facilitate rapid playtesting and balancing.
100. **Objective:** After generating a greybox for a map, create a playtesting checklist specific to that map (e.g., `beach_dead_playtest_checklist.md`) that covers all unique features (e.g., "Can you destroy Fence 1?", "Does the Fire Imp spawn correctly?").

Building upon the foundational work laid out in the previous 100 objectives, and specifically integrating the now‑first‑class status of **Fortress** and **Alien Base**, the following research topics and engineering directives are designed to ensure the fan‑made *Conker: Live & Uncut* not only replicates the **N64 *Bad Fur Day* multiplayer feel** but surpasses the limitations of the original hardware and the compromised *Live & Reloaded* release.

These items are structured to train AI‑Chat agents to produce code that balances **exact preservation of N64 game mechanics** (timing, collision, input response) with **modern scalability** (16+ players, dense AI hordes, high particle counts) while operating within a reasonable memory budget (e.g., 512 MB RAM) and without reliance on emulation.

---

### 101–120: Core Gameplay Fidelity & N64 Mechanical Preservation

101. **Research Question:** What is the exact acceleration curve, friction value, and air control coefficient of Conker's movement in the N64 *Bad Fur Day* engine? Provide a mathematical model to be implemented in modern physics engines (PhysX/Godot Physics/Jolt) to ensure the "floaty" yet snappy N64 platforming feel is preserved.
102. **Research Question:** Document the precise damage falloff and spread patterns for the N64 Shotgun and Uzi. How can we replicate the "projectile" vs "hitscan" hybrid nature of the original Bazooka in a modern networked environment?
103. **Objective:** Create a **Time Step Calibration Tool**. This tool should measure input-to-photon latency of the modern PC port against an original N64 console running on a CRT to achieve sub-frame parity in movement response.
104. **Coding Task:** Implement the N64-specific **"B-Button Context Zone"** system. This requires a volume-based trigger system that overlaps Conker's interaction sphere and switches the action prompt based on `interaction_type` (e.g., `USE_SWITCH`, `CLIMB_LADDER`, `PUSH_BLOCK`).
105. **Research Question:** How did the N64 *Bad Fur Day* handle collision detection for the "Tail Spin" and "Frying Pan" melee attacks? What is the exact hitbox shape, active frame window, and knockback vector to recreate the "juggling" combo potential?
106. **Directive:** Analyze frame data from the N64 original to build a **Master Animation Timing Table** (`animation_timing_v1.json`) that maps every `ASID` to exact duration in milliseconds. This table must be the single source of truth for the ASID state machine to prevent animation cancel exploits.
107. **Coding Task:** Implement a **Deterministic Physics Rollback System** for offline/LAN play. While online will use server-authoritative, the single-player and local split-screen experience must exactly replicate the N64's local frame-perfect behavior.
108. **Research Question:** What is the memory layout and compression algorithm used for the N64 "MusyX" audio bank? How can we extract the raw ADPCM samples and impulse responses for reverb to achieve an acoustically identical soundscape on modern audio hardware?
109. **Coding Task:** Create a **Camera Collision System** that mimics the N64 *Bad Fur Day* behavior: the camera must *slide* along geometry rather than pop forward, and it must have the exact same "ceiling bump" stiffness.
110. **Objective:** Define a strict **Vertical Slice Target** spec: "The first 30 seconds of Beach Dead gameplay on a Windows 10 PC must be indistinguishable from N64 footage in terms of control, animation timing, and audio sync when viewed at 240p."

### 121–140: Performance Optimization for High Density (512MB RAM / High Particle Counts)

111. **Research Question:** What is the most CPU-efficient method for managing 16-player **Execution Immunity (ASID)** lookups? Compare ECS (Entity Component System) bitmasks against standard OOP virtual calls to ensure 60+ FPS during Katana multi-kills with full gore VFX.
112. **Directive:** Design a **Tiered Particle Pooling System** that allocates a fixed 64MB memory block for VFX. Define the maximum concurrent instances for `VFX_010` (Blood Spray), `VFX_080` (Steam), and `VFX_020` (Sparks) with a smart FIFO culling system that prioritizes particles close to the player camera.
113. **Coding Task:** Implement **Aggressive Mesh LODing for Tilesets**. The `grid2scene` tool must generate 3 distinct LOD levels for each 4x4 tile (Full Poly, Imposter Card, and fully Culled) to keep draw calls under 1,500 in dense areas like Fortress trenches.
114. **Research Question:** How can we leverage modern GPU Compute Shaders to offload the **Zombie Crawl Transition Logic**? The N64 handled this on CPU; can we move the limb-damage threshold checks to the GPU to support hundreds of active zombies?
115. **Objective:** Create a **Memory Budget Dashboard**. A runtime overlay that tracks texture memory (target: 128MB), audio banks (target: 64MB), and dynamic actor allocations to ensure the game never exceeds the 512MB soft limit on low-end integrated graphics.
116. **Coding Task:** Implement a **Visibility Culling Volume** specifically for the **Alien Base Hub Floor Gas**. The particle VFX and damage tick should be globally disabled if no player is within a 40-unit radius of the hazard volume to save CPU cycles.
117. **Directive:** For the **Blood Count Zombie Horde**, implement an **Animation LOD System**. Zombies beyond 30 units update their skeletal mesh at 15 FPS; beyond 60 units, they swap to a vertex-animated shader (VAT) or a simple billboarded sprite.
118. **Research Question:** What is the optimal spatial hashing algorithm (e.g., Grid Hash vs. Octree) for managing the **Fire Imp's** hunting logic across the expansive Fortress map to ensure pathfinding updates do not stall the main thread?
119. **Objective:** Profile the **grid2scene Rust tool** for memory usage when compiling the 200x200 grid for Raptor Temple. Optimize the intermediate data structures to ensure the tool can process all 7 maps in under 5 seconds on a 4-core machine.
120. **Coding Task:** Write a **Custom Memory Allocator** (C++/Rust) for the game runtime that pre-allocates slabs for common objects: `ConkerPlayerState` (16 slots), `ZombieAI` (64 slots), and `BulletProjectile` (128 slots).

### 141–160: Asset Pipeline & Data-Driven Content Generation

121. **Research Question:** Can we automate the conversion of N64 **Display Lists (F3DEX2)** into modern **glTF 2.0** models using a custom Python script? Focus on preserving vertex colors (used extensively for lighting in N64 Conker) as emissive or base color layers in modern shaders.
122. **Objective:** Create a **Texture Atlas Baker** that takes the thousands of tiny 32x32 and 64x64 N64 RGBA16 textures and packs them into optimized 2048x2048 atlases while preserving the exact UV coordinates in the map grid JSON.
123. **Directive:** Develop an AI-Chat prompt template for generating **Map Grid JSON** from a simple ASCII drawing. Example: Input `SSS...TTT` outputs a valid `fortress_main_grid_v1.json` with correct `tile_type` enums.
124. **Coding Task:** Enhance the `grid2scene` tool to output a **Heatmap Preview** (PNG image) of the map where cells are colored by `role_tag` (Red=Spawn, Blue=Objective, Green=Hazard). This allows designers to validate flow without loading the engine.
125. **Research Question:** How can we use the **N64 Conker Decompilation** (`mkst/conker`) to auto-generate the `weapon_stats_v1.json` file? Write a parser that reads the C structs for `weaponInfo` to ensure 100% damage value accuracy.
126. **Objective:** Establish a **Modular Tileset Standard**. Document the pivot point (0,0,0 at bottom center) and collision primitive (a single box or slope mesh) so artists can create new "Uncut" tiles (e.g., expanded Fortress bunker) that snap perfectly to the existing N64 grid.
127. **Coding Task:** Implement a **Runtime Tileset Swapper**. This system should allow a single map grid to load a "Low Poly" tileset or a "High Fidelity" tileset based on the user's graphics settings without changing the underlying gameplay collision data.
128. **Directive:** Create a **Gore Decal Manager** that uses a single texture array for all blood splatters (`PRT_GORE_LIME`, `PRT_GORE_RED`). This reduces draw calls from 200 individual decals to just 1 or 2 batches.

### 161–180: Network Architecture for 16-Player Fidelity

129. **Research Question:** What is the exact network bandwidth cost of synchronizing the **Alien Egg's** 4-stage health and visual cracking state? Design a compact bit-packed struct to update 16 clients with minimal overhead.
130. **Coding Task:** Implement **Client-Side Prediction for Heavy Carry**. When a player picks up the Blood-Vial or Money Bag, the client should immediately apply the `ASID_050` speed penalty before the server confirms it, rolling back only if the server denies the pickup.
131. **Objective:** Design a **Lag Compensation System** for the **Chainsaw**. Since the Chainsaw is a continuous hitbox, document how to rewind the victim's position on the server to validate the hit based on the attacker's latency (up to 150ms).
132. **Directive:** Create a **Network Priority Scheduler**. During the **Fortress Gas Canister** countdown, voice lines (`SFX_800`) and siren VFX updates should take bandwidth priority over distant zombie footstep sounds.
133. **Research Question:** How can we implement a **Mesh Replication** strategy for the destructible **Beach Dead Fences**? Should they be replicated as a simple byte state (0-100 health) or as physics-enabled debris?
134. **Coding Task:** Implement **Deterministic AI Pathfinding** for the **Fire Imp**. The Imp's movement must be predictable so that the server can run the AI logic and the clients can simply interpolate the position without constant correction snapping.

### 181–200: Community, AI-Assisted Development & Tooling

135. **Objective:** Create a **`AI_CHAT_KNOWLEDGE.md`** file. This document must distill the 200+ pages of design docs into a 5,000 token context window primer on the project's **ASID** philosophy, **Grid2Scene** data flow, and **Uncut Multiplayer** goals for code generation LLMs.
136. **Directive:** Build a **Headless Game Server** executable. This build of the game should run without rendering (using a Null renderer) to allow for dedicated server hosting on Linux machines with just 256MB of RAM.
137. **Research Question:** What is the legal and technical process for loading **Original N64 Voice Lines** from a user-supplied ROM file at runtime? Can we design a hashing system that verifies the user owns the correct `baserom.us.z64` before enabling the uncensored VO pack?
138. **Objective:** Write a **Fuzz Testing Harness** for the `mapgridv1.schema.json` validator. This tool should generate millions of random but schema-valid JSON files to ensure the `grid2scene` Rust crate never panics or crashes.
139. **Coding Task:** Implement **Split-Screen Profile Management**. On PC, allow Player 1 to use Keyboard/Mouse while Players 2-4 use Xbox controllers, with individual audio mix settings and Profanity Toggles per player.
140. **Directive:** Create a **"Developer Commentary" Mode** triggerable via console command (`uncut_commentary 1`). When toggled, floating text bubbles appear in maps explaining the history of the E3 2003 demo features (e.g., "Here is where the Helicopter Gunner seat would have been in Live & Uncut.").


To achieve a total of **200** comprehensive research and development directives, here are **60 additional items** (201–260) specifically engineered to **sharpen AI‑Chat accuracy** for the *Conker: Live & Uncut* codebase. These items focus on creating **machine‑readable context**, **enforced coding patterns**, **automated test generation**, and **prompt engineering** that will make AI‑Chat a reliable contributor to the project.

### 201–220: AI-Chat Context Priming & Prompt Engineering

201. **Objective:** Create `AI_CHAT_SYSTEM_PROMPT.md`—a single Markdown file containing the **Project Manifesto**, **Glossary**, **File Structure Overview**, and **Core Philosophy** (Uncut, No Classes, Heavy Carry) to be injected as the first message in any AI‑Chat session.
202. **Directive:** Develop a **Schema‑Aware Prompt Template** for generating new map grids. The prompt must instruct the AI to first validate its JSON against `mapgridv1.schema.json` using a provided example before outputting final code.
203. **Coding Task:** Write a Python script (`verify_ai_output.py`) that AI‑Chat can be instructed to run on its own generated JSON to catch schema errors *before* the user sees the response. Provide usage instructions for the AI.
204. **Research Question:** What is the optimal token limit and compression strategy for `AI_CHAT_KNOWLEDGE.md` to fit within 8k context windows while retaining critical details about ASID execution chains and Grid2Scene emitters?
205. **Objective:** Create a **"Common AI Mistakes" Log** (`docs/ai_common_errors.md`). This file lists past hallucinations (e.g., misnaming `tile_type` enums, confusing `ASID_050` with `ASID_012`) and explicit corrections to train future model versions.
206. **Directive:** Establish a strict **AI Output Format Contract**. For every code block generated, the AI must include a header comment with `@generated by AI-Chat`, `@schema_version`, and `@expected_map` to track provenance.
207. **Coding Task:** Add a **`--ai-dry-run`** flag to the `grid2scene` Rust tool that prints a human‑readable summary of what the tool *would* do, specifically designed for AI agents to verify their logic without touching the filesystem.
208. **Objective:** Design a **Unit Test Generator Prompt**. Provide AI with a C++/C# function signature (e.g., `CalculateGasDamage`) and ask it to output a full Google Test/NUnit test fixture covering edge cases (full health, ASID immunity, max distance).
209. **Directive:** Create **`.cursorrules`** and **`.github/copilot-instructions.md`** files in the repository root that teach AI‑powered IDEs to always reference the local schema files before suggesting code.
210. **Research Question:** How can we use **Retrieval‑Augmented Generation (RAG)** with the `Docs/` folder to ensure AI‑Chat answers about Fortress gas mechanics are drawn verbatim from `Fortress_Gas_Canister_Research.md` rather than from memory of *Live & Reloaded*?

### 211–220: Automated Validation & Self‑Healing Code

211. **Coding Task:** Implement a **JSON Patch Generator** in the `grid2scene` tool. When validation fails, the tool should output a suggested `json-patch` file that AI‑Chat can apply to fix the error (e.g., adding missing `required` fields).
212. **Objective:** Create an **"AI‑Readable Error Code"** system. Instead of generic Rust panic messages, the validator must emit codes like `E-MAP-001 (Cell out of bounds)` and `E-ENT-005 (Unknown role_tag)` which AI can self‑correct against a known dictionary.
213. **Directive:** Write a **Pre‑Commit Hook** (`pre-commit-ai-check`) that runs schema validation and blocks commits if generated JSON files don't pass. The hook must provide a clear, AI‑friendly error report in Markdown format.
214. **Research Question:** Can we fine‑tune a small LLM (e.g., CodeLlama 7B) on the `Docs/GDD/` and `schemas/` folders to create a local **Conker Code Assistant** that understands the difference between `tile_type` and `role_tag` natively?
215. **Coding Task:** Create a **Snippet Library** (`ai_snippets.json`) containing canonical implementations of `ASID_StateMachine.Tick()`, `HazardVolume.ApplyDamage()`, and `PickupBase.Interact()`. AI‑Chat should be instructed to copy‑paste from this library rather than hallucinating new logic.
216. **Objective:** Build a **Semantic Version Checker** that compares the `schema_version` in an AI‑generated file against the current project version and issues a warning if the AI is using an outdated template.
217. **Directive:** For the **Unreal Engine Emitter**, ensure the generated Python script includes a `try-except` block for every `unreal.EditorLevelLibrary.spawn_actor_from_class` call, logging clear errors that AI‑Chat can interpret and fix in subsequent prompts.
218. **Coding Task:** Create a **"Heal Map"** function in the Unity `AlienBaseLevelBuilder.cs` that iterates over all placed tiles and automatically replaces any `Unknown` or `Placeholder` tile references with a bright pink "ERROR" material to visually flag AI‑generated issues.
219. **Objective:** Write a **YAML to JSON Converter** spec for AI‑Chat. Since LLMs are often better at generating readable YAML, allow AI to output grid data in YAML and provide a deterministic script to convert it to strict JSON.
220. **Research Question:** How does the presence of `// AI_GENERATED: DO NOT EDIT MANUALLY` comments affect the ability of LLMs to understand and maintain that file in future sessions?

### 221–240: Asset & Content Consistency for AI Generation

221. **Objective:** Create a **Master Tile Dictionary** (`tile_registry_v1.json`) that maps every `tile_type` string (e.g., `alien_catwalk_corner`) to a **canonical asset path** and a **preview icon URL**. AI should use this registry to ensure it never invents a tile name that doesn't exist in the project.
222. **Directive:** Develop an **AI‑Assisted Texture Packer** script. AI‑Chat can be instructed to propose which N64 textures to combine, and the script will validate if they fit in a 2048x2048 atlas without overlap, providing immediate feedback.
223. **Coding Task:** Implement a **Dummy Asset Server** (local Flask app) that serves placeholder models when AI requests an asset that hasn't been created yet, allowing AI‑Chat to proceed with layout generation even if the art pipeline is behind.
224. **Research Question:** What is the most AI‑friendly format for describing **NavMesh bounds**? Can we define a simple text‑based polygon format that AI‑Chat can easily generate for the `alien_vent_spawn` patrol routes?
225. **Objective:** Create an **Audio Cue Sheet** (`audio_cues_v1.csv`) with columns: `SFX_ID`, `Description`, `3D Attenuation Range`, `AI_Context_Note`. AI‑Chat uses this to correctly assign `PlaySFX_AtLocation(SFX_801)` rather than hardcoding "play scream sound."
226. **Directive:** Build a **"Map Linter"** that checks for **"Softlocks"** in AI‑generated layouts. For example, it must warn if `team_red_spawn` has no path to the central objective due to a wall of `impassable` tiles.
227. **Coding Task:** Write a **GDScript Validator** that AI‑Chat can run within the Godot Editor to verify that all nodes referenced in generated script (e.g., `$AnimationPlayer`) actually exist in the associated scene.
228. **Objective:** Establish a **Naming Convention Enforcement Script** that scans AI‑generated C++ classes and verifies they start with `A` for Actors or `U` for Objects, as per Unreal Engine standards referenced in the design docs.
229. **Research Question:** Can we use **Computer Vision** (e.g., comparing screenshots of the N64 original vs. the AI‑generated greybox) to automatically score the "visual fidelity" of a map layout?
230. **Directive:** Create a **"Copy‑Paste" Macro Library** for AI‑Chat. For example, when AI writes `// !ASID_CHECK_START`, it should automatically expand into the full boilerplate code for ASID immunity verification.

### 231–240: Training Data & Synthetic Example Generation

231. **Objective:** Generate **100 Synthetic Map Grids** using a procedural algorithm that follows the project's schema. These grids (with varying sizes and tile combos) will be used as **Few‑Shot Examples** in AI prompts to demonstrate the correct format.
232. **Coding Task:** Write a script that parses the **N64 Decompilation Code** (`mkst/conker`) and outputs a **Markdown‑formatted list of every ASID** with its actual C code implementation block. This is raw training material for AI‑Chat.
233. **Research Question:** How can we use **Git History** to train AI‑Chat on the evolution of a specific file (e.g., `GasCanister.cpp`)? Can we provide diffs showing the "before" and "after" of a bug fix to teach the AI to avoid that bug?
234. **Directive:** Create a **"Bad Example" Dataset**. Deliberately create invalid JSON files (wrong enum, missing field) and pair them with the exact validation error message. AI‑Chat can use this to learn the project's strictness.
235. **Objective:** Build a **Web‑Based Grid Painter** that exports valid JSON. This tool serves as a **"Ground Truth" Generator**—AI can be asked to "modify the grid that matches this human‑drawn layout" rather than inventing coordinates from scratch.
236. **Coding Task:** Implement a **Mock Game Session Recorder** that logs every function call made during a 60‑second match of Beach Dead. Provide this log as context for AI‑Chat to understand the runtime sequence of events.
237. **Research Question:** What is the optimal way to encode **3D Transform Data** in JSON for AI‑Chat? Is `[x,y,z]` array better than `{"x": 1.0, "y": 2.0}` for reducing token waste and improving coordinate prediction accuracy?
238. **Directive:** Create **`ai_prompt_templates/`** directory. For each common task (e.g., "Add a new pickup to Fortress"), provide a filled‑out template showing exactly where the AI should insert new data.
239. **Objective:** Write a **State Machine Visualizer** that converts the `AlienBaseAirlockController` enum states into a Mermaid.js diagram. AI‑Chat can read this diagram to understand the flow much faster than parsing C++ code.
240. **Coding Task:** Create a **"Reverse Prompt"** tool. Given a valid `mapgrid_v1.json` file, this tool outputs the exact prompt that *would* generate that file, teaching AI‑Chat the mapping between natural language and JSON structure.

### 241–260: Cross‑Engine Compatibility & AI‑Assisted Refactoring

241. **Research Question:** How can we use AI‑Chat to **transpile** the core `ASID_StateMachine` logic from Unreal C++ to Unity C# and Godot GDScript *simultaneously*, ensuring API differences (e.g., `GetWorld()->GetDeltaSeconds()` vs `Time.deltaTime`) are handled correctly?
242. **Objective:** Create a **Shared Behavior Definition File** (`behavior_manifest_v1.json`) that defines game rules (e.g., `Zombie_Headshot_Multiplier: 5.0`). AI‑Chat should update this one file instead of touching engine‑specific code.
243. **Coding Task:** Implement a **"Bridge Pattern"** generator. AI‑Chat is instructed to write an interface `IWeaponSystem` and then generate three concrete implementations (`UnrealWeaponSystem`, `UnityWeaponSystem`, `GodotWeaponSystem`) based on a single English description.
244. **Directive:** Ensure that the **Unreal Python Emitter** script includes `unreal.log_warning()` calls for any cell tagged with `role_tag` that doesn't have a corresponding Actor Class mapped in the Mode Profile. This is a critical AI‑Chat debugging hook.
245. **Research Question:** Can we train AI‑Chat to understand **Godot's `@export` annotation** vs **Unreal's `UPROPERTY(EditAnywhere)`**? Provide a translation table to ensure generated scripts are idiomatic for the target engine.
246. **Objective:** Create a **Cross‑Engine Test Harness**. A simple console application that loads a `mapgrid_v1.json` and verifies the **pathfinding graph** (A*) is identical whether it's calculated by the Unreal NavMesh code or the Unity NavMesh code.
247. **Coding Task:** Write a **C# Source Generator** (for Unity) that automatically creates the `[CreateAssetMenu]` entries for all **Mode Profiles** defined in the JSON, removing the need for AI‑Chat to remember Unity Editor specifics.
248. **Directive:** For the **Godot Emitter**, generate a `.tscn` file directly instead of a script that builds the scene at runtime. Provide AI‑Chat with the exact XML structure of a Godot `PackedScene` so it can edit the scene file textually.
249. **Research Question:** How can we use **AI‑Chat to update all 7 maps** when a schema change occurs (e.g., adding a `lightmap_resolution` field to every cell)? Can we provide a "find and replace" instruction that works reliably?
250. **Objective:** Build a **Runtime Cheat Menu** (`uncut_cheats`) that allows toggling of `bAI_ShowNavigationPaths`, `bAI_ShowPerception`, and `bAI_ShowGridDebug`. This visual feedback is essential for AI‑Chat to verify its generated patrol routes are working.
251. **Coding Task:** Implement a **Binary Space Partition (BSP) Tree Visualizer** for the Fortress map. AI‑Chat can use this to understand occlusion and suggest `portal` tags to improve culling performance.
252. **Research Question:** How does **Large‑Scale AI Refactoring** affect the `grid2scene` Rust codebase? Test if AI‑Chat can successfully split the monolithic `main.rs` into a `lib.rs` and a `bin/` structure without breaking the CI pipeline.
253. **Directive:** Create **`CODING_STANDARDS.md`** with explicit rules like "Never use `Sleep()` in game thread" and "Always use `TWeakObjectPtr` for Actor references in Unreal." AI‑Chat must be evaluated against this document.
254. **Objective:** Write a **Network Replication Validator** script. After AI‑Chat generates a new class (e.g., `AlienEgg`), the script checks that all `UPROPERTY(Replicated)` variables are properly marked and that `GetLifetimeReplicatedProps` is implemented.
255. **Coding Task:** Create a **"Vanilla Mode" Fallback**. If AI‑Chat introduces a bug in the **Fortress Gas Canister**, the game mode should gracefully fall back to **Beach Dead‑style rules** (just tickets and kills) to keep the game playable during development.
256. **Research Question:** What is the minimum **English Vocabulary** required to describe *Conker* map changes? Create a **Controlled Natural Language (CNL)** spec like: `"ADD [HAZARD] AT [X,Y] WITH [TYPE=GAS] AND [DAMAGE=10]"`.
257. **Directive:** Build an **AI‑Chat Playground** Docker image. This image contains the full toolchain (`grid2scene`, schema validators, a headless build of the game) and can be run by AI‑Chat in a sandboxed environment to *actually test* the code it generates before showing it to the user.
258. **Objective:** Create a **Voice Line Matcher**. AI‑Chat can input "Conker saying 'Oh bloody hell'" and the tool returns the exact `SFX_ID` and file path, preventing the AI from inventing sound effects that don't exist.
259. **Coding Task:** Implement a **Save/Load System** for **Grid2Scene Projects**. Allow AI‑Chat to output a **diff** of a map file (e.g., "Change cell [5,10] from `trench` to `mg_nest`") rather than re‑generating the entire 200KB JSON file.
260. **Research Question:** How can we **quantify the "Fun Factor"** of AI‑generated map layouts? Can we run simulated bot matches and measure metrics like **Encounter Frequency**, **Time to Action**, and **Chokepoint Death Density** to automatically rank map variants?

Based on the schema‑centric architecture outlined in your document—formalizing delivery objectives, unifying hazard profiles, and standardizing zombie kill mechanics with controlled exceptions—here are **30 additional coding tasks** (261–290) that integrate directly with the existing `Doctor0Evil/conker-live-and-uncut-fan` repository structure and its data‑driven pipelines.

These tasks are designed to be **precise directives for AI‑Chat agents**, referencing specific schemas, file paths, and engine components to ensure the generated code is immediately actionable and maintainable.

### 261–270: Implementing Generic Delivery Objective Types

261. **Directive:** Extend `schemas/multiplayer/map_entities_v1.schema.json` to include `objective_type` enum values: `"BabyDinoFeeder"` and `"EggCookPan"`. Add required sub‑properties: `accepted_carried_asid` (integer) for the feeder, and `cook_duration` (float) for the pan.

262. **Coding Task:** Create a new base class `ABaseDeliveryObjective` (Unreal) / `DeliveryObjectiveBase` (Unity/Godot) that inherits from the existing `ABaseObjective`. This class must expose virtual methods `bool CanAcceptDelivery(APawn* Carrier)` and `void ProcessDelivery(APawn* Carrier)`.

263. **Directive:** In the **Unreal Engine Emitter** (`grid2scene/src/emitter_unreal.rs`), add a case for `objective_type: "BabyDinoFeeder"` that spawns an actor of class `ABP_BabyDinoFeeder_C` and configures it with the `accepted_carried_asid` value from the JSON.

264. **Coding Task:** Implement `ABP_BabyDinoFeeder::ProcessDelivery` in Unreal C++. The function must:
    - Verify the carrier's `CurrentCarriedASID` matches `AcceptedCarriedASID`.
    - Award `ScoreValue` to the carrier's team.
    - Call `ForceASIDTransition(ASID_DEATH_FED)` on the carried pawn to trigger the hard‑lock execution state.
    - Play `SFX_DINO_EAT` and `VFX_BLOOD_SPRAY` at the feeder's location.

265. **Coding Task:** Implement `ABP_EggCookPan::ProcessDelivery` in Unity C#. The function must:
    - Verify the carrier is holding an object with `ASID_EGG_RAW`.
    - Destroy the carried egg object.
    - Start a `CookDuration` timer visible via a progress bar widget.
    - Upon completion, award `ScoreValue` and broadcast a `OnEggCooked` event to update the game mode.

266. **Objective:** Create a **Mode Profile** entry for Raptor Temple (`config/modes/raptor_temple_mode_profiles_v1.json`) that references the new objective types and enables the `ASID_RAPTOR_POUNCE_EXEC` ability for Team Raptor by default.

267. **Coding Task:** Add validation to the `grid2scene --validate` flag that checks: if an entity has `objective_type: "BabyDinoFeeder"`, its corresponding grid cell must have a `role_tag` containing `"baby_dino_nest"` or similar, ensuring logical placement.

268. **Research Question:** How should the "Egg Cooking" objective handle multiple players interacting simultaneously? Define a queuing or denial system and document it in `Docs/GDD/05_Multiplayer_Raptor_Temple.md`.

269. **Directive:** Create a **reusable UI widget** (`WBP_DeliveryObjectivePrompt`) that displays context‑sensitive text ("Feed Caveman to Baby Dino", "Cook Egg") and a progress bar. The widget must be data‑driven, reading the prompt text from the objective's `InteractionPrompt` property defined in the entities JSON.

270. **Coding Task:** Write a **unit test** (Google Test for Unreal, NUnit for Unity) that verifies `ProcessDelivery` correctly rejects a carrier who is not holding the required `accepted_carried_asid`.

### 271–280: Unifying Hazard Systems with Data‑Driven Profiles

271. **Objective:** Create the JSON schema for hazard profiles: `schemas/hazards/hazard_profiles_v1.schema.json`. It must define arrays of `damage_per_second`, `tick_interval`, `immunity_asids`, `visual_effect_id`, and `audio_cue_id`.

272. **Coding Task:** Implement `UHazardProfileDataAsset` (Unreal) / `HazardProfileSO` (Unity ScriptableObject) / `HazardProfileResource` (Godot). These assets are populated at editor time from the JSON data and used at runtime to configure `HazardVolume` actors.

273. **Directive:** Refactor `AAlienBaseVolumeHubFloorGas` and `AAlienBaseVolumeSublevelAcid` to inherit from a new base class `AHazardVolume`. Move all DoT and immunity logic into `AHazardVolume::ApplyDamageOverTime()`.

274. **Coding Task:** In `AHazardVolume::ApplyDamageOverTime()`, implement a check against `ImmunityASIDs`. For each pawn inside the volume, query its `CurrentASIDSet` and skip damage if any ASID matches the profile's immunity list.

275. **Coding Task:** Modify the **Alien Base Airlock Controller** to activate its associated `AHazardVolume` by calling `ActivateHazard()` and `DeactivateHazard()` instead of directly toggling visibility or collision. The hazard volume itself handles VFX/SFX based on its loaded profile.

276. **Directive:** Extend the **Fortress Gas Canister** state machine (`AGasCanister::ArmAtBase`) to locate the `AHazardVolume` with `role_tag: "gas_cloud_zone"` and call `ActivateHazard()` with a duration defined in the mode profile.

277. **Coding Task:** Add a **Runtime Profile Switcher** function to `AHazardVolume`: `void ReloadProfile(FName NewProfileID)`. This allows a single hazard volume to change behavior (e.g., from "hub_gas_v1" to "hub_gas_hardmode") mid‑match via console command or game mode event.

278. **Objective:** Create a **default hazard profile** file `config/hazards/default_hazard_profiles_v1.json` with entries: `"hub_gas_v1"`, `"sublevel_acid_v1"`, `"fortress_gas_v1"`, and `"fire_imp_burning_v1"`.

279. **Coding Task:** Write a **debug console command** (`uncut_hazard.reload_profile <ProfileID>`) that reloads the specified JSON profile and applies it to all active `AHazardVolume` instances, enabling rapid iteration during playtesting.

280. **Research Question:** How can we visually communicate hazard immunity during execution states? Design a **HUD icon** (e.g., a green shield) that appears when the player is inside a hazard volume but an active ASID grants immunity, and implement it in all three engines.

### 281–290: Standardizing Zombie Crawl Mechanics & Exception Handling

281. **Objective:** Add a new **damage modifier rule** to `config/damage_rules_v1.json`: `"crawling_zombie_rule"` with properties `"only_headshot_kills"`, `"shotgun_point_blank_multiplier"`, and `"exception_asids"` (array of ASIDs that bypass the rule).

282. **Coding Task:** In the shared `UDamageSubsystem` (or equivalent), implement a function `bool ShouldApplyDamageToCrawlingZombie(AActor* Victim, const FDamageEvent& DamageEvent)`. This function reads the rule from the config and evaluates:
    - Is the hit bone `"head"` or `"neck"`?
    - Is the weapon a shotgun and is the distance < `PointBlankThreshold`?
    - Is the attacker in one of the `exception_asids`?

283. **Directive:** Create a new ASID `ASID_ZOMBIE_CRAWL` (value 805). When a zombie's health falls below `CrawlHealthThreshold`, the damage system must call `ForceASIDTransition(ASID_ZOMBIE_CRAWL)` instead of applying lethal damage, unless the kill conditions are met.

284. **Coding Task:** Implement the **Crawl State Behavior** in the Zombie AI controller:
    - Movement speed reduced to 150 units/s.
    - Only head and upper torso hitboxes remain active for damage.
    - Attack range reduced and limited to a "grab" animation.

285. **Coding Task:** Add an **exception whitelist** to the Zombie's `TakeDamage` function. If the instigator pawn has an active ASID from `exception_asids` (e.g., `ASID_RAPTOR_POUNCE_EXEC`), any damage source (even a body shot) should bypass the crawl state and kill the zombie instantly.

286. **Objective:** Create a **debug visualizer** for the crawling zombie rule. When enabled (`uncut_debug.zombie_crawl 1`), draw a red sphere on the zombie's head hitbox and a green sphere on immune body parts.

287. **Coding Task:** Write a **validation test** that spawns a zombie, damages it to the crawl threshold with a pistol (body shot), and asserts that the zombie enters `ASID_ZOMBIE_CRAWL` and does not die from subsequent pistol body shots.

288. **Research Question:** Should the "Crawling Zombie" rule apply to **friendly fire** in team modes? Document the design decision and implement a configurable toggle in `gameplay_config.json` (`zombie_crawl_friendly_fire: true/false`).

289. **Directive:** In the **Blood Count entities JSON** (`blood_count_entities_v1.json`), add a `"zombie_crawl_settings"` override section that allows per‑map tuning of `CrawlHealthThreshold` and `PointBlankThreshold` without altering the global damage rules.

290. **Coding Task:** Implement the **Fire Imp's interaction** with crawling zombies. Since the Fire Imp hunts the blood‑vial carrier, it should prioritize attacking the carrier even if a crawling zombie is nearby. Add a `ThreatPriority` system that gives the carrier a higher weight than crawling zombies.

These 30 tasks seamlessly extend the existing 260‑item roadmap, ensuring that the formalized objectives, unified hazards, and standardized zombie rules are not just documented but fully realized as robust, data‑driven code that maintains the original *Conker's Bad Fur Day* feel while leveraging modern engineering practices.

This list provides a comprehensive and actionable roadmap for developing the Conker: Live & Uncut fan project. By addressing these items, AI-Chat will be able to provide more precise and helpful code generation, ensuring the final product remains faithful to the original N64 multiplayer philosophy.
