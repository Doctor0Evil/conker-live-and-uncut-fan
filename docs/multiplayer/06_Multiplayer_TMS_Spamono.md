# 06_Multiplayer_TMS_Spamono

TMS Spamono is the Uncut‑era version of the later Live & Reloaded bonus map: a T‑shaped Tediz military ship lost in space, played as a tight, lane‑based objective map that feels like a cross between football and corridor deathmatch. [youtube](https://www.youtube.com/watch?v=nnmB0e_hiaM)

## Design Goals

The ship should play like a compact “energy ball” scenario: a central neutral pickup (energy sphere) and two opposing goals at either end of the T‑shaped spine, with flanking routes and short sightlines.  The Uncut version removes class locks and upgrade terminals while preserving the tight, linear lane structure. [conker.fandom](https://conker.fandom.com/wiki/TMS_Spamono)

## Layout Overview

From above, the ship forms a T: a central midsection where the sphere spawns, with two opposing goal corridors along the main shaft and a crossbar of side routes and control rooms.  Small airlocks or side pods can serve as risk/reward shortcuts or temporary safe rooms. [conker.fandom](https://conker.fandom.com/wiki/TMS_Spamono)

## Objectives and Modes

- Primary mode: grab the energy sphere and carry it into the enemy goal to score, with a fixed score limit or timer. [conker.fandom](https://conker.fandom.com/wiki/TMS_Spamono)
- Secondary modes: team deathmatch or FFA using the same geometry but without the scoring objective.  

## Weapon and Pickup Philosophy

Most pickups are clustered around the central lane and side alcoves so that controlling mid is powerful but not mandatory.  Defenders near each goal get lighter weapons, while heavy weapons and sniping tools are placed near mid‑ship, forcing teams to commit forward to secure them. [raregamer.co](https://www.raregamer.co.uk/games/conker-live-reloaded-conquering-conker-live-walkthrough-7/)

## Visual and Material Notes

Spamono should be all steel, hazard stripes, and glowing consoles: a classic Tediz warship with exaggerated cartoon bulkheads and airlocks.  Use similar palettes and meshes to Alien Base’s human tech side to keep asset reuse high and the map pack cohesive. [conker.fandom](https://conker.fandom.com/wiki/TMS_Spamono)

## Implementation Notes

Grid‑wise, treat Spamono as a long rectangle with a perpendicular crossbar: one hub grid for mid, two smaller grids for the side wings, and linear corridor segments for goals.  The same `grid + entities + tileset` pipeline can generate a playable Uncut Spamono with sphere spawn, goals, and hazard markers from a single JSON pass. [conker.fandom](https://conker.fandom.com/wiki/TMS_Spamono)
