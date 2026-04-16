# 07_Multiplayer_The_Blood_Count

## 0. Overview

**Internal ID:** 07  
**Working Title:** The Blood Count  
**Source Chapter:** Spooky – Count Batula’s Mansion and surrounding grounds.  
**Area Name:** Count Batula’s Mansion (interior) + graveyard and hedge maze (exterior).

**Design Pillars**

- 4‑team TDM/DM arena with an optional objective layer.  
- Full access to mansion interior and exterior back‑yard/hedge maze.  
- High‑density zombies across the entire playspace.  
- Players start unarmed; all combat power is from pickups.  
- One roaming Fire Imp that only exists while the blood‑vial is held.

---

## 1. Map Layout & Zones

### 1.1 High‑Level Zones

- **Central Hall:** Main entrance hall; key crossroads for interior movement and vertical play.  
- **Dining Room (Grinder Hall):** Large interior room with balcony; optional environmental hazard in variants.  
- **Library:** Multilevel room with balconies and bookcases; core “high‑risk/high‑reward” zone.  
- **Graveyard:** Front/side exterior with tombstones and initial zombie waves.  
- **Hedge Maze:** Back‑yard maze; center is a high‑density zombie cluster and one of the key blood‑vial / Hunting Revolver spawn regions.  
- **Basement / Dungeon:** Lower level with tight corridors; good for ambushes and choke points.  
- **Four Team Wings:** Each corner of the castle interior provides a team spawn wing (Red/Blue/Green/Yellow).

### 1.2 Role Tags (Grid)

Example `role_tags` for grid cells:

- `central_hall`, `dining_room`, `library`, `graveyard`, `hedge_maze`, `dungeon`  
- `team_red_spawn`, `team_blue_spawn`, `team_green_spawn`, `team_yellow_spawn`  
- `blood_vial_spawn`, `hunting_revolver_spawn`  
- `zombie_spawn_low`, `zombie_spawn_medium`, `zombie_spawn_high`  
- `fire_imp_path`  

---

## 2. Supported Game Modes

### 2.1 Game Mode: Zombies (Objective)

Primary objective‑based mode with TDM overlay.

- Up to 4 teams (Red/Blue/Green/Yellow), 4 players per team (16‑player max).  
- Zombies populate the entire map at high density.  
- A single **Panther King’s Blood‑Vial** pickup exists at any given time:
  - Spawns at one of multiple potential `blood_vial_spawn` locations.  
  - After a decay timer expires, despawns and respawns at a different location.  
- A lone **Fire Imp** exists, but only while a player is carrying the blood‑vial.

Scoring model (suggested):

- Deliver blood‑vial to your team’s ritual altar in your wing for major score.  
- TDM scoring also active (kills vs deaths), but secondary to objective.

### 2.2 Zombies TDM (No Objective)

- Standard Team Deathmatch in the same map.  
- Zombies still spawn with high density.  
- No blood‑vial or Fire Imp; all points come from PvP plus optional PvE bonus scoring.

### 2.3 Zombies DM (Free‑for‑All)

- Free‑for‑all variant in same geometry.  
- Optional toggle for blood‑vial objective; if enabled, any player can score by delivering the vial to neutral altars.

---

## 3. Player Count, Spawns, and Flow

### 3.1 Player Limits

- 16 players maximum (LAN/online).  
- 4 teams, 4 players each for the primary objective mode.  
- 4‑player split‑screen supported.

### 3.2 Team Spawns

Team wings are anchored to the four corners of the mansion’s interior:

- Team Red: North‑West wing.  
- Team Blue: North‑East wing.  
- Team Green: South‑East wing.  
- Team Yellow: South‑West wing.

Each wing:

- Contains 4 spawn points.  
- Has a “safe” first room with minimal zombie density and low‑tier pickups (e.g., crossbow or basic shotgun).  
- Opens into the central hall or adjacent corridors, giving players a chance to arm up before entering congested zones.

Spawn system must:

- Avoid spawning players directly into zombie clusters.  
- Rotate spawn nodes when enemies (players or zombies) are too close or have direct LOS.

---

## 4. Blood‑Vial Objective

### 4.1 Core Rules

- Only **one** blood‑vial exists at any time.  
- Spawns at `blood_vial_spawn` nodes across:
  - Library (most dangerous), hedge maze center, central hall altar, selected dungeon alcove.  
- Has a **decay timer**: when timer expires, vial disappears and respawns at a new location.  
- Carrier restrictions:
  - Cannot run, cannot jump, cannot use weapons (Heavy Carry state).  
  - Moves slower than normal and is highly vulnerable.

### 4.2 Fire Imp Behavior

- Fire Imp is absent while no one holds the blood‑vial.  
- When a player picks up the blood‑vial:
  - Fire Imp spawns along a `fire_imp_path` near the vial carrier’s general area.  
  - Immediately begins pathfinding toward the carrier.  

Kill rules:

- Only the **Hunting Revolver** (any range) or **Shotgun** (close‑range lethal blast) can kill the Fire Imp.  
- Flamethrower does **not** damage, stun, stagger, or slow the Fire Imp.  
- On death, Fire Imp has a respawn cooldown (e.g., 15 seconds) to allow a “breathing room” window for the carrier.  
- After cooldown, Fire Imp respawns if the vial is still being carried.

---

## 5. Zombies: Behavior and Density

### 5.1 Core Zombie Rules

- Zombies are present throughout all major zones:
  - Graveyard and hedge maze: highest density.  
  - Library and dungeon: high density.  
  - Central hall and dining room: medium density.  
  - Team wings: low density to protect spawn stability.

Damage and death:

- Only **headshots** or **shotgun lethal‑zone blasts** (head/upper torso at close range) kill zombies.  
- Flamethrower does not kill zombies; it can optionally apply a brief stagger or purely cosmetic burning, but no lethal damage.  

Crawl state:

- When zombies take heavy body/limb damage without qualifying lethal hits:
  - Transition into a slow “crawl” state.  
  - Crawl zombies still require headshots or close shotgun blasts to die.  
- Crawl state adds visual brutality and changes pathing (lower profile).

### 5.2 Spawn Manager

- Maintain target zombie counts per zone:
  - E.g., hedge maze target higher than library; library higher than central hall.  
- Activate zombie spawns based on player proximity and line‑of‑sight:  
  - Prefer spawning out of view in adjacent corridors, graveyard edges, or maze turns.  
- Despawn or reduce zombie count in team wings to prevent immediate spawn deaths.

---

## 6. Weapon Pickups and Placement

### 6.1 Weapon Set

- Crossbow.  
- Shotgun.  
- Flamethrower.  
- Hunting Revolver (high‑power, rare).

### 6.2 Placement Logic

- All players start unarmed (or with only a weak melee hit).  
- Early pickups:
  - Crossbows and shotguns placed near exits from team wings into the central hall or graveyard.  
- Flamethrowers:
  - Mid‑risk zones such as central hall balconies or dining room side corridors.  
- Hunting Revolver:
  - Only spawns at **central and highly dangerous** locations:
    - Library center (especially in the maze‑like shops of shelves).  
    - Hedge maze center surrounded by high zombie density.  

Balancing constraints:

- At least two potential Hunting Revolver spawn points to prevent predictable camping.  
- Revolver ammo scarce – designed as a Fire Imp/Zombie elite killer rather than a general‑purpose weapon.  

---

## 7. Movement, Physics, and States

### 7.1 Heavy Carry (Blood‑Vial)

- When carrying the blood‑vial, player enters Heavy Carry state:
  - Walk speed significantly reduced.  
  - Jumping disabled.  
  - Weapon use disabled.  
- State must integrate with ASID registry as a specific ID so executions and damage rules are consistent across maps.

### 7.2 General Movement

- Standard movement for non‑heavy states; optional tweaks to keep the “slow, deliberate” N64 feel:
  - Slight inertia on direction changes.  
  - Emphasis on precise platforming on library balconies and graveyard obstacles.

---

## 8. Implementation Notes

### 8.1 Engine‑Specific Hooks

- **UE5:**
  - `BP_BloodVialObjective` actor for vial logic.  
  - `BP_FireImpAIController` + pawn for Fire Imp.  
  - `BP_ZombieBase` with state machine (Idle/Patrol/Chase/Attack/Crawl).  
- **Unity:**
  - `BloodVialObjective` MonoBehaviour with state transitions (idle/spawned/held/decayed).  
  - `FireImpAI` script using NavMesh.  
  - `ZombieController` with state enum and hitbox tagging for head vs body.  
- **Godot:**
  - `BloodVialObjective.gd`, `FireImp.gd`, `Zombie.gd` as nodes with FSMs and navigation agent integration.

### 8.2 Data Contracts

- Grid file: `maps/the_blood_count/the_blood_count_grid_v1.json`.  
- Entities file: `maps/the_blood_count/the_blood_count_entities_v1.json`.  
- Zombie spawn config: `data/ai/the_blood_count_zombies_v1.json`.  
- Fire Imp config: `data/ai/the_blood_count_fire_imp_v1.json`.  
- Objective config: `data/maps/the_blood_count_objectives_v1.json`.

---

## 9. Testing Checklist

- Four team spawns:
  - No spawn kills from zombies or Fire Imp.  
- Blood‑vial:
  - Correctly rotates spawn locations on decay.  
  - Heavy Carry state always applied and cleared correctly.  
- Fire Imp:
  - Does not spawn until a player picks up vial.  
  - Only Hunting Revolver and close‑range shotgun hits can kill it; flamethrower has no effect.  
  - Respawn timer grants a meaningful “breathing room” to vial carrier.  
- Zombies:
  - Only die from headshots / lethal shotgun blasts.  
  - Crawl behavior triggers from non‑lethal body damage.  
- Performance:
  - High zombie density with 16 players and 4‑player split‑screen remains within budget on target platforms.
