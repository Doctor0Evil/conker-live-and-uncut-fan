# 04_Multiplayer_Alien_Base – Entity & Prefab Map

This document defines canonical entity, prefab, and component names for implementing Alien Base in Unreal Engine, Unity, and Godot. The goal is to let AI‑assisted tools and code generators target the same conceptual objects across engines while you maintain a single source of truth in this repo. [github](https://github.com/tranek/GASDocumentation)

It complements:

- `04_Multiplayer_Alien_Base.md` – geometric layout, pickups, and spawns.  
- `04_Multiplayer_Alien_Base_Triggers.md` – Airlock/Gas state machine, timing, and damage rules.

## Naming Conventions

To keep things predictable across engines:

- Map‑level actors: `AlienBase_*`  
- Hazard controllers: `AlienBase_Airlock*`  
- Volumes/areas: `AlienBase_Volume_*`  
- Pickups: `Pickup_*`  
- Triggers/consoles: `AlienBase_Trigger_*`  

Each engine section lists recommended class/prefab names and how they map to the abstract objects defined in the other two documents. [docs.godotengine](https://docs.godotengine.org/en/latest/tutorials/performance/optimizing_3d_performance.html)

***

## Unreal Engine Entities

Unreal is assumed to use C++ + Blueprints + (optionally) Gameplay Ability System (GAS) for damage and status effects. [vorixo.github](https://vorixo.github.io/devtricks/gas/)

### Level & Subsystems

- `UWorld` Level Name: `LV_AlienBase_Multi`  
- Game Mode: `AGM_AlienBaseMulti`  
- Game State: `AGS_AlienBaseMulti`  
- Airlock Controller Actor: `AAlienBase_AirlockController`  

The controller owns the state machine described in the triggers doc and references all hazard volumes and consoles via `TWeakObjectPtr` or soft references.

### Hazard Volumes

Use `AActor` subclasses with `UBoxComponent` or `UCapsuleComponent` for overlaps and a custom C++ tick to apply damage, or wrap them in GAS area‑of‑effect gameplay cues. [youtube](https://www.youtube.com/watch?v=0XfM_UyJdhY)

- `AAlienBase_Volume_HubFloorGas`  
  - Components:  
    - `UBoxComponent* CollisionVolume`  
    - Niagara system for gas FX (`UNiagaraComponent* GasFX`)  
    - Audio component for hissing/alarm  
  - Tags: `Tag = "AlienBase_HubFloorGas"`  

- `AAlienBase_Volume_SublevelAcid`  
  - Components:  
    - `UBoxComponent* CollisionVolume`  
    - Acid mist FX (Niagara)  
    - Audio  
  - Tags: `Tag = "AlienBase_SublevelAcid"`  

These actors expose `SetVolumeActive(bool)` to be called from `AAlienBase_AirlockController`.

### Triggers / Consoles

Use Blueprint‑derived Actors placed at the coordinate anchors in the other docs.

- `BP_AlienBase_Trigger_AirlockNorth`  
- `BP_AlienBase_Trigger_AirlockSouth`  
- `BP_AlienBase_Trigger_PurgeEast`  
- `BP_AlienBase_Trigger_PurgeWest`  

Each contains:

- Static mesh console.  
- `UBoxComponent` or `USphereComponent` for interaction range.  
- Blueprint interface or direct reference to `AAlienBase_AirlockController` to call `RequestTriggerActivation(TriggerId, InstigatorTeam)`.

### Pickups

Core classes:

- Base C++ class: `APickupBase`  
- Derived Blueprints for each weapon:  
  - `BP_Pickup_Chainsaw`  
  - `BP_Pickup_Flamethrower`  
  - `BP_Pickup_SMG`  
  - `BP_Pickup_Shotgun`  
  - `BP_Pickup_SniperRifle`  
  - `BP_Pickup_Bazooka`  

Place instances at the coordinates documented in `04_Multiplayer_Alien_Base.md`. The AI‑chat tool can generate a placement Blueprint or construction script from those anchors, using the canonical names here.

### Spawn Points

Use `APlayerStart` or a subclass:

- `APlayerStart_ZoneA_01` … `APlayerStart_ZoneA_04`  
- `APlayerStart_ZoneB_01` … `APlayerStart_ZoneB_04`  
- `APlayerStart_ZoneC_01` … `APlayerStart_ZoneC_04`  
- `APlayerStart_ZoneD_01` … `APlayerStart_ZoneD_04`  

Game mode logic can treat their names or Gameplay Tags (`Tag="Spawn_ZoneA"`) as the linkage to the spawn zones defined in the MD layout.

***

## Unity Entities

Unity is assumed to use prefabs, C# MonoBehaviours, and trigger colliders for hazard areas and pickups. [youtube](https://www.youtube.com/watch?v=QVFUjqSuFrg)

### Scene & Managers

- Scene name: `AlienBase_Multi`  
- GameManager: `AlienBaseGameManager` (C#)  
- Airlock Controller: `AlienBaseAirlockController` (C# component on an empty `GameObject` named `AlienBase_AirlockController`)

The controller maintains the same states (`Idle`, `Arming`, `Active`, `Cooldown`) and exposes serialized fields for hazard volumes, timing values, and references to trigger GameObjects.

### Hazard Volumes

Use `GameObject` with `BoxCollider` set as `isTrigger` and a `HazardVolume` script.

- Hub Floor Gas:  
  - GameObject: `AlienBase_Volume_HubFloorGas`  
  - Components:  
    - `BoxCollider (isTrigger = true)`  
    - `HazardVolume` (C#) – fields: `damagePerSecond`, `hazardType` enum, `isActive`.  
    - Particle system for gas.

- Sublevel Acid:  
  - GameObject: `AlienBase_Volume_SublevelAcid`  
  - Similar setup with higher `damagePerSecond`.

The Airlock controller toggles `isActive` on these scripts and enables/disables FX.

### Triggers / Consoles

Use `GameObject` with `BoxCollider` (trigger) and an `AirlockTriggerConsole` script.

- `AlienBase_Trigger_AirlockNorth`  
- `AlienBase_Trigger_AirlockSouth`  
- `AlienBase_Trigger_PurgeEast`  
- `AlienBase_Trigger_PurgeWest`  

Fields:

- `public string triggerId;` (`"AirlockNorth"`, etc.)  
- `public AlienBaseAirlockController controller;`  

On player interact (E key, button prompt, etc.), they call `controller.RequestTriggerActivation(triggerId, instigatorTeam)`.

### Pickups

Base prefab:

- `PickupBase` GameObject  
  - `Collider` (trigger)  
  - `Pickup` script (type enum, respawn time, etc.)  
  - Mesh/FX

Specific prefabs:

- `Pickup_Chainsaw`  
- `Pickup_Flamethrower`  
- `Pickup_SMG`  
- `Pickup_Shotgun`  
- `Pickup_SniperRifle`  
- `Pickup_Bazooka`  

Your placement tool or editor script can read the MD layout and instantiate these prefabs at their specified coordinates.

### Spawn Points

Use empty GameObjects with an identifying component or tag:

- Names:  
  - `Spawn_ZoneA_01` … `Spawn_ZoneD_04`  

Attach a `SpawnPoint` script with fields:

- `public string zoneId;` (`"A"`, `"B"`, `"C"`, `"D"`)  
- `public int index;`

The spawn system queries these by `zoneId` to implement the LoS‑aware spawning from the layout doc.

***

## Godot Entities (Godot 4.x)

Godot is assumed to use `Area3D` for hazard/damage volumes and pickup triggers, with `Node3D`/`CharacterBody3D` for characters. [docs.godotengine](https://docs.godotengine.org/en/latest/tutorials/performance/optimizing_3d_performance.html)

### Scene & Controllers

- Main scene: `AlienBase_Multi.tscn`  
- Root node: `AlienBaseRoot` (`Node3D`)  
- Airlock controller: `AlienBaseAirlockController` (`Node` or `Node3D` child)

Controller script: `AlienBaseAirlockController.gd`, holding state machine, timers, and `on_trigger_activated(trigger_id, instigator_team)`.

### Hazard Areas

Use `Area3D` with collision shapes and a script that emits signals or directly calls `take_damage` on bodies inside. [youtube](https://www.youtube.com/watch?v=QVFUjqSuFrg)

- Hub Floor Gas:  
  - Node: `AlienBase_Volume_HubFloorGas` (`Area3D`)  
  - Children:  
    - `CollisionShape3D` (box or cylinder)  
    - `GPUParticles3D` (gas FX)  
  - Script: `HazardVolume.gd` with exports:  
    - `@export var damage_per_second: float = 60.0`  
    - `@export var active: bool = false`

- Sublevel Acid:  
  - Node: `AlienBase_Volume_SublevelAcid` (`Area3D`)  
  - Similar script with higher damage.

Airlock controller connects to `set_active(true/false)` on these nodes when entering/exiting `Active` state.

### Triggers / Consoles

Use `Area3D` or `Interactable` nodes near console meshes:

- Nodes:  
  - `AlienBase_Trigger_AirlockNorth`  
  - `AlienBase_Trigger_AirlockSouth`  
  - `AlienBase_Trigger_PurgeEast`  
  - `AlienBase_Trigger_PurgeWest`  

Attach `AirlockTriggerConsole.gd`:

- Exports:  
  - `@export var trigger_id: String`  
  - `@export var airlock_controller: NodePath`  

On player interaction (input + overlap), call `airlock_controller.on_trigger_activated(trigger_id, instigator_team)`.

### Pickups

Use `Area3D` with a pickup script and mesh.

- Base scene: `PickupBase.tscn` (`Area3D`)  
- Derived scenes:  
  - `Pickup_Chainsaw.tscn`  
  - `Pickup_Flamethrower.tscn`  
  - `Pickup_SMG.tscn`  
  - `Pickup_Shotgun.tscn`  
  - `Pickup_SniperRifle.tscn`  
  - `Pickup_Bazooka.tscn`  

Place them at the coordinate anchors from the layout doc, or spawn them procedurally from a GD script that reads a JSON or table generated from your MD.

### Spawn Points

Use `Marker3D` or `Node3D`:

- Names:  
  - `Spawn_ZoneA_01` … `Spawn_ZoneD_04`  

Attach a `SpawnPoint.gd` script with exported `zone_id` and `index` so your spawn manager can gather them and perform LoS checks before choosing a spawn.

***

## Engine‑Agnostic Mapping Table

This table summarizes how key objects map across engines.

| Concept                | Unreal Entity                          | Unity Entity                          | Godot Entity                       |
|------------------------|----------------------------------------|---------------------------------------|------------------------------------|
| Airlock controller     | `AAlienBase_AirlockController`         | `AlienBaseAirlockController`          | `AlienBaseAirlockController` node  |
| Hub floor gas volume   | `AAlienBase_Volume_HubFloorGas`        | `AlienBase_Volume_HubFloorGas`        | `AlienBase_Volume_HubFloorGas`     |
| Sublevel acid volume   | `AAlienBase_Volume_SublevelAcid`       | `AlienBase_Volume_SublevelAcid`       | `AlienBase_Volume_SublevelAcid`    |
| North airlock trigger  | `BP_AlienBase_Trigger_AirlockNorth`    | `AlienBase_Trigger_AirlockNorth`      | `AlienBase_Trigger_AirlockNorth`   |
| South airlock trigger  | `BP_AlienBase_Trigger_AirlockSouth`    | `AlienBase_Trigger_AirlockSouth`      | `AlienBase_Trigger_AirlockSouth`   |
| East purge trigger     | `BP_AlienBase_Trigger_PurgeEast`       | `AlienBase_Trigger_PurgeEast`         | `AlienBase_Trigger_PurgeEast`      |
| West purge trigger     | `BP_AlienBase_Trigger_PurgeWest`       | `AlienBase_Trigger_PurgeWest`         | `AlienBase_Trigger_PurgeWest`      |
| Chainsaw pickup        | `BP_Pickup_Chainsaw`                   | `Pickup_Chainsaw` prefab              | `Pickup_Chainsaw.tscn`             |
| Bazooka pickup         | `BP_Pickup_Bazooka`                    | `Pickup_Bazooka` prefab               | `Pickup_Bazooka.tscn`              |
| Zone A spawn point #1  | `APlayerStart_ZoneA_01`                | `Spawn_ZoneA_01` GameObject           | `Spawn_ZoneA_01` Marker3D          |
