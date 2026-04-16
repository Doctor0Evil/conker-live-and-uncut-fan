# 03 Fortress Bunker Layout (Interior Slice)

This document defines the interior layout for the Fortress bunker in **03_Multiplayer_Fortress**, using the 4x4 grid system and `MultiplayerMapGridV1` schema. It is designed as a research object for AI-Chat and tools like `grid2scene`, providing a consistent set of `role_tags` that can be referenced from JSON, Rust, C#, GDScript, and engine blueprints. [file:3]

The bunker interior connects directly to the Tediz base courtyard and roof areas already represented in `fortressgridv1.json` and `fortressentitiesv1.json`. It focuses on tight corridors, a commander room, radio/operations room, ammo cache, and multiple vertical routes (stairs, ladder, vent) leading to the roof. [file:3]

---

## Design Goals

The Fortress bunker should: [file:3]

- Provide a defensible **commander room** that can serve as a capture or defend objective in some modes.  
- Offer multiple **vertical routes** to roof positions (stairs, ladder, vent) so attackers have creative ways to flank Tediz defenders.  
- Integrate an **ammo cache** and heavy-weapon support for defenders while keeping at least one **vent/side entrance** for SHC infiltrators.  
- Maintain readability in a 4x4 grid while staying faithful to the N64 Total War bunker feel. [file:3]

---

## Bunker Role Tags

These `role_tags` are intended for `cells[].roletags` in `fortressbunkergridv1.json` and should be reused whenever possible in entities and engine scripts: [file:1][file:3]

- `bunker_interior` – Any tile that is part of the underground bunker volume.  
- `bunker_corridor` – Linear or L-shaped tunnels connecting bunker rooms.  
- `commander_room` – Main bunker room, often the target for defend/capture objectives.  
- `radio_room` – Small communications/operations room adjacent to the commander room.  
- `ammo_cache` – Storage room for ammo and heavy weapons; can anchor weapon pickups.  
- `roof_access` – Any tile that is part of a path leading up to the roof (stairs top, hatch, etc.).  
- `stairwell` – Stair tiles connecting bunker floor to an intermediate level or roof.  
- `ladder_shaft` – Narrow vertical access shaft, often one tile wide, to roof or sublevel.  
- `vent_entrance` – Small infiltration route (vents/ducts) that bypasses primary chokepoints.  
- `bunker_objective_zone` – Area for objectives such as capture zones or GasCanister arm points.  
- `tedizbase` – Tag inherited from the outer Fortress layout to mark bunker tiles belonging to Tediz base.  
- `fallback_spawn` – Optional tiles that can host fallback spawns for certain modes. [file:3]

These tags complement the existing Fortress vocabulary: `shcbase`, `tedizbase`, `capturezone`, `trench`, `tower`, `bridgecenter`, `gascanisterarmspot`, etc., and should integrate cleanly with `fortressentitiesv1.json`. [file:3]

---

## High-Level Layout

The bunker interior is organized as a compact plus-shaped cross under the Tediz courtyard: [file:3]

- **South entrance corridor** connects from the courtyard into the bunker hub.  
- **West branch** leads to the **ammo cache**.  
- **East branch** leads to the **radio room** and **ladder shaft** to the roof.  
- **North branch** leads into the **commander room**, which can host a capture or defend objective.  
- A **vent entrance** from the trench side can drop into a side corridor near the radio room, giving SHC attackers a stealthier entry. [file:3]

This is implemented as a 12x12 bunker slice (grid-local), embedded in the main Fortress grid near the Tediz courtyard coordinates used for `fortressgridv1.json`. The example JSON slice included in this document uses logical coordinates that can be merged directly. [file:1][file:3]

---

## Integration with Fortress Grid and Entities

### Grid Integration

- The bunker slice is authored as `maps/fortress/fortressbunkergridv1.json`, using `MultiplayerMapGridV1`. [file:1]  
- It shares the same `mapid: "fortress"` and uses compatible `tiletype` strings such as `TFloorBunker`, `TFloorBunkerStairs`, `TFloorRoofHatch`, and `TFloorVent`. [file:3]  
- When merged into the main `fortressgridv1.json`, bunker tiles should align under the Tediz courtyard area (e.g., slightly lower `ylevel` or negative `heightoffset` to represent underground). [file:1][file:3]

### Entity Integration

- `fortressentitiesv1.json` can place:  
  - A **capture zone** or **commander objective** in tiles tagged `commander_room` and `bunker_objective_zone`.  
  - Ammo and heavy weapon pickups (Bazooka ammo, SMG) in tiles tagged `ammo_cache`.  
  - Fallback Tediz spawns on `bunker_corridor` tiles flagged with `fallback_spawn`.  
  - A GasCanister arm/defuse point in the commander room, using `gascanisterarmspot` plus `bunker_objective_zone` tags, if desired for a mode variant. [file:3]

This allows grid2scene to place level geometry and engine emitters to spawn gameplay actors using tag-based lookups rather than hardcoded coordinates. [file:1][file:3]

---

## Next Steps

- Merge `fortressbunkergridv1.json` into `fortressgridv1.json` or expose it as an additive layer in the tooling.  
- Add matching objectives and pickups in `fortressentitiesv1.json` that reference `commander_room`, `ammo_cache`, and `roof_access` tiles.  
- Extend tilesets (`fortresstilesv1.json`) with bunker-specific tiletypes (walls, floors, hatches) following the 4x4 scale and naming conventions. [file:3]
