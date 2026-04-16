# 03_Multiplayer_Fortress

Fortress is the Uncut iteration of the N64 War/Total War map and early version of what became Fortress Deux: a two‑base battlefield with central no‑man’s‑land, chemical weapon instant‑win mechanics, and flexible rules for Total War, Colors, and straight team deathmatch. [raregamer.co](https://www.raregamer.co.uk/games/conker-live-reloaded-conquering-conker-live-walkthrough-7/)

## Design Goals

The map should feel like a direct successor to the N64 War layout: SHC and Tediz bases on opposite ends, central trenches and bunkers, and chemical canister objectives for Total War/Colors.  In Uncut form, it uses weapon pickups and fixed spawn zones rather than class terminals and upgrade stations. [ign](https://www.ign.com/wikis/conker-live-reloaded/Multiplayer_-_Map_Strategy)

## Layout Overview

Each base has an interior spawn room, armory, and outer courtyard, connected to the battlefield by ramps and trenches.  The middle of the map contains the canister spawn and key chokepoints—bunkers, bridges, and tunnels—that mirror the N64 layout with more verticality and detail. [conker.fandom](https://conker.fandom.com/wiki/Multi)

## Objectives and Modes

- Total War: knock out enemy lives/score with kills, with an “instant win” condition if the canister is brought to a designated delivery point. [gamefaqs.gamespot](https://gamefaqs.gamespot.com/n64/196973-conkers-bad-fur-day/faqs/19563)
- Colors: capture the opponent’s flag from their base and return it to yours. [reddit](https://www.reddit.com/r/n64/comments/1l5dhc8/favorite_conkers_bad_fur_day_multiplayer_mode/)
- Team Deathmatch: same geometry without canister or flag objectives.

## Weapon and Pickup Philosophy

Light weapons and grenades spawn near base interiors, while mid‑map bunkers host more powerful pickups like gatling guns or bazookas.  Exact parity of pickup type and count along mirrored routes is essential to maintain fairness. [conker.fandom](https://conker.fandom.com/wiki/Multi)

## Visual and Material Notes

Fortress should visually sit between BFD’s War and Live & Reloaded’s Fortress Deux: cartoon military architecture with slightly more detailed brickwork, sandbags, and barbed wire.  Color‑coding and signage should make base identity unmistakable even in 16‑player chaos. [raregamer.co](https://www.raregamer.co.uk/games/conker-live-reloaded-conquering-conker-live-walkthrough-7/)

## Implementation Notes

Treat Fortress as a horizontally stretched grid: two large “base boxes” and a central “battle box,” each with their own local grids that the tool chain can stitch together.  Use the same hazard volume pattern as Tanks/Total War if you decide to include a gas‑based instant win. [gamefaqs.gamespot](https://gamefaqs.gamespot.com/n64/196973-conkers-bad-fur-day/faqs/19563)
