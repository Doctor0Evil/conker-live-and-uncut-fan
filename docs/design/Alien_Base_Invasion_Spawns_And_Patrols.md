# Alien Base Invasion Mode - Spawn Points & Patrol Routes

## Overview
This document defines spawn points and patrol routes for Alien NPCs in the "Invasion" game mode variant of the Alien Base multiplayer map. The system uses `role_tags` from the grid to anchor spawn locations and define navigation paths.

---

## Alien NPC Types

| Type | Health | Damage | Speed | Behavior |
|------|--------|--------|-------|----------|
| **Drone** | 50 HP | 10 HP/sec (melee) | Fast (6 m/s) | Swarm, flank |
| **Warrior** | 150 HP | 25 HP/sec (melee), 15 HP/projectile (ranged) | Medium (4 m/s) | Aggressive pursuit |
| **Elite** | 300 HP | 40 HP/sec (melee), 30 HP/projectile (ranged) | Slow (3 m/s) | Tank, area denial |
| **Queen** (Mini-boss) | 1000 HP | 60 HP/sec (melee), 50 HP/projectile (ranged), AoE slam | Very Slow (2 m/s) | Boss mechanics |

---

## Spawn Points

### Primary Vent Spawns
These are the main alien entry points, marked with `alien_vent_spawn` role_tag in the grid.

```json
{
  "id": "alien_vent_north",
  "grid_ref": {"col": 8, "row": 2},
  "world_pos": {"x": 0.0, "y": 0.0, "z": -32.0},
  "role_tags": ["alien_vent_spawn", "corridor_north"],
  "capacity": 4,
  "spawn_delay": 2.0,
  "allowed_types": ["Drone", "Warrior"]
}
```

### Secondary Corridor Spawns
Fallback spawns when primary vents are blocked or at high wave counts.

| Spawn ID | Grid Ref (col, row) | World Position | Role Tags | Capacity | Allowed Types |
|----------|---------------------|----------------|-----------|----------|---------------|
| `alien_corridor_east` | (14, 8) | (24.0, 0.0, 0.0) | `corridor_east`, `spawn_corridor` | 3 | Drone, Warrior |
| `alien_corridor_west` | (2, 8) | (-24.0, 0.0, 0.0) | `corridor_west`, `spawn_corridor` | 3 | Drone, Warrior |
| `alien_corridor_south` | (8, 14) | (0.0, 0.0, 24.0) | `corridor_south`, `spawn_corridor` | 3 | Drone, Warrior |

### Emergency Hub Spawns
Used only during critical phases (egg ≤25% health or wave 5+).

| Spawn ID | Grid Ref (col, row) | World Position | Role Tags | Trigger Condition |
|----------|---------------------|----------------|-----------|-------------------|
| `alien_hub_center` | (8, 8) | (0.0, 0.0, 0.0) | `egg_platform`, `hub_floor` | Egg health ≤25% |
| `alien_catwalk_north` | (8, 0) | (0.0, 25.0, 0.0) | `catwalk_ring`, `vertical_access` | Wave ≥5 |
| `alien_catwalk_south` | (8, 16) | (0.0, 25.0, 0.0) | `catwalk_ring`, `vertical_access` | Wave ≥5 |

---

## Patrol Routes

Patrol routes are defined as ordered waypoints using grid cell references. Aliens follow these paths when not actively pursuing a player.

### Route 1: North Vent to Hub Loop
**Purpose**: Basic patrol for Drones entering from north vent.

```json
{
  "route_id": "patrol_north_hub_loop",
  "spawn_point": "alien_vent_north",
  "waypoints": [
    {"col": 8, "row": 2, "wait_time": 0.0},
    {"col": 8, "row": 4, "wait_time": 1.0},
    {"col": 8, "row": 6, "wait_time": 0.0},
    {"col": 6, "row": 8, "wait_time": 2.0},
    {"col": 8, "row": 10, "wait_time": 0.0},
    {"col": 10, "row": 8, "wait_time": 2.0},
    {"col": 8, "row": 6, "wait_time": 0.0},
    {"col": 8, "row": 4, "wait_time": 1.0},
    {"col": 8, "row": 2, "wait_time": 3.0}
  ],
  "loop": true,
  "behavior_on_alert": "abandon_patrol_and_pursue"
}
```

### Route 2: East-West Corridor Sweep
**Purpose**: Warrior patrol covering lateral corridors.

```json
{
  "route_id": "patrol_east_west_sweep",
  "spawn_point": "alien_corridor_east",
  "waypoints": [
    {"col": 14, "row": 8, "wait_time": 0.0},
    {"col": 12, "row": 8, "wait_time": 0.0},
    {"col": 10, "row": 8, "wait_time": 1.0},
    {"col": 8, "row": 8, "wait_time": 3.0},
    {"col": 6, "row": 8, "wait_time": 1.0},
    {"col": 4, "row": 8, "wait_time": 0.0},
    {"col": 2, "row": 8, "wait_time": 2.0},
    {"col": 4, "row": 8, "wait_time": 0.0},
    {"col": 6, "row": 8, "wait_time": 1.0},
    {"col": 8, "row": 8, "wait_time": 3.0},
    {"col": 10, "row": 8, "wait_time": 1.0},
    {"col": 12, "row": 8, "wait_time": 0.0},
    {"col": 14, "row": 8, "wait_time": 3.0}
  ],
  "loop": true,
  "behavior_on_alert": "flank_nearest_player"
}
```

### Route 3: Catwalk Ring Perimeter
**Purpose**: Elite patrol on elevated catwalks (unlocked Wave 5+).

```json
{
  "route_id": "patrol_catwalk_perimeter",
  "spawn_point": "alien_catwalk_north",
  "waypoints": [
    {"col": 8, "row": 0, "y_offset": 25.0, "wait_time": 0.0},
    {"col": 6, "row": 2, "y_offset": 25.0, "wait_time": 1.0},
    {"col": 2, "row": 6, "y_offset": 25.0, "wait_time": 1.0},
    {"col": 0, "row": 8, "y_offset": 25.0, "wait_time": 2.0},
    {"col": 2, "row": 10, "y_offset": 25.0, "wait_time": 1.0},
    {"col": 6, "row": 14, "y_offset": 25.0, "wait_time": 1.0},
    {"col": 8, "row": 16, "y_offset": 25.0, "wait_time": 2.0},
    {"col": 10, "row": 14, "y_offset": 25.0, "wait_time": 1.0},
    {"col": 14, "row": 10, "y_offset": 25.0, "wait_time": 1.0},
    {"col": 16, "row": 8, "y_offset": 25.0, "wait_time": 2.0},
    {"col": 14, "row": 6, "y_offset": 25.0, "wait_time": 1.0},
    {"col": 10, "row": 2, "y_offset": 25.0, "wait_time": 1.0},
    {"col": 8, "row": 0, "y_offset": 25.0, "wait_time": 3.0}
  ],
  "loop": true,
  "behavior_on_alert": "hold_position_and_ranged_attack"
}
```

### Route 4: Sublevel Ambush
**Purpose**: Surprise attack route through maintenance tunnels.

```json
{
  "route_id": "patrol_sublevel_ambush",
  "spawn_point": "alien_vent_north",
  "waypoints": [
    {"col": 8, "row": 2, "y_offset": 0.0, "wait_time": 0.0},
    {"col": 8, "row": 1, "y_offset": -10.0, "wait_time": 2.0},
    {"col": 7, "row": 1, "y_offset": -10.0, "wait_time": 0.0},
    {"col": 6, "row": 1, "y_offset": -10.0, "wait_time": 3.0},
    {"col": 8, "row": 1, "y_offset": -10.0, "wait_time": 2.0},
    {"col": 8, "row": 2, "y_offset": 0.0, "wait_time": 0.0}
  ],
  "loop": true,
  "behavior_on_alert": "emerge_and_attack"
}
```

---

## Wave Configuration

### Wave Structure
Each wave defines spawn count, alien types, and route assignments.

| Wave | Egg Health Trigger | Spawn Count | Composition | Routes Used | Special Behavior |
|------|-------------------|-------------|-------------|-------------|------------------|
| **1** | N/A (start) | 2 Drones | 100% Drone | `patrol_north_hub_loop` | Passive until provoked |
| **2** | 75% | 4 Aliens | 2 Drone, 2 Warrior | `patrol_north_hub_loop`, `patrol_east_west_sweep` | Aggressive on sight |
| **3** | 60% | 6 Aliens | 3 Drone, 3 Warrior | All ground routes | Coordinated flanking |
| **4** | 50% | 8 Aliens | 2 Drone, 4 Warrior, 2 Elite | All ground routes + sublevel | Elite tank pushes |
| **5** | 35% | 10 Aliens | 2 Drone, 4 Warrior, 4 Elite | All routes including catwalk | Catwalk control |
| **6** | 25% | 12 Aliens | 2 Drone, 4 Warrior, 5 Elite, 1 Queen | All routes | Queen leads assault |
| **7** | 10% | 15 Aliens | 3 Drone, 5 Warrior, 6 Elite, 1 Queen | All routes + hub center | Last stand |
| **Final** | 0% (Destroyed) | 20 Aliens | 4 Drone, 6 Warrior, 8 Elite, 2 Queens | All routes + emergency spawns | Berserk mode |

---

## Behavior Trees

### Drone Behavior
```
ROOT
├── Selector
│   ├── Sequence (Alerted)
│   │   ├── Condition: HasVisiblePlayer
│   │   └── Action: PursueAndMeleeAttack
│   └── Sequence (Patrolling)
│       ├── Action: FollowPatrolRoute
│       └── Action: WaitAtWaypoint
└── Decorator
    └── Condition: IsEggHealthBelowThreshold
        └── Action: IncreaseAggressionRadius
```

### Warrior Behavior
```
ROOT
├── Selector
│   ├── Sequence (Ranged Engagement)
│   │   ├── Condition: PlayerInRange_And_LineOfSight
│   │   └── Action: RangedAttack
│   ├── Sequence (Melee Engagement)
│   │   ├── Condition: PlayerInMeleeRange
│   │   └── Action: MeleeAttack
│   └── Sequence (Flanking)
│       ├── Action: CalculateFlankPosition
│       └── Action: MoveToFlankPosition
└── Decorator
    └── Condition: IsInGroup
        └── Action: CoordinateAttackWithPack
```

### Elite Behavior
```
ROOT
├── Selector
│   ├── Sequence (Tank Stance)
│   │   ├── Condition: AlliesUnderFire
│   │   └── Action: InterceptDamage
│   ├── Sequence (Area Denial)
│   │   ├── Condition: PlayersInChokePoint
│   │   └── Action: BlockPathway
│   └── Sequence (Heavy Assault)
│       ├── Action: ChargeToTarget
│       └── Action: HeavyMeleeCombo
└── Decorator
    └── Condition: HealthBelow30Percent
        └── Action: EnterBerserkState
```

### Queen Behavior (Boss)
```
ROOT
├── Selector
│   ├── Sequence (Spawn Minions)
│   │   ├── Condition: MinionCount < 4
│   │   └── Action: SpawnDroneAtLocation
│   ├── Sequence (AoE Slam)
│   │   ├── Condition: PlayersInRadius(5m)
│   │   └── Action: PerformGroundSlam
│   ├── Sequence (Ranged Barrage)
│   │   ├── Condition: NoPlayersInMeleeRange
│   │   └── Action: FireProjectileSpread
│   └── Sequence (Melee Crush)
│       ├── Condition: PlayerInMeleeRange
│       └── Action: GrabAndCrush
└── Decorator
    └── Condition: HealthBelow50Percent
        └── Action: SummonReinforcements
```

---

## Implementation by Engine

### Unreal Engine (C++) - AI Controller

```cpp
// AlienBase_AlienAIController.h
UCLASS()
class AAlienBase_AlienAIController : public AAIController
{
    GENERATED_BODY()

public:
    UPROPERTY(EditAnywhere, Category = "Patrol")
    TArray<FVector> PatrolWaypoints;

    UPROPERTY(EditAnywhere, Category = "Patrol")
    float WaypointWaitTime;

    UFUNCTION(BlueprintCallable, Category = "Alien|Spawn")
    void InitializeFromSpawnData(FAlienSpawnData SpawnData);

    UFUNCTION(BlueprintCallable, Category = "Alien|Patrol")
    void StartPatrolRoute(FName RouteId);

protected:
    virtual void OnPossess(APawn* InPawn) override;

private:
    void OnPatrolCompleted();
    void OnAlertStateChanged(bool bIsAlerted);
};
```

### Unity (C#) - Spawn Manager

```csharp
public class AlienSpawnManager : MonoBehaviour
{
    [System.Serializable]
    public class AlienWave
    {
        public int waveNumber;
        public float eggHealthTrigger;
        public AlienType[] composition;
        public int[] counts;
        public string[] routeIds;
    }

    public AlienWave[] waves;
    public Transform[] spawnPoints;
    public GameObject[] alienPrefabs;

    private int currentWave = 0;

    public void CheckWaveTrigger(float currentEggHealth)
    {
        if (currentWave >= waves.Length) return;

        AlienWave nextWave = waves[currentWave];
        if (currentEggHealth <= nextWave.eggHealthTrigger * 500f) // 500 = max egg health
        {
            StartCoroutine(SpawnWave(nextWave));
            currentWave++;
        }
    }

    IEnumerator SpawnWave(AlienWave wave)
    {
        for (int i = 0; i < wave.composition.Length; i++)
        {
            for (int j = 0; j < wave.counts[i]; j++)
            {
                Transform spawnPoint = GetSpawnPointForRoute(wave.routeIds[i % wave.routeIds.Length]);
                GameObject alien = Instantiate(alienPrefabs[(int)wave.composition[i]], spawnPoint.position, Quaternion.identity);
                alien.GetComponent<AlienAI>().SetPatrolRoute(wave.routeIds[i % wave.routeIds.Length]);
                yield return new WaitForSeconds(2f);
            }
        }
    }
}
```

### Godot (GDScript) - Patrol Follower

```gdscript
extends CharacterBody3D
class_name AlienPatrolFollower

@export var patrol_route: Array[Vector3]
@export var wait_times: Array[float]
@export var loop_route: bool = true

var current_waypoint_index: int = 0
var is_patrolling: bool = true
var is_alerted: bool = false

func _physics_process(delta: float) -> void:
    if is_alerted:
        pursue_player()
        return
    
    if is_patrolling and patrol_route.size() > 0:
        move_to_next_waypoint(delta)

func move_to_next_waypoint(delta: float) -> void:
    if current_waypoint_index >= patrol_route.size():
        if loop_route:
            current_waypoint_index = 0
        else:
            is_patrolling = false
            return
    
    var target = patrol_route[current_waypoint_index]
    var direction = (target - global_transform.origin).normalized()
    
    if global_transform.origin.distance_to(target) < 0.5:
        # Reached waypoint
        await get_tree().create_timer(wait_times[current_waypoint_index]).timeout
        current_waypoint_index += 1
    else:
        velocity = direction * 4.0  # 4 m/s default speed
        move_and_slide()

func on_player_spotted(player: Node3D) -> void:
    is_alerted = true
    current_target = player
```

---

## Integration with Hazard System

### Synergy Rules
1. **Aliens Immune to Friendly Hazards**: When `hazard_hub_floor_gas` is active, aliens take no damage (they are adapted).
2. **Players Trapped Between**: Hazard activation forces players onto catwalks where aliens may be waiting.
3. **Egg Destruction Override**: If egg is destroyed during hazard active state, extend hazard duration by 10s.

### State Synchronization
```cpp
void AAlienBase_AirlockController::OnEnteredActive()
{
    SetHazardVolumesActive(true);
    
    // Notify all alien controllers
    TArray<AActor*> FoundAliens;
    UGameplayStatics::GetAllActorsOfClass(GetWorld(), AAlienBase_AlienAIController::StaticClass(), FoundAliens);
    
    for (AActor* AlienActor : FoundAliens)
    {
        AAlienBase_AlienAIController* AlienCtrl = Cast<AAlienBase_AlienAIController>(AlienActor);
        if (AlienCtrl)
        {
            AlienCtrl->OnHazardActivated(); // Aliens know to avoid or ignore
        }
    }
}
```

---

## Testing Checklist

- [ ] All spawn points correctly reference grid cells
- [ ] Patrol routes form valid paths (no wall collisions)
- [ ] Wave triggers activate at correct egg health thresholds
- [ ] Alien types spawn with correct stats and behaviors
- [ ] Catwalk routes only unlock at Wave 5
- [ ] Queen boss spawns only at Wave 6+
- [ ] Aliens correctly ignore friendly hazard damage
- [ ] Emergency hub spawns trigger only when egg ≤25%
- [ ] Network replication works for all alien states
- [ ] Performance stays stable with 20+ aliens active

---

## References
- Grid: `data/alien_base_hub_grid_v1.json`
- Egg Actor: `docs/design/Alien_Egg_Objective_Actor.md`
- Triggers: `docs/multiplayer/04_Multiplayer_Alien_Base_Triggers.md`
- Main Map Doc: `docs/multiplayer/04_Multiplayer_Alien_Base.md`
