# Animation State Registry (ASID)

This file is the canonical registry for all animation state IDs (ASIDs) used by characters, NPCs, and hazards. It describes interrupt priority, lock behavior, gore timing, and associated SFX/VFX so all three engines can share the same rules.

Engine implementations (Unreal, Unity, Godot) must treat this document as the source of truth for ASID behavior and keep their enum/int tables in sync.

---

## 1. ASID Properties and Semantics

Each ASID entry defines:

- **ASID** – Unique integer or symbolic ID.
- **Name** – Short descriptive label.
- **Category** – Locomotion, Combat, Execution, Stun, Carry, etc.
- **Interrupt Priority** – Higher values override lower ones on request.
- **Lock Type**  
  - `None` – Normal control.  
  - `SoftLock` – Movement allowed, aim or turn speed reduced.  
  - `HardLock` – No input accepted except camera; no movement, jump, or attack.  
- **Cancels Into** – Allowed follow‑up ASIDs or “Any Locomotion”.
- **Gore Trigger Frame** – Normalized point in the animation (0.0–1.0) where gore and critical damage fire.
- **SFX Tags** – One or more sound effect IDs.
- **VFX Tags** – One or more visual effect IDs.

---

## 2. Core Player States

### 2.1 Locomotion and Idle

| ASID  | Name           | Category   | Priority | Lock Type | Cancels Into               | Gore Frame | SFX Tags | VFX Tags |
|------:|----------------|-----------|---------:|-----------|----------------------------|-----------:|----------|----------|
| 000   | Idle           | Locomotion| 0        | None      | Any Locomotion, Combat     | N/A        | SFX_000_IDLE_LOOP | VFX_NONE |
| 001   | Walk           | Locomotion| 0        | None      | Any Locomotion, Combat     | N/A        | SFX_001_FOOTSTEP | VFX_NONE |
| 002   | Run            | Locomotion| 0        | None      | Any Locomotion, Combat     | N/A        | SFX_002_RUN_STEP | VFX_NONE |
| 003   | Jump           | Locomotion| 1        | None      | Fall, Land, Stun           | N/A        | SFX_010_JUMP | VFX_NONE |

### 2.2 Stun and Hit‑Reacts

| ASID  | Name         | Category | Priority | Lock Type | Cancels Into           | Gore Frame | SFX Tags       | VFX Tags   |
|------:|--------------|----------|---------:|-----------|------------------------|-----------:|----------------|------------|
| 010   | Light Hit    | Stun     | 2        | SoftLock  | 000, 001, 002 after end| N/A        | SFX_020_HIT_LT | VFX_020_SPARK_LT |
| 012   | Stun Lock    | Stun     | 3        | SoftLock  | 000, 001, 002 after end| N/A        | SFX_021_HIT_STUN | VFX_021_STUN_RING |

Stun Lock (ASID_012) specifics are defined in section 5.

### 2.3 Heavy Carry

| ASID  | Name        | Category | Priority | Lock Type | Cancels Into                       | Gore Frame | SFX Tags          | VFX Tags         |
|------:|-------------|----------|---------:|-----------|------------------------------------|-----------:|-------------------|------------------|
| 050   | Heavy Carry | Carry    | 2        | SoftLock  | 000–002 when carry removed         | N/A        | SFX_030_HEAVY_BREATH | VFX_030_CARRY_TRAIL |

Heavy Carry overrides movement speed, disables jump, and blocks weapon use while active.

### 2.4 Executions (FIN_)

| ASID  | Name                   | Category   | Priority | Lock Type | Cancels Into | Gore Frame | SFX Tags          | VFX Tags           |
|------:|------------------------|------------|---------:|-----------|--------------|-----------:|-------------------|--------------------|
| 400   | FIN_RaptorPounceExec   | Execution  | 10       | HardLock  | None         | 0.45       | SFX_100_BONE_CRACK | VFX_010_GORE_SLASH |
| 405   | FIN_CavemanFeedExec    | Execution  | 10       | HardLock  | None         | 0.60       | SFX_101_CHOMP     | VFX_015_GORE_EAT   |
| 666   | FIN_GreggScytheExec    | Execution  | 10       | HardLock  | None         | 0.50       | SFX_102_SCYTHE_SWING | VFX_011_GORE_SLASH2 |

All FIN_ executions are hard‑locked and are immune to environmental hazards while active.

---

## 3. Gore Manager Contracts

The Gore Manager uses this table as its contract for when to spawn gore.

For each ASID with a non‑`N/A` Gore Trigger Frame:

- **Bone Name** – e.g., `head`, `neck`, `spine_03`, `pelvis`.
- **Gib Object ID** – e.g., `OBJ_GORE_MUSH`, `OBJ_GORE_HEAD`, `OBJ_GORE_TORSO`.
- **VFX IDs** – primary and secondary FX like `VFX_010_GORE_SLASH`, `VFX_015_GORE_EXPLODE`.
- **Decals / Blood Splats** – optional decal profiles.

Example:

| ASID | Bone      | Gore Frame | Gib ID        | VFX Primary       | VFX Secondary       | Decal ID     |
|-----:|-----------|-----------:|---------------|-------------------|---------------------|--------------|
| 400  | neck      | 0.45       | OBJ_GORE_HEAD | VFX_010_GORE_SLASH| VFX_015_GORE_SPRAY  | DECAL_BLOOD1 |
| 405  | spine_03  | 0.60       | OBJ_GORE_TORSO| VFX_011_GORE_BITE | VFX_016_GORE_DRIP   | DECAL_BLOOD2 |

The engine implementations should expose a per‑ASID metadata table (via JSON or a generated Rust/JSON schema) mirroring this section, which the Gore Manager can look up at runtime.

---

## 4. ASID Tagging for Hazard and Damage Rules

Certain ASIDs carry flags used by hazards and special damage logic:

- `bypass_zombie_damage_profile: true`
  - Executions that can kill zombies regardless of hitzone/damage profile.

- `ignore_hazard_damage: true`
  - Executions and respawn grace ASIDs that should not take DoT from gas/acid.

- `is_heavy_carry: true`
  - States that apply heavy carry movement/jump/weapon restrictions.

These flags are stored centrally in a generated `data/systems/asid_metadata_v1.json` file so Damageable and HazardVolume implementations can query them without hard‑coding enums.

---

## 5. Stun Lock (ASID_012) – Design and Implementation Notes

**Role:** Provide a meaningful reaction to melee and heavy weapon hits that interrupts the target without creating frustrating, chain‑stun loops.

Recommended baseline:

- **Duration:** 1.5 seconds.
- **Movement:** Slightly slowed; directional input allowed but speed reduced (e.g., 60–70% of normal).
- **Turning:** Turn speed reduced significantly so the stunned player cannot instantly 180‑turn and return fire.
- **Input:** Shooting and jumping disabled for the first 1.0s, then gradually restored.
- **Immunity window:** After ASID_012 ends, apply a short stun immunity (e.g., 1.0s) before another Stun Lock can be applied.

Implementation guidance is provided in the systems‑level notes accompanying this registry.
