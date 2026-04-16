// in crates/conker-schema/src/map_recipe.rs (or an adjacent file)
//
// #[derive(...)]
// pub struct ConkerMapRecipe {
//     ...
//     #[serde(default)]
//     pub npc_refs: Vec<NpcRef>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
// #[serde(rename_all = "camelCase")]
// pub struct NpcRef {
//     pub npc_id: String,
// }

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Top-level Conker Uncut map recipe.
///
/// This is a platform-agnostic description that can be compiled into:
/// - N64-era assets (Nintendoor64 / Starzip / N64 layout), and
/// - Modern engine scenes (UE, Godot, Unity),
/// while preserving N64-era design constraints.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConkerMapRecipe {
    /// Stable identifier for this map within the project.
    /// Example: "M01", "M07".
    pub id: String,

    /// Internal name used by tools and layouts.
    /// Example: "Multiplayer_Beach_Dead", "Alien_Base".
    pub internal_name: String,

    /// Human-readable title for designers.
    pub title: String,

    /// Conker multiplayer mode this map serves.
    pub mode: ConkerMode,

    /// High-level environment theme, used for art direction only.
    /// Example: "beach", "sci_fi_base", "gothic_castle", "urban_sprawl".
    pub environment_theme: String,

    /// Maximum number of players supported by the design.
    /// Must not exceed 16 under the Conker N64 design contract.
    pub max_players: u8,

    /// Whether the map is expected to be mechanically symmetric
    /// between opposing teams (even if visually asymmetric).
    pub symmetric_teams: bool,

    /// Logical grid used as a platform-agnostic representation of layout.
    /// This can be compiled into concrete N64 geometry or modern engine meshes.
    pub grid: MapGrid,

    /// Player spawn points for each team and slot index.
    pub spawn_points: Vec<SpawnPoint>,

    /// Pickup spawn locations and allowed pickup types.
    /// Constrained by the symmetric, pickup-based arsenal invariant.
    pub pickups: Vec<PickupSpawn>,

    /// Scripted hazards and instant-win events present on this map.
    /// These should be instances of generic hazard templates (gas, airlock, etc.).
    pub hazards: Vec<HazardInstance>,

    /// Optional per-map notes for design / debugging.
    #[serde(default)]
    pub notes: Vec<String>,

    /// Optional free-form metadata for engine-specific emitters.
    /// Backends must not rely on this for core mechanics.
    #[serde(default)]
    pub backend_metadata: BackendMetadata,
}

/// Known Conker multiplayer modes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConkerMode {
    Beach,
    Heist,
    Colors,
    Raptor,
    Tank,
    Race,
    Deathmatch,
    AlienBase,
    Spamono,
    BloodCount,
}

/// Coarse logical grid describing the map layout.
///
/// This is intentionally abstract and low-resolution; it is not a tilemap
/// but a design-time blueprint for lanes, chokepoints, and verticality.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapGrid {
    /// Number of cells along X.
    pub width: u32,
    /// Number of cells along Y.
    pub height: u32,

    /// Cell size in meters (or abstract units) for modern engines.
    /// N64 emitters may quantize this.
    pub cell_size: f32,

    /// Cells, row-major, from top-left (0,0).
    ///
    /// Length must be width * height.
    pub cells: Vec<GridCell>,
}

/// Logical type of a grid cell.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CellKind {
    /// Walkable ground at base level.
    Ground,
    /// Elevated ledge or platform.
    HighGround,
    /// Cover object / obstacle (sand dune, crate, rock).
    Cover,
    /// Non-navigable void or outside area.
    Void,
    /// Hazard volume (gas chamber interior, pit, etc.).
    HazardVolume,
    /// Water or liquid surface.
    Water,
    /// Reserved for future extension.
    Other,
}

/// Single grid cell descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GridCell {
    pub x: u32,
    pub y: u32,
    pub kind: CellKind,

    /// Optional label for human-readable landmarks.
    /// Example: "bridge", "bunker_interior", "spawn_lane_a".
    #[serde(default)]
    pub label: Option<String>,
}

/// Team identifier for spawns and logic.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TeamId {
    /// Used for FFA or neutral spawns.
    Neutral,
    /// Primary opposing team A.
    TeamA,
    /// Primary opposing team B.
    TeamB,
}

/// Player spawn definition.
///
/// N64-era contract: for symmetric modes, TeamA and TeamB must have
/// isomorphic spawn patterns under some symmetry of the grid.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpawnPoint {
    /// Team that owns this spawn.
    pub team: TeamId,

    /// Slot index [0, 15] for mapping to up to 16 players.
    pub slot_index: u8,

    /// Grid coordinates (cell index).
    pub grid_x: u32,
    pub grid_y: u32,

    /// Optional facing in degrees for modern engines.
    /// N64 emitters may quantize this.
    #[serde(default)]
    pub facing_degrees: Option<f32>,

    /// Whether this spawn is primary (used at match start) or secondary (used for respawns).
    #[serde(default)]
    pub kind: SpawnKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpawnKind {
    Primary,
    Secondary,
}

/// Pickup types available in Conker N64-era multiplayer.
///
/// This list is intentionally symmetric and pickup-based; it must not encode
/// any class-specific or loadout-based behavior.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PickupKind {
    Pistol,
    Shotgun,
    MachineGun,
    SniperRifle,
    RocketLauncher,
    Grenade,
    MeleeWeapon,
    Health,
    Armor,
    Special, // Rare / map-unique (e.g., airstrike beacon), still symmetric.
}

/// Pickup spawn location and rules.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PickupSpawn {
    pub grid_x: u32,
    pub grid_y: u32,

    /// Allowed pickup type at this location.
    pub kind: PickupKind,

    /// Respawn behavior: whether the pickup respawns and on what cooldown.
    #[serde(default)]
    pub respawn: PickupRespawn,

    /// Optional label for design reference.
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PickupRespawn {
    /// If false, pickup is single-use per match (e.g., unique objective item).
    #[serde(default = "default_respawn_enabled")]
    pub enabled: bool,

    /// Cooldown in seconds before pickup reappears.
    /// Ignored if enabled == false.
    #[serde(default = "default_respawn_cooldown")]
    pub cooldown_seconds: f32,
}

fn default_respawn_enabled() -> bool {
    true
}

fn default_respawn_cooldown() -> f32 {
    15.0
}

/// Identifier for a hazard template.
///
/// Concrete behavior is defined in a separate hazard template schema;
/// this recipe only binds instances to locations and parameters.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HazardKind {
    /// Gas chamber wipe (Heist, Fortress).
    GasChamber,
    /// Airlock or vacuum purge (Alien Base).
    Airlock,
    /// Fire-based purge tied to a carrier (Blood Count Fire Imp).
    FireImpPurge,
    /// Tank-dropped gas canister (Total War / Colors tank).
    TankGasCanister,
    /// Custom scripted hazard defined elsewhere.
    Custom,
}

/// Hazard occurrence bound into a specific map.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HazardInstance {
    pub id: String,

    pub kind: HazardKind,

    /// Optional reference to a named hazard template (state machine, timings).
    /// Example: "hazard.template.gas_chamber.standard".
    #[serde(default)]
    pub template_ref: Option<String>,

    /// Primary trigger volume in grid coordinates (axis-aligned bounding box).
    pub volume: HazardVolume,

    /// How this hazard is triggered.
    pub trigger: HazardTrigger,

    /// Optional cooldown override; if None, use template default.
    #[serde(default)]
    pub cooldown_seconds: Option<f32>,

    /// Whether this hazard can wipe all players (instant-win style),
    /// as opposed to being a localized threat.
    #[serde(default)]
    pub is_instant_win_capable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HazardVolume {
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HazardTriggerKind {
    /// Explicit player interaction (lever, button).
    PlayerInteract,
    /// Objective completion (e.g., vault cracked, item delivered).
    ObjectiveComplete,
    /// Scripted timeline event.
    Scripted,
}

/// Trigger configuration for a hazard.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HazardTrigger {
    pub kind: HazardTriggerKind,

    /// Optional identifier for the linked objective or script event.
    #[serde(default)]
    pub reference_id: Option<String>,

    /// Arming time in seconds before the hazard becomes active
    /// after the trigger condition is met.
    #[serde(default = "default_hazard_arming_time")]
    pub arming_time_seconds: f32,
}

fn default_hazard_arming_time() -> f32 {
    3.0
}

/// Free-form backend metadata, to be interpreted only by emitters.
/// This must not be used to bypass core invariants.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default)]
pub struct BackendMetadata {
    /// Optional engine-specific hints (e.g., UE map name, Godot scene path).
    pub engine_hints: serde_json::Map<String, serde_json::Value>,
}

impl Default for BackendMetadata {
    fn default() -> Self {
        Self {
            engine_hints: serde_json::Map::new(),
        }
    }
}
