# Fortress Map Implementation Summary (Objectives 29-36)

## Completion Status: ✅ COMPLETE

This document summarizes the implementation of objectives 29-36 for the 03_Multiplayer_Fortress map.

---

## Research Question 29: Gas Canister in 16-Player Environment ✅

**File:** `/workspace/docs/research/N64_Total_War_Gas_Canister.md`

### Findings
- **Original N64 Mechanic**: Instant-win via gas canister delivery to enemy base
- **Trigger**: Pickup-based, carrier delivers to drop-off point
- **Countdown**: 3-second warning, instant lethal effect, 10-15s duration
- **Area of Effect**: Entire enemy base (~8x8m), instant death to all inside

### Recommendation for 16-Player
**HYBRID APPROACH** - Adapted from instant-win to area denial + bonus points:
- Heavy Carry penalty (50% movement speed)
- 5-second warning before gas activates
- 15 DPS for 20 seconds (lethal in ~7s, not instant)
- +50 ticket bonus on delivery
- 60-second cooldown before respawn

**Rationale**: Reduces frustration while preserving tactical depth and N64 spirit.

---

## Objective 30: Fortress Tile Palette ✅

**File:** `/workspace/docs/multiplayer/03_Multiplayer_Fortress_Tile_Palette.md` *(Pre-existing)*

### Defined Tiles
- **Floors**: IndustrialPlain, IndustrialHazardStripe, MetalGrate, TrenchMud
- **Walls**: ConcreteBunker, TowerPanel, TowerWindow
- **Ceilings**: TunnelRibs, GratedCatwalk
- **Special**: CaptureZone, SpawnZone

All tiles use 4x4 unit world scale for consistency with Alien Base and Beach maps.

---

## Objective 31: Fortress Main Grid Creation ✅

**File:** `/workspace/maps/fortress/fortress_main_grid_v1.json`

### Grid Specifications
- **Dimensions**: 32 columns × 24 rows
- **Cell Size**: 4.0 units
- **Total Cells**: 768 (84 defined in grid)

### Key Areas Defined
| Area | Role Tags | Location |
|------|-----------|----------|
| SHC Base | `shc_base`, `team_shc_spawn` | Cols 2-5, Rows 2-5 |
| Tediz Base | `tediz_base`, `team_tediz_spawn` | Cols 26-29, Rows 2-5 |
| Central Bridge | `bridge`, `central_crossing` | Cols 12-15, Rows 6-8 |
| Left Trench | `trench`, `left_flank` | Cols 2-5, Rows 6-8 |
| Right Trench | `trench`, `right_flank` | Cols 26-29, Rows 6-8 |
| Underground Tunnel | `underground_tunnel` | Row 16, Cols 10-22 |

### Capture Points
- **Tower Alpha** (Left): `capture_point`, `tower_alpha` at (8,6)
- **Central Bridge** (Primary): `capture_point`, `central_bridge` at (13,7)
- **Tower Beta** (Right): `capture_point`, `tower_beta` at (22,6)

### Heavy Weapon Spawns
- **Bazooka Left**: Exposed position at (6,12)
- **Bazooka Right**: Exposed position at (25,12)
- **Chaingun Catwalk**: High-risk exposed position at (14,14)

### Gas Canister System
- **Spawn**: Central valley at (14,10)
- **Delivery Zones**: 
  - SHC Base interior trigger at (3,3)
  - Tediz Base interior trigger at (27,3)

---

## Objective 32: Capture Point Logic Design ✅

**File:** `/workspace/docs/design/Fortress_Capture_Point_Logic.md`

### Recommended System: Hybrid Multi-Point with Tug-of-War

#### Three Control Points
1. **Tower Alpha** - 0.5 tickets/sec, medium strategic value
2. **Central Bridge** - 1.0 tickets/sec, HIGH strategic value (primary)
3. **Tower Beta** - 0.5 tickets/sec, medium strategic value

#### Ticket Generation Formula
```
Tickets_Per_Second = Σ(Owned_Points × Point_Value) + Dominance_Bonus

Dominance_Bonus = (Leading_Points - Trailing_Points) × 0.25

Examples:
- 3 vs 0: 2.0 + 0.75 = 2.75 tickets/sec
- 2 vs 1: 1.5 + 0.25 = 1.75 tickets/sec
- 1 vs 1: 0.5 tickets/sec each (no bonus)
```

#### Capture Mechanics
- **Capture Rate**: 1.0% per second per player
- **Contest Slow**: 0.5× when enemies present
- **Radius**: 6.0 meters (~1.5 grid cells)
- **Victory Threshold**: 100 tickets
- **Estimated Match Duration**: 5-7 minutes

#### Strategic Layers
1. Point priority decisions (bridge vs towers)
2. Timing & rotation (early/mid/late game)
3. Counter-play (split vs concentrate)
4. Gas canister synergy (area denial → capture window)

---

## Objective 33: Heavy Weapon Placement ✅

**Defined in:** `/workspace/maps/fortress/fortress_main_grid_v1.json`

### Placement Philosophy
Heavy weapons spawn in **high-risk, exposed positions** requiring skill to reach and hold:

| Weapon | Location | Risk Factors | Respawn |
|--------|----------|--------------|---------|
| Bazooka (Left) | (6,12) | Open ground, visible from bridge | 45s |
| Bazooka (Right) | (25,12) | Open ground, visible from bridge | 45s |
| Chaingun | (14,14) | Catwalk exposure, no cover | 60s |

### Ammo Configuration
- **Bazooka**: 3 rockets per pickup
- **Chaingun**: 200 rounds per pickup

---

## Objective 34: Gas Chamber Trigger Design ✅

**Defined in:** `/workspace/maps/fortress/fortress_main_grid_v1.json`

### Entity: `trigger_gas_chamber`

Two delivery zone triggers defined:

#### SHC Base Delivery Zone
```json
{
  "entity_id": "trigger_gas_delivery_shc",
  "entity_type": "trigger_gas_chamber",
  "col": 3,
  "row": 3,
  "roletags": ["gas_delivery_zone", "shc_base_interior"],
  "properties": {
    "warning_time_seconds": 5.0,
    "gas_duration_seconds": 20.0,
    "damage_per_second": 15.0,
    "interaction_prompt": "Arm Gas Canister at SHC Base?"
  }
}
```

#### Tediz Base Delivery Zone
```json
{
  "entity_id": "trigger_gas_delivery_tediz",
  "entity_type": "trigger_gas_chamber",
  "col": 27,
  "row": 3,
  "roletags": ["gas_delivery_zone", "tediz_base_interior"],
  "properties": {
    "warning_time_seconds": 5.0,
    "gas_duration_seconds": 20.0,
    "damage_per_second": 15.0,
    "interaction_prompt": "Arm Gas Canister at Tediz Base?"
  }
}
```

### Interaction Flow
1. Carrier enters delivery zone with gas canister
2. Prompt appears: "Arm Gas Canister at [Team] Base?"
3. Player interacts → 5-second warning countdown begins
4. Gas activates → 20 seconds of 15 DPS damage to all inside
5. +50 tickets awarded to delivering team
6. Canister respawns after 60 seconds

---

## Coding Task 35: Gas Canister Implementation ✅

**Files Created:**
- `/workspace/src/gameplay/fortress/GasCanister.h` - Header with class definitions
- `/workspace/src/gameplay/fortress/GasCanister.cpp` - Implementation

### Class Structure

#### AGasCanister (Main Actor)
- **State Machine**: WaitingForPickup → BeingCarried → Arming → GasActive → Cooldown
- **Heavy Carry**: Applies 50% movement speed penalty to carrier
- **Replication**: Server-authoritative state with client-side visual feedback
- **Configurable Properties** (via Lua/DataTable):
  - `CarrySpeedMultiplier`: 0.5
  - `RespawnTimeSeconds`: 60.0
  - `DeliveryBonusTickets`: 50
  - `WarningTimeSeconds`: 5.0
  - `GasDurationSeconds`: 20.0
  - `DamagePerSecond`: 15.0

#### ADeliveryZone (Trigger Volume)
- Detects carrier entry
- Shows interaction prompt
- Triggers gas hazard volume on arm
- Replicates armed state to all clients

#### AGasHazardVolume (Damage Area)
- Spawns when canister is armed
- Applies 15 DPS every 0.5 seconds to overlapping actors
- Visual effects via Niagara particle system
- Auto-deactivates after 20 seconds

### Key Functions
```cpp
// Pickup/drop mechanics
void OnPickup(ACharacter* Picker);
void OnDrop(ACharacter* DroppedBy);
void Server_SetCarrier(ACharacter* NewCarrier);

// Arming sequence
void OnArmAtDeliveryPoint(ADeliveryZone* Zone);
void TriggerGasEffect(ADeliveryZone* TargetZone);

// Damage application
void ApplyGasDamage(AActor* DamagedActor);

// Lifecycle
void StartRespawnTimer();
void RespawnCanister();
```

### Network Replication
- `CurrentCarrier` - Replicated with notification
- `CurrentState` - Replicated with notification  
- `GasProgress` - Replicated for UI countdowns
- All state changes server-authoritative

---

## Integration with Existing Systems

### War Game Mode (GDD-003)
The gas canister integrates with the existing War mode ticket system:
```cpp
// In WarGameMode.h
UFUNCTION()
void OnGasCanisterArmed(ETeamAffiliation ArmedAgainstTeam, int32 BonusTickets);

// Awards tickets and forces enemy respawn disruption
```

### Map Grid System
Gas canister spawn and delivery zones defined in `fortress_main_grid_v1.json` entities array, following the same schema as `fortress_bunker_grid_v1.json`.

### Tile Palette
Uses tiles from `03_Multiplayer_Fortress_Tile_Palette.md`:
- `TFloorIndustrialHazardStripe` for canister spawn and delivery zones
- `TFloorTrenchMud` for trench routes
- `TFloorMetalGrate` for catwalk heavy weapon spawns

---

## Testing Checklist

### Functional Tests
- [ ] Gas canister pickup applies 50% speed penalty
- [ ] Canister drops on carrier death
- [ ] Delivery zone prompt only shows for carrier
- [ ] 5-second warning plays audio/visual cues
- [ ] Gas deals 15 DPS to all actors in volume
- [ ] +50 tickets awarded on successful arm
- [ ] Canister respawns after 60 seconds

### Network Tests
- [ ] State replicates correctly to all clients
- [ ] Carrier attachment visible on all clients
- [ ] Warning countdown synchronized
- [ ] Gas damage applied server-side

### Balance Tests
- [ ] Match duration ~5-7 minutes with gas mechanic
- [ ] Gas delivery not overpowered (counter-play exists)
- [ ] Heavy weapon spawns contested but not impossible to reach
- [ ] Three-point capture system feels fair

---

## Next Steps (Objectives 37+)

1. **Objective 37**: Implement War mode game logic (ticket tracking, round management)
2. **Objective 38**: Create UE5 level blockout using grid JSON
3. **Objective 39**: Design minimap UI overlay for capture points
4. **Objective 40**: Add Lua configuration interface for balance tuning

---

## Files Created/Modified

| File | Purpose | Status |
|------|---------|--------|
| `docs/research/N64_Total_War_Gas_Canister.md` | Research Q29 | ✅ Created |
| `maps/fortress/fortress_main_grid_v1.json` | Objective 31, 33, 34 | ✅ Created |
| `docs/design/Fortress_Capture_Point_Logic.md` | Objective 32 | ✅ Created |
| `src/gameplay/fortress/GasCanister.h` | Coding Task 36 | ✅ Created |
| `src/gameplay/fortress/GasCanister.cpp` | Coding Task 36 | ✅ Created |

**Pre-existing Reference Files:**
- `docs/multiplayer/03_Multiplayer_Fortress.md`
- `docs/multiplayer/03_Multiplayer_Fortress_Tile_Palette.md`
- `maps/fortress/fortress_bunker_grid_v1.json`
- `Docs/GDD/03_Multiplayer_War.md`

---

*Document generated as part of Conker: Live & Uncut fan project development.*
