# Conker: Live & Uncut – AI Usage Recipe

When asking an AI agent to work on this project:

1. Always load:
   - specs/conker_live_uncut_constitution.md
   - specs/conker_live_uncut_schemas.md
   - specs/conker_live_uncut_tooling.md
   - specs/conker_live_uncut_mechanics.md
   - specs/conker_live_uncut_maps_manifest.json

2. For a specific map <MAP_ID>, also load:
   - docs/multiplayer/<MAP_ID>.md
   - data/<map>_grid_vN.json
   - data/<map>_entities_vN.json
   - tilesets/<engine>/<map>_tiles_vN.json

3. Before generating code, summarize:
   - Map goals (pacing, player count, core loop).
   - Invariants from the constitution that apply (pickup-only, no classes, etc.).
   - Which schemas the new code must satisfy.

4. Generate:
   - Either: data (JSON grids/entities/tilesets) that remain schema-valid.
   - Or: code that only reads/writes via those schemas and CLIs.

5. Never:
   - Change constitution files without explicit human instruction.
   - Introduce mechanics that violate the mechanics constraints.
