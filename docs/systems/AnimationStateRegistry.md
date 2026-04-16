# Animation State Registry (ASID)

This document is the master registry for all Animation State IDs (ASIDs) used by Uncut multiplayer characters and NPCs.  
Each ASID entry is engine‑agnostic: Unreal, Unity, and Godot map these IDs to their own animation assets via engine‑local mapping files.

---

## Table format

For each ASID, define:

- **ASID**: Three‑digit numeric identifier.
- **Internal Name**: Short code used in code and tools.
- **Role**: High‑level description of the move or state.
- **Lock Type**: How strongly the state locks the actor (e.g., Hard Lock, Movement Lock, Soft Lock).
- **Interrupt Priority**: Lower means harder to interrupt (0 is non‑interruptible during active window).
- **Gore Trigger Frame**: Animation frame where gore or kill logic fires (if applicable).
- **Duration**: Typical duration in seconds (if applicable).
- **SFX IDs**: Primary sound effect IDs.
- **VFX IDs**: Primary visual effect IDs.
- **Notes**: Extra behavior details.

---

## ASID‑012 HIT_STUN_DAZE

- **ASID**: 012  
- **Internal Name**: HITSTUN_DAZE  
- **Role**: Stun lock after heavy impact; cancels movement and turning for a brief window.  
- **Lock Type**: Soft Lock (movement and look damped; limited input allowed).  
- **Interrupt Priority**: 2 (can be overridden by executions and higher‑priority states).  
- **Gore Trigger Frame**: N/A  
- **Duration**: 1.5 seconds (tunable per weapon).  
- **SFX IDs**: SFX110 (stun impact grunt), SFX111 (ringing ears loop).  
- **VFX IDs**: VFX030 (screen vignette / stars).  
- **Notes**:  
  - Applied by heavy weapons on non‑lethal body hits.  
  - While active, movement speed is reduced and camera turn rate is clamped.  

---

## ASID‑050 HLD_HEAVY_WALK

- **ASID**: 050  
- **Internal Name**: HLD_HEAVY_WALK  
- **Role**: Heavy carry locomotion; used when carrying large objectives or heavy weapons.  
- **Lock Type**: Movement Mode Lock (overrides base locomotion; does not hard‑lock animations).  
- **Interrupt Priority**: 3 (low; can be overridden by stun and executions).  
- **Gore Trigger Frame**: N/A  
- **Duration**: As long as the heavy item is held.  
- **SFX IDs**: SFX120 (heavy footstep loop), SFX121 (strain grunts).  
- **VFX IDs**: None mandatory.  
- **Notes**:  
  - Enforces **No Jump** flag and reduces movement speed to ~60% of baseline.  
  - Weapon firing is restricted based on weapon rules (e.g., no dual‑wield, slower ADS).  

---

## ASID‑400 FIN_CHAINSAW_V

- **ASID**: 400  
- **Internal Name**: FIN_CHAINSAW_V  
- **Role**: Chainsaw vertical execution; single‑target decapitation / bisect.  
- **Lock Type**: Hard Lock (attacker and victim locked until gore frame resolves).  
- **Interrupt Priority**: 0 (cannot be interrupted by normal gameplay once committed).  
- **Gore Trigger Frame**: 42  
- **Duration**: ~1.0–1.2 seconds total (from wind‑up to follow‑through).  
- **SFX IDs**: SFX200 (chainsaw rev), SFX201 (impact), SFX800 (gore splat).  
- **VFX IDs**: VFX010 (lime‑green gore burst), VFX015 (decap spurts).  
- **Notes**:  
  - During frames before the Gore Trigger, the victim is invulnerable to map hazards (e.g., gas) to avoid random cancels.  
  - On the Gore Trigger frame, swap victim head mesh to gore mesh and apply instant kill.  

---

## ASID‑405 FIN_SABRE_H

- **ASID**: 405  
- **Internal Name**: FIN_SABRE_H  
- **Role**: Katana horizontal execution; 360‑degree decap sweep.  
- **Lock Type**: Hard Lock.  
- **Interrupt Priority**: 0.  
- **Gore Trigger Frame**: 38  
- **Duration**: ~1.0 seconds.  
- **SFX IDs**: SFX210 (blade whoosh), SFX211 (impact), SFX801 (multi‑gore slice).  
- **VFX IDs**: VFX011 (arc slash trail), VFX016 (multi‑head spray).  
- **Notes**:  
  - Applies instant kill to valid targets within 1.5 units radius of the attacker during a 2–3 frame window around the Gore Trigger.  
  - Map hazards ignore the victim during the locked window, same as other FIN states.  

---

## ASID‑666 SPEC_GREGG_REAP

- **ASID**: 666  
- **Internal Name**: SPEC_GREGG_REAP  
- **Role**: Gregg’s scythe reap execution; unique special finisher.  
- **Lock Type**: Hard Lock.  
- **Interrupt Priority**: 0.  
- **Gore Trigger Frame**: 50  
- **Duration**: ~1.4 seconds.  
- **SFX IDs**: SFX300 (scythe swing), SFX301 (soul rip), SFX802 (heavy gore).  
- **VFX IDs**: VFX020 (soul trail), VFX017 (large gore plume).  
- **Notes**:  
  - Kill condition is neck hitbox overlap during the reap window.  
  - Always spawns a stronger gore effect (e.g., red variant) to distinguish from standard executes.  

---

## ASID‑900 ALN_POUNCE_STRK

- **ASID**: 900  
- **Internal Name**: ALN_POUNCE_STRK  
- **Role**: Alien pounce strike; high‑velocity leap that transitions to facebite on hit.  
- **Lock Type**: Movement Lock (attacker movement overridden; not full hard lock until hit confirmed).  
- **Interrupt Priority**: 1 (can be interrupted by some high‑priority hits before contact).  
- **Gore Trigger Frame**: N/A (no gore; sets up ASID901).  
- **Duration**: ~0.6 seconds (from leap start to contact).  
- **SFX IDs**: SFX400 (alien screech), SFX401 (pounce impact).  
- **VFX IDs**: VFX040 (motion blur streak), optional camera shake.  
- **Notes**:  
  - If the target is within ~2.0 units at the impact window, both attacker and victim snap into ASID901.  
  - While the leap is active, the alien ignores normal movement input.  

---

## ASID‑901 ALN_BITE_EXEC

- **ASID**: 901  
- **Internal Name**: ALN_BITE_EXEC  
- **Role**: Alien facebite execution; cinematic finisher after a successful pounce.  
- **Lock Type**: Hard Lock (attacker and victim).  
- **Interrupt Priority**: 0.  
- **Gore Trigger Frame**: 60  
- **Duration**: ~1.2 seconds.  
- **SFX IDs**: SFX402 (bite crunch), SFX403 (scream cut‑off), SFX803 (facebite gore).  
- **VFX IDs**: VFX010 (lime‑green gore), VFX018 (head burst).  
- **Notes**:  
  - At the Gore Trigger frame, replace victim head with OBJ_GORE_MUSH and apply instant kill.  
  - Hazard volumes must not tick damage on attacker or victim while this ASID is active and locked.  

---

## ASID‑920 ZMB_CRAWL_MOVE

- **ASID**: 920  
- **Internal Name**: ZMB_CRAWL_MOVE  
- **Role**: Zombie crawl locomotion; post‑maim state after heavy body damage.  
- **Lock Type**: Movement Mode Lock (changes locomotion, not a one‑shot execution).  
- **Interrupt Priority**: 2 (can be elevated to executions or hard locks).  
- **Gore Trigger Frame**: N/A  
- **Duration**: Until killed (e.g., headshot) or despawned.  
- **SFX IDs**: SFX500 (crawl groan loop), SFX501 (dragging limbs).  
- **VFX IDs**: VFX050 (blood smear trail, optional decals).  
- **Notes**:  
  - Entered when a zombie takes enough body/limb damage without a lethal headshot.  
  - Movement speed is very low; hitbox is adjusted closer to the ground, and only specific weapons / headshots can finish the zombie.  

---

## Execution and Hazard Priority Rule

- While an actor is in any **execution ASID** (FIN_ prefix or SPEC execution) and is in the **Hard Lock** phase, environmental hazard damage (e.g., gas volumes, acid) is suspended for that actor until:  
  - The Gore Trigger frame has passed and any kill logic is resolved, or  
  - The animation exits the Hard Lock state.  

Execution ASIDs currently include: **400**, **405**, **666**, **901**.  
Additional executions should be added to this table and to the helper functions in each engine.
