# 04_Multiplayer_Alien_Base – Grid, Tiles, and Materials

This document defines the spatial grid, macro layout, tile palette, and texture/material intent for the Alien Base multiplayer map. It is designed to be a medium‑large indoor arena, combining the central silo/bridge rhythm of Tank from Conker’s Bad Fur Day with the multi‑tower, vertical indoor feel of Three Towers, but with no vehicles and a focus on infantry platforming and hazard control. [conker.fandom](https://conker.fandom.com/wiki/Tank)

## 1. Global Grid and Scale

Alien Base uses a regular 3D grid to keep design portable across engines and to make it easy for tools to snap geometry.

- Base unit: **1.0** = 1 “Conker meter” (roughly one character width).  
- Corridor width: **6–8 units** (tight infantry scale, similar to It’s War interior corridors). [conker.fandom](https://conker.fandom.com/wiki/Bunker)
- Main hub radius: **24–28 units** on the floor ring, giving ~50–55 units diameter for the central airlock room.  
- Map footprint: approximately **200 x 200 units** horizontally, including four outer bunkers and buffer corridors.  
- Vertical layers:  
  - Sub‑level tunnels: `Y = -10` to `Y = -4`.  
  - Main floor: `Y = 0` ± small steps.  
  - Catwalk ring / upper corridors: `Y = 10` to `Y = 14`.  

This keeps Alien Base squarely in “medium‑large indoor” territory: bigger and more layered than Bunker, smaller footprint than the full outdoor expanse of Three Towers but with comparable traversal time due to interior turns and stairs. [conker.fandom](https://conker.fandom.com/wiki/Three_Towers)

### Coordinate Ranges (High‑Level)

- X axis: `-100` (West reactor) to `+100` (East medical wing).  
- Z axis: `-100` (South hangar) to `+100` (North quarantine).  
- Y axis: `-12` (lowest tunnels) to `+20` (highest catwalk/rafters).

Everything you build (walls, floors, pickups, spawns, hazards) should snap to this grid so that future tools can round or quantize coordinates cleanly.

## 2. Macro Layout: Rings and Spokes

At the block level, Alien Base is a hub‑and‑spokes layout built from Tanks’ central silo + four bunkers and Bunker’s indoor “ring with side rooms.” [youtube](https://www.youtube.com/watch?v=o140m97FpSE)

### 2.1 Central Hub / Airlock Chamber

- Center at `(0, 0, 0)`.  
- Circular or octagonal floor with radius ~25 units, ringed by a raised lip or railing.  
- Four exit corridors at cardinal directions (N, S, E, W).  
- Alien Egg platform in the very center (1–2 steps up), with a small moat or grate around it hinting at gas ducts below.

Grid approximation:

- Inner “Egg platform” radius: 5 units.  
- Walkable floor ring: between radii 8 and 24 units.  
- Outer wall ring: radius ~28 units.  

### 2.2 Outer Bunkers / Quarantine Zones

Four outer zones match the spawn philosophy from earlier docs and map to the Tank bunkers conceptually. [conker.fandom](https://conker.fandom.com/wiki/Canister)

- Zone A (North Quarantine): centered at `(0, 0, 90–100)`.  
- Zone B (South Hangar): centered at `(0, 0, -90–100)`.  
- Zone C (East Medical Wing): centered at `(90–100, 5, 0)`.  
- Zone D (West Reactor Core): centered at `(-90–100, 5, 0)`.

Each zone should be roughly **30 x 20 units** in floor footprint, with at least two entrances: one direct corridor to the hub and one to a side corridor or upper route.

### 2.3 Corridors and Loops

Corridors connect hub ↔ bunkers and bunkers ↔ each other, forming loops so players can flank without re‑entering the hub every time.

- Main hub corridors (N, S, E, W): 8 units wide, 18–24 units long.  
- Secondary cross corridors (NE, NW, SE, SW): 6 units wide, linking midpoints of main corridors and creating a square ring around the hub at ~40 units radius.

Conceptually, from above you get:

- Inner circle (hub).  
- Square ring of corridors at radius ~40.  
- Four rectangles (bunkers) just beyond that square.

## 3. Logical Tile Palette

To keep the map “N64 but up‑rezzed,” define a limited, reusable set of tile types. Think in terms of Bunker’s named rooms and Tank’s bridge/bunker structures. [conker.fandom](https://conker.fandom.com/wiki/Tank)

### 3.1 Structural Tile Types

Each tile is a reusable modular block that spans **4 x 4 units** (for floors) or **4 units** long segments (for walls).

- `T_Floor_IndustrialPlain`  
  - Flat metal plating, slightly worn. Base floor tile for corridors and bunker rooms.  

- `T_Floor_IndustrialHazardStripe`  
  - Same as above but with yellow/black striping along edges for hazard zones, ledges, and near the Egg grate.  

- `T_Floor_AlienResin`  
  - Dark, glossy bio‑organic material; used near the Egg, in vents, and around alien spawn points.  

- `T_Wall_PanelRiveted`  
  - Standard bulkhead wall, used throughout base.  

- `T_Wall_WindowObservation`  
  - Bulkhead with inset window; used for viewing the central hub from upper corridors or bunker control rooms.  

- `T_Wall_AlienVeins`  
  - Wall with embedded resin veins/vines, where aliens have started infesting the base.

- `T_Ceiling_PipeCluster`  
  - Ceiling panel with pipes/conduits, especially in sub‑level tunnels.  

- `T_Ceiling_GratedVent`  
  - Ceiling with large vents and grates, used above the hub Egg and hazard volumes.

### 3.2 Special Tiles (Hazard and Objective)

- `T_Floor_GrateGasEmitter`  
  - Floor grate tile with visible gas nozzles; used within `hazard_hub_floor_gas` radius.  

- `T_Floor_AcidDrain`  
  - Grated floor with green residue; used in sub‑level tunnels within `hazard_sublevel_acid`.  

- `T_Platform_EggBase`  
  - Circular or octagonal pedestal tile set (centered at `(0, 0, 0)`).

These tiles visually telegraph where it will be dangerous during an Airlock event, in the same way Tanks uses bunker interiors and ground textures to communicate safe zones vs exposure. [conker.fandom](https://conker.fandom.com/wiki/Canister)

## 4. Texture and Material Families

Texture sets should be direct spiritual successors to Bad Fur Day’s 64x64 industrial and war tiles, upscaled and sharpened rather than replaced by photorealistic detail. [en.wikipedia](https://en.wikipedia.org/wiki/Conker's_Bad_Fur_Day)

### 4.1 Industrial / Human Tech

- Resolution: authored at **256x256** or **512x512**, derived from AI‑upscaled BFD textures where possible.  
- Base metals: gunmetal and dull steel with moderate roughness, low specular.  
- Hazard stripes: saturated yellow/black, matching war maps and Three Towers hazard sign language. [conker.fandom](https://conker.fandom.com/wiki/Three_Towers)

### 4.2 Alien Organic

- `M_AlienResin` material: very low roughness, high specular, almost black with subtle deep blue/green tints.  
- `M_AlienGore` decal: bright lime green emissive veins and splashes, matching Conker’s alien blood color. [en.wikipedia](https://en.wikipedia.org/wiki/Conker's_Bad_Fur_Day)
- Use these sparingly to keep human/alien contrast strong.

### 4.3 Atmospheric FX

- Gas: volumetric particle system tinted pale green, with density highest near `T_Floor_GrateGasEmitter` tiles.  
- Acid mist: thicker, brighter green near drains in sub‑level tunnels.  

The MD only names these families; engine‑specific materials are defined elsewhere.

## 5. Grid‑Level Layout Plan (Research Objectives)

To make this buildable by tools, define several research objectives that each produce one slice of the final grid spec.

### 5.1 Objective A – Hub Floor Grid

**Goal:** Explicitly define a 2D grid for the main hub at `Y = 0`, marking each cell’s tile type, walkability, and role.

- Grid extent: X and Z from `-32` to `+32`, step 4 (floor tiles).  
- For each `(gx, gz)` tile:  
  - `tile_type`: one of the structural/special tiles above.  
  - `collision`: solid, walkable, or void.  
  - `role_tags`: e.g., `["hub_floor", "hazard_zone"]`, `["egg_platform"]`, `["corridor_entry_north"]`.

This grid is the basis for later auto‑meshing or BSP generation.

### 5.2 Objective B – Corridor Ring Grid

**Goal:** Describe the ring of corridors that link hub to bunkers and bunkers to each other.

- Grid extent: X and Z from `-72` to `+72`, excluding the central hub radius.  
- Corridor widths: 2 tiles (8 units) for main corridors, 1–2 tiles (4–8 units) for secondary links.  
- Tags: `["corridor_main"]`, `["corridor_secondary"]`, `["corner_turn"]`.

### 5.3 Objective C – Bunker Interior Grids

**Goal:** Per‑zone interior layout for Zones A–D.

Each bunker gets its own local grid in a smaller file or section:

- Dimensions: 8 x 6 tiles (32 x 24 units) minimum.  
- Rooms:  
  - Entry hall (facing hub).  
  - Side room / equipment room.  
  - Rear safe room (spawn area) with higher cover density.

Role tags like `["spawn_zone_a"]`, `["cover_high"]`, `["weapon_spawn_slot"]` inform procedural decoration and spawn/weapon logic.

### 5.4 Objective D – Vertical Layer Linking

**Goal:** Define where stairs, lifts, and ladders connect `Y = 0` to `Y = 10–14` (catwalks) and to `Y = -10` (sub‑level).

- For each link:  
  - `start_cell`: `(gx, gz, y_start)`  
  - `end_cell`: `(gx, gz or offset, y_end)`  
  - `type`: `stairs`, `ladder`, `elevator`.  
  - `width_tiles`: 1–2.

This gives tools the data to generate ramps and staircases that preserve classic “no jump with heavy weapons” constraints by controlling path widths and slopes.

### 5.5 Objective E – Hazard Volume Alignment

**Goal:** Ensure hazard volumes align with grate and drain tiles.

- For each hazard volume:  
  - `volume_id`: `hub_floor_gas`, `sublevel_acid`.  
  - `grid_inclusion_rule`: e.g., include all cells tagged `hazard_zone` and at `Y = 0 ± 2`.  
  - `exclude_cells`: specific safe islands (e.g., a small platform near the Egg that stays safe for a second).

This lets engines generate actual collision shapes from tile tags instead of hand‑placed boxes.

## 6. Example Hub Tile Sketch (Conceptual)

The following is a conceptual slice of the hub floor at `Y = 0`, in 4‑unit grid steps, for visualization only:

- Cells with radius ≤ 5 units: `T_Platform_EggBase`, role `["egg_platform"]`.  
- Radius 8–20: `T_Floor_IndustrialPlain`, role `["hub_floor"]`.  
- Radius 20–24: `T_Floor_GrateGasEmitter`, role `["hub_floor", "hazard_zone"]`.  
- Radius 24–28: `T_Wall_PanelRiveted`, collision `solid`.

Cardinal corridor entries (N, S, E, W) replace wall tiles in a 2‑cell wide window, using `T_Floor_IndustrialHazardStripe` to mark transitions.

## 7. Map Size and Travel Time Considerations

The final grid is sized so that:

- Time from bunker spawn to hub (straight route): ~5–7 seconds at N64‑style run speed, matching the time you need to reach the Tank canister bridge or a key chokepoint in Three Towers. [youtube](https://www.youtube.com/watch?v=o140m97FpSE)
- Time to loop the outer corridor ring: ~12–16 seconds, encouraging flanking but not making it trivial to avoid the hub entirely.

If playtests show the hub is too dominant, you can widen the ring corridors or add one or two small “service rooms” mid‑ring for cover and pickups.
