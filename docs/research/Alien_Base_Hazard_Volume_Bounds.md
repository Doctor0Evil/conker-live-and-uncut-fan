# Research Question 35: Alien Base Hazard Volume Geometric Bounds

## Objective
Define precise geometric bounds (radius, height, world units) for hazard volumes to ensure correct grid cell coverage in the Alien Base multiplayer map.

## Coordinate System Reference
- **Origin**: Central Hub at `(0, 0, 0)`
- **Cell Size**: 4.0 world units per grid cell
- **Grid Dimensions**: 17×17 cells (68×68 world units total)
- **Y-Axis**: Vertical (height)

---

## Hazard Volume 1: `hazard_hub_floor_gas`

### Design Intent
Low-lying toxic gas or vacuum that covers the floor ring of the Central Hub, sparing the elevated catwalk ring above. Punishes players caught on the ground level during airlock events.

### Geometric Specifications

#### Option A: Cylinder Volume (Recommended)
```
Shape: Cylinder
Center: (0.0, 1.0, 0.0)
Radius: 28.0 world units (7.0 cells)
Height Min: -2.0 world units
Height Max: 4.0 world units
Total Height: 6.0 world units
```

#### Option B: Box Volume
```
Shape: Box
Center: (0.0, 1.0, 0.0)
Extent X: 28.0 world units
Extent Y: 3.0 world units (half-height)
Extent Z: 28.0 world units
Bounds: X[-28, 28], Y[-2, 4], Z[-28, 28]
```

### Grid Cell Coverage
The volume should cover these grid cells (col, row) centered at (8, 8):
- **Inner Ring** (radius 3 cells): All cells within distance ≤3 from center
  - (8,5), (8,6), (8,7), (8,8), (8,9), (8,10), (8,11)
  - (5,8), (6,8), (7,8), (9,8), (10,8), (11,8)
  - Diagonals: (6,6), (6,10), (10,6), (10,10), etc.
  
- **Outer Ring** (radius 5 cells, max coverage): All cells within distance ≤5 from center
  - Extends to columns 3-13 and rows 3-13 (with circular clipping)

### Damage Parameters
- **Damage Rate**: 60 HP/sec (lethal in ~1.7 seconds for 100 HP player)
- **Tick Interval**: 0.5 seconds (30 HP per tick)
- **Immunity**: Catwalk elevation (Y > 4.0) is safe

### Visual Feedback
- Gas emitter height: Y = 0.0 to 3.0
- Warning particles: Activate during Arming state at Y = 3.5
- Safe zone indicator: Catwalk railings glow green during Cooldown

---

## Hazard Volume 2: `hazard_sublevel_acid`

### Design Intent
Concentrated acid vapor or coolant leak in the sub-level maintenance tunnels. Faster lethality than floor gas, specifically punishing players camping on heavy weapon routes.

### Geometric Specifications

#### Option A: Cylinder Volume (Recommended)
```
Shape: Cylinder
Center: (0.0, -9.0, 40.0)
Radius: 16.0 world units (4.0 cells)
Height Min: -12.0 world units
Height Max: -6.0 world units
Total Height: 6.0 world units
```

#### Option B: Box Volume
```
Shape: Box
Center: (0.0, -9.0, 40.0)
Extent X: 16.0 world units
Extent Y: 3.0 world units (half-height)
Extent Z: 16.0 world units
Bounds: X[-16, 16], Y[-12, -6], Z[24, 56]
```

### Grid Cell Coverage
The volume centers around sub-level access near column 8, row 4-6:
- **Primary Coverage**: Cells within radius 4 from (8, 4)
  - Columns 4-12, Rows 1-7 (with circular/elliptical clipping)
  - Specifically covers: (8,2), (8,3), (8,4), (8,5), (8,6)
  - Heavy weapon pickup at (8, 1) should be at edge of volume

### Damage Parameters
- **Damage Rate**: 90 HP/sec (lethal in ~1.1 seconds for 100 HP player)
- **Tick Interval**: 0.5 seconds (45 HP per tick)
- **Immunity**: Upper levels (Y > -6.0) are safe

### Visual Feedback
- Acid mist color: Green-yellow (lime #32CD32)
- Bubble emitters: Spaced every 4 world units within volume
- Corrosion decals: Appear on walls during Active state

---

## Engine-Specific Implementation Notes

### Unreal Engine
```cpp
// Cylinder component for hub floor gas
UCylinderComponent* HubGasCylinder = CreateDefaultSubobject<UCylinderComponent>(TEXT("HubGasCylinder"));
HubGasCylinder->SetRelativeLocation(FVector(0.0f, 1.0f, 0.0f));
HubGasCylinder->SetCylinderHalfExtents(28.0f, 3.0f); // Radius, Half-Height

// Box component for sublevel acid
UBoxComponent* SublevelAcidBox = CreateDefaultSubobject<UBoxComponent>(TEXT("SublevelAcidBox"));
SublevelAcidBox->SetRelativeLocation(FVector(0.0f, -9.0f, 40.0f));
SublevelAcidBox->SetBoxExtent(FVector(16.0f, 3.0f, 16.0f));
```

### Unity
```csharp
// Cylinder collider (custom mesh or capsule approximation)
var hubGasCollider = gameObject.AddComponent<SphereCollider>();
hubGasCollider.center = new Vector3(0.0f, 1.0f, 0.0f);
hubGasCollider.radius = 28.0f;
// Use PhysicsMaterial to define vertical bounds via trigger logic

// Box collider for sublevel
var acidCollider = sublevelObject.AddComponent<BoxCollider>();
acidCollider.center = new Vector3(0.0f, -9.0f, 40.0f);
acidCollider.size = new Vector3(32.0f, 6.0f, 32.0f);
```

### Godot
```gdscript
# Cylinder for hub gas
var hub_gas_area = Area3D.new()
var cylinder_shape = CylinderShape3D.new()
cylinder_shape.radius = 28.0
cylinder_shape.height = 6.0
hub_gas_area.shape = cylinder_shape
hub_gas_area.transform.origin = Vector3(0.0, 1.0, 0.0)

# Box for sublevel acid
var acid_area = Area3D.new()
var box_shape = BoxShape3D.new()
box_shape.extents = Vector3(16.0, 3.0, 16.0)
acid_area.shape = box_shape
acid_area.transform.origin = Vector3(0.0, -9.0, 40.0)
```

---

## Validation Checklist

### Pre-Implementation
- [ ] Confirm cell size (4.0 units) matches engine scale
- [ ] Verify origin (0,0,0) aligns with egg platform center
- [ ] Test that catwalk (Y > 4.0) is outside gas volume
- [ ] Ensure sublevel heavy weapon pickup is at volume edge

### Post-Implementation
- [ ] Player standing at (0, 0, 0) takes damage when Active
- [ ] Player on catwalk at (0, 6, 0) does NOT take damage
- [ ] Player in sublevel at (0, -10, 40) takes increased damage
- [ ] Execution animations (ASID 400, 405, 901) ignore hazard damage
- [ ] Visual FX align with volume boundaries

---

## Tuning Guidelines

### If hazards feel too lethal:
- Reduce damage_per_second by 10-15 HP increments
- Increase warning (Arming) duration by 1-2 seconds
- Shrink radius by 4 units (1 cell) to create more safe paths

### If hazards feel too weak:
- Increase damage_per_second to 75-100 HP/sec (floor) or 120 HP/sec (sublevel)
- Extend Active duration by 3-5 seconds
- Add secondary effect (movement slow, vision blur) at 50% health threshold

### If players get stuck:
- Add escape ladders at cardinal directions (N/S/E/W) from hub floor
- Ensure at least one safe path exists from any spawn to catwalk
- Reduce Cooldown duration to 20 seconds for faster recovery

---

## References
- Source: `docs/multiplayer/04_Multiplayer_Alien_Base_Triggers.md`
- Grid: `data/alien_base_hub_grid_v1.json`
- Entities: `data/alien_base_hub_entities_v1.json`
