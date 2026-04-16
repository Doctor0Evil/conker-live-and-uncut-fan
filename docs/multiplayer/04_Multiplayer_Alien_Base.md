# 04_Multiplayer_Alien_Base

This document defines the core layout, spawn logic, weapon placement, and environmental hazard scripting for the Alien Base multiplayer map as it would exist in the cancelled Conker: Live & Uncut era, following the N64 pickup‑driven design rather than the later class‑based system of Live & Reloaded. [conker.fandom](https://conker.fandom.com/wiki/Conker:_Live_&_Uncut)

## Design Goals

Alien Base is a 16‑player, pickup‑centric arena inspired by the Tanks multi mode in Conker’s Bad Fur Day and the Alien chapter’s space station aesthetic. The goal is to feel like a direct sequel to N64 multiplayer: every player starts equal, traverses a readable hub‑and‑spokes layout, and fights around scripted “Tanks‑style” gas/airlock events rather than modern live‑service progression systems. [youtube](https://www.youtube.com/watch?v=b78rFlrcLfA)

The map is tuned for:

- 16‑player LAN / online matches.  
- Up to 4‑player split‑screen on a single machine.  
- No classes, no XP gain, no loadout unlocks: all combat power comes from map control and pickups. [conker.fandom](https://conker.fandom.com/wiki/Conker:_Live_&_Uncut)

## Coordinate System

All coordinates in this file are in world units relative to a Central Hub origin at \((0, 0, 0)\). The Tanks‑style “central silo with spokes” layout is preserved conceptually, with the Alien Egg / reactor taking the role of the canister bridge. [ign](https://www.ign.com/wikis/conkers-bad-fur-day/Chapter_8:_It's_War)

- X axis: width (East/West).  
- Y axis: height (vertical).  
- Z axis: depth (North/South).  

You can directly map these into Unreal (FVector), Unity (Vector3), or Godot (Vector3) with no rotation changes.

## Central Hub Layout

The Central Hub is a circular, multi‑level chamber housing the primary Alien objective. It is the main combat space and the anchor for both weapon pickups and environmental triggers. [ign](https://www.ign.com/articles/2003/05/13/e3-2003-conker-live-and-uncut)

The hub is structured as:

- Ground Ring: infantry‑scale ring around the Alien Egg with four corridor exits leading to the spawn zones.  
- Catwalk Ring: elevated sniping and support loop with limited cover.  
- Sub‑Level: maintenance tunnels used for heavy weapon stashes and flanking routes.

The Alien Egg or core objective sits directly at the origin.

```text
              Z+
              N
       (-15,0,15)         (15,0,15)
          [Flame]        [Chainsaw]

   W  (-100,5,0)          (100,5,0)  E
Zone D [Spawns]          [Spawns] Zone C

         (-15,0,-15)    (15,0,-15)
            [Shotgun]   [SMG]

              S
              Z-
```

## Weapon Pickup Nodes (Central Hub)

These nodes represent “canonical” placements for N64‑style pickups: every player spawns with a basic weapon and must control these nodes to gain situational advantages. You can swap weapon types while preserving the layout. [conker.fandom](https://conker.fandom.com/wiki/Tank)

### 1. Objective / Alien Egg

- **Name:** `pickup_objective_alien_egg`  
- **Position:** `(0.0, 0.0, 0.0)`  
- **Role:** Visual centerpiece and scripting anchor for Invasion‑type modes. In some variants it may be purely decorative, in others it can be an interactable that drives hazard events or wave spawns. [ign](https://www.ign.com/articles/2003/05/13/e3-2003-conker-live-and-uncut)

### 2. Melee / Close‑Quarters Slot (NE)

- **Name:** `pickup_melee_slot_ne`  
- **Default Weapon:** Chainsaw / Sabre  
- **Position:** `(15.0, 0.0, 15.0)`  
- **Use Case:** Short‑range defense for players holding the Egg or controlling the north‑east corridor. High risk/high reward location within grenade range of the objective.

### 3. Area Denial Slot (NW)

- **Name:** `pickup_area_denial_slot_nw`  
- **Default Weapon:** Flamethrower  
- **Position:** `(-15.0, 0.0, 15.0)`  
- **Use Case:** Crowd control during pushes from Zone A into the hub. Ideal for defending the “north” entrance or denying aliens as they spill out of vents. [conker.fandom](https://conker.fandom.com/wiki/Tank)

### 4. Mid‑Range Slot (SE)

- **Name:** `pickup_midrange_slot_se`  
- **Default Weapon:** SMG / Assault Rifle  
- **Position:** `(15.0, 0.0, -15.0)`  
- **Use Case:** Bread‑and‑butter weapon balanced for strafing around the Egg and contesting the southern entrance corridor.

### 5. Flank Shotgun Slot (SW)

- **Name:** `pickup_flank_slot_sw`  
- **Default Weapon:** Shotgun  
- **Position:** `(-15.0, 0.0, -15.0)`  
- **Use Case:** Rewards players who loop around the hub and attempt close‑in ambushes from the rear corridors.

### 6. Elevated Sniper Nest

- **Name:** `pickup_sniper_nest`  
- **Default Weapon:** Sniper Rifle  
- **Position:** `(0.0, 25.0, -30.0)`  
- **Access:** Narrow stair or lift route from southern corridor; requires exposing yourself to hub fire to reach.  
- **Use Case:** Long‑range coverage of the hub floor and partial sightlines into Zone B approach, mirroring the sniper perches seen in N64’s War/Tanks setups. [ign](https://www.ign.com/wikis/conkers-bad-fur-day/Chapter_8:_It's_War)

### 7. Sub‑Level Heavy Weapon

- **Name:** `pickup_heavy_bazooka_sublevel`  
- **Default Weapon:** Bazooka (or equivalent heavy launcher)  
- **Position:** `(0.0, -10.0, 40.0)`  
- **Access:** Ladder/elevator shaft from the north corridor, with tight corners to prevent instant firing into the hub.  
- **Use Case:** Power position that comes with mobility penalties (no jumping while carried) to keep it in line with original Bad Fur Day heavy weapons. [conker.fandom](https://conker.fandom.com/wiki/Tank)

### Symmetry Rules for New Weapons

When you introduce new Alien‑flavored or tech weapons (Plasma Rifle, Bio‑Spitter, etc.), follow these alignment rules to preserve original balance philosophy:

- **Mirrored Power:** Any high‑tier weapon placed at `(15, 0, 15)` must have a counterpart of comparable power at `(-15, 0, 15)` or one of the other quadrants.  
- **Weight Penalty:** Heavy weapons must re‑enable the classic “no jump while equipped” limitation and slower turning speed. [conker.fandom](https://conker.fandom.com/wiki/Tank)
- **Hazard Buffer:** Keep all pickup nodes at least `5.0` units away from hazard volumes and airlock vents, so players are not wiped mid‑pickup by scripted events.

## Spawn Layout for 16 Players

Spawn points are grouped into four “Quarantine Zones,” reflecting the bunker layout of Tanks and providing safe staging for squads. Each zone hosts up to four spawns. [conker.fandom](https://conker.fandom.com/wiki/Tank)

### Zone A – North Quarantine

- **Zone Label:** `spawn_zone_a_north`  
- **Approx Center:** `(0.0, 0.0, 100.0)`  
- **Purpose:** Direct route into the hub and Alien Egg chamber.

Example per‑player nodes (you can jitter by ±2 units per engine):

- `spawn_a1`: `( -4.0, 0.0,  96.0 )`  
- `spawn_a2`: `(  4.0, 0.0,  96.0 )`  
- `spawn_a3`: `( -4.0, 0.0, 104.0 )`  
- `spawn_a4`: `(  4.0, 0.0, 104.0 )`  

### Zone B – South Hangar

- **Zone Label:** `spawn_zone_b_south`  
- **Approx Center:** `(0.0, 0.0, -100.0)`  
- **Purpose:** Access to sub‑level heavy weapon routes and rear flank paths.

Nodes:

- `spawn_b1`: `( -4.0, 0.0, -96.0 )`  
- `spawn_b2`: `(  4.0, 0.0, -96.0 )`  
- `spawn_b3`: `( -4.0, 0.0,-104.0 )`  
- `spawn_b4`: `(  4.0, 0.0,-104.0 )`  

### Zone C – East Medical Wing

- **Zone Label:** `spawn_zone_c_east`  
- **Approx Center:** `(100.0, 5.0, 0.0)`  
- **Purpose:** Slightly elevated approach with sniper vantage and longer route to hub, suitable for more cautious squads.

Nodes:

- `spawn_c1`: `( 96.0, 5.0, -4.0 )`  
- `spawn_c2`: `(104.0, 5.0, -4.0 )`  
- `spawn_c3`: `( 96.0, 5.0,  4.0 )`  
- `spawn_c4`: `(104.0, 5.0,  4.0 )`  

### Zone D – West Reactor Core

- **Zone Label:** `spawn_zone_d_west`  
- **Approx Center:** `(-100.0, 5.0, 0.0)`  
- **Purpose:** Mirror of Zone C with heavier cover and shorter distance to some hazard controls.

Nodes:

- `spawn_d1`: `(-96.0, 5.0, -4.0 )`  
- `spawn_d2`: `(-104.0,5.0, -4.0 )`  
- `spawn_d3`: `(-96.0, 5.0,  4.0 )`  
- `spawn_d4`: `(-104.0,5.0,  4.0 )`  

### Spawn Selection Logic

Spawn logic should mimic a refined N64 philosophy rather than the turret‑guarded class spawns seen in Live & Reloaded: [conker.fandom](https://conker.fandom.com/wiki/Conker:_Live_&_Uncut)

- **Line‑of‑Sight Checks:** A node is invalid if an enemy is within a configurable radius (e.g. 20 units) and has clear line of sight. The system rotates to the next node in the zone or fails over to a different zone if all are compromised.  
- **Zone Cycling:** In 4‑player split‑screen, bias initial spawns so each local player starts in a different zone, minimizing screen‑peeking and providing diverse routes.  
- **No Fixed Classes:** Every spawn uses generic SHC/Tediz‑style soldiers or story characters, with no stat differences at spawn time.

## Environmental Hazard Triggers (Airlock / Gas)

The Alien Base inherits the “map‑wide hazard” concept from the Tanks canister mechanic, but reframed as airlock cycles, acid leaks, or containment breaches rather than chemical gas flags. [ign](https://www.ign.com/wikis/conkers-bad-fur-day/Chapter_8:_It's_War)

### Core Hazard Concept

- A discrete player action (button, carried item, or Alien interaction) triggers a timed sequence.  
- After a warning grace period, low sections of the hub become lethal or heavily damaging, forcing players to fight on catwalks and in elevated corridors.  
- Hazard events are rare but decisive, intended as comeback tools or objective pivots rather than constant spam.

### Hazard Volumes

Define at least two main hazard regions in the hub:

1. `hazard_hub_floor_gas`  
   - **Bounds:** Roughly radius 25–30 units around `(0, 0, 0)` from `Y = -2.0` to `Y = 4.0`.  
   - **Effect:** On activation, applies continuous damage over time to players and AI on the floor ring, sparing catwalk/corridor heights.

2. `hazard_sublevel_acid`  
   - **Bounds:** Tunnels around the Bazooka sub‑level `(0, -10, 40)`, radius 15 units, `Y = -12.0` to `Y = -6.0`.  
   - **Effect:** Faster damage but shorter duration, discouraging camping on the heavy weapon.

Exact radius and heights can be tuned in‑engine, but the intent is “floor bad, high ground safe” during events, exactly like hiding in bunkers during Tanks gas releases. [conker.fandom](https://conker.fandom.com/wiki/Tank)

### Trigger Locations

To keep things readable and symmetric:

- `trigger_airlock_north`: `(0.0, 0.0, 60.0)` – console near Zone A corridor entrance.  
- `trigger_airlock_south`: `(0.0, 0.0, -60.0)` – console near Zone B corridor entrance.  
- (Optional) `trigger_emergency_purge_east`: `(60.0, 5.0, 0.0)` – controls acid in sub‑level tunnels.  
- (Optional) `trigger_emergency_purge_west`: `(-60.0, 5.0, 0.0)` – mirrored control.

You can bind these to Conker‑style B‑button pads or lever meshes.

### Hazard Script Timeline (Pseudo‑Logic)

A typical Airlock event might follow this pattern:

1. **Activation:** Player interacts with a trigger console.  
2. **Warning (3–5 seconds):**  
   - Sirens, flashing lights, and VO callouts.  
   - Floor decals and vents start steaming to telegraph safe vs unsafe zones.  
3. **Active Phase (10–15 seconds):**  
   - `hazard_hub_floor_gas` and `hazard_sublevel_acid` enable.  
   - Any player or alien in these volumes takes periodic damage; catwalks and upper corridors are unaffected.  
4. **Cooldown (20–40 seconds):**  
   - Hazards disable.  
   - Trigger consoles are locked out until the cooldown expires.

You can implement this via timer‑driven state machines or blueprint/scene scripts per engine.

### Integration with Invasion Mode

For an Alien “horde” variant, hazard events can be tied to Alien behavior: [gamespot](https://www.gamespot.com/articles/conker-live-and-uncut-e3-2003-preshow-report/1100-6027554/)

- **Alien Overrun:** If aliens control the hub for a set time (e.g., 30 seconds), they automatically trigger an infestation purge: floor becomes hazardous to both teams, forcing defenders to higher ground.  
- **Egg Phase:** Damaging or interacting with the Alien Egg can cause short, localized acid spills, using mini hazard volumes around `(0, 0, 0)`.

## Visual and Material Notes

To keep the Live & Uncut feel while taking advantage of modern engines, base your materials on upscaled N64 assets and bio‑industrial themes: [conker.fandom](https://conker.fandom.com/wiki/Tank)

- **Ground / Mud:** Wet clay or dark loam with high roughness, echoing the Tanks map floor but darker and more saturated for alien contrast.  
- **Walls / Structure:** Metallic greys with occasional hazard stripes, overgrown by black, glossy “xenomorph resin” veins near the Egg and Alien vents.  
- **Alien Gore:** Lime‑green blood and mucus against black alien bodies, staying consistent with the Bad Fur Day finale and OG alien palette rather than the later teal/tiger stripe variants. [youtube](https://www.youtube.com/watch?v=b78rFlrcLfA)

These notes can be turned into engine‑specific material graphs or C++ PCG systems, but the .md file only needs to describe intent and reference names for your asset pipeline.

## Future Work

- Define per‑mode variations (Deathmatch, Invasion, Objective) that reuse this layout but tweak hazard cooldowns and objective scripting.  
- Add a table of engine‑specific anchor IDs (Unreal Actor names, Unity prefabs, Godot nodes) mapped to each pickup, spawn, and hazard trigger for automated scene generation.  
- Integrate character roster and skin notes (Berri, Gregg, Rodent, aliens) once you finalize how closely you mirror the single‑player Alien chapter designs. [youtube](https://www.youtube.com/watch?v=b78rFlrcLfA)
