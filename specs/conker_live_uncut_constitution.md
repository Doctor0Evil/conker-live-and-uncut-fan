# Conker: Live & Uncut – Design Constitution

This document defines the non-negotiable design rules for the Conker: Live & Uncut revival. All code generation, level generation, and tooling must comply with these rules.

## Core Philosophy

- The project is a spiritual continuation of Conker's Bad Fur Day (N64) Multi, not a remake of the class-based Live & Reloaded.
- Multiplayer is pickup-based, N64-style:
  - No classes.
  - No XP, no progression unlocks, no perk trees.
  - Every player spawns with the same baseline capability and acquires power through map pickups.
- Map feel must match N64 Multi pacing:
  - Slower movement, inertia, and limited jump while carrying heavy weapons.
  - Tight, readable arenas with clear chokepoints and hazard telegraphs.

## Canonical Modes (N64 Multi)

AI-assisted tools must treat the following modes as canonical references: Total War, Colors, Beach, Raptor, Heist, Race, Tank, Deathmatch. [Multi reference]  
- Total War / Colors: team-based objective and deathmatch on a shared war arena.  
- Beach: asymmetrical beach assault.  
- Raptor: Uga Buga cavemen vs raptors on an altar/temple arena.  
- Heist: money-bag CTF in a bank with gas chamber instant-win.  
- Race: hoverboard racing in a lava track.  
- Tank: vehicle combat in a canyon with gas canister instant-win.  
- Deathmatch: free-for-all variants using maps above. [web:123]

## Uncut Map Set (Target)

For the 2003-style timeline, tools must recognize and preserve this map set and ID scheme:

- 01_Multiplayer_Beach_Dead
- 02_Multiplayer_The_Heist
- 03_Multiplayer_Fortress
- 04_Multiplayer_Alien_Base
- 05_Multiplayer_Raptor_Temple
- 06_Multiplayer_TMS_Spamono

Each map's MD, JSON grids, and entities must use the same ID prefix to ensure cross-file consistency.

## Data-Driven Design

- Map layout, entities, and tiles are defined in JSON + JSON Schema.
- Rust/C++ tools read JSON and generate engine scenes; manual engine edits are discouraged.
- When conflicts arise, JSON/Markdown specs are the source of truth; code is regenerated to match them, not edited by hand.

## AI Codegen Requirements

- AI must read:
  - The constitution (this file).
  - The relevant map design MD.
  - The grid + entities + tileset JSON.
- AI must not introduce:
  - Class-specific mechanics.
  - XP or meta-progression systems.
  - Non-Conker weapons or aesthetics that contradict the N64 palette and tone.
- AI may extend:
  - Tooling for grids/scenes.
  - Engine bindings (Unreal, Unity, Godot).
  - Non-invasive convenience features (debug views, analytics, etc.).
