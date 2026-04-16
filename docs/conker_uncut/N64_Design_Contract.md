# Conker: Live & Uncut — N64 Design Contract

This document defines the canonical design invariants for reconstructing Conker: Live & Uncut as an N64‑era experience. It is the human counterpart of the machine‑readable Conker design contract schema and must remain in sync with that schema and with SessionProfile invariants for Conker sessions.

The goal is to preserve the mechanical and aesthetic identity of Conker’s Bad Fur Day multiplayer while integrating Uncut‑era concepts only when they can be expressed in N64‑faithful form.

---

## 1. Scope and Canonical Sources

1. The Nintendo 64 release of Conker’s Bad Fur Day is the primary source of truth for mechanics, pacing, and multiplayer structure.
2. The 2003 E3 Live & Uncut Xbox prototype is a secondary source for map concepts, themes, and player counts, but not for Xbox‑specific mechanics or visual detail.
3. Additional Uncut‑only maps (Alien Base, Spamono, Blood Count) must be interpreted as N64‑era designs derived from early Xbox materials, not as Xbox‑native feature sets.
4. Later products (e.g., Conker: Live & Reloaded) are treated as structural anti‑patterns and must not influence core design decisions.

Any ambiguity must be resolved in favor of N64‑era behavior and fairness.

---

## 2. Multiplayer Game Structure

### 2.1 Player Counts

1. Maximum supported players per match: **16**.
2. Minimum supported players per match: **2** (1v1 or FFA).
3. All modes and maps must behave correctly for 2–16 players without introducing per‑player power scaling or progression.

### 2.2 Modes and Map Set

The canonical multiplayer set includes:

- Beach
- Heist
- Colors (Total War)
- Raptor
- Tank
- Race
- Deathmatch
- Alien Base (Uncut‑derived)
- Spamono (Uncut‑derived)
- Blood Count (Uncut‑derived)

For each mode:

1. At least one N64‑faithful map must exist that reproduces the original flow (lanes, chokepoints, pickup pacing).
2. Uncut‑only maps must be designed to fit the same mode‑specific rules and pacing as their closest N64 counterparts.

### 2.3 Arsenal and Progression

1. The multiplayer arsenal is **symmetric and pickup‑based**.
2. There are **no** player classes, no fixed loadouts, and no per‑class weapon restrictions.
3. There is **no** XP, meta‑progression, or persistent unlock system.
4. All players spawn with a basic kit (e.g., melee and/or light weapon) and must acquire power via map pickups.
5. Any Uncut‑inspired abilities must be realized as pickups or temporary states, not permanent unlocks.

Any proposal that introduces classes, loadouts, or persistent stats violates this contract.

---

## 3. Core Mechanics

### 3.1 Movement and Weapons

1. Heavy weapons (e.g., rocket launchers, heavy machine guns) must **disable jumping** while equipped.
2. Weapons are categorized into at least:
   - Light firearms (pistol, SMG, basic rifle)
   - Heavy weapons (MG, rockets, explosives)
   - Melee / close‑range specials
3. The heavy‑weapon no‑jump rule is enforced through animation state IDs (ASIDs) and must be encoded as `can_jump = false` for all heavy‑weapon movement states.
4. Movement speed and jump behavior must match N64‑era feel within the limits of the target engine and hardware profile.

### 3.2 Melee and Executions

1. Melee attacks and executions are **hard‑locking** animation states, not flexible combo systems.
2. Executions:
   - Must fully lock player input for the duration of the animation.
   - Must be identifiable via specific ASIDs tagged as `is_executing = true` and `locks_input = true`.
3. Melee attacks:
   - Must be short, high‑impact states with clearly defined startup and recovery windows.
   - Must not chain into extended combo trees.

Any system that attempts to replace hard‑lock executions with free‑flow combos is out of scope.

### 3.3 Hazards and Instant‑Win Events

1. High‑impact hazards (gas chambers, tank gas drops, purges) are **rare, scripted events**, not continuous environmental pressure.
2. Hazards must:
   - Be player‑triggered or tied to clearly telegraphed objectives.
   - Use a shared state machine pattern (Idle → Arming → Active → Cooldown).
   - Offer a reasonable response window for attentive players.
3. “Instant win” or team‑wipe effects are allowed only via these hazards and must be infrequent in a typical match.
4. Continuous Battle Royale–style shrinking zones or constantly ticking death fields are explicitly disallowed.

---

## 4. Map Design and Symmetry

### 4.1 Layout Principles

1. Each map must be described by a platform‑agnostic logical grid (lanes, chokepoints, verticality) that can be compiled into N64 or modern geometry.
2. For symmetric team modes (e.g., Colors, Beach variants):
   - Team A and Team B must have equivalent spawn quality, travel times to key pickups, and access to vantage points, up to a map symmetry transform.
3. Asymmetric scenarios (e.g., Beach offense/defense) must remain faithful to N64 roles and win conditions, not Xbox‑era objective redesigns.

### 4.2 Spawns and Pickups

1. Spawn points:
   - Must be defined for up to 16 slots, per team where applicable.
   - Must avoid spawn camping via cover and line‑of‑sight constraints.
2. Pickup spawns:
   - Must enforce symmetric access to powerful weapons and powerups.
   - Must be tuned so that no single pickup dominates match outcome on its own.
3. Class‑specific spawns or weapon stations (as in Live & Reloaded) must not be implemented.

### 4.3 Uncut‑Only Maps

For Alien Base, Spamono, and Blood Count:

1. Environmental themes and high‑level flow may draw from Xbox‑era descriptions.
2. All mechanics, spawns, pickups, and hazards must conform to the N64 contract:
   - Symmetric, pickup‑based arsenals.
   - No class or loadout mechanics.
   - Hazard behavior via the shared instant‑win template.

---

## 5. Aesthetics and Content Boundaries

### 5.1 Visual Style

1. Character and world silhouettes must be recognizable as N64‑era Conker, with:
   - Chunky forms.
   - Low to medium polygon counts appropriate for the original hardware.
2. Textures:
   - Must originate from or be consistent with **64×64‑scale** sources for primary materials.
   - May be up‑rezzed via allowed scalers for modern targets, but base detail must remain N64‑plausible.
3. Color palettes must evoke N64 output: limited bands, non‑HDR lighting, and stylized contrast.

### 5.2 Thematic Motifs

1. Black, xenomorph‑style aliens with lime green gore are canonical for alien enemies and must be preserved as such.
2. Gore, humor, and adult themes must align with the original N64 game’s tone (irreverent, crude, but focused), not later sanitized or reimagined variants.
3. Any new enemies or props introduced for Uncut‑only maps must be stylistically consistent with the N64 game’s proportions and palette.

### 5.3 Character Roster

1. Multiplayer rosters must preferentially use story characters (Conker, Berri, Gregg, Rodent, etc.) and established factions.
2. Generic modern soldier archetypes are not allowed as primary avatars.
3. If generic variants are needed for technical reasons, they must still conform to Conker’s N64 visual language and not resemble Live & Reloaded’s class soldiers.

---

## 6. Hardware and Performance Constraints

1. All N64 builds must respect platform constraints as encoded in `N64Constraints` and budget profiles:
   - ROM size ceiling (e.g., 32 MiB cart profile).
   - RDRAM usage bounds.
   - Per‑segment budgets for code, textures, audio, scripts, and mission data.
2. Any Conker map or asset manifest that exceeds these bounds must be rejected or down‑scaled by the budget planner before a build is considered valid.
3. Determinism:
   - Core simulation paths for multiplayer must be deterministic under fixed inputs.
   - Non‑deterministic APIs (e.g., unseeded RNG, non‑stable collections) are forbidden in deterministic Conker core crates.

---

## 7. Machine‑Verifiable Invariants

The following invariants must be represented in the Conker design contract schema and enforced via CI and AI checklists:

1. `max_players <= 16` for all Conker sessions and map recipes.
2. `arsenal_mode == "PICKUP_SYMMETRIC"`; no classes, no loadouts.
3. `progression_mode == "MATCH_BOUND"`; no persistent XP or unlock trees.
4. All heavy‑weapon ASIDs have `can_jump = false`.
5. All execution ASIDs have `is_executing = true` and `locks_input = true`.
6. All hazard instances reference a valid instant‑win hazard template and use the shared state machine pattern.
7. No Conker map recipe introduces per‑class spawn points or weapon stations.
8. All N64 Conker builds must pass the N64 budget report with `is_within_budget = true`.

Any automated or human change that violates these invariants must fail validation before N64 ROM or modern engine builds are produced.

---

## 8. Evolution and Governance

1. Changes to this contract must be mirrored in:
   - The `ConkerDesignContract` Rust type.
   - The corresponding JSON Schema.
   - SessionProfile invariant bundles for Conker sessions.
2. Any breaking change (tightening or relaxing core invariants) must:
   - Bump the contract version.
   - Be reviewed against existing map recipes and builds.
3. AI‑driven tools must treat this contract as authoritative:
   - They may propose new content only within its bounds.
   - They must refuse to synthesize code or data that would knowingly violate it.

This contract is the normative reference for all N64‑era Conker: Live & Uncut reconstruction work within Nintendoor64 and GAMEMODE.ai.
