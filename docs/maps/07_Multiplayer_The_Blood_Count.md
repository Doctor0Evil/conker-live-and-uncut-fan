# 07_Multiplayer_The_Blood_Count

## High-Level Overview

**Internal ID:** 07  
**Canonical Name:** The Blood Count  
**Location:** Count Batula's Mansion and surrounding grounds (Spooky chapter)  
**Primary Modes:** 
- Blood Hunt (Objective – Zombies)
- Team Deathmatch – Spooky (Zombies On)
- Deathmatch – Spooky (Zombies On)

**Player Count Target:** 
- 2–4 teams (Red / Blue / Green / Yellow)  
- Up to 16 players total (4 per team)  

**Core Fantasy:**
Teams of greedy, undead‑adjacent misfits fight through Count Batula’s zombie‑infested mansion to seize the Panther King’s blood‑vial and perform a ritual at their team altar while a relentless Fire Imp hunts the carrier. Zombies are everywhere. Headshots and close‑range shotgun blasts are the only way to keep them down.

---

## Canonical References and Pillars

- **Source Chapter:** Spooky (Conker’s Bad Fur Day) – Count Batula’s Mansion and its surrounding graveyard.[^ref_spooky]  
- **Key Elements Preserved:**
  - Zombie headshots as the core lethality rule.
  - Grinder in the dining hall as a lethal environmental hazard.
  - Mansion wings (library, basement, entrance, exterior) as primary routes.
- **Spiritual Ancestors:**
  - Heist (The Vault) – four‑team, central objective loop.
  - Raptor and Tank – asymmetrical risk/reward routes and instant‑win style objectives.

> **Design Goal:** This map must feel like a direct multiplayer sequel to Spooky, not a generic horror arena.

---

## Spatial Layout and Zones

### 1. Global Geometry Summary

The Blood Count uses a hub‑and‑wings layout centered on the **Dining Hall (Grinder Room)**. All interior wings and the back‑yard maze connect through this central hub. Verticality comes from balconies, staircases, and library mezzanines overlooking the hub and corridors.

- **Central Hub:** Dining Hall with Grinder pit.  
- **Primary Wings (Team Spawns):**
  - North Wing – Library and Study
  - South Wing – Crypts / Basement Access
  - East Wing – Entrance Hall & Front Graveyard
  - West Wing – Kitchen & Service Corridors
- **Exterior Zone:** Back‑yard hedge/library maze (high zombie density, Hunting Revolver hotspot).

### 2. Zone Breakdown

#### 2.1 Dining Hall – Grinder Hub

- Large rectangular room with central Grinder pit (instant‑death hazard).  
- Balconies on at least two sides provide elevated sightlines into hub.  
- Multiple doorways to each wing ensure flow and flanking options.

**Gameplay Roles:**
- Central convergence point for Blood Hunt routes.
- Risky shortcut between spawn wings.
- Potential shotgun and ammo pickups near balconies and pillars.

#### 2.2 Library Wing (North)

- Multi‑level library stacks, narrow aisles, ladders/ramps to upper mezzanine.  
- Connects directly to the back‑yard maze through broken wall/doorway.

**Gameplay Roles:**
- Primary **Hunting Revolver** spawn zone (highest zombie density).  
- Long sightlines for precision weapons.  
- Stronghold for teams willing to brave heavy zombie presence.

#### 2.3 Basement / Crypts (South)

- Tight corridors, low ceilings, occasional open crypt chambers.  
- Some dead‑end rooms for ambushes and zombie spawn clusters.

**Gameplay Roles:**
- Close‑quarters shotgun territory.  
- Natural zombie emergence zone.  
- Offers flanking routes to the hub beneath main corridors.

#### 2.4 Entrance Hall & Front Graveyard (East)

- Entrance foyer with grand staircase and double doors leading outside.  
- Front graveyard area with tombstones and sparse cover.

**Gameplay Roles:**
- Mid‑range engagement area.  
- Safer space for newly spawned players to orient and grab basic weapons.  
- Connects to other wings via interior corridors and exterior wrap‑around paths.

#### 2.5 Kitchen & Service Wing (West)

- Multi‑room kitchen, pantry, and servant corridors, with back‑of‑house access to the Dining Hall.  
- Narrow, looping routes that favor shotgun and flamethrower.

**Gameplay Roles:**
- Flanking routes into the hub that avoid main sightlines.  
- Good place for mid‑tier weapon pickups (SMG / shotgun, light ammo).

#### 2.6 Back‑Yard Maze / Garden

- Hedge‑style or bookshelf‑style maze behind the mansion, attached to Library Wing.  
- Limited visibility, heavy zombie spawns, ambient fog.

**Gameplay Roles:**
- **Most dangerous area on the map; primary Hunting Revolver spawn location.**  
- Ideal ground for zombie swarms and Fire Imp ambushes.  
- High‑risk, high‑reward traversal route between wings.

---

## Teams, Spawns, and Flow

### 1. Team Structure

- Up to **4 teams**: Red / Blue / Green / Yellow.  
- Each team has a home “wing” associated with a mansion quadrant.

Suggested mapping:

- **Team Red:** Library Wing (North)  
- **Team Blue:** Crypt/Basement Wing (South)  
- **Team Green:** Entrance/Graveyard Wing (East)  
- **Team Yellow:** Kitchen/Service Wing (West)

### 2. Spawn Bands and Safe Rooms

Each team begins in a **safe spawn room** at the far end of its wing:

- One main spawn room per team with 3–4 spawn nodes.  
- Room exits into a short “buffer corridor” with low zombie presence and basic weapon pickups (e.g., crossbow or low‑ammo shotgun).  
- The buffer corridor leads to a wing “midfield” that connects to the Dining Hall hub.

**Spawn Rules:**

- No player should spawn within direct line‑of‑sight of an enemy player.  
- If all spawn nodes in a team’s wing are compromised (enemy presence within line‑of‑sight or range threshold), the spawn manager rotates to the next safest node cluster in that wing.  
- DM/TDM variants reuse the same spawn bands but may allocate teams differently.

---

## Game Modes

### 1. Blood Hunt (Objective – Zombies)

Core mode for The Blood Count. Zombies are always active. Four teams compete to capture the Panther King’s blood‑vial and score at their ritual altar.

#### 1.1 Win and Score Conditions

- **Primary Objective:** 
  - Pick up the **Blood Vial** and transport it to your team’s **Ritual Altar** in your home wing.
- **Scoring:**
  - Each successful vial turn‑in scores 1 point for that team.
  - Default match: first team to N points (tunable; e.g., 3) or highest score at time limit.
- **Sudden Death:** 
  - Optional rule where the last available vial (after a global timer) is worth extra points or triggers End‑Game panic (increased zombies).

#### 1.2 Blood Vial Behavior

- **Single instance** of the vial active at any time.  
- **Spawn logic:**
  - Vial spawn locations rotate among pre‑defined points across the map (e.g., Library maze, Dining Hall hub, Graveyard, Crypt chamber).  
  - Each spawn has a **decay timer**; if not picked up within time, vial despawns and reappears at the next location.
- **Heavy Carry Rules (Carrier):**
  - Cannot sprint/run.  
  - Cannot jump.  
  - Cannot use weapons or throw grenades.  
  - Movement speed reduced to “encumbered walk” profile.
- **Visibility:**
  - Vial carrier is highlighted on HUD/minimap with reduced precision (e.g., approximate direction but not exact position) to preserve tension.

#### 1.3 Ritual Altars

- Each team has a unique altar in its home wing (e.g., a blood‑stained shrine, ritual circle, or makeshift sacrificial slab).  
- Vial turn‑in requires the carrier to reach altar and channel briefly (short interaction time) while completely vulnerable.

---

### 2. Team Deathmatch – Spooky (Zombies On)

- **Objective:** Team‑based kill count; zombies remain active as environmental threats.  
- **Vial and Fire Imp:** Disabled in this variant.  
- **Scoring:** Kills against enemy players; zombie kills may or may not contribute (tunable).

### 3. Deathmatch – Spooky (Zombies On)

- **Objective:** Free‑for‑all kill count; same geometry and zombie behavior.  
- **Vial and Fire Imp:** Disabled.  
- **Spawn:** Spread across wings and certain central positions to minimize spawn‑camping.

---

## Zombies – Behavior, Spawning, and Lethality

### 1. Core Behavior

Zombies follow a simple but brutal rule set, modeled on Spooky:

- **Movement:** Slow shamble toward nearest living player in line‑of‑sight; slightly faster when they detect the blood‑vial carrier.  
- **States:** Idle, Shamble, Attack, Crawl, Stunned (optional).  
- **Attack:** Melee swipe that deals moderate damage and can stagger lightly armored characters.

### 2. Lethality Rules

- **Only true death from:**
  - **Headshots** from projectile weapons.  
  - **Close‑range shotgun blasts** that encompass the head/upper torso in their lethal cone.
- **Non‑lethal damage:**
  - Body shots from any weapon do not kill.  
  - Excessive limb/body damage pushes zombies into a **Crawl State**:
    - Slower movement.
    - Lower attack range.
    - Still lethal if ignored.
  - Crawlers still require headshots or lethal shotgun cone aimed at upper torso/head to die.

### 3. Flamethrower Interactions

- Flamethrower:
  - **Does not kill zombies.**  
  - Optional: briefly slows their movement or creates short “panic” animation, but does not change their health state permanently.
- Design Note:
  - Emphasizes flamethrower as crowd‑control / area denial rather than hard counter.

### 4. Zombie Spawning and Density

- **Spawn Zones:** 
  - Back‑yard maze, basement corridors, graveyard, and select hallways.  
- **Density Target:** 
  - Always maintain a minimum number of zombies per zone (tunable; e.g., 10–20 actors globally at lower player counts, scaling up to a higher cap for full 16‑player lobbies).
- **Spawn Logic:** 
  - Respawn zombies outside immediate player view to avoid popping.  
  - Adjust spawn rates based on:
    - Number of active players in the zone.  
    - Current score (trailing teams may see fewer zombies in their path).

---

## Fire Imp – Behavior and Rules

### 1. Activation Conditions

- Fire Imp is **not present** on the map until a player picks up the Blood Vial.  
- Once the vial is held:
  - Fire Imp spawns at a designated “hell portal” or fireplace location.  
  - Immediately acquires the vial carrier as primary target.

### 2. Targeting and Movement

- **Primary Target:** Blood‑vial carrier at all times.  
- **Secondary Targets:** Only attacks other players if they significantly obstruct its path or inflict damage.  
- **Movement Profile:**
  - Fast, erratic ground movement with small hops.  
  - Can navigate tight spaces and jump minor gaps; cannot fly.

### 3. Damage and Vulnerabilities

- **Fire Imp can only be killed by:**
  - **Hunting Revolver:** Any range – a direct hit kills or inflicts very high damage.  
  - **Shotgun:** Only at close range (within defined lethal radius), requiring accurate blasts.
- **Immune to:**
  - Flamethrower (no damage, no stun/slow).  
  - Light weapons (e.g., SMG, pistols) – optional minor flinch but no real damage.
- **On Death:**
  - Fire Imp enters a death animation and despawns.  
  - Starts a **respawn cooldown** (e.g., 15 seconds) during which the carrier has “breathing room”.  
  - Triggers a small-radius fire blast on death that **cripples nearby zombies** and **sets players on fire** (damage-over-time burn effect).

### 4. Respawn Behavior

- After the cooldown:
  - Fire Imp respawns in a location that favors interception of the current vial carrier (e.g., on their likely route to the altar, not directly in front of them).
- If the vial is dropped or returned:
  - Fire Imp despawns or becomes dormant until next vial pickup.

---

## Weapons and Pickups

### 1. Weapon Set

- **Available Weapons:**
  - Crossbow (precision, slower rate of fire).  
  - Shotgun (close‑range power, key vs zombies).  
  - Flamethrower (crowd control, anti‑player but weak vs zombies & Fire Imp).  
  - Hunting Revolver (high damage, key vs Fire Imp and distant threats).
- **Optional Additions:**  
  - Light SMG or pistol for basic self‑defense.

### 2. Placement Philosophy

- **Crossbows:** 
  - Near balconies, library upper levels, and graveyard overlook points.  
- **Shotguns:** 
  - Near stair bases, crypt entrances, and kitchen corridors (tight quarters).  
- **Flamethrowers:** 
  - Along chokepoints where players confront zombie clusters (hallway pinch points, maze choke points).  
- **Hunting Revolver:**
  - Spawns only in the **back‑yard library/hedge maze** (most dangerous zone with highest zombie density).  
  - Potential secondary rare spawn in a risky crypt chamber, if needed for balance.

### 3. Ammo and Health

- Ammo and health pickups are scattered near but not on altars and vial spawn points, to prevent camping.  
- Zombie‑dense areas may have slightly better ammo density as reward.

---

## Technical Constraints and Tuning

### 1. Player Count and Split-Screen

- Map designed to scale **cleanly to 16 players**, with:
  - Clearly separated team spawn wings.  
  - Multiple traversal routes to avoid choke congestion.
- Must remain readable in **4‑player split‑screen**:
  - Strong silhouettes and lighting contrast in key combat spaces.  
  - Avoid excessive narrow corridors that cause screen chaos.

### 2. Performance Targets

- Zombies and Fire Imp count toward AI budget; tuning targets:
  - Maintain stable framerate with up to 16 players + dozens of zombies + Fire Imp.  
  - LOD and pooling for zombies, gore decals, and particle effects.

### 3. Exposed Variables for Playtesting

List of config parameters to be driven via external config (for rapid tuning):

- Zombie global cap and per‑zone caps.  
- Zombie respawn delay.  
- Blood Vial decay timer per spawn location.  
- Fire Imp respawn interval.  
- Points per vial delivery, kill values, and DM/TDM scoring thresholds.  
- Heavy carry movement speed, and line‑of‑sight spawn radius.

---

## Playtest Checklist

A non‑exhaustive checklist to verify core behaviors:

- [ ] Zombies only die from headshots or close‑range shotgun lethal cone.  
- [ ] Zombies reliably enter crawl state after sufficient limb/body damage.  
- [ ] Flamethrower does not kill zombies or Fire Imp.  
- [ ] Fire Imp only spawns once the Blood Vial is picked up.  
- [ ] Fire Imp can only be killed by Hunting Revolver (any range) and close‑range shotgun blasts.  
- [ ] Fire Imp respects the respawn cooldown after death (e.g., ~15 seconds).  
- [ ] Blood Vial carrier cannot run, jump, or fire weapons.  
- [ ] Vial spawn/decay cycle correctly rotates among all intended locations.  
- [ ] Each team’s Ritual Altar correctly accepts vial turn‑ins and scores points.  
- [ ] No spawn‑camping: spawn manager avoids placing players in direct enemy line‑of‑sight.  
- [ ] TDM and DM variants function correctly with zombies enabled and vial/Fire Imp disabled.

---

[^ref_spooky]: See Conker’s Bad Fur Day – Chapter 7: Spooky and Count Batula’s Mansion coverage for canonical layout and zombie behavior details.  
