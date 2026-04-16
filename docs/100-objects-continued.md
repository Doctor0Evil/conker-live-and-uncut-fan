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

This list provides a comprehensive and actionable roadmap for developing the Conker: Live & Uncut fan project. By addressing these items, AI-Chat will be able to provide more precise and helpful code generation, ensuring the final product remains faithful to the original N64 multiplayer philosophy.
