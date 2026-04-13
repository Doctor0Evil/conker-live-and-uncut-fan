# 03_Multiplayer_War.md
## Destination: conker-live-and-uncut-fan/Docs/GDD/03_Multiplayer_War.md

# Game Design Document: Conker: Live & Uncut — Multiplayer War Mode

> **Document ID:** GDD-003  
> **Version:** 0.1.0  
> **Status:** Draft — AI-Generated First Pass  
> **Last Updated:** 2026-04-12  
> **Author:** GAMEMODE.ai Codegen System

---

## 1. Mode Overview

**War** (also known as Blitzkrieg) is a fast-paced, objective-based team deathmatch mode where two teams compete to control strategic points on the map while eliminating opponents. The mode emphasizes aggressive combat, tactical positioning, and dynamic map control.

### 1.1 Core Loop
```
Round Start (15s)
├─ Teams spawn at base with initial loadout
├─ Control points activate across map

Combat Phase (5-7 min)
├─ Teams fight to capture/defend control points
├─ Eliminated players respawn after short delay
├─ Capturing points generates tickets for team

Round End Conditions
├─ Team reaches ticket threshold (e.g., 100 points)
├─ Time limit expires → team with most tickets wins
├─ One team eliminates all opponents (sudden death)

Post-Round
├─ Stats tracking, XP rewards, cosmetic unlocks
├─ Map rotation or rematch vote
```

### 1.2 Win Conditions
| Condition | Description | Tiebreaker |
|-----------|-------------|-----------|
| **Ticket Victory** | First team to reach 100 tickets | N/A |
| **Time Expiry** | Team with most tickets when timer ends | Head-to-head eliminations |
| **Team Wipe** | Eliminate all opponents in sudden death | Fastest elimination time |

### 1.3 Map Design Principles
- **Symmetrical Layout:** Balanced spawn points and control point distribution
- **Three-Lane Structure:** Left flank, center choke, right flank for tactical variety
- **Vertical Play:** Elevated positions for snipers, underground routes for flanking
- **Dynamic Events:** Moving cover, destructible bridges, timed power-ups

---

## 2. Gameplay Mechanics

### 2.1 Control Point System
Each control point has a capture progress bar and team ownership state:

```
Capture Progress(t+Δt) = Progress(t) + (Team_Presence × Capture_Rate - Enemy_Presence × Contest_Rate) × Δt

Where:
- Team_Presence = Number of allied players in capture radius
- Enemy_Presence = Number of enemy players in capture radius
- Capture_Rate = Base rate (e.g., 1.0% per second per player)
- Contest_Rate = Slows capture when enemies are present (e.g., 0.5×)
```

**Point States:**
- `Neutral` → Can be captured by either team
- `Contested` → Both teams present; capture paused
- `Owned` → Generates tickets for owning team; can be recaptured

### 2.2 Class System (Optional Loadout Choice)
Players select a class before spawning, each with unique abilities:

| Class | Primary Weapon | Special Ability | Role |
|-------|---------------|----------------|------|
| **Assault** | Assault Rifle | Grenade Launcher (AoE damage) | Frontline push |
| **Medic** | SMG | Heal Beam (restore ally health) | Support/sustain |
| **Engineer** | Shotgun | Deployable Turret (auto-targets enemies) | Area denial |
| **Recon** | Sniper Rifle | Spotter Drone (reveals enemies in radius) | Intel/flanking |

*Implementation Note: Class balance tuned via Lua-configurable damage/health/cooldown values.*

### 2.3 Ticket Generation Math
Tickets are generated based on control point ownership and bonus actions:

```
Tickets_Per_Second = Σ(Owned_Points × Base_Rate × Multipliers)

Where:
- Base_Rate = 0.5 tickets/sec per point
- Multipliers:
  - All points owned: ×2.0 (domination bonus)
  - Holding point for >60s uninterrupted: ×1.2 (stability bonus)
  - Eliminating enemy near owned point: +5 tickets (defense bonus)
```

*All calculations deterministic and server-authoritative for netcode integrity.*

### 2.4 Respawn System
- **Base Spawn:** Safe area with brief invulnerability (3s)
- **Forward Spawn:** Unlocked when team controls adjacent point; higher risk/reward
- **Sudden Death:** No respawns after final round trigger

---

## 3. Technical Implementation

### 3.1 UE5 Class Structure
```cpp
// Engine/Unreal/Source/Public/Multiplayer/War/CLUWarGameMode.h

UCLASS()
class CONKERLIVEUNCUT_API ACLUWarGameMode : public AGameModeBase
{
    GENERATED_BODY()
    
public:
    // Round management
    UPROPERTY(EditDefaultsOnly, Category = "War")
    float RoundDurationSeconds;
    UPROPERTY(EditDefaultsOnly, Category = "War")
    int32 VictoryTicketThreshold;
    
    // Control point system
    UFUNCTION(BlueprintCallable, Category = "Points")
    void RegisterControlPoint(FWarControlPointDef Def);
    
    // Class/loadout system
    UFUNCTION(Server, Reliable)
    void Server_SelectClass(APlayerController* PC, EWarClass Class);
    
    // Ticket replication
    UPROPERTY(ReplicatedUsing = OnRep_Tickets)
    int32 TeamATickets;
    UPROPERTY(ReplicatedUsing = OnRep_Tickets)
    int32 TeamBTickets;
    
private:
    UFUNCTION()
    void OnRep_Tickets();
};
```

### 3.2 Knowledge Graph Registration
```json
{
  "id": "systems.conker.multiplayer.war",
  "type": "GameMode",
  "tags": ["Multiplayer", "TeamDeathmatch", "ObjectiveControl", "ClassBased"],
  "path": "Engine/Unreal/Source/Public/Multiplayer/War/CLUWarGameMode.h",
  "dependencies": [
    "systems.conker.core.combat",
    "systems.conker.core.spawn",
    "systems.conker.core.networking"
  ],
  "ai_generation_rules": {
    "determinism_required": true,
    "replication_strategy": "server_authoritative",
    "tuning_via_lua": true
  }
}
```

### 3.3 Lua Scripting Interface
```lua
-- scripts/war/defaults.lua
War.Config {
  base_capture_rate = 1.0,        -- % per second per player
  contest_slow_factor = 0.5,      -- Capture slowed by this factor when contested
  domination_bonus_multiplier = 2.0,
  stability_bonus_threshold_seconds = 60,
  defense_elimination_bonus = 5
}

War.RegisterClass {
  id = "assault",
  primary_weapon = "assault_rifle_mk2",
  special_ability = {
    type = "grenade_launcher",
    damage = 75,
    radius = 3.5,
    cooldown_seconds = 30
  },
  health = 120,
  speed_multiplier = 1.0
}
```

---

## 4. Balance & Progression

### 4.1 Scoring & Rewards
| Action | XP Reward | Notes |
|--------|----------|-------|
| Capture control point | +25 XP | Bonus for first capture of round |
| Eliminate opponent | +15 XP | +5 XP for headshot |
| Assist elimination | +8 XP | Damage contribution >30% |
| Win round | +50 XP | Team-based bonus |
| MVP of round | +25 XP | Highest score in winning team |

### 4.2 Weapon Balance Framework
Weapons tuned via Lua-configurable parameters:
```lua
Weapons.Define {
  id = "assault_rifle_mk2",
  damage_per_shot = 22,
  fire_rate_hz = 8.0,
  mag_size = 30,
  reload_time_s = 2.1,
  spread_pattern = "conical",
  spread_base_degrees = 2.5,
  recoil_vertical = 0.8,
  recoil_horizontal = 0.3
}
```

**Balance Validation:**
- CI job runs damage-per-second (DPS) calculations across all weapons
- Ensures no weapon dominates all engagement distances
- Flags outliers for human review

### 4.3 Map Rotation & Voting
- **Default Rotation:** 3-map cycle to prevent fatigue
- **Player Voting:** Post-round vote for next map (majority wins)
- **Skill-Based Matching:** New players see simpler maps first

---

## 5. Testing & Validation

### 5.1 Determinism Checks
- Verify ticket calculations produce identical results across client/server simulations
- Test control point capture math under edge cases (simultaneous capture/contest)

### 5.2 Netcode Stress Tests
- Simulate packet loss (5-10%) → ensure ticket replication remains consistent
- Validate respawn logic works correctly under high-latency conditions

### 5.3 AI-Chat Generation Prompts
Example prompt:
```
Generate a full implementation for 
Engine/Unreal/Source/Private/Multiplayer/War/CLUWarGameMode.cpp 
that:
- Implements the control point capture formula from GDD-003
- Supports class selection with Lua-configurable abilities
- Includes ticket replication with server-authoritative validation
- Handles round end conditions (ticket victory, time expiry, team wipe)
Reference the public header at 
Engine/Unreal/Source/Public/Multiplayer/War/CLUWarGameMode.h
```

---

## 6. Next Objectives

### 6.1 Immediate AI-Chat Tasks
1. Generate `Docs/GDD/04_Multiplayer_Alien_Base.md` using this document's structure
2. Create initial `CLUWarGameMode.cpp` implementation stub with control point system
3. Add War mode entries to `knowledgegraphsystems.json` via repo_index_generator

### 6.2 Human Review Checklist
- [ ] Confirm class balance feels fair across engagement distances
- [ ] Validate control point capture math produces intuitive team play
- [ ] Approve weapon tuning interface before community exposure
- [ ] Sign off on map rotation logic for public matchmaking

---

*This document is part of the Conker: Live & Uncut fan project. All content is fan-created and non-commercial. Conker and related properties are trademarks of their respective owners.*
