# multiplayer_map_grid_v1 Schema

File: `schemas/multiplayer_map_grid_v1.schema.json`  
Purpose: define a stable, engine-agnostic contract for all `*_grid_v1.json` files used by the Conker: Live & Uncut multiplayer maps.

## Overview

Each grid file describes a 2D array of cells on a fixed 4x4 world-unit grid. The schema keeps per-cell data minimal and pushes complex logic into tools like `alien_base_grid2scene`.

Key invariants:

- `schema_version` must follow `MAJOR.MINOR.PATCH`.
- `map_id` is stable and matches directories like `maps/beach_dead/`.
- `tileset_id` links to a tileset definition used to validate `tile_type` values.
- `grid.width * grid.height` must match the number of cells in `grid.rows`.

## Coordinate model

Cells are stored row-major in `grid.rows[y][x]`:

- `x` ranges from `0 .. width-1` (east/right).
- `y` ranges from `0 .. height-1` (south/down or forward, depending on engine convention).
- World position for a cell is typically:

  - `world_x = origin.x + x * cell_size`
  - `world_y = origin.y`
  - `world_z = origin.z + y * cell_size`

The `origin` field is optional; tools default to `(0, 0, 0)` when it is missing.

## Cell fields

- `tile_type` (string, required)  
  Logical tile identifier, such as `"floor_concrete_a"` or `"wall_bunker_inner"`. The grid2scene validator must check that the value exists in the tileset referenced by `tileset_id`.

- `role_tags` (string array, optional)  
  Semantic labels for gameplay roles, like:

  - `"trench"`, `"fence_1"`, `"fence_2"` (Beach Dead)
  - `"vault"`, `"gas_chamber"` (The Heist)
  - `"shc_base"`, `"tediz_base"`, `"capture_zone"` (Fortress)
  - `"uga_base"`, `"raptor_nest"` (Raptor Temple)

  Multiple tags can be applied to the same cell.

- `height_offset` (number, optional)  
  Per-cell vertical offset in world units. Used for raised walkways, ramps, or multi-story structures without changing the fundamental 2D topology.

- `rotation_deg` (number, optional)  
  Yaw rotation applied when spawning the tile instance. Standard values are `0`, `90`, `180`, `270`, but the schema permits any value between `-360` and `360`.

- `variant_id` (string, optional)  
  Selects alternate models or materials within the same `tile_type`, for visual variety without new tile IDs.

- `flags` (string array, optional)  
  Implementation hints for emitters, such as `"no_collision"`, `"nav_blocker"`, or `"visibility_only"`. These are intentionally free-form so different engine backends can interpret them as needed.

## Versioning

- `schema_version` tracks breaking and non-breaking changes to the JSON structure and validation rules.
- `content_version` is optional but recommended for tracking map layout revisions independently of the tooling.

When `schema_version` is bumped in a breaking way, grid2scene should refuse to process grids with incompatible versions until updated.
