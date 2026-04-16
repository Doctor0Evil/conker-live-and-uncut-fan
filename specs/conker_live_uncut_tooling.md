# Conker: Live & Uncut – Tooling Contracts

## grid2scene Family

Each map has a dedicated grid2scene tool crate, following this pattern:

- crates/alien_base_grid2scene
- crates/beach_dead_grid2scene
- crates/heist_grid2scene
- etc.

### Required Behavior

Given:
- --input <grid.json>
- --entities <entities.json>
- --tileset <tileset.json>
- --engine <unreal|unity|godot>
- --out <output-path>

The tool must:
- Parse grid, entities, tileset.
- Validate against schemas.
- Emit engine-specific layout:
  - Unreal: JSON for instanced static meshes, spawns, hazards, objectives.
  - Unity: JSON for prefab instantiation and hazard setup.
  - Godot: .tscn scene with GridMap + nodes or equivalent JSON.

### AI Codegen Rules

When AI generates or modifies tooling code, it must:
- Preserve CLI signature and flags.
- Preserve JSON field names and structures.
- Never hard-code engine asset paths; always read them from tileset.json.
- Maintain deterministic mapping: same JSON input → same scene output.
