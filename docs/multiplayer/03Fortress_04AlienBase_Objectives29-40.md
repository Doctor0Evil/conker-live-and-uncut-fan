# 03 Fortress & 04 Alien Base – Objectives, Hazards, and Shared Systems (Items 29–40)

This document consolidates items 29–40 from the multiplayer roadmap into a single, engine‑agnostic spec for Fortress and Alien Base objectives and hazards. It is written so grid2scene, mode controllers, and future maps can all rely on the same data‑driven patterns.

---

## 1. Fortress – Gas Canister Objective and Capture Backbone (29, 32, 34)

### 1.1 Design Role of the Gas Canister

In the 16‑player Fortress layout, the gas canister is a **power objective** layered on top of a stable three‑point capture system, not a hard instant‑win. Delivering the canister to the enemy base arms a short warning, then triggers a strong but time‑limited gas wave that clears defenders, spikes ticket gain, and then goes on cooldown.

Baseline tuning:

- Heavy Carry penalty: approximately 50% movement speed while holding the canister.
- Arming warning: 5 seconds of siren, VO bark, and base‑localized telegraph.
- Gas effect: 15 damage per second for 20 seconds to all non‑immune players in the affected base.
- Reward: +50 tickets to the attacking team when the gas arms successfully.
- Canister respawn: 60 seconds after the previous gas wave concludes, or after a failed arming window.

These values are starting points; actual numbers are controlled by data profiles so they can be tuned without code changes.

### 1.2 Capture Logic and Ticket Flow

Fortress uses a three‑zone hybrid domination / tug‑of‑war system that provides a predictable scoring backbone:

- Capture Zone A: SHC forward trench, just outside the SHC base.
- Capture Zone B: central bridge span over the valley.
- Capture Zone C: Tediz fortress courtyard in front of the main bunker.

Each zone is an Objective entity with `rtype: "CaptureZone"` in `maps/fortress/fortress_entities_v1.json`, tagged with roletags `capture_a`, `capture_b`, `capture_c`, and additional flavor tags such as `trench`, `bridgecenter`, or `courtyard`.

Ticket accrual:

- Base rate scales with the number of zones owned (0–3).
- A dominance bonus applies when a team holds a majority (2 or 3 points).
- Tuning is targeted so that, with even play and no gas events, a team reaches 100 tickets in roughly 5–7 minutes.

Interaction between gas and captures:

- Successfully arming gas at an enemy base strongly increases the chance to flip Capture Zone A or C (depending on which side is attacked) by clearing the home base and disrupting defender rotations.
- Gas does not reset capture progress; it simply removes defenders, giving attackers a high‑leverage push window.

### 1.3 Gas Canister State Machine (AGasCanister)

The canister is implemented as an engine‑agnostic state machine, mirrored in:

- Unreal: `AGasCanister` actor in `src/gameplay/fortress/`.
- Unity: `FortressGasCanister` component/MonoBehaviour.
- Godot: `FortressGasCanister.gd` node script.

Shared states:

1. **Waiting**
   - Canister is at its neutral spawn, interactable.
   - Not currently held; marker visible on HUD and minimap.

2. **Carried**
   - A player has picked up the canister.
   - Heavy Carry penalty applies (ASID_050): reduced movement speed, no jumping, no weapon use.
   - Canister world position is driven by the carrier’s root bone.

3. **Arming**
   - Triggered when a carrier enters a valid Gas Inlet objective volume at the enemy base.
   - Player must remain inside the inlet for the full arming duration (5 seconds by default).
   - Warning sirens, flashing hazard lights, and VO lines play in the target base.

4. **GasActive**
   - On successful arming, the FortressGasWave hazard activates for that base.
   - Attackers gain the configured ticket reward (+50 tickets default).
   - A localized gas volume is enabled that deals DoT to non‑immune players in the base.

5. **Cooldown**
   - After the gas wave duration (20 seconds), the hazard deactivates.
   - The canister is temporarily unavailable; respawn is delayed by the configured cooldown (60 seconds default).
   - Once cooldown expires, the state machine returns to Waiting and a new canister spawns at its neutral point.

All server‑authoritative transitions are driven by a small set of input events:

- `OnPickup(player)`: Waiting → Carried.
- `OnEnterGasInlet(player, team)`: Carried → Arming (if inlet belongs to enemy team).
- `OnArmingComplete(attackerTeam)`: Arming → GasActive, then into Cooldown.
- `OnCarrierDeath()`: Carried → Waiting (canister dropped or reset depending on mode rules).
- `OnGasWaveFinished()`: GasActive → Cooldown.

### 1.4 Gas Inlet Objectives

Each base exposes a small objective volume that defines where gas can be armed:

- Two volumes per map: `GasInlet_SHC` and `GasInlet_Tediz`.
- Represented as Objective entries in `fortress_entities_v1.json` with `rtype: "GasInlet"` and roletags `shc_gas_inlet` and `tediz_gas_inlet`.
- The canister logic only transitions to Arming when:
  - The carrier’s team is the opposite of the inlet’s owning team.
  - The carrier is currently in Carried state.
  - No other gas event is in Arming or Active.

These objectives serve as stable anchor points for both code and AI‑chat, ensuring consistent behavior across engines.

---

## 2. Fortress – Tiles, Grid, and Heavy Weapons (30, 31, 33)

### 2.1 Fortress Tile Palette

Fortress uses the shared 4×4‑unit grid and a war‑torn industrial tile palette defined in `03MultiplayerFortress_Tile_Palette.md`. Key floor tiles include:

- `TFloorIndustrialPlain` – base interiors, tower interiors, and non‑hazard exterior ground.
- `TFloorIndustrialHazardStripe` – bridge edges, capture zone boundaries, and vehicle lanes.
- `TFloorMetalGrate` – catwalks over the valley and elevated platforms.
- `TFloorTrenchMud` – trenches, shell‑cratered no‑man’s land, and forward assault lanes.
- `TFloorCaptureZone` – a visual base for capture points, tagged with `capture_zone`.
- `TFloorSpawnZone` – base spawn staging areas, tagged with `spawn_zone`.

Walls and ceilings:

- Bunker concrete, tower panel/window tiles, tunnel ribs, and catwalk undersides.
- All tuned to read clearly at N64‑style distances and resolutions while still supporting higher fidelity materials.

The tileset is referenced by `maps/fortress/fortress_grid_v1.json` via tiletype strings and roletags so grid2scene can generate instanced geometry deterministically.

### 2.2 Fortress Grid Layout

`maps/fortress/fortress_grid_v1.json` defines a 32×32 grid of 4‑unit cells (128×128 world units). High‑level structure:

- Southern rows: SHC base, using tiles like `TFloorSHCBase`, roletagged `shcbase`.
- Central band: depressed valley and bridge span:
  - Valley: `TFloorValley` tiles tagged `centralvalley`.
  - Bridge: `TFloorBridge` tiles tagged `bridgecenter`.
- Northern rows: Tediz fortress courtyard and ramparts:
  - Courtyard: `TFloorTedizCourtyard` tiles, roletagged `tedizbase`.
  - Ramparts and towers: `TFloorTedizRamparts`, roletagged `tower`, `rampart`.

Trenches:

- Side trenches lead from SHC base towards the bridge using `TFloorTrench` tiles roletagged `trench` and `shcforward`.
- Some trench segments are additionally tagged `gascanisterroute` to indicate preferred flank paths for canister runs.

Roletags on grid cells are the glue between:

- Grid geometry.
- Entities such as spawns, objectives, heavy weapons, and hazards.
- Tools like grid2scene that must place actors at correct positions in each engine.

### 2.3 Heavy Weapon Placement

Heavy weapons in Fortress are deliberately placed as high‑risk rewards:

- Bazooka pickups:
  - Located on valley flank tiles with minimal cover.
  - Grid cells tagged `valleyflank`, `bazookaroute`, and `gascanisterdefense`.
  - Force players to drop into low ground to secure and reload, exposing them to fire from both bases and the bridge.

- Chaingun pickup:
  - Placed on a bridge tower tile (`TFloorBridgeTower`), roletagged `tower`, `chaingunperch`.
  - Provides commanding sightlines over the valley and central bridge, but requires climbing exposed ladders/ramps.

- Extra rocket ammo:
  - Positioned closer to each base but still in forward or lateral positions near trenches, requiring brief but risky pushes to resupply.

All heavy weapon pickups are defined in `fortress_entities_v1.json` as `WeaponPickup` entities with roletags that align with the tile roletags, ensuring consistent placement across engines.

---

## 3. Alien Base – Hazards, Grid, and Airlock Controller (35, 36, 37, 40)

### 3.1 Hazard Volume Bounds

Hazard bounds for Alien Base are captured in `docs/research/AlienBaseHazardVolumeBounds.md` and realized in the grid and entities files:

- `hazard_hub_floor_gas`
  - Radius: 28 world units around the hub center (Egg pedestal).
  - Vertical coverage: Y from −2 to +4.
  - Tiles: ring of `TFloorGrateGasEmitter`, roletagged `floorgaszone`, `hazardhubfloorgas`.

- `hazard_sublevel_acid`
  - Radius: 16 world units in the sublevel tunnels.
  - Vertical coverage: Y from −12 to −6.
  - Tiles: `TFloorAcidDrain` and adjacent floor pieces, roletagged `hazardsublevelacid`.

In `maps/alien_base/alien_base_hub_grid_v1.json`, these bounds are expressed in grid terms:

- Cell size: 4 units.
- Radius in cells: `radius_cells = radius_world / cellsize`.
- Center cell: typically `col: 11, row: 11` for the Egg pedestal.

`hazardvolumes` entries in `alien_base_hub_entities_v1.json` reference these bounds via `center_col`, `center_row`, `radius_cells`, and `y_min_offset` / `y_max_offset`.

### 3.2 Hub Grid Layout

`maps/alien_base/alien_base_hub_grid_v1.json` (v1.1.0) defines the hub floor and immediate surroundings:

- Grid: 24×24 cells of 4 units (96×96 world units).
- Center: Egg platform (`TFloorPlatformEggBase`, roletags `hubcenter`, `eggplatform`).
- Inner ring: `TFloorIndustrialPlain`, roletagged `hubfloor`.
- Gas ring: `TFloorGrateGasEmitter`, roletagged `hubfloor`, `floorgaszone`, `hazardhubfloorgas`.
- Outer ring: walls and corridor mouths:
  - Corridor entries tagged `corridorentrance_north/south/east/west`.
  - Outer wall tiles roletagged `hubwall`.

Sublevel:

- `sublevelentrance` roletags mark shafts or lifts down to the acid layer.
- Sublevel tiles use `TFloorAcidDrain` and associated roletags `hazardsublevelacid`.

This layout intentionally mirrors the Tanks canister bunker dynamic: low central area becomes hazardous during events while catwalks, raised corridors, and specific safe islands remain viable.

### 3.3 AlienBaseAirlockController State Machine

The airlock controller is a shared four‑state system implemented in:

- Unreal: `AAlienBaseAirlockController`.
- Unity: `AlienBaseAirlockController` MonoBehaviour.
- Godot: `AlienBaseAirlockController.gd`.

States:

1. **Idle**
   - Hub gas and sublevel acid hazards are disabled.
   - Triggers (consoles, Egg events) are ready.

2. **Arming**
   - Starts when a trigger is activated (console use, Egg health threshold).
   - Duration: 5 seconds.
   - FX: sirens, flashing lights, VO callouts, steam vents, and floor decals.

3. **Active**
   - Duration: 12 seconds.
   - `hazard_hub_floor_gas` and `hazard_sublevel_acid` volumes are enabled.
   - DoT: 60 HP/s for hub gas, 90 HP/s for sublevel acid (with tick interval ~0.25–0.5 seconds).

4. **Cooldown**
   - Duration: 30 seconds.
   - Hazards are disabled.
   - Triggers are locked out until cooldown completes.

Timing and damage values are data‑driven from a small trigger profile JSON, allowing other maps to reuse the same state machine with different parameters.

### 3.4 Damage‑over‑Time and ASID‑Based Immunity

Hazard DoT respects your ASID animation state system:

- If a pawn is in any execution ASID (e.g., 400, 405, 666, 901), that pawn is immune to environmental DoT for the tick.
- Players in a respawn grace ASID are also immune (no instant spawn kills).
- All other states receive damage based on the hazard’s configured DPS.

Simplified engine‑agnostic sketch:

```text
function ApplyHazardTick(hazard, pawn, deltaTime):
    if pawn.ASIDSet.intersects(EXECUTION_IMMUNITY_ASIDS):
        return

    if pawn.ASIDSet.intersects(RESPAWN_GRACE_ASIDS):
        return

    if not pawn.IsAlive():
        return

    dps = hazard.damagePerSecond
    damage = dps * deltaTime
    pawn.ApplyDamage(damage, source = hazard)
```

This logic is identical for Fortress gas waves, Alien Base hub gas, and sublevel acid; only the profile parameters and affected volumes differ.

---

## 4. Alien Egg Objective and Invasion Spawns (38, 39)

### 4.1 Alien Egg Objective Actor

The Alien Egg is a shared objective actor defined in `docs/design/AlienEggObjectiveActor.md` and implemented for all engines.

Core fields:

- Health: ~1500 HP by default.
- Stages: four visual states:
  - Intact (100–75%).
  - Cracked (75–50%).
  - Pulsing (50–25%).
  - Critical (25–0%).
- Mode variants:
  - **Airlock Focus** – Egg damage triggers airlock events but does not spawn aliens.
  - **Spawn Focus** – Egg damage spawns alien waves but does not affect hazards.
  - **Hybrid** (default) – Egg damage both escalates alien waves and arms the airlock at specific thresholds.

Replication:

- Health and current stage are replicated via your existing objective framework so all clients see consistent visuals and receive correct feedback.

### 4.2 Invasion NPC Spawners and Patrols

Invasion‑mode alien spawners are documented in `docs/design/AlienBaseInvasionSpawnsAndPatrols.md` and bound to the map via `maps/alien_base/alien_base_hub_entities_v1.json`.

Spawner characteristics:

- Example IDs: `spawner_alien_vent_north`, `spawner_alien_vent_east`, `spawner_alien_vent_south`, `spawner_alien_vent_west`.
- Roletags: `npcspawn`, `alien`, `vent`, plus a zone label like `zone_north`.
- Parameters:
  - `max_alive` – cap for simultaneously active aliens per spawner.
  - `spawn_interval_seconds` – base delay between spawns.
  - `activation_role_tags` – e.g., `invasion_active`, `egg_damaged_stage_2`.

Patrol routes:

- Stored in `data/ai/alien_routes_v1.json`, referenced by `route_id` in each spawner.
- Define sequences of waypoints (grid cell references or world coordinates) that run along vents, walls, and ceiling paths to keep aliens threatening from multiple angles.

Alien archetypes:

- Drone, Warrior, Elite, and Queen variants, each with different health, speed, and behaviors.
- Eight‑wave progression table controls composition and timing, escalating difficulty as the Egg approaches destruction.

---

## 5. Cross‑Map Hazard & Objective System Improvements (Fortress + Alien Base)

### 5.1 HazardVolume Abstraction and Profiles

Fortress gas, Alien Base hub gas, and sublevel acid are all modeled as `hazardvolumes` in `*_entities_v1.json`. This makes it natural to unify them behind a shared `HazardVolume` abstraction:

- A base class or interface in each engine that:
  - Receives `damagePerSecond`, `tickInterval`, `asidImmunitySet`, and VFX/SFX IDs from data.
  - Exposes `SetActive(bool)` for controllers like `AGasCanister` or `AlienBaseAirlockController`.
  - Emits analytics/events for kills, assists, and mode hooks.

Tuning is moved into `config/hazards/hazard_profiles_v1.json`:

- Fortress:
  - `fortress_gas_base_v1` – DPS 15, duration 20s, profile for base‑localized gas wave.
- Alien Base:
  - `alien_hub_gas_v1` – DPS 60, radius 28, duration 12s.
  - `alien_sublevel_acid_v1` – DPS 90, radius 16, duration 12s.

Each `hazardvolume` references its profile by ID, allowing Fortress and Alien Base to share the same HazardVolume logic with different parameters.

### 5.2 Heavy Carry Deliverable Objective Base

All carryable objectives can reuse a single Heavy Carry base:

- Gas Canister (Fortress).
- Money Bag (Heist).
- Blood Vial (The Blood Count).
- Talking Money and other future “payloads”.

This base implements:

- Heavy Carry ASID application (`ASID_050`):
  - Movement penalty.
  - No jump.
  - Weapon restrictions.
- Drop‑on‑death:
  - Spawn a carryable instance where the carrier died.
  - Optionally apply a decay timer that respawns the objective at a safe spawn if not re‑picked up.
- HUD and marker integration:
  - World marker and minimap ping when objective is free or dropped.
  - Context‑sensitive prompts at valid delivery/arming zones.

Scoring and side‑effects (ticket reward, hazard activation, wave triggers) are defined in per‑objective data profiles rather than baked into the base class.

### 5.3 Mode Profiles, CI, and Tooling

Fortress and Alien Base are already in the map manifest used by grid2scene and CI:

- `maps/multiplayer/map_manifest_v1.json` lists:
  - Fortress and Alien Base grid, entities, and tileset paths.
  - Supported game modes and recommended player counts.

Next steps:

- `config/modes/fortress_mode_profiles_v1.json`
  - Capture point ticket values.
  - Gas reward tickets, durations, and cooldowns.
  - Variants for standard, “Stronghold”, or experimental modes.

- `config/modes/alien_base_mode_profiles_v1.json`
  - Airlock timings.
  - Hazard severity for TDM vs Invasion.
  - Egg health and wave pacing for different difficulty levels.

Tooling and CI:

- Ensure `grid2scene --validate --all` runs for Fortress and Alien Base on every push.
- Finish Unreal and Unity editor‑builder scripts so designers can:
  - Regenerate Fortress and Alien Base greyboxes from JSON with one click.
  - Flip mode profiles and hazard profiles from the editor via small configuration UIs.

These improvements keep existing implementations (29–40) intact while pushing the architecture toward fully data‑driven hazards and objectives that are easy to extend to future maps and modes.
