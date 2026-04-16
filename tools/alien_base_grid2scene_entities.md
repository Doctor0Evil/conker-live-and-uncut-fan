# Alien Base – grid2scene Entity Extension

This document extends the `alien_base_grid2scene` tool so that, in addition to tiles, it also emits spawn points, weapon pickups, and hazard volume configuration from:

- `data/alien_base_hub_grid_v1.json` – hub floor tiles.  
- `data/alien_base_hub_entities_v1.json` – logical entity definitions bound to grid or world positions.

The objective is: “hub‑only playable scene” for Unreal, Unity, and Godot, generated from a small set of JSON files in one command. [youtube](https://www.youtube.com/watch?v=GqNTn7zCoY8)

***

## 1. New Input: `entities.json` Contract

Add a new file: `data/alien_base_hub_entities_v1.json`.

### 1.1 Top‑Level Structure

```json
{
  "version": "1.0.0",
  "spawn_points": [
    {
      "id": "Spawn_ZoneA_01",
      "zone": "A",
      "col": 8,
      "row": 2,
      "y_offset": 0.0
    }
  ],
  "weapon_pickups": [
    {
      "id": "Pickup_Chainsaw_NE",
      "type": "Chainsaw",
      "col": 12,
      "row": 5,
      "y_offset": 0.0
    }
  ],
  "hazard_volumes": [
    {
      "id": "HubFloorGas",
      "type": "FloorGas",
      "center_col": 8,
      "center_row": 8,
      "radius_cells": 5,
      "y_min_offset": -2.0,
      "y_max_offset": 4.0
    }
  ],
  "objectives": [
    {
      "id": "AlienEgg",
      "type": "Egg",
      "col": 8,
      "row": 8,
      "y_offset": 0.0
    }
  ]
}
```

Each entity uses grid indices (col, row) plus an optional `y_offset`; the tool converts these to world positions using the same grid math as tiles.

### 1.2 Semantics

- `spawn_points`: player start locations in the hub layer (hub‑only build can use a subset of the earlier Zone A–D plan).  
- `weapon_pickups`: static weapon spawn anchors; the engine maps `type` to a concrete class or prefab.  
- `hazard_volumes`: area definitions for map‑wide gas/acid; radii are expressed in grid cells for round hubs, then converted to world units.  
- `objectives`: major interactables/targets like the Alien Egg.

***

## 2. Extended CLI

The tool gains one more argument:

- `--entities data/alien_base_hub_entities_v1.json`

Example:

- Unreal:  
  `alien_base_grid2scene --input data/alien_base_hub_grid_v1.json --entities data/alien_base_hub_entities_v1.json --engine unreal --out build/unreal/AlienBase_Hub.json`

- Unity:  
  `alien_base_grid2scene --input data/alien_base_hub_grid_v1.json --entities data/alien_base_hub_entities_v1.json --engine unity --out build/unity/AlienBase_Hub.json`

- Godot:  
  `alien_base_grid2scene --input data/alien_base_hub_grid_v1.json --entities data/alien_base_hub_entities_v1.json --engine godot --out build/godot/AlienBase_Hub.tscn`

If `--entities` is not supplied, the tool behaves as before (tiles only).

***

## 3. Shared Entity Interpretation

Given grid, origin, and `cell_size`:

- For any entity with `(col, row, y_offset)`:

  - \( x = origin.x + (col - center\_col) \times cell\_size \)  
  - \( z = origin.z + (row - center\_row) \times cell\_size \)  
  - \( y = y\_level + y\_offset \)

For hazard volumes:

- `center_col`, `center_row` → center `(x, y, z)` using same formula (with `y = y_level`).  
- `radius_cells` → \( radius\_world = radius\_cells \times cell\_size \).  
- `y_min_offset`/`y_max_offset` → vertical bounds relative to `y_level`.

This keeps all entity positions consistent with tile centers, making it simple to derive collision bounds and spawn/weapon placement in tools and engines. [forums.unrealengine](https://forums.unrealengine.com/t/read-data-from-file-some-kind-of-dynamic-level/2631)

***

## 4. Unreal Output Extension

Extend the Unreal output JSON to include spawns, pickups, and hazards.

### 4.1 Updated Unreal Layout

`build/unreal/AlienBase_Hub.json`:

```json
{
  "level_name": "LV_AlienBase_Multi",
  "hub_y_level": 0.0,
  "tile_sets": [
    { "tile_type": "T_Floor_IndustrialPlain", "mesh_path": "/Game/AlienBase/Meshes/SM_Floor_IndustrialPlain", "instances": [ /* ... */ ] }
  ],
  "special": {
    "egg_platform": {
      "tile_type": "T_Platform_EggBase",
      "mesh_path": "/Game/AlienBase/Meshes/SM_EggPlatform",
      "position": { "x": 0.0, "y": 0.0, "z": 0.0 }
    }
  },
  "spawns": [
    {
      "id": "Spawn_ZoneA_01",
      "zone": "A",
      "position": { "x": 0.0, "y": 0.0, "z": 28.0 },
      "yaw_degrees": 180.0
    }
  ],
  "weapon_pickups": [
    {
      "id": "Pickup_Chainsaw_NE",
      "type": "Chainsaw",
      "class_path": "/Game/AlienBase/Pickups/BP_Pickup_Chainsaw",
      "position": { "x": 12.0, "y": 0.0, "z": 12.0 }
    }
  ],
  "hazards": [
    {
      "id": "HubFloorGas",
      "type": "FloorGas",
      "volume_class_path": "/Game/AlienBase/Hazards/BP_AlienBase_Volume_HubFloorGas",
      "center": { "x": 0.0, "y": 0.0, "z": 0.0 },
      "radius": 24.0,
      "y_min": -2.0,
      "y_max": 4.0
    }
  ],
  "objectives": [
    {
      "id": "AlienEgg",
      "type": "Egg",
      "class_path": "/Game/AlienBase/Actors/BP_AlienEgg",
      "position": { "x": 0.0, "y": 0.0, "z": 0.0 }
    }
  ]
}
```

### 4.2 Unreal Import Behavior

Your Unreal importer (C++ or Blueprint):

- Reads `spawns[]` and creates `APlayerStart_ZoneX_##` actors at those positions with appropriate yaw.  
- Reads `weapon_pickups[]` and spawns `BP_Pickup_*` actors.  
- Reads `hazards[]` and spawns volume actors (`BP_AlienBase_Volume_HubFloorGas`) centered at `center` with shape/scale derived from radius and Y bounds.  
- Reads `objectives[]` and spawns main objectives (Alien Egg, consoles) at their positions.

This matches common data‑table‑driven spawning in UE5, except you are using JSON as the data source instead of CSV. [youtube](https://www.youtube.com/watch?v=GqNTn7zCoY8)

***

## 5. Unity Output Extension

Extend the Unity output to include the same logical entities.

### 5.1 Updated Unity Layout

`build/unity/AlienBase_Hub.json`:

```json
{
  "scene_name": "AlienBase_Multi",
  "hub_y_level": 0.0,
  "tiles": [
    {
      "tile_type": "T_Floor_IndustrialPlain",
      "prefab": "Prefabs/AlienBase/T_Floor_IndustrialPlain",
      "positions": [ { "x": 0.0, "y": 0.0, "z": 4.0 } ]
    }
  ],
  "spawns": [
    {
      "id": "Spawn_ZoneA_01",
      "zone": "A",
      "prefab": "Prefabs/AlienBase/SpawnPoint",
      "position": { "x": 0.0, "y": 0.0, "z": 28.0 },
      "yaw_degrees": 180.0
    }
  ],
  "weapon_pickups": [
    {
      "id": "Pickup_Chainsaw_NE",
      "type": "Chainsaw",
      "prefab": "Prefabs/AlienBase/Pickup_Chainsaw",
      "position": { "x": 12.0, "y": 0.0, "z": 12.0 }
    }
  ],
  "hazards": [
    {
      "id": "HubFloorGas",
      "type": "FloorGas",
      "prefab": "Prefabs/AlienBase/Hazard_FloorGas",
      "center": { "x": 0.0, "y": 0.0, "z": 0.0 },
      "radius": 24.0,
      "y_min": -2.0,
      "y_max": 4.0
    }
  ],
  "objectives": [
    {
      "id": "AlienEgg",
      "type": "Egg",
      "prefab": "Prefabs/AlienBase/AlienEgg",
      "position": { "x": 0.0, "y": 0.0, "z": 0.0 }
    }
  ]
}
```

### 5.2 Unity Builder Behavior

A `HubBuilder` C# script can:

- Instantiate floor and wall prefabs similar to before.  
- Instantiate a generic spawn point prefab at each `spawns[]` position and rotate it by `yaw_degrees`.  
- Instantiate weapon pickup prefabs and set their type via a component.  
- Instantiate hazard prefabs (gas/acid) and configure collider radius/height based on `radius`, `y_min`, `y_max`.  
- Instantiate the Alien Egg prefab and link it to your game mode logic.

This is a straightforward extension of prefab instantiation patterns already used for tile‑based maps. [stackoverflow](https://stackoverflow.com/questions/53507919/instantiating-prefabs-from-an-imported-string-to-form-a-tile-map)

***

## 6. Godot Output Extension

For Godot, the entity data can either be embedded inside the `.tscn` as extra nodes or referenced by a `HubEntities.gd` script that reads a JSON at runtime.

### 6.1 Embedding Entity Nodes

Extend `AlienBase_Hub.tscn` with spawn markers, hazard volumes, and objectives:

```text
[node name="Spawn_ZoneA_01" type="Marker3D" parent="AlienBaseHub"]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 28)

[node name="Pickup_Chainsaw_NE" type="Node3D" parent="AlienBaseHub"]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 12, 0, 12)
script = ExtResource("res://AlienBase/Pickup_Chainsaw.gd")

[node name="Hazard_HubFloorGas" type="Area3D" parent="AlienBaseHub"]
script = ExtResource("res://AlienBase/HazardVolume.gd")
```

The grid2scene tool, when targeting Godot, now:

- Emits `GridMap` cell data like before.  
- Adds extra `node` blocks for each spawn, pickup, hazard, and objective with computed `transform`.  
- Sets `script` references if you want them directly wired.

### 6.2 Alternative: Runtime Builder

Alternatively, you can emit a simple JSON layout like the Unity one and attach a `HubEntities.gd` script that loads it on `_ready()` and spawns entities from PackedScenes, using `SpawnPoint` arrays and hazard Area3D nodes. [youtube](https://www.youtube.com/watch?v=BLmo7oY_VL4)

***

## 7. Example `entities` Data for Hub‑Only Prototype

Here is a small, hub‑only `data/alien_base_hub_entities_v1.json` that lines up with the grid center (`col=8,row=8`) and uses only the hub for now:

```json
{
  "version": "1.0.0",
  "spawn_points": [
    { "id": "Spawn_Hub_North", "zone": "Hub", "col": 8, "row": 4, "y_offset": 0.0 },
    { "id": "Spawn_Hub_South", "zone": "Hub", "col": 8, "row": 12, "y_offset": 0.0 },
    { "id": "Spawn_Hub_East",  "zone": "Hub", "col": 12, "row": 8, "y_offset": 0.0 },
    { "id": "Spawn_Hub_West",  "zone": "Hub", "col": 4,  "row": 8, "y_offset": 0.0 }
  ],
  "weapon_pickups": [
    { "id": "Pickup_Chainsaw_NE", "type": "Chainsaw", "col": 11, "row": 5, "y_offset": 0.0 },
    { "id": "Pickup_Flamethrower_NW", "type": "Flamethrower", "col": 5, "row": 5, "y_offset": 0.0 },
    { "id": "Pickup_SMG_SE", "type": "SMG", "col": 11, "row": 11, "y_offset": 0.0 },
    { "id": "Pickup_Shotgun_SW", "type": "Shotgun", "col": 5, "row": 11, "y_offset": 0.0 }
  ],
  "hazard_volumes": [
    {
      "id": "HubFloorGas",
      "type": "FloorGas",
      "center_col": 8,
      "center_row": 8,
      "radius_cells": 5,
      "y_min_offset": -2.0,
      "y_max_offset": 4.0
    }
  ],
  "objectives": [
    {
      "id": "AlienEgg",
      "type": "Egg",
      "col": 8,
      "row": 8,
      "y_offset": 0.0
    }
  ]
}
```

Combined with your hub grid JSON and this spec, one CLI call can now generate, per engine:

- Tiles/walls.  
- Four hub spawns.  
- Four key pickup spots.  
- The main Alien Egg objective.  
- A functional gas hazard volume aligned with your grate tiles.
