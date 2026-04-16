# Animation State Registry (ASID)

This document defines canonical Animation State Identifiers (ASIDs) for executions, heavy carry, stun, and other core states. All engines map their animation graphs to these IDs.

The heavy‑carry behavior (ASID050) is defined here so Unreal, Unity, and Godot adapters can be validated against the same rules.

---

## Format

Each ASID entry uses this format:

- **ASID:** Numeric code (e.g., `050`)
- **Code:** Symbolic identifier (e.g., `HLD_HEAVY_WALK`)
- **Role:** Short description of the state’s purpose.
- **Lock Type:** `HardLock` | `SoftLock` | `MovementMode`
- **Movement Rules:** How it affects movement, jump, and rotation.
- **Gore Trigger Frame:** For executions; `N/A` for non‑gore states.
- **Notes:** Extra constraints.

---

## Core States

### ASID050 – Heavy Carry Walk

- **ASID:** `050`
- **Code:** `HLD_HEAVY_WALK`
- **Role:** Movement mode when carrying heavy weapons or objectives (Bazooka, gas canister, blood vial, etc.).
- **Lock Type:** `MovementMode`
- **Movement Rules:**
  - Jump **disabled** while ASID050 is active.
  - Movement speed multiplier ≈ 0.6 of base walk speed (see weapon stats for exact value per weapon).
  - Turn speed reduced (stiffer camera / yaw acceleration).
- **Gore Trigger Frame:** N/A
- **Notes:**
  - Can stack with hit‑reactions but must not be active during execution hard‑lock states.
  - Cleared immediately when the heavy weapon or objective is dropped or consumed.

**Engine Requirements:**

- **Unreal:**  
  - Character implements `UUncutCharacterInterface` with `EnterHeavyCarry` / `ClearHeavyCarry`.  
  - `EnterHeavyCarry` sets an internal flag and switches locomotion to a Heavy Carry movement mode that enforces the rules above.  
- **Unity:**  
  - Character implements `IHeavyCarryAdapter`.  
  - `EnterHeavyCarry(stats)` applies `stats.MovementSpeedMult`, disables jump in the controller, and clamps rotation speed.  
- **Godot:**  
  - Character script exposes `enter_heavy_carry(stats)` / `clear_heavy_carry()`.  
  - Heavy carry state routes through a central ASID state machine node.

---

### ASID400 – Chainsaw Vertical Execution

- **ASID:** `400`
- **Code:** `FIN_CHAINSAW_V`
- **Role:** Vertical chainsaw execution (decap or bisect).
- **Lock Type:** `HardLock`
- **Movement Rules:**
  - Movement, jump, and roll input ignored until gore trigger frame is reached.
- **Gore Trigger Frame:** `42`
- **Notes:**
  - Cancels heavy carry (ASID050) on entering; executions always take priority.

---

### ASID405 – Katana Sweep Execution

- **ASID:** `405`
- **Code:** `FIN_SABRE_H`
- **Role:** 360‑degree katana decap sweep.
- **Lock Type:** `HardLock`
- **Movement Rules:**
  - Same as ASID400: full hard lock until exit.
- **Gore Trigger Frame:** `N` (tuned per animation set)
- **Notes:**
  - Executes any characters in radius at gore trigger frame; immune to hazard knockback for duration.

---

### ASID012 – Hit Stun Daze

- **ASID:** `012`
- **Code:** `HIT_STUN_DAZE`
- **Role:** Short stun lock used by heavy impacts and explosives.
- **Lock Type:** `SoftLock`
- **Movement Rules:**
  - Cancels sprint; reduces movement speed and turn rate.
  - Jump input ignored for the first 0.5s of the stun.
- **Gore Trigger Frame:** N/A
- **Notes:**
  - Must not override ASID400/405/901 executions.

---

## Engine Mapping and CI Validation

- **Mapping Files:**
  - Unreal: `Config/ASIDMappings.ini` (ASID ↔ Montage / AnimState names)
  - Unity: `Assets/Config/ASIDMappings.json`
  - Godot: `res://Systems/ASID/ASIDMappings.gd` (dictionary)

- **CI Checks:**
  - Parse `docs/systems/AnimationStateRegistry.md` to extract ASID codes used in the project (`050`, `400`, `405`, `012`, etc.).
  - Validate that each engine’s mapping file contains a matching entry.
  - Validate that heavy‑carry adapters call the correct state:
    - Unreal: `EnterHeavyCarry` sets `CurrentASID = 50`.
    - Unity: character’s ASID state field equals `50` when heavy carry is active.
    - Godot: ASID state machine enters `"050"` on heavy carry.

Any change to ASID definitions in this document must be accompanied by a mapping update and passing CI for all three engines.
