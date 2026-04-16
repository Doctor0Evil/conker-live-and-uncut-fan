# 07_Multiplayer_The_Blood_Count

The Blood Count is a four-team Zombies arena set in and around Count Batula’s mansion from the Spooky chapter.[web:167][web:168] Teams fight across the full interior and exterior courtyard/maze while a dense zombie population and a roaming fire imp turn every route into a risk. The map supports an objective-based Zombies mode plus TDM and DM variants.

## Design Goals

The map should feel like a fully opened-up version of Count Batula’s Mansion and hedgemaze: all major rooms, balconies, and the courtyard maze are traversable, and zombie density is high enough that gunfire is almost constant.[web:164][web:167][web:168] Players start unarmed, scavenging crossbows, shotguns, flamethrowers, and a rare hunting revolver from pickups scattered through the mansion and grounds.[web:169][web:171][web:172]

Core intent:

- Emphasize survival and route planning under pressure from zombies, not class abilities.
- Make the library and outside hedge maze the most dangerous zones, with the best loot (hunting revolver).
- Force teams to coordinate around the Panther King’s blood‑vial objective while being stalked by a fire imp that only appears once the vial is in play.

## Mode Overview

The Blood Count exposes three modes:

- Zombies (Objective)
- Team Deathmatch (TDM)
- Deathmatch (DM)

Zombies uses the full objective and AI rules below. TDM and DM reuse the same layout, weapons, and zombie behavior but change win conditions.

### Zombies (Objective Mode)

- Up to four teams (Red, Blue, Green, Yellow), four players per team (16 players total).
- Each team spawns in a different corner of the mansion, matching the four main wings off the central hall / staircase.[web:167][web:168]
- A single Panther King blood‑vial pickup spawns at one of several predefined locations; it decays and relocates after a timer if no team secures it.
- When any player picks up the blood vial, a fire imp spawns and begins defending the carrier.
- The first team to successfully carry the vial to their ritual point and complete the channel wins, or the leading team at time‑out wins on “blood score”.

### TDM / DM Variants

- TDM: teams spawn in the same four corners but there is no blood‑vial or fire imp. Zombies and weapons behave as in Zombies mode, and matches are decided by kills.  
- DM: all players spawn individually in more neutral locations, but zombie density and weapon rules stay the same, making the mansion a chaotic, free‑for‑all hunt.

## Map Structure

The Blood Count is built around Count Batula’s mansion and its hedgemaze courtyard.[web:164][web:167][web:168][web:172] The map uses the same 4x4 unit grid system as Alien Base and Beach Dead.

### Zones

1. **Mansion Interior**
   - **Grand Hall / Staircase:** central hub with four wings leading to team spawn corridors, library, grinder room, and dining room.[web:167][web:168][web:171]
   - **Library:** two‑level room with central bookcase island and crossbow B‑pad location in the single‑player game.[web:171][web:172]
   - **Dining Room and Grinder Corridor:** long hall leading to the grinder room, good for chokepoints.[web:167][web:168]
   - **Upper Walkways / Balconies:** overhead paths around zombies, offering sniper angles but constrained movement.[web:164][web:172]

2. **Courtyard / Hedge Maze**
   - Exterior maze in the back yard, with a key‑style center reminiscent of the single‑player hedgemaze.[web:164][web:167][web:172]
   - Library maze: the outer maze region around the library window/exit is the highest zombie density zone and the primary hunting‑revolver spawn area.

3. **Castle Corners (Team Wings)**
   - Four themed spawn wings (R/B/G/Y), each with:
     - A safe spawn corridor.
     - One “armory” alcove with basic pickups.
     - A short route to the grand hall and another to the courtyard.

## Teams and Spawns

- Teams: Red, Blue, Green, Yellow.
- Players per team: up to 4.
- Each team has a primary spawn cluster in one of the four mansion corners:

  - Team Red: North‑West wing.
  - Team Blue: North‑East wing.
  - Team Green: South‑West wing.
  - Team Yellow: South‑East wing.

Spawn rules:

- Initial spawns are always in team wings, away from the central hall.
- On respawn, players may be redirected to “fallback” spawns near their wing but slightly closer to the hall to reduce downtime.
- Zombies do not spawn inside safe spawn corridors; they can, however, path up to the thresholds.

## Weapon and Pickup Rules

Players begin unarmed and must find weapons in the mansion and courtyard.[web:164][web:169][web:171][web:172]

### Weapon Types

- **Crossbow**
  - Found primarily in the library and upper walkways, echoing the single‑player B‑pad location.[web:171][web:172]
  - High precision, ideal for zombie headshots and picking off rival players at range.
- **Shotgun**
  - Found in the grand hall, grinder room, dining room, and some courtyard alcoves.[web:164][web:169][web:171]
  - Primary anti‑zombie weapon for close and mid‑range; required for some zombie lethals.
- **Flamethrower**
  - Found in tighter interior choke points (grinder corridor, narrow halls).
  - Crowd‑control: can stagger or briefly repel zombies but does not kill them outright.
- **Hunting Revolver**
  - Rare, high‑power sidearm.
  - Spawn locations:
    - Library maze center (primary).
    - Occasional alternative spawn in the deepest part of the hedgemaze.
  - Always located in zones of highest zombie density and most sightline risk.

### Hunting Revolver Special Rules

- Only weapon that can kill the fire imp at any range.
- Deals heavy damage to players and zombies but still requires zombie headshots to kill.
- Limited ammo; no resupply pickups, only full respawns after depleting the weapon.

## Blood‑Vial Objective

The Panther King’s blood‑vial is the central objective in Zombies mode.

### Spawn and Decay

- Single instance per match.
- Spawns at one of several predefined locations:
  - Grand hall altar.
  - Center of library maze.
  - Center of hedgemaze.
  - Grinder room altar.
- If untouched for a given “decay” duration (e.g., 90 seconds), the vial despawns with a visual/sound cue and respawns at a different objective node.

### Carrying Rules

- When a player picks up the blood‑vial:
  - Movement locks into heavy carry mode:
    - No run, no jump.
    - No weapon use.
    - Movement speed reduced significantly.
  - The carrier becomes a visible target (glow, marker, or VO callout).
- If the carrier dies:
  - The vial drops at that location and can be picked up by anyone after a short delay.

### Scoring / Win Condition

- Each team has a ritual point (altar) in their wing.
- To score, a team must:
  - Grab the blood‑vial.
  - Deliver it to their altar.
  - Survive a short channel (e.g., 10 seconds) during which the carrier must stand on the altar while teammates defend.
- First team to complete the ritual wins, or highest number of completed rituals when time expires.

## Fire Imp Behavior

The fire imp is a roaming, high‑threat AI that only exists when the blood‑vial is held.

### Spawn Trigger

- The imp does not appear at match start.
- When any player picks up the blood‑vial for the first time, the imp spawns from a fixed “Hellgate” point (e.g., a fireplace or crypt in the mansion).
- If the vial drops and remains unclaimed for a long period, the imp can despawn until the next pickup, depending on tuning.

### Targeting and Combat Rules

- The imp’s primary behavior:
  - If the blood‑vial is held:
    - Prioritize attacking the current carrier.
    - If the carrier is within a dense zombie cluster, the imp circles and then charges.
  - If no carrier:
    - Wander near the last vial location or patrol central high‑traffic routes.

- Damage and weaknesses:
  - The imp can only be killed by:
    - Hunting revolver (any range).
    - Shotgun at very close range (lethal only when almost point‑blank).
  - Flamethrower and other weapons:
    - Do not damage, stun, or slow the imp.
  - When killed:
    - The imp despawns and is unable to respawn for a cooldown interval (e.g., 15 seconds of “breathing room”).

## Zombie Behavior

Zombies are numerous and dangerous, but mechanically simple.[web:164][web:167][web:169][web:171][web:172]

### Spawn and Density

- Zombies spawn from graves in the courtyard, floor hatches in the mansion, and “crawl out” of environmental holes.
- Density is highest:
  - In the library maze.
  - In the hedgemaze center.
  - In narrow corridors leading to the grinder room.

The design target is that every major route has zombie presence; empty corridors should be rare.

### Damage and Death Rules

- Zombies only die from:
  - Headshots (any suitable weapon).
  - Shotgun “lethal zone” blasts (close range, centered on the head/upper torso).
- Flamethrower:
  - Does not kill zombies.
  - Can apply a brief stagger or fire‑panic animation, but zombies eventually recover.
- Body/limb damage:
  - Excessive damage to legs or lower torso triggers a “crawl” state:
    - The zombie drops and begins crawling toward players.
    - Crawlers move slower but are harder to hit if you aim too high.
  - Crawlers still only die from headshots or close shotgun blasts.

### Interaction with Blood‑Vial and Fire Imp

- Zombies do not target the blood‑vial directly.
- They continue to attack any nearby players, including the carrier.
- The fire imp can damage or kill zombies incidentally, but does not deliberately clear them for players.

## Implementation Notes

- Use the same 4x4 grid system and JSON structures as Alien Base and Beach Dead:
  - `data/blood_count_grid_v1.json` for tiles and role tags (mansion interior, library, courtyard, maze, spawn wings).
  - `data/blood_count_entities_v1.json` for team spawns, zombie spawn anchors, fire imp spawn, and blood‑vial objective nodes.
  - `tilesets/*/blood_count_tiles_v1.json` for per‑engine tileset mappings.
- Zombies and fire imp should be defined as separate AI archetypes, with:
  - Zombies: slow, stagger‑capable, headshot‑only kill logic.
  - Fire imp: fast, focused, with strict damage source filters.

The Blood Count should feel like stepping into the climax of the Spooky chapter with multiplayer rules layered on top: crowded, lethal, and unforgiving, but always readable and driven by clear pickups and objectives.
