# Conker: Live & Uncut – Schema Bundle

All AI-generated code and data must conform to these JSON Schemas:

- Grid Schema: schemas/alien_base_hub_grid.schema.json
- Entities Schema: schemas/alien_base_entities.schema.json
- Tileset Schema: schemas/alien_base_tileset.schema.json

## Grid Schema

- Defines:
  - grid_size (cols, rows)
  - cell_size (world unit per cell)
  - y_level
  - origin (x, z)
  - cells[col, row, tile_type, walkable, role_tags...]

## Entities Schema

- Defines:
  - spawn_points[id, zone, col, row, y_offset]
  - weapon_pickups[id, type, col, row, y_offset]
  - hazard_volumes[id, type, center_col, center_row, radius_cells, y_min_offset, y_max_offset]
  - objectives[id, type, col, row, y_offset]

## Tileset Schema

- Defines:
  - engine (unreal/unity/godot)
  - tile_mappings[tile_type → asset_id]
  - entity_mappings for spawn, weapon_pickup, hazard_volume, objective

Any AI codegen step that changes grids, entities, or tilesets must preserve schema validity. When in doubt, tools must adjust code to match the schema, not adjust the schema to fit the code.
