To ensure AI-Chat can effectively assist with the development of your Conker: Live & Uncut fan project, the following 100 research topics, objectives, and coding directives are provided. These items are designed to fully specify the details of the seven multiplayer maps and the underlying systems, moving the project from concept to concrete, implementable tasks.

### 🏗️ Core Infrastructure & Tools (Grid2Scene, Data Schemas, Engine Integration)

The foundation of the project's data-driven pipeline.

1.  **Objective:** Create a comprehensive JSON Schema for all map grid files (`*_grid_v1.json`) to validate tile types, `role_tags`, and cell properties.
2.  **Objective:** Create a comprehensive JSON Schema for all map entity files (`*_entities_v1.json`) covering spawns, pickups, hazards, and objectives.
3.  **Directive:** Refactor the `alien_base_grid2scene` Rust crate to load map data dynamically based on a `--map` argument, eliminating the need for separate binaries per map.
4.  **Directive:** Implement a `--validate` flag in the grid2scene tool that checks grid and entities JSON against their schemas and verifies that all referenced `tile_type` and `role_tags` exist in the specified tileset.
5.  **Research Question:** What is the most efficient way to handle texture atlasing and material instancing when generating levels from grid data in Unreal Engine 5, Unity 6, and Godot 4?
6.  **Coding Improvement:** Modify the Unreal output emitter to generate a Python script that can be run inside the Unreal Editor to automate level construction using its API, instead of a manual JSON import process.
7.  **Coding Improvement:** For the Unity output emitter, generate a single C# script (`AlienBaseLevelBuilder.cs`) that can be attached to a GameObject and will build the entire level in the Editor using `PrefabUtility` and `Undo` system for proper editor integration.
8.  **Objective:** Design a versioning system for grid, entity, and tileset files to track changes and ensure compatibility between different versions of the map data and the build tool.
9.  **Objective:** Create a "Map Manifest" JSON file that lists all available maps, their file paths, supported game modes, and recommended player counts. The grid2scene tool should be able to read this to batch-process all maps.
10. **Coding Improvement:** Add a `--dry-run` flag to the grid2scene tool that performs all validation and processing but does not write any output files.
11. **Directive:** Ensure all generated C++/C#/GDScript code adheres to the naming conventions established in the `04_Multiplayer_Alien_Base_Entities.md` document.
12. **Objective:** For each engine, create a "Base" actor class (e.g., `BP_AlienBase_PickupBase`, `PickupBase.cs`, `PickupBase.gd`) that all weapon pickups can inherit from, standardizing behavior.
13. **Research Question:** What is the best practice for managing and versioning large binary assets (models, textures, sounds) in the GitHub repository for this fan project?
14. **Directive:** Create a CI/CD pipeline (e.g., using GitHub Actions) that automatically runs the grid2scene tool with the `--validate` flag on every push to ensure data integrity.
15. **Objective:** Write a detailed guide for artists on how to create new tileset assets that conform to the project's 4x4 unit grid scale, pivot point conventions, and naming scheme.

### 🏝️ Map-Specific Specifications

Detailed items for each of the seven maps to ensure they are fully fleshed out.

#### 01_Multiplayer_Beach_Dead
16. **Research Question:** What is the exact geometric layout of the three "Fences" from the N64 Beach map, and how should they be translated into a grid of 4x4 tiles?
17. **Objective:** Define the specific `role_tags` for the Beach Dead grid, such as `"trench"`, `"fence_1"`, `"fence_2"`, `"fence_3"`, `"attacker_spawn_band"`, `"defender_spawn_band"`, and `"mg_nest"`.
18. **Objective:** Create the `beach_dead_entities_v1.json` file, specifying the exact grid coordinates for all defender MG nests, attacker spawn ships, and the final bunker objective.
19. **Directive:** Implement the "fallback spawn" logic for Attackers as they capture each fence line, as described in the map's core loop.
20. **Coding Task:** Write a C++/C#/GDScript component for the "Fence" objective that tracks its health, changes its visual state, and triggers a global event upon destruction to update spawn points.
21. **Research Question:** What are the specific damage values and fire rates for the N64-era weapons (Pistol, Uzi, Sniper Rifle, Bazooka) that should be used in Beach Dead?
22. **Objective:** Design the layout and `role_tags` for the interior of the fortress bunker, including corridors, the commander room, and roof access points.

#### 02_Multiplayer_The_Heist
23. **Research Question:** How was the "gas chamber" instant-win mechanic triggered in the N64 Heist map, and what was its timer and area of effect?
24. **Objective:** Create the grid for The Heist, defining the central vault, the four team corridors, and the gas chamber. Use `role_tags` like `"vault"`, `"team_red_spawn"`, `"gas_chamber"`.
25. **Objective:** Define the logic for the "money bag" pickup: its spawn location, the movement speed penalty for the carrier, and the scoring condition for returning it to a team base.
26. **Directive:** Design the environmental hazard trigger for the gas chamber. Should it be a button, a tripwire, or a timed event? Define its interaction logic.
27. **Coding Task:** Implement the "Heist" game mode logic: track which team holds the money bag, update score when delivered, and trigger the gas chamber win condition.
28. **Research Question:** Are there any unique architectural features from the Feral Reserve Bank in the single-player game that should be incorporated into the multiplayer map's visual design?

#### 03_Multiplayer_Fortress
29. **Research Question:** How does the "Total War" game mode from N64 translate to a larger 16-player Uncut environment? Should the gas canister mechanic be retained, adapted, or replaced?
30. **Objective:** Define the tile palette for Fortress, focusing on industrial and war-torn variants as specified in the `03_Multiplayer_Fortress_Tile_Palette.md` document.
31. **Objective:** Create the grid for Fortress, defining the central valley/bridge, the two main bases (SHC and Tediz), and the network of trenches and towers. Use `role_tags` like `"shc_base"`, `"tediz_base"`, `"capture_zone"`, `"trench"`.
32. **Directive:** Design the capture point logic for Fortress. Should it be a single central point, multiple points, or a "tug-of-war" style objective?
33. **Objective:** Specify the placement of heavy weapon pickups (Bazooka, Chaingun) and vehicles (if any) within the Fortress map.
34. **Coding Task:** If the gas canister is retained, implement its logic: player pickup, carrying penalty, and the ability to "arm" it at an enemy base to trigger a map-wide hazard.

#### 04_Multiplayer_Alien_Base
35. **Research Question:** What are the specific geometric bounds (radius, height) for the `hazard_hub_floor_gas` and `hazard_sublevel_acid` volumes in world units?
36. **Objective:** Finalize the `alien_base_hub_grid_v1.json` to include all necessary tiles for a complete greybox, including the outer wall ring and all corridor entrances.
37. **Coding Task:** Implement the `AlienBaseAirlockController` state machine in C++ (Unreal), C# (Unity), and GDScript (Godot) based on the specifications in `04_Multiplayer_Alien_Base_Triggers.md`.
38. **Directive:** Create the "Alien Egg" objective actor. It should have health, visual stages, and trigger the airlock hazard or alien spawns when damaged or destroyed.
39. **Objective:** Define the spawn points and patrol routes for the Alien NPCs in the "Invasion" game mode variant.
40. **Coding Task:** Implement the damage-over-time logic for the hazard volumes, ensuring it correctly interacts with the execution state immunity (ASID system) as defined.

#### 05_Multiplayer_Raptor_Temple
41. **Research Question:** What were the original N64 Raptor mode's specific mechanics for Raptors feeding cavemen to the baby dino and Cavemen cooking eggs?
42. **Objective:** Translate the N64 Raptor mode mechanics into a 16-player Uncut environment. Should it remain asymmetrical or be adapted into a team-based objective mode?
43. **Objective:** Create the grid for Raptor Temple, defining the S-shaped valley, the central two-story temple, and the Uga Buga and Raptor bases. Use `role_tags` like `"uga_base"`, `"raptor_nest"`, `"temple_ground"`, `"temple_upper"`.
44. **Directive:** Design the "Egg" and "Baby Dino" objectives. How are they represented in the world? How do players interact with them to score?
45. **Coding Task:** Implement the "Raptor Pounce" and "Caveman" abilities as distinct character states or temporary pickups, in line with the Uncut "no classes" philosophy.
46. **Research Question:** What is the intended layout of the central temple's interior? Should it have multiple floors, traps, or specific chokepoints?

#### 06_Multiplayer_TMS_Spamono
47. **Research Question:** Based on the Live & Reloaded implementation, what are the exact dimensions and layout of the T-shaped corridor in TMS Spamono?
48. **Objective:** Create the grid for TMS Spamono, defining the long central corridor, the "airlock" sections, and the team bases at each end. Use `role_tags` like `"corridor_main"`, `"airlock"`, `"team_a_spawn"`, `"team_b_goal"`.
49. **Directive:** Design the objective for TMS Spamono. The Live & Reloaded version involved capturing codes. How can this be adapted to a non-class-based, pickup-driven Uncut mode?
50. **Coding Task:** Implement the "airlock" door logic for the central corridor. When should it open or close? Is it tied to an objective, a timer, or a player-activated trigger?
51. **Objective:** Define the placement of weapon pickups along the central corridor to encourage strategic pushes and defense.
52. **Coding Task:** Implement the `SpamonoCorridor` helper struct in Rust as defined in `crates/alien_base_grid2scene/src/model/spamono_corridor.rs` to assist with AI navigation and scripting.

#### 07_Multiplayer_The_Blood_Count
53. **Research Question:** What are the exact names and layouts of the four wings off the central hall in Count Batula's Mansion to correctly assign team spawns?
54. **Objective:** Create the grid for The Blood Count, defining the mansion interior, the four team wings, and the exterior hedge maze. Use `role_tags` like `"team_red_spawn"`, `"library"`, `"hedge_maze"`, `"ritual_altar"`.
55. **Coding Task:** Implement the "Panther King's Blood-Vial" pickup logic: heavy carry penalties, drop-on-death, and scoring at the team's ritual altar.
56. **Directive:** Design the spawning and respawning logic for the blood-vial. What are its possible spawn locations? What is the decay timer, and how is it communicated to players?
57. **Coding Task:** Implement the "Fire Imp" AI behavior. It should be invisible/inactive until the vial is picked up, then relentlessly hunt the carrier. Define its navigation, attack patterns, and vulnerability to the Hunting Revolver and Shotgun.
58. **Objective:** Define the zombie spawn points, density, and patrol areas for The Blood Count.
59. **Coding Task:** Implement the zombie "crawl" state logic: when limb/body damage exceeds a threshold, transition to a crawling animation and movement mode, but only allow death from headshots or close-range shotgun blasts.
60. **Research Question:** What is the ideal respawn time for the Fire Imp after being killed (e.g., 15 seconds) to provide a balanced "breathing room"?

### 🧠 Systems, Mechanics & ASID Registry

Items related to core gameplay, character states, and the "brutal" feel.

61. **Objective:** Create a master `docs/systems/Animation_State_Registry.md` file that lists all ASIDs, their properties (Interrupt Priority, Lock Type, Gore Trigger Frame), and associated SFX/VFX.
62. **Coding Task:** Implement the core ASID state machine for player characters in all three engines. It must enforce "Hard Lock" rules, preventing movement, jumping, or other actions during execution animations.
63. **Coding Task:** Implement the "Heavy Carry" state (ASID_050) that overrides movement speed, disables jumping, and prevents weapon use.
64. **Directive:** Create a "Gore Manager" system that is responsible for spawning the correct gib meshes and particle effects at specific bone locations based on the ASID and Gore Trigger frame.
65. **Research Question:** How should the "Stun Lock" (ASID_012) state be implemented to balance melee combat and heavy weapon hits, ensuring it's impactful but not frustrating?
66. **Objective:** Define a complete table of weapon statistics for all Uncut weapons (Chainsaw, Katana, Crossbow, Shotgun, Flamethrower, Hunting Revolver, etc.), including damage, range, fire rate, and special properties.
67. **Coding Task:** Implement the weapon pickup system. Ensure it correctly handles ammo, weapon switching, and the "no jump" flag for heavy weapons.
68. **Objective:** Design the "Profanity Toggle" system. How will the game load either uncensored or bleeped audio files based on a user setting, defaulting to "Uncut"?
69. **Coding Task:** Implement the "Line-of-Sight (LoS)" spawning logic for 16-player matches to prevent spawn camping, as defined in the map documents.
70. **Directive:** Create a unified "Damageable" interface (e.g., `IDamageable` in C#, an Unreal `UInterface`, a Godot `Node` with a `take_damage` method) for all actors that can receive damage (players, zombies, objectives).
71. **Objective:** Define the rules for "Friendly Fire" in team-based modes. Should it be enabled or disabled? How does it interact with execution animations?
72. **Research Question:** What is the best way to implement the "context-sensitive" action button (B-button) from the N64 original in a modern multi-platform context?

### 👾 AI & NPC Behavior (Zombies, Aliens, Fire Imp)

73. **Coding Task:** Implement the base Zombie AI: a simple state machine (Idle, Patrol, Chase, Attack, Crawl) with navigation using the engine's built-in pathfinding (NavMesh).
74. **Directive:** Implement the Zombie damage model: they only take lethal damage from headshots or point-blank shotgun blasts to the head/upper torso. Body shots cause them to enter the "Crawl" state.
75. **Objective:** Create a spawn manager for zombies that can activate spawn points based on player proximity and maintain a target density in different map zones.
76. **Coding Task:** Implement the Alien NPC AI for Alien Base: focus on "pounce" (ASID_900) and "facebite" (ASID_901) execution attacks.
77. **Directive:** Design the Alien's pathfinding to utilize vents and ceilings, making them a threat from multiple angles.
78. **Coding Task:** Implement the Fire Imp's unique targeting logic: it should only become active and target players when the blood-vial is held.
79. **Objective:** Create a "threat assessment" system for AI that allows them to prioritize targets (e.g., the blood-vial carrier, a player performing an execution, the nearest attacker).
80. **Research Question:** How can AI behavior be optimized for 16-player matches with dozens of active NPCs to maintain stable performance?
81. **Coding Task:** Implement a "horde mode" director AI for Invasion-style games that dynamically adjusts alien spawn rates and types based on player performance.

### 🔊 Audio-Visual & VFX Registry

82. **Objective:** Create a master `docs/systems/Audio_Visual_Registry.md` file that catalogs all SFX IDs (e.g., `SFX_001`, `SFX_800`) and VFX IDs (e.g., `VFX_010`, `VFX_020`) with their descriptions and intended use.
83. **Coding Task:** Implement an "Audio Manager" that can play sounds with correct 3D spatialization, attenuation, and random pitch variation as specified.
84. **Directive:** Create a "Decal Manager" that handles the infinite lifetime and culling (FIFO, max count 200) for gore decals like `PRT_GORE_LIME` to maintain performance.
85. **Objective:** Define the visual language for the "Airlock Hazard" telegraph: the sequence of sirens, flashing lights, and venting steam (`VFX_080`) that precede the gas release.
86. **Research Question:** What modern rendering techniques (e.g., bloom, ambient occlusion, volumetric fog) can be used to enhance the "N64 but up-rezzed" aesthetic without betraying the original visual style?
87. **Coding Task:** Implement the "Chainsaw Sparks" logic: when the chainsaw hits a surface with the `MAT_METAL_BASE` tag, spawn `VFX_020` instead of gore.

### ⚡ Performance & Optimization for 16 Players + Split-Screen

88. **Objective:** Establish performance budgets for each target platform (PC, Xbox, potentially Switch-like handhelds) for CPU, GPU, and memory.
89. **Directive:** Implement a Level of Detail (LOD) system for all map tiles and character models to reduce polycount at a distance.
90. **Coding Task:** Implement the foliage/entity pooling system as described in the C++ snippets to minimize instantiation overhead for common effects and NPCs.
91. **Research Question:** How can the grid2scene tool be used to pre-compute occlusion culling data (e.g., PVS) to improve rendering performance in dense indoor maps like Alien Base and The Blood Count?
92. **Objective:** Create a performance test map that can spawn 16 players and hundreds of zombies to stress-test and profile the game's systems.
93. **Directive:** Optimize network code for online play. Use relevancy and prioritization to only send critical updates for distant NPCs and players.

### 📚 Documentation & Tooling for AI-Chat

94. **Objective:** Create a `CONTRIBUTING.md` file that explains the data-driven pipeline (JSON schemas, grid2scene tool) and how to add a new map or modify an existing one.
95. **Directive:** Add comments to all JSON schemas and example files explaining the purpose and expected values of each field.
96. **Objective:** Create a template file (`map_template.md`) that can be copied and filled out to create a design document for any new map.
97. **Directive:** Write a "Glossary of Terms" document defining all project-specific jargon (ASID, role_tag, Heavy Carry, Uncut philosophy, etc.) for AI-Chat context.
98. **Objective:** Create a simple web-based viewer for the map grid JSON files that renders a 2D top-down view of the tiles, color-coded by `tile_type`, to aid in design and debugging.

### 🧪 Community & Playtesting

99. **Directive:** Design a system for toggling game rules and variables (e.g., zombie density, hazard timers) via a configuration file or console commands to facilitate rapid playtesting and balancing.
100. **Objective:** After generating a greybox for a map, create a playtesting checklist that covers all unique features of that map (e.g., "Can you destroy Fence 1?", "Does the Fire Imp spawn correctly?").

This list provides a comprehensive and actionable roadmap for developing the Conker: Live & Uncut fan project. By addressing these items, AI-Chat will be able to provide more precise and helpful code generation.
