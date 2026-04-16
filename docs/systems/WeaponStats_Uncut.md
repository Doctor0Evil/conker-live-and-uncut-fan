# Weapon Stats – Live & Uncut Multiplayer

This document is the human‑readable source of truth for `data/weapons/weaponstatsv1.json`. Each weapon in the JSON file **must** have a matching entry here using the same `id` string.

All engines (Unreal, Unity, Godot) and AI‑Chat helpers should treat this file as canonical for naming, roles, and balance intent.

---

## Format

Each weapon entry uses this format:

- **ID:** `WeaponId` (must match `id` in JSON)
- **Category:** Melee | Primary | Secondary | Heavy | Special
- **One‑Line Description:** Single sentence describing fantasy and primary use.
- **Gameplay Notes:** Short bullet list for behavior, counters, and map usage.
- **JSON Fields:** Mapping to `weaponstatsv1.json` keys.

---

## Weapons

### Chainsaw

- **ID:** `Chainsaw`
- **Category:** Melee
- **One‑Line Description:** Brutal close‑quarters melee that excels at executions and finishing stunned targets.
- **Gameplay Notes:**
  - No heavy‑carry penalty; player retains normal jump and movement.
  - Highest single‑target damage at point‑blank, but very short range.
  - Synergizes with stun states and narrow corridors (Alien Base hub spokes).
- **JSON Fields:**
  - `damage_per_hit`: 100
  - `range_m`: 1.5
  - `fire_rate_hz`: 1.2
  - `is_heavy_carry`: false
  - `clip_size`: 0
  - `reserved_ammo`: 0
  - `movement_speed_mult`: 1.0

---

### Shotgun

- **ID:** `Shotgun`
- **Category:** Primary
- **One‑Line Description:** Close‑range spread weapon tuned to shred zombies and punish overextended carriers.
- **Gameplay Notes:**
  - Balanced around lethal headshots at short range, strong vs. zombies and Fire Imp.
  - No heavy‑carry; standard movement and jump preserved.
  - Ideal for ambush routes and Blood Count interior fights.
- **JSON Fields:**
  - `damage_per_pellet`: 12
  - `pellets`: 8
  - `range_m`: 12.0
  - `fire_rate_hz`: 1.0
  - `is_heavy_carry`: false
  - `clip_size`: 8
  - `reserved_ammo`: 32
  - `movement_speed_mult`: 1.0

---

### Bazooka

- **ID:** `Bazooka`
- **Category:** Heavy
- **One‑Line Description:** High‑damage explosive launcher that trades mobility and jump for overwhelming area denial.
- **Gameplay Notes:**
  - Triggers Heavy Carry (ASID050): no jump, slower turn, reduced movement speed.
  - Strongest area damage; splash radius tuned for hub floor denial.
  - Placement is limited to high‑risk nodes (e.g., Alien Base sub‑level).
- **JSON Fields:**
  - `damage_direct`: 200
  - `damage_splash`: 120
  - `splash_radius_m`: 4.0
  - `fire_rate_hz`: 0.4
  - `is_heavy_carry`: true
  - `clip_size`: 1
  - `reserved_ammo`: 4
  - `movement_speed_mult`: 0.6

---

## Versioning

- **Document version:** 1.0.0
- **JSON file:** `data/weapons/weaponstatsv1.json`
- Any change to weapon damage, range, fire rate, or `is_heavy_carry` **must** be reflected both here and in the JSON file in the same pull request.
