# 05_Multiplayer_Raptor_Temple – First-Pass Layout

The Raptor/Temple map is an S-shaped valley with bases at both ends and a two-story temple at the center, mirroring the N64 Raptor arena while targeting 16-player Uncut play.

## Global Shape

- Footprint: 220 x 140 units approximate.
- Path: Broad "S" curve from Uga Buga base (south-west) to Raptor nest (north-east).
- Elevation:
  - Uga Buga base: Y = 0.
  - Central temple: rises to Y = 10 (upper floor).
  - Raptor nest: slightly above: Y = 4–6.

## Key Zones

1. Uga Buga Base
- Location: (-80, 0, -40) approximate.
- Features:
  - Hut cluster with egg frying pan objective.
  - Short ramp up to a defensive ridge overlooking the lower valley.
- Spawn logic: four spawn points around the huts, safe from direct temple fire.

2. Lower Valley Bend
- Location: central-south section of the "S".
- Features:
  - Shallow riverbed or dried creek.
  - Sparse rock cover.
- Tactical role: choke point; Raptors can ambush Uga as they funnel through.

3. Temple Complex (Center)
- Location: (0, 0, 0).
- Structure:
  - Ground floor: four entrances (N, S, E, W) to a central hall.
  - Upper floor: balcony ring around a central hole, open to the hall below.
- Objectives:
  - Egg nests on temple outskirts.
  - Uga frying pan station near Uga side of temple.

4. Raptor Nest
- Location: (80, 4, 40).
- Features:
  - Raised rocky ledge with central baby dinosaur pit.
  - Narrow ramp or natural staircase leading up from the upper valley.
- Spawn logic: Raptors spawn close to the nest, with quick drops into the valley.

5. Upper Valley Bend
- Location: central-north section of the "S".
- Features:
  - Tighter canyon walls than lower bend.
  - Higher density of cover rocks and trees to offset Raptor mobility.

## Movement Flow

- Uga Buga flow:
  - Spawn → lower valley → temple → upper valley → nest.
- Raptor flow:
  - Spawn → drop into upper valley → temple or lower bend ambushes → fall back to nest.

The grid implementation should preserve the S-curve path and central two-story temple with clear vertical play, while keeping traversal times similar to the N64 original.
