## Execution Animation States (ASID)

- ASID_400 (FIN_CHAINSAW_V)
  - Role: Chainsaw execute (vertical).
  - Lock: Hard Lock, Interrupt Priority 0.
  - Gore Trigger: Frame 42 → GORE_DECAP (head mesh swap + VFX_015 + PRT_GORE_*).

- ASID_405 (FIN_SABRE_H)
  - Role: Katana decap, 360° sweep.
  - Lock: Hard Lock, Interrupt Priority 0.
  - Hit Shape: Radius 1.5 units from attacker origin; applies instant kill on valid targets.

- ASID_666 (SPEC_GREGG_REAP)
  - Role: Gregg scythe reap, unique to Gregg.
  - Lock: Hard Lock.
  - Kill Condition: Neck hitbox overlap; instant kill, spawns PRT_GORE_RED.

- ASID_900 (ALN_POUNCE_STRK)
  - Role: Alien pounce.
  - Lock: Movement override; transitions to ASID_901 on hit.

- ASID_901 (ALN_BITE_EXEC)
  - Role: Alien facebite execution.
  - Gore Trigger: Frame N (e.g. 60) → head replacement with OBJ_GORE_MUSH + VFX_010.

- ASID_050 (HLD_HEAVY_WALK)
  - Role: Heavy carry walk cycle.
  - Effect: No Jump flag, move speed × 0.6.

- ASID_012 (HIT_STUN_DAZE)
  - Role: Stun lock.
  - Duration: 1.5 seconds; cancels movement input and reduces turn speed.
