# Fortress Capture Point Logic Design

## Research Question 32: Capture Point System for 16-Player Fortress

### Recommended Approach: HYBRID MULTI-POINT WITH TUG-OF-WAR ELEMENTS

For a 16-player (8v8) Fortress map, a **single central bridge control point** is too restrictive, while **pure tug-of-war** may be too abstract. The optimal solution combines:

1. **Three fixed capture points** (Tower Alpha, Central Bridge, Tower Beta)
2. **Tug-of-war ticket generation** based on point ownership differential
3. **Dynamic front line** that shifts based on control

---

## System Architecture

### Control Point Configuration

```lua
Fortress.CapturePoints {
  total_points = 3,
  
  points = {
    {
      id = "tower_alpha",
      location = "left_flank",
      grid_position = { col = 8, row = 6 },
      capture_rate_per_second = 1.0,
      ticket_generation_rate = 0.5,  -- Base tickets/sec when owned
      strategic_value = "medium",
      initial_owner = "neutral"
    },
    
    {
      id = "central_bridge",
      location = "center_valley",
      grid_position = { col = 13, row = 7 },
      capture_rate_per_second = 1.0,
      ticket_generation_rate = 1.0,  -- DOUBLE value - primary objective
      strategic_value = "high",
      initial_owner = "neutral",
      is_primary = true
    },
    
    {
      id = "tower_beta",
      location = "right_flank",
      grid_position = { col = 22, row = 6 },
      capture_rate_per_second = 1.0,
      ticket_generation_rate = 0.5,
      strategic_value = "medium",
      initial_owner = "neutral"
    }
  }
}
```

---

## Ticket Generation Formula

### Base Generation (Per Second)
```
Tickets_Per_Second = Σ(Owned_Points × Point_Value) + Ownership_Bonus

Where:
- Owned_Points = Number of control points held (0-3)
- Point_Value = 
    * Tower Alpha/Beta: 0.5 tickets/sec each
    * Central Bridge: 1.0 tickets/sec
- Ownership_Bonus = Additional tickets for dominant control
```

### Tug-of-War Bonus (Dominance Multiplier)
```
if Team_A_points > Team_B_points:
  Dominance_Bonus = (Team_A_points - Team_B_points) × 0.25
  
Examples:
- Holding 3 vs 0: +0.75 tickets/sec bonus (total: 2.75/sec)
- Holding 2 vs 1: +0.25 tickets/sec bonus (total: 1.75/sec)
- Holding 1 vs 1: No bonus (ticket generation cancels)
```

### Victory Threshold
```
Victory_Tickets = 100
Estimated_Match_Duration = 5-7 minutes (with balanced teams)
```

---

## Capture Mechanics

### State Machine
```
Point_States = [Neutral, Contested, Owned_SHC, Owned_Tediz]

State_Transitions:
  Neutral → Contested: When any player enters capture radius
  Contested → Owned: When one team maintains presence for capture_duration
  Owned → Contested: When enemy team enters with equal/greater presence
  Contested → Neutral: When all players leave before capture completes
```

### Capture Progress Formula
```
Capture_Progress(t+Δt) = Progress(t) + Net_Capture_Rate × Δt

Net_Capture_Rate = 
  (Allied_Presence × Capture_Rate) - (Enemy_Presence × Contest_Rate)

Where:
- Allied_Presence = Number of friendly players in radius
- Enemy_Presence = Number of enemy players in radius
- Capture_Rate = 1.0% per second per player (configurable)
- Contest_Rate = 0.5 × Capture_Rate (slows but doesn't reverse)
```

### Capture Radius
```
Default_Radius = 6.0 meters (approximately 1.5 grid cells)
Max_Players_In_Radius = 8 per team (prevents zerg rush exploits)
```

---

## Visual Feedback Requirements

### UI Elements
1. **Top Bar**: Shows all three points with ownership indicators
   - Left icon (Tower Alpha): SHC/Tediz/Neutral
   - Center icon (Bridge): SHC/Tediz/Neutral (larger/highlighted)
   - Right icon (Tower Beta): SHC/Tediz/Neutral

2. **Progress Bars**: Per-point capture progress
   - Horizontal bar under each point icon
   - Color gradient: Red (Tediz) ↔ Gray (Neutral) ↔ Blue (SHC)
   - Animated fill during capture

3. **Ticket Counter**: Live score display
   - SHC Tickets: [0-100]
   - Tediz Tickets: [0-100]
   - Leading team highlighted

4. **Dominance Indicator**: Shows tug-of-war state
   - Arrow pointing to leading team
   - Size proportional to ticket generation advantage

### World-Space Indicators
1. **Flag Pole Model**: Physical flag changes based on ownership
   - Neutral: Gray/white flag, drooping
   - SHC: Blue flag, raised high with particle effects
   - Tediz: Red flag, raised high with particle effects
   - Contested: Flag oscillates between colors

2. **Ground Projection**: Circular capture zone marker
   - Color indicates current owner
   - Pulsing intensity shows capture progress
   - Player icons show team presence inside

3. **Audio Cues**:
   - Capture start: Distinct "beep" tone
   - Capture complete: Fanfare/chime (team-specific)
   - Contested state: Urgent pulsing sound
   - Dominance shift: Dramatic sting when lead changes

---

## Strategic Depth Layers

### Layer 1: Point Priority
Players must decide:
- Rush central bridge (high value, high traffic)
- Flank via towers (lower value, easier to hold)
- Split team coverage (risk/reward balance)

### Layer 2: Timing & Rotation
- Early game: All points contested → focus on bridge
- Mid game: Establish foothold → defend + expand
- Late game: Desperate defense or final push

### Layer 3: Counter-Play
- If enemy holds bridge → take both towers to negate bonus
- If enemy splits → punish with concentrated force
- If ahead → play defensively; if behind → aggressive pushes

### Layer 4: Gas Canister Synergy
- Deliver gas canister → forces enemy respawn → free capture window
- Defend armed canister → area denial enables point capture
- Gas delivery point often overlaps with capture zones

---

## Implementation Pseudocode

```cpp
// Server-authoritative capture point logic
void ACapturePoint::Tick(float DeltaTime)
{
    // Count players in radius
    int32 SHC_Count = CountPlayersInRadius(Team::SHC);
    int32 Tediz_Count = CountPlayersInRadius(Team::Tediz);
    
    // Determine state
    if (SHC_Count > 0 && Tediz_Count > 0)
    {
        CurrentState = ECaptureState::Contested;
        // No progress change when contested equally
    }
    else if (SHC_Count > Tediz_Count)
    {
        CurrentState = ECaptureState::CapturingSHC;
        Progress += SHC_Count × CaptureRate × DeltaTime;
    }
    else if (Tediz_Count > SHC_Count)
    {
        CurrentState = ECaptureState::CapturingTediz;
        Progress += Tediz_Count × CaptureRate × DeltaTime;
    }
    
    // Check for capture completion
    if (Progress >= 100.0f)
    {
        CompleteCapture(CurrentState == ECaptureState::CapturingSHC 
                        ? Team::SHC 
                        : Team::Tediz);
        Progress = 100.0f;
    }
    
    // Replicate to clients
    Multicast_UpdateState(CurrentState, Progress);
}

void AWarGameMode::GenerateTickets(float DeltaTime)
{
    int32 SHC_Points = CountOwnedPoints(Team::SHC);
    int32 Tediz_Points = CountOwnedPoints(Team::Tediz);
    
    float SHC_Rate = CalculateTicketRate(SHC_Points, Tediz_Points);
    float Tediz_Rate = CalculateTicketRate(Tediz_Points, SHC_Points);
    
    TeamATickets += SHC_Rate × DeltaTime;
    TeamBTickets += Tediz_Rate × DeltaTime;
    
    // Check victory condition
    if (TeamATickets >= VictoryThreshold)
    {
        EndRound(Team::SHC);
    }
    else if (TeamBTickets >= VictoryThreshold)
    {
        EndRound(Team::Tediz);
    }
}
```

---

## Balance Considerations

### Anti-Snowball Mechanics
1. **Respawn Protection**: Defending team gets forward spawn near contested points
2. **Catch-Up Bonus**: Team behind by 20+ tickets gains +10% capture rate
3. **Overtime Rule**: If time expires within 5 tickets, extend until lead > 5

### Map Flow Optimization
- **Left Flank**: Tower Alpha → trench route → SHC base
- **Center**: Bridge → direct confrontation → highest skill expression
- **Right Flank**: Tower Beta → catwalk → Tediz base
- **Underground**: Tunnel provides alternative route, avoids chokepoints

### Player Count Scaling
```
For 8v8 (16 players):
- 3 control points (as designed)

For 6v6 (12 players):
- Reduce to 2 points (remove one tower)

For 4v4 (8 players):
- Single central bridge only

For 10v10 (20 players):
- Add 4th point (bunker interior from fortress_bunker_grid)
```

---

## Conclusion

The **Hybrid Multi-Point with Tug-of-War** system provides:
- ✅ Clear primary objective (bridge) without single-point bottleneck
- ✅ Strategic flanking options (towers) for tactical diversity
- ✅ Comeback mechanics via dominance bonus reversal
- ✅ Synergy with gas canister and other objectives
- ✅ Scalable design for different player counts

This approach honors the N64 Total War spirit while adapting to modern 16-player competitive expectations.
