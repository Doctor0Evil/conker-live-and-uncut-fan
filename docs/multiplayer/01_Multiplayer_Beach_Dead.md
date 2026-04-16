# 01_Multiplayer_Beach_Dead

Beach Dead is the Uncut‑era evolution of the N64 Beach scenario: an asymmetrical assault where unarmed or lightly armed attackers sprint up a killing field while entrenched defenders use fixed positions and heavy weapons. The map is designed for N64‑style pickup combat, not classes, and for 16 players (8 vs 8) with 4‑player split‑screen support.

## Design Goals

The level should feel like a higher‑fidelity version of the original N64 Beach: same basic geography and objective, with more cover, deeper trenches, and weapon pickups instead of class loadouts or XP. It must support a 16‑player cap and optional team‑deathmatch / FFA modes, without losing the core “push up the beach under fire” identity.

Key goals:

- Preserve the iconic beach assault silhouette and fence progression.
- Keep defender dominance at long range while allowing coordinated attacker breakthroughs.
- Ensure all combat power comes from pickups and map control, not classes or progression systems.

## Core Loop

Attackers (Frenchies / SHC) spawn on landing craft at the waterline and must breach three barricades (Fence 1, Fence 2, Fence 3) before storming the bunker interior. Each destroyed fence unlocks a closer attacker spawn, shifting the frontline up the beach.

Defenders (Tediz) spawn in and behind the cliff‑top fortress, using turrets, MG nests, and elevated firing positions to slow attackers until the round timer expires. In alternative modes, both sides can roam freely, but spawn logic and weapon availability still bias defenders toward the high ground.

## N64 Integrity Requirements

To keep this map in line with the Uncut intent and the N64 original:

- No classes  
  All players share the same base movement, health, and interaction rules. The “Long Ranger / Demolisher / Sneeker” roles from Live & Reloaded are represented instead by weapon combinations (sniper rifle, explosives, melee) placed as pickups at fixed map locations.

- No XP or meta progression  
  Every match is self‑contained. There are no unlock trees, leveling, or stat modifiers between rounds. Advantage comes from positioning and pickups.

- Movement and heavy‑weapon handling  
  Attackers have slightly higher base speed and less armor, emphasizing sprinting and low‑profile movement between bits of cover. Defenders move marginally slower but benefit from better cover and access to heavier weapons. Heavy pickups (bazooka, large explosive packs, potentially deployable MGs) impose a “no jump while equipped” penalty to mirror the N64 heavy‑weapon limitation.

- Objective logic  
  In classic Beach mode, attackers win by getting a required number of villagers to the extraction point (truck or bunker interior) before the timer expires. For Uncut, this can be represented either as:
  - Triggering a plunger/charge inside the bunker once enough attackers reach it, or
  - Reaching and eliminating a commander NPC at the back of the fortress.
  Defenders win by preventing that objective from being completed before time runs out.

## Layout Overview

Beach Dead is a long, sloped beach leading up to a fortified cliff line, with three primary lanes separated by obstacles and funnelled into a central bunker complex.

High‑level zones:

- Beach Zone  
  - Attacker spawn boats along the waterline at the lower Z edge of the map.  
  - Sand flats with scattered craters and tank traps.  
  - Three barbed‑wire Fences (Fence 1–3), each running roughly perpendicular to the shoreline, with trenches immediately in front to provide partial cover.

- Fortress Zone  
  - Bunker wall and main gate situated just behind Fence 3 on the upper plateau.  
  - Interior bunker corridors leading to a commander room / objective chamber.  
  - Roof‑mounted turrets, pillboxes, and sniper posts overlooking all three lanes.

The entire layout is defined on a 4x4 unit grid to match the shared grid system used by the other Uncut maps, allowing tools to build geometry directly from JSON descriptors.

## Map Structure (Lanes and Frontlines)

The beach is divided into three conceptual lanes:

- Left Lane  
  Slightly more cover but longer path; good for flanking and closer‑range engagements.  
- Center Lane  
  Most direct path to the bunker, but also the most exposed to MG nests and sniper fire.  
- Right Lane  
  Intermediate distance with a mix of low dunes and craters; often used for sneaking or coordinated pushes when other lanes are heavily suppressed.

Each fence line is a “frontline”:

- Fence 1  
  Closest to the shore. Once destroyed or bypassed, attackers start spawning at a trench just behind this fence instead of at the waterline.  
- Fence 2  
  Mid‑beach blockade. Its destruction moves attacker spawns further up and intensifies fights in the mid‑slope area.  
- Fence 3  
  Final external barrier. Its loss effectively pins defenders back to the bunker gate and interior.

## Weapon and Pickup Philosophy

All firepower flows from pickups and map control:

- Attacker pickups  
  - Light SMGs and pistols scattered near early trenches.  
  - Grenade and explosive packs near downed vehicles or craters, used to blow fences and MG nests.  
  - Occasional sniper rifle or scoped weapon closer to Fence 2–3 to reward successful pushes.

- Defender pickups  
  - Heavy MG nests and fixed turrets on the bunker roof and in pillboxes.  
  - Limited use rocket launchers or high‑damage explosives deeper in the fortress for last‑stand defense.  
  - Extra grenades and ammo inside the bunker interior.

Pickup placement should preserve strong initial defender advantage but allow attackers to snowball pressure once they secure new trench lines.

## Visual and Material Notes

Beach Dead’s visual identity should lean toward the saturated, slightly exaggerated N64 palette:

- Sand: bright, warm tones with stylized craters and tracks rather than photorealistic noise.  
- Bunkers: blue‑grey and concrete, with strong silhouettes and readable firing slits.  
- Uniforms: clear team‑color cues on helmets and armbands while staying within the Conker aesthetic.

Atmospheric elements such as offshore battleships, distant explosions, and skybox bombardment are implemented as non‑interactive VFX to avoid cluttering the gameplay space.

## Implementation Notes

Use the shared grid/tile JSON structure defined for Alien Base, with the following adjustments:

- Footprint  
  - Rectangular grid longer on the Z axis (shoreline to fortress) than on X.  
  - Gentle Y rise from the shoreline up to the cliff, with discrete stair/step changes instead of a continuous slope to keep platforming predictable.

- Data files  
  - `data/beach_dead_grid_v1.json`: tiles and role tags (beach, trench, fence_1, fence_2, fence_3, bunker_interior).  
  - `data/beach_dead_entities_v1.json`: spawn points, fence destructible anchors, MG nests, bunker door / objective actors.  
  - `tilesets/*/beach_dead_tiles_v1.json`: tile and entity mappings per engine.

Spawn, objective, and MG nest placements should be driven entirely by the entities JSON so you can switch between classic Beach mode and more freeform deathmatch variants without touching the underlying geometry.
