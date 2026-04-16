# Alien Base – grid2scene Tool Spec

This document defines a small Rust/C++ command‑line tool that converts the hub floor grid JSON (`data/alien_base_hub_grid_v1.json`) into engine‑specific scene data for Unreal, Unity, and Godot. The goal is: one JSON → one command → playable greybox hub in any supported engine.

## 1. CLI Contract

Tool name: `alien_base_grid2scene`

### 1.1 Invocation

- Basic:  
  - `alien_base_grid2scene --input data/alien_base_hub_grid_v1.json --engine unreal --out build/unreal/AlienBase_Hub.json`  
  - `alien_base_grid2scene --input data/alien_base_hub_grid_v1.json --engine unity --out build/unity/AlienBase_Hub.json`  
  - `alien_base_grid2scene --input data/alien_base_hub_grid_v1.json --engine godot --out build/godot/AlienBase_Hub.tscn`

### 1.2 Inputs

- `--input`: path to JSON file that conforms to `schemas/alien_base_hub_grid.schema.json`.  
- `--engine`: one of `unreal`, `unity`, `godot`.  
- `--out`: output file path (engine‑specific format below).  

Internally, the tool always parses the same grid structure and then branches to one of three emitters.

## 2. Shared Grid Interpretation

The tool reads:

- `grid_size.cols`, `grid_size.rows`.  
- `cell_size`.  
- `origin.x`, `origin.z` (world offset).  
- `y_level`.  
- `cells[]` (sparse list).

World position of a cell:

- Let `center_col = (cols - 1) / 2`.  
- Let `center_row = (rows - 1) / 2`.  
- For a cell `(col, row)`:

  - \( world\_x = origin.x + (col - center\_col) \times cell\_size \)  
  - \( world\_z = origin.z + (row - center\_row) \times cell\_size \)  
  - \( world\_y = y\_level \)

Tiles with `walkable = false` are treated as solid (walls). Tiles with `role_tags` including `hazard_zone` mark where gas floor meshes and hazard volumes should exist. Tiles with `egg_platform` mark the Alien Egg pedestal.

The tool maintains a simple internal mapping from `tile_type` to engine‑side mesh IDs or prefab names; these are configurable via a separate `tilesets/*.json` file per engine if you want more flexibility later.

## 3. Unreal Output: Instanced Static Mesh Layout

The Unreal target writes a small JSON blob that a C++/Blueprint importer reads and turns into `UInstancedStaticMeshComponent` instances for each tile type. [reddit](https://www.reddit.com/r/unrealengine/comments/1ote6qz/should_you_create_levels_using_instanced_static/)

### 3.1 Output Format

`build/unreal/AlienBase_Hub.json`:

```json
{
  "level_name": "LV_AlienBase_Multi",
  "hub_y_level": 0.0,
  "tile_sets": [
    {
      "tile_type": "T_Floor_IndustrialPlain",
      "mesh_path": "/Game/AlienBase/Meshes/SM_Floor_IndustrialPlain",
      "instances": [
        { "x": 0.0, "y": 0.0, "z": 4.0 },
        { "x": 4.0, "y": 0.0, "z": 0.0 }
      ]
    },
    {
      "tile_type": "T_Floor_GrateGasEmitter",
      "mesh_path": "/Game/AlienBase/Meshes/SM_Floor_GrateGasEmitter",
      "instances": [
        { "x": 0.0, "y": 0.0, "z": 12.0 }
      ]
    },
    {
      "tile_type": "T_Wall_PanelRiveted",
      "mesh_path": "/Game/AlienBase/Meshes/SM_Wall_PanelRiveted",
      "instances": [
        { "x": 0.0, "y": 0.0, "z": 20.0 }
      ]
    }
  ],
  "special": {
    "egg_platform": {
      "tile_type": "T_Platform_EggBase",
      "mesh_path": "/Game/AlienBase/Meshes/SM_EggPlatform",
      "position": { "x": 0.0, "y": 0.0, "z": 0.0 }
    },
    "hazard_floor_volume_bounds": {
      "center": { "x": 0.0, "y": 0.0, "z": 0.0 },
      "radius": 24.0
    }
  }
}
```

The tool builds `instances[]` per tile_type by iterating `cells[]`, computing `(x, y, z)` and grouping by `tile_type`. Hazard radius can be inferred as max distance of any `hazard_zone` tile from the Egg cell.

### 3.2 Unreal Import Usage

In Unreal, you write a small C++/Blueprint utility that:

- Loads `AlienBase_Hub.json`.  
- For each `tile_sets` entry:  
  - Creates a `UInstancedStaticMeshComponent` with the given `mesh_path`.  
  - Adds instances at the `instances` positions.  
- Places a single `SM_EggPlatform` StaticMeshActor at `special.egg_platform.position`.  
- Sets up `AAlienBase_Volume_HubFloorGas` with `hazard_floor_volume_bounds.center` and `radius`.

This aligns with Epic’s recommended ISM tile‑based workflows. [youtube](https://www.youtube.com/watch?v=cfR36FTbvcQ)

## 4. Unity Output: Prefab Instantiation Instructions

The Unity target writes a JSON layout that a C# editor script or runtime bootstrapper can read to instantiate prefabs in a grid, similar to common tile‑map loading patterns. [stackoverflow](https://stackoverflow.com/questions/53507919/instantiating-prefabs-from-an-imported-string-to-form-a-tile-map)

### 4.1 Output Format

`build/unity/AlienBase_Hub.json`:

```json
{
  "scene_name": "AlienBase_Multi",
  "hub_y_level": 0.0,
  "tiles": [
    {
      "tile_type": "T_Floor_IndustrialPlain",
      "prefab": "Prefabs/AlienBase/T_Floor_IndustrialPlain",
      "positions": [
        { "x": 0.0, "y": 0.0, "z": 4.0 },
        { "x": 4.0, "y": 0.0, "z": 0.0 }
      ]
    },
    {
      "tile_type": "T_Floor_GrateGasEmitter",
      "prefab": "Prefabs/AlienBase/T_Floor_GrateGasEmitter",
      "positions": [
        { "x": 0.0, "y": 0.0, "z": 12.0 }
      ]
    },
    {
      "tile_type": "T_Wall_PanelRiveted",
      "prefab": "Prefabs/AlienBase/T_Wall_PanelRiveted",
      "positions": [
        { "x": 0.0, "y": 0.0, "z": 20.0 }
      ]
    }
  ],
  "egg_platform": {
    "prefab": "Prefabs/AlienBase/T_Platform_EggBase",
    "position": { "x": 0.0, "y": 0.0, "z": 0.0 }
  }
}
```

A Unity script attached to an empty `HubBuilder` GameObject can:

- Load `AlienBase_Hub.json` (from StreamingAssets or Resources).  
- For each `tiles` entry, load the given prefab and call `Instantiate(prefab, position, Quaternion.identity)` in nested loops. [docs.unity3d](https://docs.unity3d.com/2017.4/Documentation/Manual/InstantiatingPrefabs.html)
- Instantiate the Egg platform prefab at the specified position.

You can optionally ignore wall tiles and generate colliders only, depending on how you want to greybox.

## 5. Godot Output: GridMap Scene

The Godot target directly emits a `.tscn` scene that contains a `GridMap` node and tile placements using `set_cell_item`, matching how Godot expects grid‑based 3D worlds to be authored. [godot-doc.readthedocs](https://godot-doc.readthedocs.io/en/3.0/classes/class_gridmap.html)

### 5.1 MeshLibrary Assumptions

Assume you have a `MeshLibrary` resource at `res://AlienBase/HubMeshLibrary.tres` with item indices:

- 0: `T_Floor_IndustrialPlain`  
- 1: `T_Floor_GrateGasEmitter`  
- 2: `T_Wall_PanelRiveted`  
- 3: `T_Platform_EggBase`  

The tool needs a small mapping from `tile_type` to `mesh_item_index` (configurable via a sidecar JSON if needed).

### 5.2 Output Scene Skeleton

`build/godot/AlienBase_Hub.tscn`:

```text
[gd_scene load_steps=3 format=3]

[ext_resource path="res://AlienBase/HubMeshLibrary.tres" type="MeshLibrary" id=1]

[node name="AlienBaseHub" type="Node3D"]

[node name="HubGrid" type="GridMap" parent="."]
mesh_library = ExtResource("1")
cell_size = Vector3(4, 4, 4)

[node name="EggPlatform" type="Node3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
```

The tool will then append `GridMap` cell placements after the nodes:

```text
[editable path="HubGrid"]

```

and generate a series of `set_cell_item` calls that Godot executes when the scene loads (or you can generate a script that runs on `_ready()`).

Because `.tscn` is text, you can choose either:

- Pure data: use Godot’s `GridMap` stored cells in the scene; or  
- Generated script: attach a `HubGridBuilder.gd` that sets cells in `_ready()`.

At minimum, the spec requires:

- For each walkable floor tile: `set_cell_item(grid_x, 0, grid_z, mesh_item_index, 0)`.  
- For wall tiles: `set_cell_item(grid_x, 0, grid_z, wall_item_index, 0)`.

The tool computes `grid_x` and `grid_z` by mapping `col`/`row` indices directly (or offsetting by `-center_col`, `-center_row` depending on how you want the origin).

## 6. Implementation Notes (Rust/C++)

The actual implementation language is up to you, but the core steps are identical:

1. Parse JSON (`serde_json` in Rust, `nlohmann::json` or RapidJSON in C++). [ahl.dtrace](https://ahl.dtrace.org/2024/01/22/rust-and-json-schema/)
2. Build an in‑memory representation:

   - `struct Cell { int col, row; std::string tile_type; bool walkable; std::vector<std::string> role_tags; }`  
   - `struct Grid { int cols, rows; float cell_size; float y_level; float origin_x, origin_z; std::vector<Cell> cells; }`

3. Compute center indices and world positions.  
4. Group cells by `tile_type` for Unreal/Unity outputs.  
5. Align hazard radius with the farthest `hazard_zone` tile from Egg.

Because this spec is deterministic and schema‑based, AI‑chat codegen can safely produce or modify the Rust/C++ tool without drifting engine behavior.
