# multiplayer_map_entities_v1 Schema

File: `schemas/multiplayer_map_entities_v1.schema.json`  
Purpose: define a stable, engine-agnostic contract for all `*_entities_v1.json` files used by the Conker: Live & Uncut multiplayer maps.

## Overview

Entities files describe gameplay actors and volumes that sit on top of a map's tile grid:

- Player spawns and spawn bands.
- Weapon and ammo pickups.
- Objectives (fences, money bag, alien egg, eggs/baby dino, blood vial, gas canisters).
- Hazards (damage volumes, gas, acid, airlocks).
- NPC spawners (zombies, aliens, Fire Imp).
- Helper volumes and decorators.

The file is organized into `layers` for readability and future tooling, but the schema keeps each entity type small and focused.

## Positioning and transforms

Each entity can be authored in grid or world space:

- `grid_coord` gives integer tile coordinates `(x, y)` in grid space.
- `transform` gives full world-space position and yaw rotation.

Tools may:

- Derive `transform` from `grid_coord` and `grid_ref.cell_size` if `transform` is omitted.
- Respect explicit `transform` for fine tweaks (e.g. centering in trenches, offsetting objectives).

## Spawns and fallback rules

`PlayerSpawn` entities cover:

- `kind: "point"` – single spawn marker.
- `kind: "band"` – a point plus `radius` that tools can use to randomize spawn within a disc.
- `kind: "fallback_rule"` – a logical rule (e.g. `fallback_condition: "after_fence_1_destroyed"`) that map code interprets using objective events.

`team` identifies the owning team, for example `"attackers"`, `"defenders"`, `"shc"`, `"tediz"`, `"team_red"`, `"team_blue"`.

## Objectives

`Objective` entities unify map-specific objectives:

- Beach Dead fences: `objective_type: "fence"`, `role_tags: ["fence_1"]`, `max_health` set.
- Heist money bag: `objective_type: "money_bag"`, `heavy_carry: true`, `drop_on_death: true`, `delivery_role_tags` pointing at base goal volumes.
- Alien Base egg: `objective_type: "alien_egg"`, `trigger_events: ["alien_egg_stage_1", "alien_egg_destroyed"]`, `linked` from hazards.
- Raptor Temple eggs / baby dino: `objective_type: "egg"` or `"baby_dino"`.
- Blood Count vial: `objective_type: "blood_vial"`, `heavy_carry: true`, `drop_on_death: true`.

The `custom_data` object is where engine-specific fields (Unreal blueprint names, Unity script IDs) can live without affecting the core contract.

## Hazards and helper volumes

`Hazard` and `HelperVolume` share the same basic volume description:

- `bounds.shape`: `"box"` or `"cylinder"`.
- `bounds.extent`: for boxes `(x, y, z)`, for cylinders `(radius, height)`.

`Hazard` adds:

- `hazard_type`: `"volume_damage_over_time"`, `"airlock"`, `"gas_release"`, `"acid_pool"`, `"fire_zone"`, or `"custom"`.
- `damage_per_second` and `execution_immunity_role_tags` for ASID-aware volume damage.
- `linked_objectives` and `trigger_role_tags` to model cases like airlocks triggered by alien eggs or buttons.

`HelperVolume` is explicitly non-damaging and is intended for:

- Spawn/LoS logic (e.g. `"spawn_blocker"`, `"los_blocker"`).
- Delivery zones (`"team_a_goal"`, `"team_b_goal"`).
- Scripting hooks for map-specific events.

## AI spawners

`AiSpawn` entities define:

- `npc_type`: `"zombie"`, `"alien"`, `"fire_imp"`, etc.
- `max_alive` and `respawn_delay_seconds` for spawn manager behavior.
- `spawn_on_role_tags_visible` to support line-of-sight aware spawning (e.g., only spawn zombies when no player can see a given helper volume).
- Optional `patrol_path_id` to link into engine-side patrol graphs.

## Versioning

Like the grid schema:

- `schema_version` tracks structural changes to the entities JSON.
- `content_version` tracks map-specific entity layout revisions.

The `grid2scene --validate` step should:

- Validate all entities files against this schema.
- Cross-check `map_id`, `grid_ref` fields against the corresponding grid.
- Verify any referenced IDs (e.g. `linked_objectives`, `TeamId` conventions) if you later add auxiliary manifest schemas.
