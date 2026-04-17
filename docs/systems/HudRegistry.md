# HUD Registry

## Overview

The HUD Registry defines all Heads-Up Display elements for Uncut Multiplayer. Following **N64 design principles**, the HUD is minimal: no XP bars, no perk icons, no class clutter. Only essential combat information is displayed.

---

## HUD ID Ranges

| Range | Category | Description |
|-------|----------|-------------|
| 001–009 | Reticles | Weapon crosshairs |
| 010–019 | Counters | Health, armor, ammo numeric displays |
| 020–029 | Icons | Pickup, objective, waypoint icons |
| 030–039 | Killfeed | Kill notification elements |
| 099 | Special | Match events, sudden death overlays |

---

## Registered HUD Elements

### Reticles (HUD_001–009)

#### HUD_001 – RETICLE_PISTOL
- **Category**: ui
- **Description**: Pistol crosshair reticle
- **Usage Notes**: Default sidearm; N64-style simple dot; no spread indicator
- **n64_ancestor**: `BFD_HUD_RETICLE_PISTOL`
- **Weapon Mapping**: Pistol, Revolver

#### HUD_002 – RETICLE_SHOTGUN
- **Category**: ui
- **Description**: Shotgun spread reticle
- **Usage Notes**: Dynamic spread based on movement; mimics BFD layout; expands when moving
- **n64_ancestor**: `BFD_HUD_RETICLE_SHOTGUN`
- **Weapon Mapping**: Shotgun, Auto-Shotgun

#### HUD_003 – RETICLE_CHAINGUN
- **Category**: ui
- **Description**: Chaingun heavy weapon reticle
- **Usage Notes**: Large circular reticle; expands during sustained fire; red at max spread
- **n64_ancestor**: `BFD_HUD_RETICLE_CHAINGUN`
- **Weapon Mapping**: Chaingun, Minigun

#### HUD_004 – RETICLE_BAZOOKA
- **Category**: ui
- **Description**: Bazooka rocket launcher reticle
- **Usage Notes**: Simple circle with range tick marks; no lead indicator
- **n64_ancestor**: `BFD_HUD_RETICLE_BAZOOKA`
- **Weapon Mapping**: Bazooka, Rocket Launcher

#### HUD_005 – RETICLE_FLAMETHROWER
- **Category**: ui
- **Description**: Flamethrower cone indicator
- **Usage Notes**: Arc-shaped reticle showing burn cone; no traditional crosshair
- **n64_ancestor**: null
- **Weapon Mapping**: Flamethrower

#### HUD_006 – RETICLE_KATANA
- **Category**: ui
- **Description**: Katana melee swing indicator
- **Usage Notes**: Small center dot only; swing arc shown via VFX_112 trail
- **n64_ancestor**: null
- **Weapon Mapping**: Katana, Chainsaw

---

### Counters (HUD_010–019)

#### HUD_010 – COUNTER_HEALTH
- **Category**: ui
- **Description**: Health counter display
- **Usage Notes**: N64-style numeric + chocolate bar (if health is chocolate pickup); **no XP bars**
- **n64_ancestor**: `BFD_HUD_HEALTH`
- **Display Format**: `###` (0-999)
- **Visual Style**: White numbers; optional brown chocolate bar underneath

#### HUD_011 – COUNTER_ARMOR
- **Category**: ui
- **Description**: Armor/vest counter display
- **Usage Notes**: Numeric display; vest icon; **no class-based armor systems**
- **n64_ancestor**: `BFD_HUD_ARMOR`
- **Display Format**: `##` (0-100)
- **Visual Style**: Green numbers; vest icon to left

#### HUD_012 – COUNTER_AMMO
- **Category**: ui
- **Description**: Ammo counter display
- **Usage Notes**: Current/magazine format; mimics BFD bottom-right placement
- **n64_ancestor**: `BFD_HUD_AMMO`
- **Display Format**: `## / ###` (current clip / total reserve)
- **Visual Style**: Yellow numbers; low ammo (<10%) flashes red

---

### Icons (HUD_020–029)

#### HUD_020 – ICON_OBJ_ALIEN_EGG
- **Category**: ui
- **Description**: Alien Egg objective marker
- **Usage Notes**: Pulsing icon over OBJ_050; distance readout
- **n64_ancestor**: null
- **Map Usage**: 04MultiplayerAlienBase

#### HUD_021 – ICON_OBJ_VAULT_KEYCARD
- **Category**: ui
- **Description**: Vault keycard carry indicator
- **Usage Notes**: Shows above player head when carrying OBJ_051
- **n64_ancestor**: `BFD_HUD_KEY_ICON`
- **Map Usage**: 02MultiplayerTheHeist

#### HUD_022 – ICON_WAYPOINT_TEAM
- **Category**: ui
- **Description**: Team waypoint marker
- **Usage Notes**: Player-placed ping; visible to team only
- **n64_ancestor**: null
- **Map Usage**: All maps

---

### Killfeed (HUD_030–039)

#### HUD_030 – KILLFEED_ENTRY
- **Category**: ui
- **Description**: Kill notification element
- **Usage Notes**: `[Killer Icon] [Killer Name] → [Victim Name] [Victim Icon]`; **no XP gain shown**
- **n64_ancestor**: `BFD_HUD_KILLFEED`
- **Display Duration**: 4 seconds per entry
- **Style Constraints**: No perk icons, no class badges, no XP numbers

#### HUD_031 – KILLFEED_HEADSHOT
- **Category**: ui
- **Description**: Headshot kill variant
- **Usage Notes**: Same as HUD_030 but with skull icon; **no bonus XP text**
- **n64_ancestor**: null
- **Display Duration**: 4 seconds

---

### Special Overlays (HUD_099)

#### HUD_099 – OVERLAY_SUDDEN_DEATH
- **Category**: cinematic
- **Description**: Sudden Death screen overlay
- **Usage Notes**: Red vignette + pulsing text; triggered by SEQ_002
- **n64_ancestor**: `BFD_HUD_SUDDEN_DEATH`
- **Display Duration**: Until match end

---

## Style Constraints (N64 Fidelity)

Per roadmap requirements (**RO-103**, **RO-203**), the HUD enforces:

1. **No XP Bars**: Experience systems are excluded from Uncut Multiplayer.
2. **No Perk Icons**: No class abilities, no cooldown wheels.
3. **Pickups Only**: Weapons and items spawn as world objects (OBJ_010–060), not loadout selections.
4. **Numeric Simplicity**: Health/Armor/Ammo are raw numbers, not segmented bars.
5. **Killfeed Clarity**: Only killer/victim names and weapon icons; no assist tracking, no score popups.
6. **Chocolate Bar Optional**: Health may show chocolate bar visual (legacy) but still displays numeric value.

---

## Engine Integration

### UE5 Widget Binding
```cpp
// Map HUD IDs to UMG widgets
void UUncutHUD::BindWidgets()
{
    WidgetMap.Add("HUD_010", HealthCounterWidget);
    WidgetMap.Add("HUD_011", ArmorCounterWidget);
    WidgetMap.Add("HUD_012", AmmoCounterWidget);
    WidgetMap.Add("HUD_001", PistolReticleWidget);
    WidgetMap.Add("HUD_002", ShotgunReticleWidget);
    // ... etc
}
```

### Unity Canvas Reference
```csharp
public enum HudElementId {
    ReticlePistol = 1,
    ReticleShotgun = 2,
    CounterHealth = 10,
    CounterArmor = 11,
    CounterAmmo = 12,
    KillfeedEntry = 30
}
```

---

## Cross-Reference

- `avregistryv1.json` – AV IDs used by HUD (none directly; HUD is UI layer only)
- `objregistryv1.json` – Objects that trigger HUD icons (OBJ_050 Egg, OBJ_051 Keycard)
- `weaponstatsv1.json` – Weapon-to-reticle mapping (hud_icon_id field)
- `AudioVisualRegistry.md` – General AV registry documentation

---

## Files

| File | Purpose |
|------|---------|
| `docs/systems/HudRegistry.md` | This documentation |
| `data/av/avregistryv1.json` | HUD entries in AV registry |
| `schemas/systems/avregistryv1.schema.json` | Schema validation |
