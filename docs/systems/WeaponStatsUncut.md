# Weapon Stats – Live & Uncut Multiplayer

This document is the human‑readable source of truth for `data/weapons/weaponstatsv1.json`. Each weapon in the JSON file must have a matching entry here using the same `id` string.

All engines (Unreal, Unity, Godot) and AI‑Chat helpers should treat this file as canonical for naming, roles, and balance intent.

---

## 1. Schema and Versioning

All weapon stats are stored in `data/weapons/weaponstatsv1.json`.

- `schemaversion` matches the JSON schema for weapon stats (shape of fields).
- `contentversion` tracks gameplay tuning changes.

Validation rules:

- All referenced weapons in entities files (weapon pickups, loadouts) must exist in `weaponstatsv1.json`.
- Numeric fields must stay within reasonable ranges (no negative damage, no nonsensical fire rates).

Any change to weapon damage, range, fire rate, heavy flags, or movement modifiers must be reflected in both this document and the JSON file in the same pull request.

---

## 2. Shared Assumptions

- Baseline player health: 100 HP.
- Tick rate: 60 fps (fire rates are defined in rounds per second).
- Movement and ASID heavy‑carry rules are defined in the Animation State / ASID registry docs.
- Weapon stats reference movement and state via:
  - `is_heavy` (heavy weapon handling).
  - `blocks_jump_when_equipped` (no jump while this weapon is active).
  - Optional movement multipliers for specific melee or special weapons.

Weapons can be used by any character state that is not locked by an ASID (for example, execution, stun, heavy carry), consistent with the ASID registry.

---

## 3. Authoring Format

Each weapon entry in this document uses this format:

- **ID:** `WeaponId` (must match `id` in JSON)
- **Category:** Melee | Primary | Secondary | Heavy | Special
- **One‑Line Description:** Single sentence describing fantasy and primary use.
- **Gameplay Notes:** Short bullet list for behavior, counters, and map usage.
- **Key Fields:** Human‑readable summary of the most important JSON fields.

The JSON file can contain more detailed fields (spread, recoil, lock‑on cones, etc.); list only the ones designers and reviewers need to think about regularly under **Key Fields**.

---

## 4. Weapons

### 4.1 Chainsaw

- **ID:** `Chainsaw`
- **Category:** Melee
- **One‑Line Description:** Brutal close‑quarters melee that excels at executions and finishing stunned targets.
- **Gameplay Notes:**
  - No heavy‑carry penalty; player retains normal jump and movement.
  - Highest single‑target damage at point‑blank, but very short range.
  - Synergizes with stun states and narrow corridors (Alien Base hub spokes).
- **Key Fields (JSON):**
  - `damage_per_hit`: 100
  - `range_m`: 1.5
  - `fire_rate_hz`: 1.2
  - `clip_size`: 0
  - `reserved_ammo`: 0
  - `movement_speed_mult`: 1.0
  - `is_heavy`: false
  - `blocks_jump_when_equipped`: false

---

### 4.2 Shotgun

- **ID:** `Shotgun`
- **Category:** Primary
- **One‑Line Description:** Close‑range spread weapon tuned to shred zombies and punish overextended carriers.
- **Gameplay Notes:**
  - Balanced around lethal headshots at short range, strong versus zombies and the Fire Imp.
  - No heavy‑carry; standard movement and jump preserved.
  - Ideal for ambush routes and Blood Count interior fights.
- **Key Fields (JSON):**
  - `damage_per_pellet`: 12
  - `pellets`: 8
  - `range_m`: 12.0
  - `fire_rate_hz`: 1.0
  - `magazine_size` / `clip_size`: 8
  - `max_ammo` / `reserved_ammo`: 32
  - `movement_speed_mult`: 1.0
  - `is_heavy`: false
  - `blocks_jump_when_equipped`: false

---

### 4.3 Pistol

- **ID:** `Pistol`
- **Category:** Secondary
- **One‑Line Description:** Reliable sidearm for finishing fights and covering mid‑range gaps when primary ammo runs dry.
- **Gameplay Notes:**
  - Baseline weapon: always usable, moderate damage, controllable recoil.
  - Five body shots or three headshots to kill a full‑health target.
  - Encourages rhythmic firing instead of full‑speed spam.
- **Key Fields (JSON):**
  - `damage_per_shot`: 20
  - `damage_headshot_multiplier`: 2.0
  - `rounds_per_second`: 4.0 (semi‑auto)
  - `magazine_size`: 12
  - `max_ammo`: 72
  - `base_spread_deg`: 1.5 (up to ~4.0)
  - `reload_seconds`: 1.6
  - `is_heavy`: false
  - `blocks_jump_when_equipped`: false

---

### 4.4 Uzi

- **ID:** `Uzi`
- **Category:** Primary
- **One‑Line Description:** Wild close‑range spray SMG that deletes targets at short range but falls off hard at distance.
- **Gameplay Notes:**
  - Extremely low TTK if multiple shots land at close range.
  - Wide hip‑fire spread; ADS is recommended beyond knife‑fighting distance.
  - Poor at long range; controlled bursts work better than mag‑dumps.
- **Key Fields (JSON):**
  - `damage_per_shot`: 10
  - `damage_headshot_multiplier`: 1.6
  - `rounds_per_second`: 12.0 (full auto)
  - `magazine_size`: 30
  - `max_ammo`: 240
  - `base_spread_deg`: 3.0 (up to ~10.0)
  - `reload_seconds`: 2.0
  - `is_heavy`: false
  - `blocks_jump_when_equipped`: false

---

### 4.5 Sniper Rifle

- **ID:** `SniperRifle`
- **Category:** Heavy
- **One‑Line Description:** Precision rifle that rewards patient, stationary play with one‑shot headshot kills.
- **Gameplay Notes:**
  - One‑shot headshot, two body shots to kill a full‑health target.
  - Strong recoil and slow cycling; reposition between shots.
  - Firing while moving or jumping is heavily penalized; best used while scoped and still.
- **Key Fields (JSON):**
  - `damage_per_shot`: 80
  - `damage_headshot_multiplier`: 2.5
  - `rounds_per_second`: 1.0 (bolt‑action cadence)
  - `magazine_size`: 5
  - `max_ammo`: 30
  - `base_spread_deg`: 0.1 (effectively 0 when ADS and stationary)
  - `reload_seconds`: 2.8
  - `is_heavy`: true
  - `blocks_jump_when_equipped`: true

---

### 4.6 Bazooka

- **ID:** `Bazooka`
- **Category:** Heavy
- **One‑Line Description:** High‑damage explosive launcher that trades mobility and jump for overwhelming area denial.
- **Gameplay Notes:**
  - Heavy weapon: severely restricts mobility and disables jumping while equipped.
  - Inner blast radius is lethal; outer radius leaves targets heavily wounded.
  - Self‑damage is significant; careless players can kill themselves at close range.
  - Optional lock‑on mode supports anti‑vehicle or anti‑air gameplay later.
- **Key Fields (JSON):**
  - `damage_per_shot`: 120 (direct impact)
  - `explosion_radius_meters`: 4.5
  - `explosion_inner_radius_meters`: 1.5
  - `explosion_damage_min`: 40
  - `explosion_damage_max`: 120
  - `self_damage_multiplier`: 0.5
  - `rounds_per_second`: 0.4
  - `magazine_size`: 1
  - `max_ammo`: 6
  - `reload_seconds`: 3.2
  - `is_heavy`: true
  - `blocks_jump_when_equipped`: true
  - `supports_lock_on`: true (cone and distance defined in JSON)

---

## 5. Integration with ASID and Heavy Carry

Heavy behavior is controlled by two booleans in weapon stats:

- `is_heavy`: marks a weapon as heavy for movement rules and certain ASID transitions.
- `blocks_jump_when_equipped`: when true, the movement system must prevent jumps while this weapon is active.

Suggested mapping to ASIDs:

- Heavy weapons (Sniper Rifle, Bazooka) should transition the player into a movement‑limited ASID such as `ASID051 HeavyWeaponEquip` that:
  - Reduces base movement speed.
  - Disables jump and similar impulses.
  - May affect roll/dive abilities.

Objective pickups (Money Bag, Blood Vial, Gas Canister) use a separate Heavy Carry ASID (for example, `ASID050`) that disables all weapons except a context‑sensitive action. Weapon stats still apply whenever the player is not in a Heavy Carry state.

---

## 6. Engine Implementation Notes

### 6.1 Unreal (C++ / Blueprint)

- Load `weaponstatsv1.json` into a data asset or table at startup.
- Use `rounds_per_second` to compute fire intervals.
- Use spread and recoil fields to build a firing cone and camera kick.
- Apply `is_heavy` and `blocks_jump_when_equipped` to the Character Movement Component or input layer.

### 6.2 Unity (C#)

- Import stats into a `WeaponStatsDatabase` ScriptableObject.
- Drive projectile prefabs, recoil animations, and crosshair bloom from JSON fields.
- Enforce heavy weapon movement restrictions in the player controller.

### 6.3 Godot (GDScript / C#)

- Load stats via a singleton `WeaponStats` autoload.
- Provide a `get_weapon(id)` API to gameplay scripts.
- Apply spread, recoil, and heavy movement rules in the firing and movement scripts.

---

## 7. Next Steps

- Add a JSON Schema `schemas/weaponstatsv1.schema.json` to validate `weaponstatsv1.json` in CI.
- Wire `grid2scene --validate --all` so that any unknown weapon ID in entities files fails validation.
- After first greybox playtests, bump `contentversion` (for example, `1.1.0`) when adjusting damage, ROF, magazine sizes, or heavy flags.
