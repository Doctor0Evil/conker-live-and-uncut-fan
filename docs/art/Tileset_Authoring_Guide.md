# Tileset Authoring Guide (4×4 Uncut Grid)

This guide explains how to create new tileset assets for the Conker: Live & Uncut multiplayer maps so they work seamlessly with the grid2scene tool and the JSON tileset definitions.

It covers:
- Grid scale and pivot conventions.
- Mesh authoring rules for floors, walls, ceilings, and props.
- Naming scheme for tiles and materials.
- How `tile_type` strings map to engine assets (Blueprints, prefabs, scenes).
- How to add a new tileset without breaking existing maps.

---

## 1. Grid Scale and Pivots

All multiplayer maps use a **4×4 world-unit grid** for floor tiles.

- **Cell size:** 4.0 units (world space).
- **Grid origin:** The center of the overall grid is treated as (0, 0, 0) in world space by the tools.
- **Coordinate system:**
  - X axis: horizontal (left/right from the hub).
  - Y axis: vertical (up/down).
  - Z axis: depth (forward/back from the hub).

### 1.1 Floor Tiles

- Each floor tile must cover exactly **4 units in X** and **4 units in Z**.
- The mesh pivot must be at the **center of the tile**, on the **floor surface**:
  - Position: `(0, 0, 0)` at center.
  - Bounds: X from −2 to +2, Z from −2 to +2, Y from 0 up.
- When placed at grid position `(col, row)` the tool computes world position from the grid origin and multiplies by `cell_size = 4.0`. The mesh should “snap” perfectly edge-to-edge with its neighbors.

### 1.2 Wall and Column Tiles

- Standard walls are authored as **4 units wide** and typically **4–6 units tall**, depending on the map.
- Pivot should be at the **center of the tile on the floor**, just like floor tiles:
  - Position: `(0, 0, 0)` at the point where the wall meets the floor.
- The wall mesh can extend in Y as needed but should never protrude outside the 4×4 footprint in XZ, unless intentionally designed as an overhang.

### 1.3 Ceiling and Overhead Tiles

- Ceiling pieces that align with the grid should also use a 4×4 footprint.
- Pivot can be:
  - At the **center of the tile at floor level** (preferred, consistent with floors/walls), or
  - At the **center of the ceiling volume** if it is always placed with a fixed Y offset by code.
- For Alien Base, prefer floor-level pivots and let grid2scene set the Y offset for ceiling tiles.

---

## 2. Tile and Material Naming Scheme

Tiles are referenced from map grid JSON files by a `tile_type` string. That string must map to engine assets via a tileset JSON.

Examples from Alien Base:

- `TFloor_Industrial_Plain`
- `TFloor_Industrial_HazardStripe`
- `TFloor_Grate_GasEmitter`
- `TFloor_AlienResin`
- `TWall_Panel_Riveted`
- `TWall_AlienVeins`
- `TCeiling_PipeCluster`
- `TCeiling_GratedVent`
- `TPlatform_EggBase`

### 2.1 Tile Type Conventions

Use the following pattern:

```text
T<Layer>_<Theme>_<Variant>
```

Where:

- `<Layer>` is one of:
  - `Floor`, `Wall`, `Ceiling`, `Platform`, `Prop`.
- `<Theme>` is a short descriptor:
  - `Industrial`, `Alien`, `Stone`, `Metal`, `Wood`, etc.
- `<Variant>` is optional detail:
  - `Plain`, `HazardStripe`, `GrateGasEmitter`, `Resin`, `PanelRiveted`.

Examples:

- `TFloor_Industrial_Plain`
- `TFloor_Industrial_HazardStripe`
- `TFloor_Grate_GasEmitter`
- `TWall_Panel_Riveted`
- `TWall_AlienVeins`
- `TCeiling_PipeCluster`

These names are **engine-agnostic** and appear directly in the grid JSON.

### 2.2 Material Naming

Materials should follow a similar convention but are engine-specific:

- Unreal:
  - `M_IndustrialFloor_Base`
  - `M_IndustrialFloor_GrateGas`
  - `M_AlienWall_Resin`
- Unity:
  - `Mat_IndustrialFloor_Base`
  - `Mat_IndustrialFloor_GrateGas`
  - `Mat_AlienWall_Resin`
- Godot:
  - `mat_industrial_floor_base`
  - `mat_industrial_floor_grate_gas`
  - `mat_alien_wall_resin`

Keep the number of unique materials low and prefer texture atlases or texture arrays when possible.

---

## 3. Tileset JSON Mapping

Each engine has its own tileset JSON that maps `tile_type` to a concrete asset path or identifier.

### 3.1 Unreal Tileset

Example: `tilesets/unreal/alienbase_tiles_v1.json`

```json
{
  "version": "1.0.0",
  "engine": "unreal",
  "tile_mappings": [
    {
      "tile_type": "TFloor_Industrial_Plain",
      "asset_id": "/Game/AlienBase/Meshes/SM_Floor_Industrial_Plain.SM_Floor_Industrial_Plain"
    },
    {
      "tile_type": "TFloor_Grate_GasEmitter",
      "asset_id": "/Game/AlienBase/Meshes/SM_Floor_Grate_GasEmitter.SM_Floor_Grate_GasEmitter"
    },
    {
      "tile_type": "TWall_Panel_Riveted",
      "asset_id": "/Game/AlienBase/Meshes/SM_Wall_Panel_Riveted.SM_Wall_Panel_Riveted"
    },
    {
      "tile_type": "TPlatform_EggBase",
      "asset_id": "/Game/AlienBase/Meshes/SM_Platform_EggBase.SM_Platform_EggBase"
    }
  ],
  "entity_mappings": {
    "spawn": {
      "prefab_or_class": "/Game/AlienBase/Actors/BP_PlayerStart_AlienBase.BP_PlayerStart_AlienBase"
    },
    "weapon_pickup": {
      "Chainsaw": "/Game/AlienBase/Pickups/BP_Uncut_Pickup_Chainsaw.BP_Uncut_Pickup_Chainsaw",
      "Shotgun": "/Game/AlienBase/Pickups/BP_Uncut_Pickup_Shotgun.BP_Uncut_Pickup_Shotgun"
    },
    "hazard_volume": {
      "FloorGas": "/Game/AlienBase/Hazards/BP_AlienBase_Volume_HubFloorGas.BP_AlienBase_Volume_HubFloorGas",
      "SublevelAcid": "/Game/AlienBase/Hazards/BP_AlienBase_Volume_SublevelAcid.BP_AlienBase_Volume_SublevelAcid"
    },
    "objective": {
      "Egg": "/Game/AlienBase/Actors/BP_AlienEgg.BP_AlienEgg"
    }
  }
}
```

Unreal artists should:

- Place meshes in `/Game/<MapName>/Meshes/`.
- Place Blueprints in `/Game/<MapName>/Actors/` or `/Game/<MapName>/Pickups/`.
- Use the `SM_`, `BP_` prefixes consistently.

### 3.2 Unity Tileset

Example: `tilesets/unity/alienbase_tiles_v1.json`

```json
{
  "version": "1.0.0",
  "engine": "unity",
  "tile_mappings": [
    {
      "tile_type": "TFloor_Industrial_Plain",
      "asset_id": "Assets/Prefabs/AlienBase/TFloor_Industrial_Plain.prefab"
    },
    {
      "tile_type": "TFloor_Grate_GasEmitter",
      "asset_id": "Assets/Prefabs/AlienBase/TFloor_Grate_GasEmitter.prefab"
    }
  ],
  "entity_mappings": {
    "spawn": {
      "prefab_or_class": "Assets/Prefabs/AlienBase/SpawnPoint.prefab"
    },
    "weapon_pickup": {
      "Chainsaw": "Assets/Prefabs/AlienBase/PickupChainsaw.prefab",
      "Shotgun": "Assets/Prefabs/AlienBase/PickupShotgun.prefab"
    },
    "hazard_volume": {
      "FloorGas": "Assets/Prefabs/AlienBase/AlienBaseVolumeHubFloorGas.prefab",
      "SublevelAcid": "Assets/Prefabs/AlienBase/AlienBaseVolumeSublevelAcid.prefab"
    },
    "objective": {
      "Egg": "Assets/Prefabs/AlienBase/AlienEgg.prefab"
    }
  }
}
```

Unity artists should:

- Put prefabs under `Assets/Prefabs/<MapName>/`.
- Match prefab names to the design doc: `PickupChainsaw`, `AlienBaseVolumeHubFloorGas`, etc.

### 3.3 Godot Tileset

Example: `tilesets/godot/alienbase_tiles_v1.json`

```json
{
  "version": "1.0.0",
  "engine": "godot",
  "tile_mappings": [
    {
      "tile_type": "TFloor_Industrial_Plain",
      "asset_id": "0"
    },
    {
      "tile_type": "TFloor_Grate_GasEmitter",
      "asset_id": "1"
    }
  ],
  "entity_mappings": {
    "spawn": {
      "prefab_or_class": "res://AlienBase/SpawnPoint.tscn"
    },
    "weapon_pickup": {
      "Chainsaw": "res://AlienBase/PickupChainsaw.tscn",
      "Shotgun": "res://AlienBase/PickupShotgun.tscn"
    },
    "hazard_volume": {
      "FloorGas": "res://AlienBase/AlienBaseVolumeHubFloorGas.tscn",
      "SublevelAcid": "res://AlienBase/AlienBaseVolumeSublevelAcid.tscn"
    },
    "objective": {
      "Egg": "res://AlienBase/AlienEgg.tscn"
    }
  }
}
```

Godot artists should:

- Create a `MeshLibrary` resource with stable indices for each tile mesh.
- Ensure indices are stable once in use, as they are referenced by stringified numbers in the tileset JSON.

---

## 4. Adding a New Tile

To add a new tile:

1. Choose a `tile_type` name following the `T<Layer>_<Theme>_<Variant>` pattern.
2. Create the mesh with correct 4×4 footprint and centered pivot.
3. Set up materials using the established naming scheme.
4. Add the mesh as:
   - Unreal: `SM_<Name>` in `/Game/<MapName>/Meshes/`.
   - Unity: prefab in `Assets/Prefabs/<MapName>/`.
   - Godot: new `MeshLibrary` item and optional `.tscn` wrapper.
5. Add a `tile_mappings` entry for each engine’s tileset JSON.
6. Use the new `tile_type` in map grid JSON cells.
7. Run `grid2scene --validate --all` to ensure the new tile is recognized across tilesets.

---

## 5. Adding a New Tileset for a Map

For a completely new map (e.g., Fortress, Raptor Temple):

1. Define the tile palette and `tile_type` names in the map’s design document.
2. Author meshes and materials according to the 4×4 rules.
3. Create three tileset JSON files (Unreal, Unity, Godot) in `tilesets/<engine>/<map>_tiles_v1.json`.
4. Add the tileset paths into `maps/map_manifest_v1.json` for that map.
5. Validate by running:
   - `grid2scene --validate --map <id> --engine unreal`
   - `grid2scene --validate --map <id> --engine unity`
   - `grid2scene --validate --map <id> --engine godot`

Once validation passes, the map is ready for greybox generation in all three engines.
