# 02_Multiplayer_The_Heist

The Heist is a high‑fidelity port of the N64 Feral Reserve Bank multi mode, keeping the four‑team cash‑grab rules and the infamous gas chamber “instant win” mechanic in a more detailed interior environment. [conker.fandom](https://conker.fandom.com/wiki/Multi)

## Design Goals

The core goal is to preserve the chaotic, circular routing of the N64 Heist—four team vaults, a central money room, looping corridors—and to reintroduce the gas chamber as a high‑risk, high‑reward objective.  The Uncut build should scale to 16 online players while still feeling like a 4‑player N64 match turned up to eleven. [gamefaqs.gamespot](https://gamefaqs.gamespot.com/n64/196973-conkers-bad-fur-day/faqs/19563)

## Layout Overview

At the center sits the main cash vault with the money bag pedestal, ringed by corridors that lead to four color‑coded team rooms (Red, Blue, Green, Yellow).  One corner of the complex houses the gas chamber and control room, accessible via a side corridor and a secure control console. [conker.fandom](https://conker.fandom.com/wiki/Multi)

## Objectives and Modes

Primary mode: grab the money bag from the central vault and return it to your team’s safe, with scoring rules mirroring N64 Heist.  An optional “Gas Play” rule enables the gas chamber: a player who delivers the bag to a specific intake can trigger a lethal gas event that wipes other teams and awards bonus points. [gamefaqs.gamespot](https://gamefaqs.gamespot.com/n64/196973-conkers-bad-fur-day/faqs/19563)

## Weapon and Pickup Philosophy

Weapons are placed in predictable, symmetric spots around each team base and the central ring: pistols and SMGs near spawns, heavier gear like shotguns and bazookas further out into neutral territory.  Pickup parity between the four teams is critical; no class‑locked weapons or permanent upgrades. [conker.fandom](https://conker.fandom.com/wiki/Multi)

## Visual and Material Notes

Geometry should look like a modernized Feral Reserve: marble floors, gold accents, heavy vault doors, but with exaggerated, cartoon‑bank proportions.  Gas chamber materials can reuse Alien Base’s grate and hazard palettes to visually reinforce “do not stand here when the sirens go off.” [wikiwand](https://www.wikiwand.com/en/Conker's_Bad_Fur_Day)

## Implementation Notes

Use a square hub grid similar to Alien Base’s, but with four clearly partitioned wings instead of outer bunkers.  The gas chamber logic can reuse the Airlock/Gas state machine; the only difference is the trigger condition (bag delivered vs console activated). [gamefaqs.gamespot](https://gamefaqs.gamespot.com/n64/196973-conkers-bad-fur-day/faqs/19563)
