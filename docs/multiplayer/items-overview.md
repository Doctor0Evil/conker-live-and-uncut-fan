# Conker: Live & Uncut – Item and Pickup Overview

This document defines the gameplay-facing item vocabulary for Conker: Live & Uncut. It is the human mirror of the OBJ, HUD, ASID, SFX, VFX, and GMR/RUL registries in `conkerregistry`, and is intended as a first stop for AI-assisted design and code generation.[file:5][file:4][file:3]

The philosophy is **N64-first**: no classes, no XP, no loadouts. All combat power comes from pickups, objectives, vehicles, and hazards placed in the map, as originally planned for Conker: Live & Uncut and preserved from Bad Fur Day’s multiplayer design.[file:15][file:4]

---

## 1. Power-Up Pickups (Orbs and Boosters)

Power-ups are short-duration, pickup-only buffs that do not create classes or persistent progression. They are implemented as OBJ entries plus optional ASIDs (for state) and HUD overlays (for feedback).[file:5][file:4]

| Category        | Canonical Intent                                                                                   | Notes for Engine / Codegen                                                                                                                       |
|----------------|-----------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| Movement boost | Temporary run-speed increase (e.g., +25% for 8–10 seconds).                                        | Implement as an OBJ* pickup that applies a movement ASID modifier; must not stack with heavy-carry ASID050.[file:5][file:4]                     |
| Slide buff     | Grants a longer, lower-friction baseball slide (ties into ASID451 MOVSLIDE).                       | Use as a counterplay tool for escaping chokepoints; should never override core stun or execution ASIDs.[file:5]                                 |
| Damage boost   | Short damage multiplier for light weapons (SMG, pistol, shotgun).                                  | Keep magnitude modest (1.25–1.5×) to preserve N64-style TTK and avoid Reloaded-style class spike damage.[file:15][file:4]                       |
| Gas mask       | Immunity to ROLEHAZARDGAS areas for a brief period.                                                | Implement as power-up that toggles hazard immunity tags rather than permanent equipment; integrate with hazard profiles.[file:5][file:9]       |
| Armor orb      | Temporary light armor, weaker than `OBJ004 OBJARMORVEST`.                                          | Treat as a one-shot, small shield; share underlying armor bar HUD icon (`HUDICONARMOR`) for consistency.[file:4][file:5]                        |
| Vision / HUD   | Short-lived enhancements (highlight objectives, reveal gas timers).                                | Use sparingly and only as overlay HUD widgets, not wallhack-style wall penetration; keep fidelity with BFD’s minimalist HUD style.[file:4]     |

Implementation guidance: power-ups **must** be placed sparingly and symmetrically, and should never be required for basic map flow. They are for spice, not progression.

---

## 2. Weapons (Held Items)

Weapons are always obtained from pickups placed in the map. There are no loadouts, no classes, and no unlock trees.[file:15][file:4] Each weapon corresponds to a WPN id in `weaponstatsv1.json`, an OBJ pickup row, and related SFX/VFX/ASID entries.[file:5][file:4][file:3]

### 2.1 Core N64 / Uncut Weapons

| Weapon           | Registry Backbone                                                                                                        | Gameplay Notes                                                                                                                                       |
|-----------------|---------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| Chainsaw        | `OBJ010 WPNPICKUPCHAINSAW` with chainsaw SFX150–153 and execution ASID401 FINCHAINSAWH.[file:5][file:4]                  | High-risk melee with execution focus; slow approach, instant kill at close range with gore-heavy FIN sequences.                                     |
| Bazooka         | `OBJ011 WPNPICKUPBAZOOKA` heavy weapon; uses lime-gore impacts VFX111, PRT scorch marks, and heavy-carry ASID050.[file:5][file:4] | Denies corridors and structures; carrier cannot jump and moves slower; ammo scarce and placed in exposed positions.                                 |
| Katana          | `OBJ012 WPNPICKUPKATANA` plus ASID406 FINKATANASTAB and SEQ010 execution cam.[file:5][file:4]                             | Precision melee; strong flanking tool, trades reach for speed relative to Chainsaw; should be mirrored across sides on symmetrical maps.           |
| SMG             | Implied by `OBJ020 AMMOPICKUPSMG` and SMG weapon stats.[file:4][file:5]                                                  | Mid-range workhorse based on N64’s rapid-fire weapons; moderate recoil, good for suppression; never given as a default spawn weapon.               |
| Shotgun         | `OBJ021 AMMOPICKUPSHELLS` plus muzzle VFX110 and shell eject VFX113.[file:5][file:4]                                     | Close-quarters dominance; strong in bunkers and trenches; limited range and slower reloads; shells placed near, but not on top of, shotgun pickup. |
| Sniper rifle    | Backed by HUDRETICLESNIPER and zones tagged `ROLEVANTAGESNIPER`/tower zones.[file:4][file:5][file:15]                    | Sparse placements in towers and ridges; one rifle per team in most modes to avoid dominance; lens-sway and longer reload to keep BFD pacing.       |
| Flamethrower    | Uses SFX150–151 (pilot / loop) and high-cost continuous VFX for fire.[file:5][file:4]                                    | Area denial and zombie control; strong DoT but short range and limited fuel; must obey strict PRT/VFX budgets from the gore manager.[file:3]       |
| Pistol / sidearm| Default light weapon if desired; can be left out to match pure BFD behavior.[file:15]                                     | If included, damage and ammo must be tuned so that pickup weapons remain the main differentiator between players.                                   |

### 2.2 Alien Base and Special Weapons

| Weapon / Tool      | Registry Backbone                                                                                                 | Gameplay Notes                                                                                                                           |
|--------------------|--------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------|
| Plasma Rifle       | `pickupplasmariflehubcenter` in Alien Base entities; maps to WPNPLASMARIFLE id.[file:9][file:15]                 | High-accuracy, high-damage rifle tuned for Alien waves; ammo limited, central hub control point.                                        |
| Bio-spitter / acid | Uses MATACIDPOOL, SFX302 ENVACIDDRIPLOOP, VFX111 WPNIMPACTACIDSPLASH.[file:5][file:4]                             | Could be either a dedicated alien weapon or hazard emission; if a weapon, ammo must be scarce and damage moderate to avoid frustration. |
| Execution-only limbs| Alien tail whip `ASID411 FINALIENTAILWHIP`, zombie bite, Gregg scythe etc.[file:5][file:4]                       | Not normal weapons; triggered via executions and AI behaviors; referenced by ASID and SEQ, not as map pickups.                          |

Heavy weapons (Bazooka, gas canister, money bag, egg shards, blood vials) must always invoke ASID050 or equivalent heavy-carry state and respect RULHEAVYCARRYNOJUMP.[file:5][file:4][file:3]

---

## 3. Ammunition, Health, Armor, Utility

This section encodes all resource pickups that keep weapons and players in the fight. They are standard OBJ IDs and appear across multiple maps.[file:5][file:4]

### 3.1 Ammunition

| Item                     | OBJ / Registry Backbone                                                                                  | Placement Rules                                                                                                                       |
|--------------------------|----------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| Small ammo crate         | `OBJ002 OBJAMMOCRATESMALL`.[file:4][file:5]                                                             | Light weapons only; placed along flanks and mid-lanes; never directly under power weapons.                                           |
| Large ammo crate         | `OBJ003 OBJAMMOCRATELARGE`.[file:4]                                                                     | Supports heavy weapons; placed in riskier zones, often exposed or in contested midfield.                                             |
| SMG ammo pack            | `OBJ020 AMMOPICKUPSMG`.[file:4][file:5]                                                                 | Near SMG spawn lanes; should be mirrored and placed just behind main cover in corridors.                                            |
| Shotgun shell box        | `OBJ021 AMMOPICKUPSHELLS` when used as ammo.[file:4][file:5]                                            | Couples with shotgun corridors and breach points; placed retreat-side of doors, not entry-side.                                     |
| Special ammo (flamer, etc.) | Future OBJ ids mapped to specific WPN ids in weaponstatsv1.json.[file:3][file:4]                   | All special ammo must be exclusive to its weapon and placed such that that weapon remains a map-specific advantage.                 |

### 3.2 Health, Armor, Utility

| Item                  | OBJ Backbone                                                                               | Gameplay Notes                                                                                          |
|-----------------------|--------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------|
| Health box            | `OBJ001 OBJHEALTHBOX`.[file:4]                                                             | Generic crate; moderate heal; appears in mid-risk, not on objectives; core to all modes.              |
| Chocolate bar         | `OBJ030 HEALTHPICKUPCHOCOLATE`.[file:5][file:4]                                            | N64-faithful; small to medium heal; used sparingly in safe rooms and side corridors.                   |
| Armor vest            | `OBJ004 OBJARMORVEST`.[file:4]                                                             | Substantial damage soak; placed at higher-risk vantage points or deep behind lines.                    |
| Gas mask pickup       | New OBJ id (e.g., `OBJ0xx OBJGASMASKPICKUP`).                                             | Grants temporary gas immunity tags and HUD indicator; appears in Fortress/Alien Base gas zones only.   |
| Utility (boots, etc.) | Future OBJ ids for anti-slip, anti-acid, etc., tied to specific ROLEHAZARDDAMAGEOVERTIME.[file:5][file:3] | Keep durations short and map-specific; no global progression or stacking beyond one utility at a time. |

Health and armor should never be tied to classes; all players must have equal access based on map control, matching Bad Fur Day’s philosophy.[file:15]

---

## 4. Objective Items and Environment “Ordnance”

Objective items are heavy-carry pickups and interactables that drive win conditions. Ordnance covers explosive charges, switches, and environmental hazard triggers.[file:5][file:4][file:15]

### 4.1 Heavy-Carry Objective Items

| Objective           | Registry Backbone                                                                                                   | Mechanics                                                                                                                           |
|---------------------|----------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| Gas canister        | `OBJ031 OBJGASCANISTEROBJECTIVE` with GMR Total War / Tanks modes.[file:4][file:5]                                  | Delivered to a base or chamber to trigger SEQINSTANTWINGAS; activates ROLEHAZARDAIRLOCK / ROLEHAZARDGAS volumes.[file:4][file:15]  |
| Money bag           | Heist bag; dedicated OBJ id plus `GMRHEIST*` modes.[file:4][file:15]                                                | Heavy-carry; no jump; objective of Heist-style maps; spawn and score zones defined via ROLEOBJECTIVEITEM/ROLEOBJECTIVECAPTURE.     |
| Blood vial          | `OBJ072 OBJBLOODCOUNTBLOODVIAL` with altar and Fire Imp logic.[file:4][file:5]                                      | Dropped on death; must be brought to `OBJ071 OBJBLOODCOUNTALTAR`; Fire Imp AI keyed to vial-carrier using AI/ASID tags.            |
| Alien Egg / shards  | `OBJ042 OBJALIENEGG` plus `pickupalieneggshardcenter` in Alien Base entities.[file:9][file:4]                       | Shards use heavy-carry ASID050; delivering enough shards or destroying egg completes Invasion; ties directly to SEQALIENBASE*.[file:15] |

All such items must set heavy-carry state (ASID050) and obey RULHEAVYCARRYNOJUMP and movement penalties specified in ASID metadata.[file:5][file:3]

### 4.2 Interactables and Charges

| Item / Prop             | OBJ Backbone                                                                                                       | Usage                                                                                                                                |
|-------------------------|----------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| Fence charges           | `OBJ021 OBJBEACHFENCECHARGE`.[file:4]                                                                               | Placed on Beach Dead fences; armed by attackers and detonated via plunger box.                                                     |
| Plunger detonation box  | `OBJ022 OBJBEACHPLUNGERBOX`.[file:4]                                                                               | Activates fence charges; SEQBEACHINTROASSAULT / escape sequences tie into door destruction and camera shakes.                     |
| Airlock switches        | `OBJ040 INTERACTSWITCHAIRLOCK`.[file:4][file:15]                                                                    | Toggles airlock doors and hub gas sequences in Alien Base; connected to ENVVENTSTEAMHISS and gas hazard volumes.                  |
| Blast doors             | `OBJ041 INTERACTDOORBLAST` (collision with OBJALIENEGG resolved in registry).[file:4][file:5]                      | Breachable doors linked to Bazooka or objective triggers; they gate powerful pickups or shortcut routes.                           |
| Stasis pods, props      | `OBJ043 OBJALIENSTASISPOD` and other decorative / bonus objective containers.[file:4]                               | Optional side objectives, bonus spawns, or narrative props; must not grant permanent perks or XP.                                  |

### 4.3 Environmental Hazards

Hazards are defined primarily through hazard volumes (`hazards` array in MapEntitiesV1) and ENV/SFX/PRT/VFX tags, not as held items.[file:9][file:5]

| Hazard Type        | Data Backbone                                                                                               | Behavior                                                                                                                               |
|--------------------|------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| Floor gas          | hazardType HubFloorGas, ROLEHAZARDHUBFLOORGAS, SFX301–304 ambience, SEQINSTANTWINGAS.[file:5][file:9][file:15] | Fills hub floor after canister or switch; instant-kill or rapid DoT; scripted Tanks/Heist style, not random circles.                 |
| Acid pools         | hazardType SublevelAcid, MATACIDPOOL, SFX302 ENVACIDDRIPLOOP.[file:5][file:9]                              | Damage-over-time zones; often guard heavy objectives or short-cuts; may couple with AI spawn triggers.                                |
| Artillery barrages | hazardType Artillery; hazard volumes tied to Beach Dead’s artillery hill zones.[file:12][file:5]           | Timed shelling of trenches; telegraphed via warning SFX and VFX; not constant; triggered or periodic.                                 |
| Fire Imp triggers  | hazardType FireImpTrigger; AI/ASID tags for AIFIREIMPHUNTER, SEQBLOODCOUNTIMPALERT HUD.[file:5][file:4]    | Spawn Fire Imp when specific conditions met (e.g., blood vial picked up); not run-and-gun class ability.                             |

All hazards must be scripted around mode objectives rather than free-form battle royale damage rings, to maintain Bad Fur Day’s pacing.[file:15]

---

## 5. Vehicles and Vehicle Pads

Vehicles are spawned via dedicated world objects (pads) and never via classes or loadouts.[file:15][file:4]

| Vehicle / Pad        | OBJ Backbone                                                                                           | Rules and Notes                                                                                                                  |
|----------------------|--------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| Tank spawn pad       | `OBJ060 VEHTANKSPAWNER`.[file:4][file:5]                                                               | Represents a single tank slot; vehicle spawns on use or periodically; appears in Tank and Canyon maps only.                     |
| Hoverboard / racer   | Future OBJ ids for Race modes; mapped to race track zones.[file:15]                                    | Vehicles are shared resources; map-limited; players dismount or lose them on death with normal spawn rules.                     |
| Special transports   | `OBJ023 OBJBEACHESCAPETRUCK` and similar objective vehicles.[file:4]                                   | Take role of extraction / victory vector; not player-controlled in most modes; use SEQ* outro sequences for match end.         |

Design constraint: vehicles must be balanced around map control and spawn timers, not class choice. They are map-level features.

---

## 6. HUD and Match Widgets

HUD items are not pickups but are treated as a formal family (HudId) to keep modes and items consistent. They connect directly to items via ammo icons, health bars, objective indicators, and cinematics.[file:4][file:3]

### 6.1 Core HUD Elements (Global)

| HUD Element         | HUD Backbone                                                                              | Purpose                                                                                                       |
|---------------------|-------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| Basic reticle       | `HUD001 HUDRETICLEDOT`.[file:5][file:4]                                                  | Default crosshair for hip-fire weapons; used when no mode-specific reticle override is set.                   |
| Crosshair reticle   | `HUD002 HUDRETICLECROSS`.[file:5][file:4]                                                | Used for SMG/shotgun when tighter cone is desired; matches N64 “crosshair” feel.                              |
| Sniper reticle      | `HUD003 HUDRETICLESNIPER`.[file:5][file:4]                                               | Scoped view for sniper rifles; overlay only while zoomed.                                                     |
| Ammo icon           | `HUD010 HUDICONAMMO`.[file:5][file:4]                                                    | Non-map-specific bullet icon for ammo counters.                                                               |
| Health icon         | `HUD011 HUDICONHEALTH`.[file:5][file:4]                                                  | Chocolate bar health icon; used in all modes.                                                                 |
| Armor icon          | `HUD012 HUDICONARMOR`.[file:5][file:4]                                                   | Armor vest; triggered when armor > 0; must not show when N64-accurate “no armor” modes are used.             |
| Killfeed row        | `HUD030 HUDKILLFEEDENTRY`.[file:5][file:4]                                               | Template for killfeed entries showing killer/weapon/victim; used across Deathmatch and team modes.            |

### 6.2 Mode- and Map-Specific HUD

| HUD Widget                | HUD Backbone                                                                                   | Mode / Map Context                                                                                                      |
|---------------------------|------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------|
| Beach fence status        | `HUDBEACHFENCESTATUS` (HUD01x band).[file:4]                                                   | Shows live/breached state of fences 1–3 on Beach Dead during assaults.                                                 |
| Beach escape arrow        | `HUDBEACHESCAPEARROW`.[file:4]                                                                 | Points attackers toward escape truck during final phase.                                                                |
| Fortress canister bar     | `HUD020 HUDFORTRESSCANISTERPROGRESS`.[file:4]                                                  | Shows capture/arm status for gas canister in Total War modes.                                                           |
| Alien egg integrity       | `HUD030 HUDALIENEGGSTATUS` band entry for Alien Base.[file:4][file:5]                          | Indicates egg health or shard threshold; used in Invasion modes.                                                        |
| Alien wave counter        | `HUD031 HUDALIENWAVECOUNTER`.[file:4]                                                          | Shows current wave number in Alien Base PvE-style variants.                                                             |
| Blood Count vial counter  | `HUD040 HUDBLOODCOUNTVIALCOUNTER`.[file:4][file:5]                                            | Displays how many vials have been delivered; may show max cap for ritual victory.                                       |
| Fire Imp alert            | `HUD041 HUDBLOODCOUNTIMPALERT`.[file:4]                                                       | Signals Fire Imp’s focus or proximity; triggered when certain objectives are carried or thresholds reached.             |
| Spamono corridor progress | `HUD050 HUDSPAMONOSECTIONPROGRESS`.[file:4]                                                   | Visualizes progress through corridor segments in TMS Spamono.                                                           |
| Match timer / score panel | Generic HUD IDs for timers and team score overlays per GMR mode.[file:4][file:3]              | All timed modes must specify match timer and default score panel HUD ids in their ModeProfileV1 JSON.                   |

Mode profiles should define their HUD layout explicitly (reticle choices, timer, objective widgets) instead of hardcoding HUD behavior.

---

## 7. Sequences (Cinematic Cameras) – Context for Items

Sequences are not items, but many items trigger sequences (e.g., canisters, eggs, executions). This section explains their relationship to the item system.[file:4][file:3]

| Sequence Type       | SEQ Backbone                                                                                       | Items / Events Involved                                                                                                 |
|---------------------|----------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------|
| Map intro flythrough| `SEQ001 SEQINTROMAPFLY`.[file:4]                                                                    | Plays at match start; independent of items but must frame key objectives and weapon areas.                             |
| Victory outro       | `SEQ002 SEQOUTROVICTORY`.[file:4]                                                                   | Plays when team wins by kills or objectives; may highlight carriers of key items (gas canister, vials, eggs).         |
| Instant gas win     | `SEQ003 SEQINSTANTWINGAS`.[file:5][file:4]                                                         | Activated by `OBJ031` or equivalent on delivery; used in Fortress, Tanks, Alien Base gas events.                       |
| Execution cam       | `SEQ010 SEQEXECUTIONCAM` and SEQ400/401 for specific executions.[file:4][file:5]                  | Tied to ASIDs like FINCHAINSAWH, FINALIENTAILWHIP; triggered when special kills occur.                                  |
| Map-specific intros | SEQBEACHINTROASSAULT, SEQALIENBASEINTROINVASION, SEQBLOODCOUNTINTRO, etc.[file:4][file:15]        | Bound to specific maps and modes; referenced from GMR/ModeProfileV1 as `introSeqId`/`victorySeqId` fields.             |

Items that trigger sequences must list `linkedSeqId` in their registry rows (`data/registry/registryobjects.csv`) so tools and AI-chat can trace the connection without code inspection.[file:4][file:3]

---

## 8. Design and Codegen Guidelines

This section captures the “rules of the road” that keep Live & Uncut faithful to Bad Fur Day and distinct from Live & Reloaded, and it gives AI-chat concrete levers to use when generating code and content.[file:15][file:4][file:3]

### 8.1 Global Rules

- All players start with the same baseline stats and minimal or no weapons; everything else is acquired via pickups and objectives, not via classes or XP.[file:15][file:4]  
- Heavy-carry items (Bazooka, gas canister, money bag, egg shard, blood vial) must enable ASID050 and respect RULHEAVYCARRYNOJUMP and movement penalties.[file:5][file:3]  
- Hazards (gas, acid, artillery, Fire Imp triggers) are always scripted around objectives; they must never be random circles or constant “battle royale” hazards.[file:15][file:5]  
- Censorship and gore are governed by RUL toggles (e.g., RULUNCENSOREDDIALOG, RULGORELEVELFULL), with the Uncut build defaulting to full gore/uncensored.[file:5][file:15]

### 8.2 Engine and Tooling Integration

- All items referenced in `mapentitiesv1.json` must use canonical OBJ ids; ASIDs, SFX, VFX, HUD, and SEQ ids must also come from enums and CSV registries, not raw strings.[file:9][file:4][file:3]  
- Engine emitters (Unreal, Unity, Godot) must treat ObjId, HudId, SeqId, AsidId, etc. as the only legal IDs when instantiating actors, prefabs, and sequences.[file:12][file:15]  
- AI-chat should query `data/registry/*.csv` first to discover legal ids and semantics, then generate contracts (MapEntitiesV1, ModeProfileV1, WeaponStatsV1) that reference those ids and validate via schemaguard.[file:4][file:3][file:9]

### 8.3 Suggested Next Steps

- Fill out `docs/conkerregistry/04-objects-hud-seq.md` and associated CSVs so every item, HUD widget, and sequence in this document has a concrete registry row.[file:4][file:3]  
- Wire ObjId, HudId, SeqId, AsidId, SfxId, VfxId into all relevant schemas, so AI-generated JSON can be validated strictly against the registries before calling grid2scene or engine emitters.[file:9][file:12][file:3]  
- For each map (Beach Dead, Fortress, Alien Base, Blood Count, Heist, Raptor Temple, TMS Spamono), create a dedicated item-layout doc (e.g., `docs/multiplayer/01MultiplayerBeachDeadItems.md`) referencing this overview to keep per-map item setups consistent.
