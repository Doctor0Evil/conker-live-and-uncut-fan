# 05_Multiplayer_Raptor_Temple

Raptor Temple revives the N64 Raptor and Temple deathmatch experiences as a single multi‑layered arena where cavemen and raptors clash around an altar and vertical ruins. [hardcoregamer](https://hardcoregamer.com/features/member/member-conkers-bad-fur-day/312612/)

## Design Goals

The core is a three‑tiered arena: bottom pit with altar and baby raptor, mid‑tier walkways with egg nests and cooking pans, and upper platforms for ambushes.  The Uncut version removes class systems but may still allow role‑select (Raptor vs Caveman) as a cosmetic or ruleset choice rather than a stat class. [hardcoregamer](https://hardcoregamer.com/features/member/member-conkers-bad-fur-day/312612/)

## Layout Overview

The lowest level houses the altar and baby raptor area, with ramps leading up to mid‑tier rock shelves and temple passages.  The top tier includes narrow stone bridges and balconies that overlook the altar, allowing for long‑range harassment and drop‑down attacks. [hardcoregamer](https://hardcoregamer.com/features/member/member-conkers-bad-fur-day/312612/)

## Objectives and Modes

- Raptor mode: raptors feed cavemen to the baby raptor; cavemen try to steal eggs and cook them at pans around the mid‑tier. [reddit](https://www.reddit.com/r/n64/comments/1l5dhc8/favorite_conkers_bad_fur_day_multiplayer_mode/)
- Deathmatch: use the same space with standard FFA scoring.  
- Optional “Egg Ball” variant: a single egg functions as a mobile objective similar to a skull or ball.

## Weapon and Pickup Philosophy

Melee and short‑range weapons dominate the lower tiers, with more ranged pickups (slingshots, rifles) on upper platforms to recreate Temple’s vertical fights.  Egg items and sacrifice triggers are handled as special interactables, not generic pickups. [hardcoregamer](https://hardcoregamer.com/features/member/member-conkers-bad-fur-day/312612/)

## Visual and Material Notes

Materials should lean into Uga Buga’s stylized stone, wood, and bone, with torches and lava pits providing light and hazard.  Alien/industrial cues should be kept out of this map to preserve thematic purity. [wikiwand](https://www.wikiwand.com/en/Conker's_Bad_Fur_Day)

## Implementation Notes

Use a cylindrical or octagonal hub grid (like Alien Base) but swap sci‑fi tiles for stone/altar variants.  The same JSON grid/entity structure works; only the tileset and entity types change. [gamefaqs.gamespot](https://gamefaqs.gamespot.com/n64/196973-conkers-bad-fur-day/faqs/19563)
