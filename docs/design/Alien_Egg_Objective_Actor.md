# Alien Egg Objective Actor Specification

## Overview
The Alien Egg is the central objective actor for the 04_Multiplayer_Alien_Base map. It serves as both a visual centerpiece and an interactive gameplay element that can trigger hazard events or alien spawns when damaged or destroyed.

---

## Actor Properties

### Base Configuration
```
Actor Class: AAlienBase_EggObjective
Default Health: 500 HP
Damage Resistance: 0.8 (takes 20% reduced damage from non-explosive sources)
Respawn Time: N/A (persistent throughout match)
```

### Visual Stages
The egg progresses through 4 visual states based on health percentage:

| Stage | Health % | Visual Description | Audio Cue |
|-------|----------|-------------------|-----------|
| **Intact** | 100-76% | Smooth obsidian shell, faint purple pulse every 3s | Low hum |
| **Cracking** | 75-51% | Visible hairline fractures, increased pulse frequency (every 1.5s), green fluid seeping | Cracking sounds, dripping |
| **Pulsing** | 50-26% | Deep fractures glow lime-green, violent pulsing (every 0.5s), shell fragments floating | Intense throbbing, alien chirps |
| **Critical** | 25-0% | Shell breaking apart, bright green core exposed, debris orbiting | High-pitched whine, imminent rupture |

---

## Damage Response System

### Damage Type Modifiers
| Damage Type | Modifier | Notes |
|-------------|----------|-------|
| Ballistic (Pistol, SMG, Shotgun) | 0.8 | Reduced effectiveness |
| Explosive (Bazooka, Grenades) | 1.5 | Highly effective |
| Energy (Flamethrower, Plasma) | 1.2 | Moderately effective |
| Melee (Chainsaw, Sabre) | 0.5 | Minimal damage |
| Environmental (Gas, Acid) | 0.0 | Immune |

### Stage Transition Triggers
When health crosses thresholds (75%, 50%, 25%):
1. Play stage-specific VFX (particle burst)
2. Trigger audio stinger
3. Broadcast network event to all clients
4. Optionally trigger secondary effects (see below)

---

## Interactive Behaviors

### Behavior Mode 1: Hazard Trigger (Default)
When the egg reaches **Critical** stage (≤25% health) OR is destroyed:

**Option A: Immediate Trigger**
- Instantly activate `hazard_hub_floor_gas` and `hazard_sublevel_acid`
- Skip Arming state, go directly to Active (5-second warning only)
- Duration: 15 seconds (extended from normal 12s)

**Option B: Delayed Trigger (Configurable)**
- Start 3-second countdown
- On completion: trigger airlock hazard sequence
- VO announcement: "Containment breach! Evacuate lower levels!"

### Behavior Mode 2: Alien Spawn Wave
When the egg reaches specific health thresholds:

| Threshold | Spawn Location | Alien Type | Count | Delay |
|-----------|---------------|------------|-------|-------|
| 75% | `alien_vent_spawn` (col 8, row 2) | Drone | 2 | 2s |
| 50% | `alien_vent_spawn` + corridors | Drone + Warrior | 4 | 3s |
| 25% | All spawn points + hub center | Warrior + Elite | 6 | 5s |
| 0% (Destroyed) | Hub center (egg position) | Elite + Queen (mini-boss) | 8 | 0s |

Spawn Logic Pseudocode:
```cpp
void OnEggHealthThreshold(float HealthPercent)
{
    if (HealthPercent <= 75.0f && !bSpawnedAt75) {
        SpawnAliens("Drone", 2, GetSpawnPoint("alien_vent_north"));
        bSpawnedAt75 = true;
    }
    if (HealthPercent <= 50.0f && !bSpawnedAt50) {
        SpawnAliens("Drone", 2, GetSpawnPoint("alien_vent_north"));
        SpawnAliens("Warrior", 2, GetSpawnPoint("corridor_east"));
        bSpawnedAt50 = true;
    }
    // ... etc
}
```

### Behavior Mode 3: Hybrid (Recommended for Invasion Mode)
Combines both behaviors:
- At 75%/50%: Spawn alien waves
- At 25%: Spawn elite wave + trigger hazard after 5s delay
- At 0%: Spawn boss wave + extended hazard (20s)

---

## Implementation by Engine

### Unreal Engine (C++)

#### Header: `AlienBase_EggObjective.h`
```cpp
#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "AlienBase_EggObjective.generated.h"

UENUM(BlueprintType)
enum class EEggVisualStage : uint8
{
    Intact,
    Cracking,
    Pulsing,
    Critical
};

UCLASS()
class AAlienBase_EggObjective : public AActor
{
    GENERATED_BODY()

public:
    AAlienBase_EggObjective();
    virtual void Tick(float DeltaSeconds) override;

    UFUNCTION(BlueprintCallable, Category = "AlienBase|Egg")
    void ApplyDamage(float DamageAmount, AController* InstigatedBy, AActor* DamageCauser);

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "AlienBase|Egg")
    float MaxHealth;

    UPROPERTY(VisibleAnywhere, BlueprintReadWrite, Category = "AlienBase|Egg")
    float CurrentHealth;

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "AlienBase|Egg")
    EEggVisualStage CurrentStage;

    UFUNCTION()
    void OnHealthChanged(float OldHealth, float NewHealth);

    UFUNCTION()
    void OnStageChanged(EEggVisualStage OldStage, EEggVisualStage NewStage);

protected:
    virtual void BeginPlay() override;

private:
    void UpdateVisualStage();
    void TriggerHazardEvent();
    void SpawnAlienWave(FName WaveId);

    bool bSpawnedAt75;
    bool bSpawnedAt50;
    bool bSpawnedAt25;
    bool bDestroyed;
};
```

#### Implementation: `AlienBase_EggObjective.cpp`
```cpp
#include "AlienBase_EggObjective.h"
#include "AlienBase_AirlockController.h"
#include "Kismet/GameplayStatics.h"

AAlienBase_EggObjective::AAlienBase_EggObjective()
{
    PrimaryActorTick.bCanEverTick = true;
    MaxHealth = 500.0f;
    CurrentHealth = MaxHealth;
    CurrentStage = EEggVisualStage::Intact;
    bSpawnedAt75 = false;
    bSpawnedAt50 = false;
    bSpawnedAt25 = false;
    bDestroyed = false;
}

void AAlienBase_EggObjective::BeginPlay()
{
    Super::BeginPlay();
}

void AAlienBase_EggObjective::Tick(float DeltaSeconds)
{
    Super::Tick(DeltaSeconds);
    // Optional: Passive regeneration when not in combat
}

void AAlienBase_EggObjective::ApplyDamage(float DamageAmount, AController* InstigatedBy, AActor* DamageCauser)
{
    if (bDestroyed) return;

    float OldHealth = CurrentHealth;
    CurrentHealth = FMath::Clamp(CurrentHealth - DamageAmount, 0.0f, MaxHealth);

    OnHealthChanged(OldHealth, CurrentHealth);

    if (CurrentHealth <= 0.0f && !bDestroyed)
    {
        bDestroyed = true;
        // Trigger destruction sequence
        TriggerHazardEvent();
        SpawnAlienWave("Destruction");
    }
}

void AAlienBase_EggObjective::OnHealthChanged(float OldHealth, float NewHealth)
{
    UpdateVisualStage();

    // Check thresholds
    if (OldHealth > 75.0f && NewHealth <= 75.0f && !bSpawnedAt75)
    {
        SpawnAlienWave("Threshold75");
        bSpawnedAt75 = true;
    }
    if (OldHealth > 50.0f && NewHealth <= 50.0f && !bSpawnedAt50)
    {
        SpawnAlienWave("Threshold50");
        bSpawnedAt50 = true;
    }
    if (OldHealth > 25.0f && NewHealth <= 25.0f && !bSpawnedAt25)
    {
        SpawnAlienWave("Threshold25");
        // Delayed hazard trigger
        GetWorld()->GetTimerManager().SetTimerForNextTick(this, &AAlienBase_EggObjective::TriggerHazardEvent);
        bSpawnedAt25 = true;
    }
}

void AAlienBase_EggObjective::UpdateVisualStage()
{
    EEggVisualStage NewStage = EEggVisualStage::Intact;
    float HealthPercent = (CurrentHealth / MaxHealth) * 100.0f;

    if (HealthPercent <= 25.0f) NewStage = EEggVisualStage::Critical;
    else if (HealthPercent <= 50.0f) NewStage = EEggVisualStage::Pulsing;
    else if (HealthPercent <= 75.0f) NewStage = EEggVisualStage::Cracking;

    if (NewStage != CurrentStage)
    {
        EEggVisualStage OldStage = CurrentStage;
        CurrentStage = NewStage;
        OnStageChanged(OldStage, NewStage);
    }
}

void AAlienBase_EggObjective::TriggerHazardEvent()
{
    // Find airlock controller in level
    TArray<AActor*> FoundControllers;
    UGameplayStatics::GetAllActorsOfClass(GetWorld(), AAlienBase_AirlockController::StaticClass(), FoundControllers);
    
    if (FoundControllers.Num() > 0)
    {
        AAlienBase_AirlockController* Controller = Cast<AAlienBase_AirlockController>(FoundControllers[0]);
        if (Controller)
        {
            // Trigger with instigator team = -1 (environmental)
            Controller->RequestTriggerActivation("EggDestruction", -1);
        }
    }
}

void AAlienBase_EggObjective::SpawnAlienWave(FName WaveId)
{
    // Delegate to alien spawner system
    // Implementation depends on your AI spawning framework
}
```

### Unity (C#)

```csharp
using UnityEngine;
using Uncut.Multiplayer.Systems;

public class AlienBaseEggObjective : MonoBehaviour
{
    [Header("Health")]
    public float maxHealth = 500f;
    public float currentHealth;
    
    [Header("Visual Stages")]
    public GameObject[] stageVisuals; // Array of 4 child objects
    public ParticleSystem[] stageVFX;
    public AudioClip[] stageAudio;
    
    [Header("Spawn Configuration")]
    public GameObject alienDronePrefab;
    public GameObject alienWarriorPrefab;
    public GameObject alienElitePrefab;
    public Transform[] spawnPoints;
    
    public enum EggStage { Intact, Cracking, Pulsing, Critical }
    public EggStage currentStage = EggStage.Intact;
    
    private bool spawnedAt75;
    private bool spawnedAt50;
    private bool spawnedAt25;
    private bool destroyed;
    
    void Start()
    {
        currentHealth = maxHealth;
        UpdateVisualStage();
    }
    
    public void ApplyDamage(float damage, Controller instigator, GameObject damageSource)
    {
        if (destroyed) return;
        
        float oldHealth = currentHealth;
        currentHealth = Mathf.Clamp(currentHealth - damage, 0f, maxHealth);
        
        OnHealthChanged(oldHealth, currentHealth);
        
        if (currentHealth <= 0f && !destroyed)
        {
            destroyed = true;
            TriggerHazardEvent();
            SpawnAlienWave("Destruction");
        }
    }
    
    void OnHealthChanged(float oldHealth, float newHealth)
    {
        UpdateVisualStage();
        
        float oldPercent = (oldHealth / maxHealth) * 100f;
        float newPercent = (newHealth / maxHealth) * 100f;
        
        if (oldPercent > 75f && newPercent <= 75f && !spawnedAt75)
        {
            SpawnAlienWave("Threshold75");
            spawnedAt75 = true;
        }
        if (oldPercent > 50f && newPercent <= 50f && !spawnedAt50)
        {
            SpawnAlienWave("Threshold50");
            spawnedAt50 = true;
        }
        if (oldPercent > 25f && newPercent <= 25f && !spawnedAt25)
        {
            SpawnAlienWave("Threshold25");
            Invoke(nameof(TriggerHazardEvent), 3f); // 3s delay
            spawnedAt25 = true;
        }
    }
    
    void UpdateVisualStage()
    {
        EggStage newStage = EggStage.Intact;
        float percent = (currentHealth / maxHealth) * 100f;
        
        if (percent <= 25f) newStage = EggStage.Critical;
        else if (percent <= 50f) newStage = EggStage.Pulsing;
        else if (percent <= 75f) newStage = EggStage.Cracking;
        
        if (newStage != currentStage)
        {
            currentStage = newStage;
            // Activate corresponding visual child
            for (int i = 0; i < stageVisuals.Length; i++)
            {
                stageVisuals[i].SetActive(i == (int)currentStage);
            }
            // Play VFX and audio
            if (stageVFX[(int)currentStage] != null)
                stageVFX[(int)currentStage].Play();
            if (stageAudio[(int)currentStage] != null)
                AudioSource.PlayClipAtPoint(stageAudio[(int)currentStage], transform.position);
        }
    }
    
    void TriggerHazardEvent()
    {
        var controller = FindObjectOfType<AlienBaseAirlockController>();
        if (controller != null)
        {
            controller.RequestTriggerActivation("EggDestruction", -1);
        }
    }
    
    void SpawnAlienWave(string waveId)
    {
        // Implement alien spawning logic
        switch (waveId)
        {
            case "Threshold75":
                SpawnAlien(alienDronePrefab, 2, spawnPoints[0]);
                break;
            case "Threshold50":
                SpawnAlien(alienDronePrefab, 2, spawnPoints[0]);
                SpawnAlien(alienWarriorPrefab, 2, spawnPoints[1]);
                break;
            // ... etc
        }
    }
    
    void SpawnAlien(GameObject prefab, int count, Transform spawnPoint)
    {
        for (int i = 0; i < count; i++)
        {
            Instantiate(prefab, spawnPoint.position, Quaternion.identity);
        }
    }
}
```

### Godot (GDScript)

```gdscript
extends Area3D

class_name AlienBaseEggObjective

enum EggStage { INTACT, CRACKING, PULSING, CRITICAL }

@export var max_health: float = 500.0
@export var current_health: float = 500.0
@export var stage_visuals: Array[Node3D]
@export var stage_vfx: Array[GPUParticles3D]
@export var stage_audio: Array[AudioStream]

var current_stage: EggStage = EggStage.INTACT
var spawned_at_75: bool = false
var spawned_at_50: bool = false
var spawned_at_25: bool = false
var destroyed: bool = false

signal health_changed(old_health: float, new_health: float)
signal stage_changed(old_stage: EggStage, new_stage: EggStage)

func _ready() -> void:
    update_visual_stage()

func apply_damage(damage: float, instigator: Node, damage_source: Node) -> void:
    if destroyed:
        return
    
    var old_health := current_health
    current_health = clamp(current_health - damage, 0.0, max_health)
    
    health_changed.emit(old_health, current_health)
    on_health_changed(old_health, current_health)
    
    if current_health <= 0.0 and not destroyed:
        destroyed = true
        trigger_hazard_event()
        spawn_alien_wave("Destruction")

func on_health_changed(old_health: float, new_health: float) -> void:
    update_visual_stage()
    
    var old_percent := (old_health / max_health) * 100.0
    var new_percent := (new_health / max_health) * 100.0
    
    if old_percent > 75.0 and new_percent <= 75.0 and not spawned_at_75:
        spawn_alien_wave("Threshold75")
        spawned_at_75 = true
    
    if old_percent > 50.0 and new_percent <= 50.0 and not spawned_at_50:
        spawn_alien_wave("Threshold50")
        spawned_at_50 = true
    
    if old_percent > 25.0 and new_percent <= 25.0 and not spawned_at_25:
        spawn_alien_wave("Threshold25")
        get_tree().create_timer(3.0).timeout.connect(trigger_hazard_event)
        spawned_at_25 = true

func update_visual_stage() -> void:
    var new_stage: EggStage = EggStage.INTACT
    var percent := (current_health / max_health) * 100.0
    
    if percent <= 25.0:
        new_stage = EggStage.CRITICAL
    elif percent <= 50.0:
        new_stage = EggStage.PULSING
    elif percent <= 75.0:
        new_stage = EggStage.CRACKING
    
    if new_stage != current_stage:
        var old_stage := current_stage
        current_stage = new_stage
        stage_changed.emit(old_stage, new_stage)
        
        # Update visuals
        for i in range(stage_visuals.size()):
            stage_visuals[i].visible = (i == int(current_stage))
        
        # Play VFX
        if stage_vfx[int(current_stage)]:
            stage_vfx[int(current_stage)].restart()
        
        # Play audio
        if stage_audio[int(current_stage)]:
            var audio_player := AudioStreamPlayer3D.new()
            audio_player.stream = stage_audio[int(current_stage)]
            add_child(audio_player)
            audio_player.play()
            audio_player.queue_free()

func trigger_hazard_event() -> void:
    var controller = get_node_or_null("/root/Main/AlienBaseAirlockController")
    if controller and controller.has_method("request_trigger_activation"):
        controller.request_trigger_activation("EggDestruction", -1)

func spawn_alien_wave(wave_id: String) -> void:
    match wave_id:
        "Threshold75":
            spawn_alien("Drone", 2, get_spawn_point("alien_vent_north"))
        "Threshold50":
            spawn_alien("Drone", 2, get_spawn_point("alien_vent_north"))
            spawn_alien("Warrior", 2, get_spawn_point("corridor_east"))
        "Threshold25":
            spawn_alien("Warrior", 3, get_spawn_point("hub_center"))
            spawn_alien("Elite", 3, get_spawn_point("corridor_west"))
        "Destruction":
            spawn_alien("Elite", 4, get_spawn_point("hub_center"))
            spawn_alien("Queen", 1, get_spawn_point("egg_position"))

func spawn_alien(alien_type: String, count: int, spawn_point: Vector3) -> void:
    # Delegate to alien spawner
    pass

func get_spawn_point(point_name: String) -> Vector3:
    # Return spawn point coordinates
    return Vector3.ZERO
```

---

## Network Replication

### Key Properties to Replicate
- `CurrentHealth` (float, reliable)
- `CurrentStage` (enum, reliable)
- `bDestroyed` (bool, reliable)
- Spawn wave triggers (RPC, unreliable but sequenced)

### RPC Events
```cpp
// Unreal
UFUNCTION(Reliable, Server)
void Server_ApplyDamage(float DamageAmount, AController* Instigator);

UFUNCTION(Reliable, NetMulticast)
void Multicast_StageChanged(EEggVisualStage NewStage);

UFUNCTION(Reliable, NetMulticast)
void Multicast_SpawnWave(FName WaveId);
```

---

## Testing Checklist

- [ ] Egg takes correct damage from all weapon types
- [ ] Visual stages transition at correct health thresholds
- [ ] Audio cues play on stage transitions
- [ ] Alien waves spawn at correct thresholds with proper delays
- [ ] Hazard event triggers at 25% (delayed) and 0% (immediate)
- [ ] Network replication works for all clients
- [ ] Egg cannot be damaged during execution animations (ASID check)
- [ ] Destruction sequence completes even if player dies mid-damage

---

## References
- Grid: `data/alien_base_hub_grid_v1.json`
- Triggers: `docs/multiplayer/04_Multiplayer_Alien_Base_Triggers.md`
- ASID System: `Unity/Assets/Scripts/Systems/ASID/ASIDHelpers.cs`
