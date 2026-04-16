# 04_Multiplayer_Alien_Base

## 0. Overview

**Internal ID:** 04  
**Working Title:** Alien Base  
**Lineage:**  
- Geometry & pacing: N64 _War_ (Total War), _Tank_ (Canyon), _Heist_ (Vault).  
- Thematic: Final Alien encounter in single-player (lime‑green gore, black xenomorph drones) and “future war” aesthetic later seen in Three Towers.

**Design Pillars**

- No classes, no XP, no loadouts – weapon pickups only.  
- 16 players max (LAN/online), 4‑player split‑screen supported.  
- Brutal N64‑style movement constraints (no jumping with heavy weapons, hard‑lock executions).  
- Central hub + ring corridors + sub‑level hazards (gas, acid).

---

## 1. Map Layout & Topology

### 1.1 High‑Level Structure

- **Central Hub:** Circular Alien Queen chamber / reactor room at world origin (0, 0, 0).  
- **Spoke Corridors:** Four main corridors (N, S, E, W) connecting hub to team zones and auxiliary rooms (airlock control, med bay, reactor maintenance).  
- **Outer Ring:** Continuous outer corridor loop allowing flanks and rotation.  
- **Sub‑Level:** Lower tunnels beneath hub floor for heavy pickups and hazard routes.  
- **Verticality:** Catwalks and rafters above hub; vent network connecting high points and some outer rooms.

### 1.2 Role Tags (Grid)

Standardized `role_tags` for grid cells:

- `hub_center`, `hub_outer_ring`  
- `corridor_n`, `corridor_s`, `corridor_e`, `corridor_w`  
- `team_a_spawn`, `team_b_spawn`, `team_c_spawn`, `team_d_spawn` (if mode uses more than 2 teams)  
- `airlock_room`, `airlock_control`, `reactor_room`, `med_wing`, `armory`  
- `hazard_hub_floor_gas`, `hazard_sublevel_acid`  
- `alien_vent`, `alien_spawn`, `alien_patrol_route`  

---

## 2. Supported Game Modes

### 2.1 Invasion (Primary Mode)

- PvE‑heavy mode; players (SHC/Tediz/story characters) defend against waves of AI Aliens.  
- Objective variants:
  - **Hold‑out:** Survive X waves while protecting Alien Egg containment.  
  - **Purge:** Escort gas/acid canisters to purge the hub periodically, clearing drones but forcing players to higher paths.

### 2.2 Total War (Variant)

- Classic N64‑style team vs team with an instant‑win gas mechanic adapted from War/Tank.  
- Teams fight to deliver a gas/acid canister to the enemy airlock or reactor, triggering map‑wide hazard.

### 2.3 Deathmatch / Team Deathmatch

- Standard kill‑scoring in the Alien Base layout.  
- Weapons and hazards unchanged; Alien spawns optional.

---

## 3. Player Count, Spawns, and Flow

### 3.1 Player Limits

- **Max players:** 16 (LAN or online).  
- **Teams:** Up to 4 (Red/Blue/Green/Yellow), depending on mode.  
- **Split‑screen:** Up to 4 local players; default to distributing local players across different team zones.

### 3.2 Spawn Zones

Four symmetric “Quarantine Zones” at map extremities:

- Zone A (North): `team_a_spawn` – access to hub via north corridor.  
- Zone B (South): `team_b_spawn` – access via south corridor and sub‑level entrance.  
- Zone C (East): `team_c_spawn` – med wing + sniper catwalk access.  
- Zone D (West): `team_d_spawn` – reactor wing + heavy cover.

Each zone defines 4 spawn nodes, for a total of 16 spawns. Spawn logic must:

- Use **Line‑of‑Sight anti‑camping**: avoid spawning into zones where enemy players have direct view within a defined radius.  
- Prefer spawns that give 1–2 low‑tier pickups before players enter the central hub.

---

## 4. Weapon Pickups & Heavy Carry Rules

### 4.1 Pickup Philosophy

- All players spawn **unarmed or with minimal melee**, mirroring N64 Multi.  
- Weapons exist only as pickups placed in symmetric patterns.  
- Heavy weapons (Bazooka, Chaingun, experimental alien weapons) impose:
  - No jumping.  
  - Slower movement.  
  - Longer weapon ready/holster times.

### 4.2 Central Hub Weapon Layout (Baseline)

Define pickups relative to hub origin (0, 0, 0) to be reused across engines:

- Hub objective (Alien Egg/Queen core) at (0, 0, 0).  
- Short‑range and mid‑range weapons at NE/NW/SE/SW hub quadrants.  
- Long‑range (sniper) on elevated catwalks overlooking hub entrances.  
- One heavy weapon located in a risk‑heavy sub‑level chamber beneath hub.

All placements must be mirrored or otherwise balanced between sides.

---

## 5. Hazards & Environmental Triggers

### 5.1 Airlock Hazard

- **Volumes:** `hazard_hub_floor_gas` and `hazard_sublevel_acid` define world‑space volumes used for damage‑over‑time.  
- **States:** Safe → Warning → Active → Venting (cooldown).  
- **Telegraph:** Sirens, warning lights, and steam vents leading into the activation.

### 5.2 Alien Egg Objective

- Shared objective actor:
  - Health with multiple visual stages (intact, cracked, critical).  
  - On damage/destruction:
    - Option A: triggers airlock hazard.  
    - Option B: triggers Alien assault wave from vents.

---

## 6. Aliens & Invasion Behavior

### 6.1 Alien NPC Types

- Small “facehugger” style, medium drone, and large elite (Heinrich‑like).  
- All share lime‑green gore and black/grey body palette for Uncut authenticity.

### 6.2 AI Behavior

- Key execution states: _Pounce_ and _Facebite_, integrated with ASID registry.  
- Pathfinding that uses:
  - Floors and stairs.  
  - Ceiling‑mounted vents and walls tagged `alien_vent`.  
- Threat priority:
  - Objective carriers.  
  - Isolated players.  
  - Players performing executions.

---

## 7. Movement & Physics Constraints

- Heavy weapons: no jumping; reduced move speed; turning inertia.  
- Executions: hard‑lock ASIDs; victims cannot break out once execution has triggered.  
- Airlock gas/acid:
  - Periodic damage‑over‑time unless in flagged safe zones (sealed rooms, upper walkways).

---

## 8. Implementation Notes

### 8.1 Engine‑Specific

- UE5:
  - `AlienBaseAirlockController` actor for hazard state machine.  
  - `BP_AlienBase_PickupBase` as parent for all pickups.  
- Unity:
  - `AlienBaseAirlockController` MonoBehaviour.  
  - `PickupBase` MonoBehaviour + ScriptableObjects for weapons.  
- Godot:
  - `AlienBaseAirlockController` node script.  
  - `PickupBase` scene with GDScript behavior.

### 8.2 Data Contracts

- Grid file: `maps/alien_base/alien_base_hub_grid_v1.json`.  
- Entities file: `maps/alien_base/alien_base_entities_v1.json`.  
- Hazard config: `data/maps/alien_base/alien_base_hazards_v1.json`.  
- Alien spawn/patrol config: `data/ai/alien_base_invasion_v1.json`.

---

## 9. Testing Checklist

- Hub weapon pickups are reachable and balanced.  
- Airlock hazard:
  - Telegraph timing feels fair.  
  - Safe zones work reliably.  
- Alien AI:
  - Uses vents properly.  
  - Prioritizes objectives and isolated players.  
- 16‑player tests:
  - No persistent spawn‑camp scenarios.  
  - 4‑player split‑screen remains readable.
