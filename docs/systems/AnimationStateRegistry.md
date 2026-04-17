# Animation State Registry

## Overview

The Animation State Registry defines all animation states (ASID) with lock behavior, SFX/VFX triggers, and gore frame events. This extends **RO-201** and integrates with the **Audio/Visual Registry** (Task B1/B2).

---

## ASID Numeric Ranges

| Range | Category | Description |
|-------|----------|-------------|
| 001–099 | Core Movement | Walk, run, crouch, jump, land |
| 100–199 | Weapon Fire | Primary fire, alt-fire, reload |
| 200–299 | Hit Reactions | Flinch, stagger, knockback |
| 300–399 | Death Animations | Ragdoll trigger, death poses |
| 400–499 | Finishers/Executions | Hard-lock brutal animations |
| 450–479 | Advanced Movement | Crouch walk, slide, mantle |
| 500–599 | Idles/Emotes | Idle variants, taunts, special |
| 900–999 | Alien/Creature | Pounce, facebite, tailwhip |

---

## Seeded ASID Entries

### Finishers (Hard Lock, High Priority)

#### ASID_401 – FIN_CHAINSAW_H
- **Lock Type**: Hard
- **Interrupt Priority**: 9
- **Description**: Chainsaw horizontal execution; victim bisected at waist
- **sfx_on_start**: `["SFX_152"]` (chainsaw idle rev)
- **sfx_on_gore**: `["SFX_153"]` (chainsaw gore impact)
- **vfx_on_gore**: `["VFX_016", "VFX_012"]` (heavy slice + lime spurt)
- **gore_decal**: `PRT_102` (deep cut slice)
- **gore_frame**: 14
- **honors_n64_no_jump_heavy**: false (execution locks all movement)
- **n64_ancestor**: `BFD_ANIM_CHAINSAW_EXEC`
- **usage_notes**: Victim cannot cancel; spawns 2 GoreChunks (torso halves); decal persists 60s

#### ASID_406 – FIN_KATANA_STAB
- **Lock Type**: Hard
- **Interrupt Priority**: 8
- **Description**: Katana thrust through chest; single-target impalement
- **sfx_on_start**: `[]` (silent approach)
- **sfx_on_gore**: `["SFX_501"]` (victim pain vocal)
- **vfx_on_gore**: `["VFX_016"]` (heavy slice trail)
- **gore_decal**: `PRT_101` (small floor blood pool)
- **gore_frame**: 8
- **honors_n64_no_jump_heavy**: false
- **n64_ancestor**: null
- **usage_notes**: Cleaner than chainsaw; no dismemberment, just puncture

#### ASID_410 – FIN_NECK_SNAP
- **Lock Type**: Hard
- **Interrupt Priority**: 7
- **Description**: Rear takedown; cervical snap
- **sfx_on_start**: `[]`
- **sfx_on_gore**: `["SFX_502"]` (death crunch sound)
- **vfx_on_gore**: `[]` (no visible gore, internal injury)
- **gore_decal**: null
- **gore_frame**: 6
- **honors_n64_no_jump_heavy**: false
- **n64_ancestor**: `BFD_ANIM_NECK_SNAP`
- **usage_notes**: Silent kill; no blood unless head explodes (rare variant)

#### ASID_411 – FIN_ALIEN_TAILWHIP
- **Lock Type**: Hard
- **Interrupt Priority**: 8
- **Description**: Alien tail impalement through torso
- **sfx_on_start**: `["SFX_701"]` (alien screech)
- **sfx_on_gore**: `["SFX_702"]` (alien death as tail withdraws)
- **vfx_on_gore**: `["VFX_013", "VFX_012"]` (brain matter + lime spurt)
- **gore_decal**: `PRT_101`
- **gore_frame**: 10
- **honors_n64_no_jump_heavy**: false
- **n64_ancestor**: `BFD_ANIM_ALIEN_TAIL`
- **usage_notes**: Alien-specific finisher; victim lifted off ground

---

### Advanced Movement (Soft Lock or None)

#### ASID_450 – MOV_CROUCH_WALK
- **Lock Type**: None
- **Interrupt Priority**: 1
- **Description**: Slow, quiet crouch walk
- **sfx_on_start**: `[]`
- **vfx_on_gore**: null
- **gore_frame**: null
- **honors_n64_no_jump_heavy**: true (cannot jump while crouch-walking with heavy)
- **n64_ancestor**: `BFD_ANIM_CROUCH_WALK`
- **usage_notes**: Reduced footstep audio; used for stealth approach

#### ASID_451 – MOV_SLIDE
- **Lock Type**: Soft
- **Interrupt Priority**: 3
- **Description**: Knee slide from sprint
- **sfx_on_start**: `[]`
- **vfx_on_gore**: null
- **gore_frame**: null
- **honors_n64_no_jump_heavy**: true (slide cancels if holding heavy)
- **n64_ancestor**: null
- **usage_notes**: Can fire pistol during slide; chaingun/bazooka cancel slide

#### ASID_050 – HEAVY_CARRY
- **Lock Type**: None (passive state)
- **Interrupt Priority**: 0
- **Description**: Carrying heavy weapon (chaingun, bazooka)
- **sfx_on_start**: `[]`
- **vfx_on_gore**: null
- **gore_frame**: null
- **honors_n64_no_jump_heavy**: **true** (CRITICAL: disables jump entirely)
- **n64_ancestor**: `BFD_STATE_HEAVY_WEAPON`
- **usage_notes**: Movement speed reduced 20%; jump disabled; climb mantles slower

---

### Idles/Emotes

#### ASID_500 – IDLE_DRUNK
- **Lock Type**: None (interruptible)
- **Interrupt Priority**: 0
- **Description**: Swaying drunk idle (easter egg)
- **sfx_on_start**: `["SFX_503"]` (drunk burp vocal)
- **vfx_on_gore**: null
- **gore_frame**: null
- **honors_n64_no_jump_heavy**: false
- **n64_ancestor**: `BFD_ANIM_IDLE_DRUNK`
- **usage_notes**: Random 1% chance on spawn; purely cosmetic

#### ASID_501 – IDLE_PEE
- **Lock Type**: Soft (can cancel with fire/move)
- **Interrupt Priority**: 1
- **Description**: Urination idle (N64 easter egg revival)
- **sfx_on_start**: `[]` (stream sound optional)
- **vfx_on_gore**: null
- **gore_frame**: null
- **honors_n64_no_jump_heavy**: false
- **n64_ancestor**: `BFD_ANIM_IDLE_PEE`
- **usage_notes**: Triggers after 60s idle; creates puddle VFX (not in AV registry; engine-specific)

---

### Alien Creature Attacks

#### ASID_900 – ALIEN_POUNCE
- **Lock Type**: Hard (on victim)
- **Interrupt Priority**: 9
- **Description**: Alien leaps and pins victim
- **sfx_on_start**: `["SFX_701"]` (screech)
- **sfx_on_gore**: `["SFX_702", "SFX_502"]` (alien + victim death)
- **vfx_on_gore**: `["VFX_013"]` (brain matter from facebite)
- **gore_decal**: `PRT_101`
- **gore_frame**: 12
- **honors_n64_no_jump_heavy**: false
- **n64_ancestor**: `BFD_ANIM_ALIEN_POUNCE`
- **usage_notes**: Instant-kill if victim < 50 HP; otherwise QTE escape

#### ASID_901 – ALIEN_FACEBITE
- **Lock Type**: Hard
- **Interrupt Priority**: 10
- **Description**: Facehugger-style head bite
- **sfx_on_start**: `["SFX_701"]`
- **sfx_on_gore**: `["SFX_502"]` (neck crunch)
- **vfx_on_gore**: `["VFX_013", "VFX_012"]` (brain + lime)
- **gore_decal**: `PRT_101`
- **gore_frame**: 8
- **honors_n64_no_jump_heavy**: false
- **n64_ancestor**: `BFD_ANIM_FACEBITE`
- **usage_notes**: Head detaches as GoreChunk; body slumps

---

## N64 No-Jump with Heavy Guns Enforcement

Per **RO-203**, the following ASIDs enforce the classic rule:

| ASID | Name | Jump Behavior |
|------|------|---------------|
| `ASID_050` | HEAVY_CARRY | **Jump completely disabled** |
| `ASID_450` | MOV_CROUCH_WALK | Jump disabled if heavy equipped |
| `ASID_451` | MOV_SLIDE | Slide cancels if heavy equipped |
| `ASID_100–199` | Weapon Fire (Chaingun/Bazooka) | Jump allowed but accuracy penalty |

Implementation note: Check `UncutCharacter->GetCurrentASID()` and `UncutCharacter->GetEquippedWeapon()->IsHeavy()` before processing jump input.

---

## Cross-Reference Validation

CI validates (Task G1):

1. All `sfx_*` IDs exist in `avregistryv1.json`
2. All `vfx_*` IDs exist in `avregistryv1.json`
3. All `gore_decal` PRT IDs exist in `avregistryv1.json`
4. `gore_frame` is within animation length (engine-side check)

---

## Files

| File | Purpose |
|------|---------|
| `schemas/systems/asidregistryv1.schema.json` | JSON Schema |
| `data/systems/asidregistryv1.json` | Full ASID data (to be populated) |
| `docs/systems/AnimationStateRegistry.md` | This documentation |

---

## Example: Wiring ASID to Gore Manager

```cpp
// UE5: Animation notify for gore frame
void UAnimNotify_GoreEvent::Notify(USkeletalMeshComponent* MeshComp, UAnimSequenceBase* Animation)
{
    const FAnimationStateEntry& Entry = GetASIDEntry(MeshComp->GetOwner());
    
    // Spawn VFX
    for (const FString& VfxId : Entry.vfx_on_gore) {
        GoreManager->SpawnVFX(VfxId, MeshComp->GetComponentLocation());
    }
    
    // Apply decal
    if (!Entry.gore_decal.IsEmpty()) {
        GoreManager->ApplyDecal(Entry.gore_decal, MeshComp);
    }
    
    // Play SFX
    for (const FString& SfxId : Entry.sfx_on_gore) {
        UGameplayStatics::PlaySoundAtLocation(MeshComp->GetWorld(), SfxId, ...);
    }
    
    // Spawn GoreChunks (dismembered limbs)
    if (Entry.lock_type == "Hard" && Entry.interrupt_priority >= 8) {
        GoreManager->SpawnDismemberment(MeshComp->GetOwner(), Entry.gore_decal);
    }
}
```
