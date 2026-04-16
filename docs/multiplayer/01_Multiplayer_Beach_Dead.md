# 01_Multiplayer_Beach_Dead

Beach Dead is the Uncut‑era remake of the N64 Beach scenario: asymmetrical attackers (Frenchies / villagers) rush the shoreline and must reach a truck or extraction point while defenders (Tediz) hold fixed or semi‑fixed firing positions, now expanded slightly for 16‑player support. [ign](https://www.ign.com/articles/2005/04/07/conker-live-and-reloaded-hands-on-multiplayer)

## Design Goals

The map should feel like a higher‑fidelity version of the N64 Beach: same basic geography and objective, with more cover, slightly deeper trenches, and N64‑style weapon pickups instead of class loadouts or XP.  It also needs to support a 16‑player cap and optional team deathmatch / FFA modes without breaking the original “push up the beach” identity. [gamefaqs.gamespot](https://gamefaqs.gamespot.com/n64/196973-conkers-bad-fur-day/faqs/19563)

## Layout Overview

The map is a long, sloped beach leading up to a fortified cliff‑line, with three main lanes separated by barbed wire and tank traps.  The attacking spawn is at the waterline, the defending spawn is behind the cliff bunker complex, and the primary objective truck or gate sits near the center of the upper plateau. [conker.fandom](https://conker.fandom.com/wiki/Multi)

## Objectives and Modes

In “Beach” mode, attackers must get a set number of villagers to the truck before a timer expires while defenders prevent escapes; this mirrors the N64 setup.  In Uncut team deathmatch, both teams can roam freely, but spawn logic and weapon placement still bias defenders toward high ground and fixed guns. [gamefaqs.gamespot](https://gamefaqs.gamespot.com/n64/196973-conkers-bad-fur-day/faqs/19563)

## Weapon and Pickup Philosophy

Attackers rely on light weapons and grenades picked up from beach craters and destroyed bunkers, while defenders have access to heavier MG nests and limited explosives in the upper fortifications.  No classes or XP are used; all firepower is tied to map control. [conker.fandom](https://conker.fandom.com/wiki/Multi)

## Visual and Material Notes

The materials should keep the saturated, almost toy‑like N64 palette—bright sand, blue‑grey bunkers, olive uniforms—rather than the grittier look of the shipped Xbox war maps.  Smoke, distant ships, and skybox bombardment can be handled as non‑interactive VFX. [hardcoregamer](https://hardcoregamer.com/features/member/member-conkers-bad-fur-day/312612/)

## Implementation Notes

Use the same grid/tile JSON structure as Alien Base, but with a rectangular footprint that’s longer on the Z axis and a gentler vertical rise from shoreline to cliff.  Spawn, objective, and MG nest entities should be driven by `entities.json` so you can toggle between pure Beach mode and deathmatch without touching meshes. [gamefaqs.gamespot](https://gamefaqs.gamespot.com/n64/196973-conkers-bad-fur-day/faqs/19563)
