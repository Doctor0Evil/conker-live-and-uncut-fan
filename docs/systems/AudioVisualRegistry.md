# Audio/Visual Registry

## Overview

The Audio/Visual Registry (`avregistryv1.json`) is the master schema for all sound effects (SFX), visual effects (VFX), particle decals (PRT), environmental hazards (ENV), HUD elements (HUD), and cinematic sequences (SEQ) in the Uncut Multiplayer project.

This registry is **engine-agnostic** and serves as the single source of truth for AI-Chat codegen, CI validation, and cross-platform asset mapping (UE5, Unity, Godot).

---

## ID Prefixes and Numeric Ranges

| Prefix | Range | Category | Description |
|--------|-------|----------|-------------|
| `SFX_` | 100–199 | Weapon | Weapon fire, reload, loop sounds |
| `SFX_` | 200–299 | Gore | Dismemberment, impact, splatter sounds |
| `SFX_` | 300–399 | Hazard | Environmental hazard triggers and loops |
| `SFX_` | 400–499 | UI | Menu, countdown, match event sounds |
| `SFX_` | 500–599 | Voice (SHC) | Soldier Human Coalition vocals |
| `SFX_` | 600–699 | Voice (Tediz) | Conk/Tediz faction vocals |
| `SFX_` | 700–799 | Voice (Alien) | Xenomorph-style alien vocals |
| `SFX_` | 800–899 | Voice (Raptors) | Dinosaur creature vocals |
| `SFX_` | 900–999 | Voice (Frenchies/Cavemen) | Remaining faction vocals |
| `VFX_` | 001–099 | Gore | Blood, dismemberment, limb detachment |
| `VFX_` | 100–199 | Weapon | Muzzle flashes, trails, shell casings |
| `VFX_` | 200–299 | Ambient | Steam, fog, environmental particles |
| `PRT_` | 100–199 | Decals | Blood pools, cut marks, scorch marks |
| `ENV_` | 050–099 | Hazards | Gas, electric, crushing hazards |
| `ENV_` | 300–399 | Objectives | Pulsing eggs, destructible objectives |
| `HUD_` | 001–009 | Reticles | Weapon crosshairs |
| `HUD_` | 010–019 | Counters | Health, armor, ammo displays |
| `SEQ_` | 001–099 | Cinematics | Match start/end, sudden death, instant-win events |

---

## Schema Validation

All IDs must conform to `schemas/systems/avregistryv1.schema.json`:

- **Pattern**: `^(SFX|VFX|PRT|ENV|HUD|SEQ)_[0-9]{3}$`
- **Categories**: `weapon`, `gore`, `ambient`, `ui`, `cinematic`, `misc`, `voice`, `hazard`
- **Required fields**: `id`, `internal_name`, `category`, `description`
- **Optional fields**: `usage_notes`, `n64_ancestor`, `decal_id` (VFX only), `material_tag` (PRT only)

---

## Usage in Game Code

### Example: Chainsaw Execution (ASID_401)

```cpp
// UE5 C++ example
void UGoreManager::ExecuteChainsawFinisher(AActor* Victim)
{
    // Play idle rev sound on start
    UAudioComponent::PlaySoundAtLocation(SFX_152_WPN_CHAINSAW_IDLE_REV);
    
    // At gore_frame 14 (defined in ASID registry):
    SpawnVFX(VFX_016_PRT_GORE_HEAVY_SLICE, Victim->GetActorLocation());
    SpawnVFX(VFX_012_PRT_GORE_LIME_SPURT, Victim->GetActorLocation());
    ApplyDecal(PRT_102_DECAL_DEEP_CUT_SLICE, Victim->GetMesh());
    UAudioComponent::PlaySoundAtLocation(SFX_153_WPN_CHAINSAW_GORE_IMPACT);
    
    // Spawn detached limb chunks
    SpawnGoreChunk(EGoreType::LimeGreen, Victim->GetMesh(), "head");
}
```

### Example: Material-Context Gore (Gore Manager Logic)

```rust
// Rust pseudo-code for Gore Manager
match material_tag {
    MAT_FLESH => {
        spawn_vfx(VfxId::Vfx012_PrtGoreLimeSpurt);
        apply_decal(PrtId::Prt101_DecalBloodPoolLime);
    }
    MAT_METAL_BASE => {
        // NO GORE on metal! Spawn sparks instead
        spawn_vfx(VfxId::Vfx020_SparkMetalImpact);
        // Skip PRT decals entirely
    }
    MAT_CONCRETE => {
        spawn_vfx(VfxId::Vfx018_PrtGoreBoneShard);
        apply_decal(PrtId::Prt103_DecalScorchMark);
    }
}
```

---

## Profanity Toggle Integration (RO-203)

Voice SFX entries support alternate "bleeped" variants when the profanity toggle is enabled:

| Original SFX | Bleeped Variant | Trigger |
|--------------|-----------------|---------|
| `SFX_502 VO_SHC_DEATH_01` | `SFX_502B VO_SHC_DEATH_01_BLEEP` | Profanity disabled |
| `SFX_503 VO_SHC_TAUNT_01` | `SFX_503B VO_SHC_TAUNT_01_BLEEP` | Profanity disabled |
| `SFX_668 VO_CONK_DEATH_01` | `SFX_668B VO_CONK_DEATH_01_BLEEP` | Profanity disabled |

Implementation note: The audio manager checks `UncutGameSettings.bEnableProfanity` before playing voice SFX. If `false`, append `B` suffix to internal name and load alternate asset.

---

## N64 Violence and Pacing Guidelines

**CRITICAL**: This project preserves the **N64 Bodycount Force / Uncut Games aesthetic**. Do NOT drift toward modern, floaty, bloodless design.

### Core Principles

1. **No Classes, No XP**: Pickups only. No perk icons, no XP bars, no class-based loadouts. HUD reflects this simplicity (`HUD_010`–`HUD_012` are numeric counters only).

2. **Pickups Only**: Weapons spawn as `OBJ_010`–`OBJ_020` pickups on map. No loadout screens.

3. **No Jump with Heavy Guns**: Movement ASIDs enforce this (`ASID_050 HEAVY_CARRY` locks jump; see `AnimationStateRegistry.md`).

4. **Hard-Lock Executions**: Finishers (`ASID_401`–`ASID_499`) use hard locks with interrupt priority. Player cannot cancel mid-animation.

5. **Gore is Mandatory**: Lime-green blood (`VFX_012`, `PRT_101`) is signature. Metal surfaces get sparks (`VFX_020`), NOT blood. Decal budget: max 200, FIFO eviction.

6. **Instant-Win Hazards**: Gas chamber (`SEQ_003`) kills immediately—no damage-over-time modernization. Preserves N64 trap lethality.

7. **Voice Authenticity**: SHC/Tediz/Alien vocals reference original BFD samples (`n64_ancestor` field). Do not replace with generic modern voice packs.

---

## Adding New IDs

1. **Choose prefix and next available number** in range (see table above).
2. **Add entry to `data/av/avregistryv1.json`** with all required fields.
3. **Update schema if adding new category enum value**.
4. **Run CI validation**: `cargo run --bin av_asid_validate` (Task G1).
5. **Wire into engine code** via `core_ids` Rust enums (Task D1).

---

## Cross-Reference Documents

- `AnimationStateRegistry.md` – ASID-to-SFX/VFX mappings
- `GoreManagerDesign.md` – Material-tag logic and decal budget
- `weaponstatsv1.json` – Weapon-to-OBJ/HUD/SFX/VFX links
- `objregistryv1.json` – Pickup/object definitions
- `HudRegistry.md` – UI element specifications

---

## Files

| File | Purpose |
|------|---------|
| `schemas/systems/avregistryv1.schema.json` | JSON Schema for validation |
| `data/av/avregistryv1.json` | Master data file (100+ tags) |
| `docs/systems/AudioVisualRegistry.md` | This documentation |
