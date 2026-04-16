# 04_Multiplayer_Alien_Base – Triggers & Hazards

This file specifies the Airlock/Gas hazard state machine, timing, and damage model for the Alien Base multiplayer map, inspired by the gas canister mechanics from Total War and Tank in Conker’s Bad Fur Day while remaining compatible with modern engines and a 16‑player cap.

## Design Philosophy

The hazard system is a controllable “Tanks‑style” map event: rare, readable, and powerful, not a random battle royale storm. Players trigger events through consoles or objectives, which start a countdown; after a brief warning, lethal gas or vacuum floods low areas, forcing everyone onto catwalks and upper corridors like bunkers in the original Tank mode.

This preserves several core traits of the N64 multiplayer:

- Objective‑driven wipes as comeback tools.  
- Clear telegraphing and known safe zones.  
- Minimal HUD dependency; players learn by sight and sound.

## Named Trigger & Volume Anchors

These names should line up with scene objects in Unreal/Unity/Godot and with the main layout defined in `04_Multiplayer_Alien_Base.md`.

### Triggers (Interactables)

- `trigger_airlock_north` – Console near north corridor entrance, around `(0.0, 0.0, 60.0)`.  
- `trigger_airlock_south` – Console near south corridor entrance, around `(0.0, 0.0, -60.0)`.  
- `trigger_emergency_purge_east` – Optional console above sub‑level access at `(60.0, 5.0, 0.0)`.  
- `trigger_emergency_purge_west` – Mirror console above sub‑level access at `(-60.0, 5.0, 0.0)`.

Each trigger is a single‑use input during an event cycle, broadcasting a map event when activated.

### Hazard Volumes

- `hazard_hub_floor_gas`  
  - Coverage: floor ring in the Central Hub, roughly radius 25–30 units around `(0, 0, 0)` between `Y = -2.0` and `Y = 4.0`.  
  - Effect: low‑lying toxic gas or vacuum damage; the catwalk ring above this height is safe.

- `hazard_sublevel_acid`  
  - Coverage: sub‑level tunnels around `(0, -10, 40)`, radius ~15 units between `Y = -12.0` and `Y = -6.0`.  
  - Effect: concentrated acid vapor or coolant; punishes camping on heavy weapon routes.

Exact shapes can be box, cylinder, or mesh‑based volumes depending on engine, as long as they match the design intent of “floor bad, high ground safe” during events.

## Airlock/Gas State Machine

The Airlock system is modeled as a shared map‑level finite state machine. All hazard volumes and triggers reference this single controller to avoid overlapping events.

### State Definitions

- `Idle` – No hazard active; all triggers are ready.  
- `Arming` – Event has been triggered; countdown is running; warning FX active.  
- `Active` – Hazard volumes are live; damage is applied on tick.  
- `Cooldown` – Hazard is off; triggers temporarily disabled; system waits before allowing another event.

### Timing Parameters (Initial Targets)

These are “Uncut‑feeling” starting values you can tune after playtests:

- `arming_duration_sec` = 5.0  
  - Short warning countdown before gas/airlock release.

- `active_duration_sec` = 12.0  
  - Long enough to change positioning without locking players out of the hub for an entire match.

- `cooldown_duration_sec` = 30.0  
  - Prevents repeated spam and keeps events special.

- `damage_per_second_floor` = 60 HP/sec  
  - About 1.7 seconds to kill a stationary player, giving a small window to escape.

- `damage_per_second_sublevel` = 90 HP/sec  
  - About 1.1 seconds to kill, intentionally punishing sub‑level camping.

You can scale absolute numbers by your engine’s default health values while preserving these relationships.

### State Transitions (Pseudo‑Logic)

Conceptual behavior (engine‑agnostic):

1. **Idle → Arming**  
   - Condition: Valid trigger used (`trigger_airlock_north` or `trigger_airlock_south`) AND not currently in `Arming` / `Active` / `Cooldown`.  
   - Actions:  
     - Record `event_instigator_team` for scoreboard/announcements.  
     - Start `arming_timer`.  
     - Play sirens, flashing lights, VO warnings, and vent FX.  

2. **Arming → Active**  
   - Condition: `arming_timer >= arming_duration_sec`.  
   - Actions:  
     - Enable `hazard_hub_floor_gas` and `hazard_sublevel_acid`.  
     - Start `active_timer`.  
     - Update VO and UI to “Airlock sealed / gas released”.

3. **Active → Cooldown**  
   - Condition: `active_timer >= active_duration_sec`.  
   - Actions:  
     - Disable all hazard volumes.  
     - Start `cooldown_timer`.  
     - Play venting FX and VO to indicate safe conditions.

4. **Cooldown → Idle**  
   - Condition: `cooldown_timer >= cooldown_duration_sec`.  
   - Actions:  
     - Reset timers.  
     - Re‑enable trigger interactables.  
     - Clear `event_instigator_team`.

## Damage Application Model

Damage is applied per‑tick or per‑second to characters and AI whose origin (or defined damage sample point) lies inside a live hazard volume.

### Core Rules

- **Tick Damage**  
  - `damage_this_tick = damage_rate_per_sec * delta_time`.  
  - Apply separately for floor and sub‑level volumes.

- **Stacking**  
  - If a character is simultaneously inside overlapping volumes (floor + sub‑level), use the higher damage rate rather than stacking both, to keep balancing simpler.

- **Immunity / Gear**  
  - Optionally allow certain pickups (e.g., a “Gas Mask” equivalent) to negate or reduce damage, mirroring bunker/gas mask behavior in earlier modes.

- **Aliens vs Players**  
  - In PvE‑heavy Invasion variants, aliens can be immune or partially resistant, turning hazards into tools against human teams, or vice versa.

### Recommended Values

Assuming default infantry health of 100 HP:

- Floor gas: lethal in ~1.7 seconds if a player remains standing still, but survivable with immediate sprint to a ladder or corridor.  
- Sub‑level acid: lethal in ~1.1 seconds, strongly discouraging heavy‑weapon camping.

These numbers echo the “instantly kill everyone in the open” tone of original gas events while respecting that players are on foot with less armor.

## Execution vs Hazard Priority

Alien Base uses cinematic execution animations (chainsaw, sabre, alien bite) that should not be randomly interrupted by map hazards.

- While an actor is in a FIN_* execution state (e.g. `ASID_400`, `ASID_405`, `ASID_901`), periodic hazard damage is suspended for that actor until:
  - The Gore Trigger frame has passed, or  
  - The animation exits the Hard Lock state.

- This allows executions to complete cleanly even during active Airlock events, preserving the “brutal” feel without randomizing kill windows.

Hazard volumes should query the character’s current animation state or a simple `IsInExecutionState()` flag before applying damage.

## Trigger Interaction Rules

Triggers need to be simple and consistent across engines, and readable to split‑screen players.

### Use Constraints

- Global: Only one Airlock event can be in `Arming` or `Active` at a time.  
- Per‑Trigger: All triggers share the same global cooldown; once one is used, all are disabled until the state machine returns to `Idle`.  
- Team Ownership (Optional):  
  - In team modes, only the team controlling a nearby objective or console can activate a trigger.  
  - In FFA modes, any player may activate.

### UX Behavior

- Interact range: 1.5–2.0 meters from the console.  
- Hold vs tap:  
  - Tap or short hold (≤ 0.5 sec) is preferred to keep pacing snappy.  
- Feedback:  
  - Clear “arming” VO (“Airlock cycle initiated!”) and HUD/announcement label showing which team triggered the event.

## Example Logic Sketch (Engine‑Agnostic)

This describes how you might structure the controller in C++/C#/GDScript, without binding to a specific API.

### Airlock Controller Structure

Conceptual data:

- `state: enum { Idle, Arming, Active, Cooldown }`  
- `time_in_state: float`  
- `event_instigator_team: int or None`  
- `arming_duration_sec: float`  
- `active_duration_sec: float`  
- `cooldown_duration_sec: float`  
- `damage_per_second_floor: float`  
- `damage_per_second_sublevel: float`

Core methods:

- `RequestTriggerActivation(trigger_id, instigator_team)`  
- `Update(delta_time)`  
- `ApplyHazardDamage(delta_time)`  

The key requirement is that the MD file’s state names and parameters match what you expose to tooling or AI‑chat helpers, so changes remain transparent and versionable.

## Integration with Game Modes

The same state machine can power multiple rule variants without changing geometry.

### Deathmatch / Standard Multi

- Hazards are pure map‑control tools: no team ownership, any player can trigger.  
- Optional score bonus to the instigator for kills caused during the active phase.

### Invasion (Alien‑Focused)

- Aliens may automatically trigger events when they overrun the hub for a set period (e.g., 30 seconds of uncontested control).  
- Player‑controlled triggers remain, allowing defenders to pre‑emptively purge certain areas at the cost of temporary map access.

### Objective Variants

- Optional: tie Airlock events to progress on an Alien Egg health bar or charge level, using state transitions as telegraphed “phase changes” during a longer match.

## Balancing Notes

To keep the experience aligned with N64 and early Live & Uncut intent:

- Keep events rare and memorable; the cooldown must be long enough that players do not feel locked out of the hub.  
- Maintain at least one fully safe path between any spawn zone and the hub via catwalks or high corridors during events.  
- Never spawn players directly into a live hazard volume; if necessary, push spawns to the far end of their corridors when `state == Active`.
