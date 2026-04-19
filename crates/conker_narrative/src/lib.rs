//! # Conker Narrative Generation Crate
//!
//! This crate provides AI-Chat-friendly types and utilities for generating
//! narrative content, contextual dialog, and story beats for Conker: Live & Uncut.
//!
//! All types derive `JsonSchema` for machine-readable contracts and validation.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod dialog;
pub mod story;
pub mod prompt;

pub use dialog::*;
pub use story::*;
pub use prompt::*;

/// A narrative beat: the smallest unit of story progression.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NarrativeBeat {
    /// Unique identifier for this beat (e.g., "BEAT_INTRO_CONKER_TAUNT").
    pub id: String,
    
    /// Human-readable label for designers.
    pub label: String,
    
    /// Beat type: controls pacing and UI presentation.
    #[serde(rename = "type")]
    pub beat_type: BeatType,
    
    /// Optional registry references for audio/visual assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sfx_id: Option<conker_registry::SfxId>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vfx_id: Option<conker_registry::VfxId>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asid_id: Option<conker_registry::AsidId>,
    
    /// Dialog lines associated with this beat.
    #[serde(default)]
    pub dialog_lines: Vec<DialogLine>,
    
    /// Conditions that must be true for this beat to trigger.
    #[serde(default)]
    pub conditions: Vec<BeatCondition>,
    
    /// Effects that occur when this beat completes.
    #[serde(default)]
    pub effects: Vec<BeatEffect>,
    
    /// Optional branching: next possible beats based on player choice.
    #[serde(default)]
    pub branches: Vec<BeatBranch>,
    
    /// Metadata for AI-Chat conditioning.
    #[serde(default)]
    pub ai_metadata: AiBeatMetadata,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BeatType {
    /// Opening exposition or cutscene.
    Intro,
    /// Player-triggered dialog or event.
    Trigger,
    /// Combat or action sequence.
    Action,
    /// Puzzle or objective completion.
    Objective,
    /// Humor/taunt moment (Conker-specific).
    Taunt,
    /// Transition between areas or states.
    Transition,
    /// End-of-mission wrap-up.
    Outro,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DialogLine {
    /// Speaker identifier (e.g., "CONKER", "GREGG", "BERRI").
    pub speaker: String,
    
    /// The actual dialog text. Supports simple templating: {player_name}, {item}.
    pub text: String,
    
    /// Optional emotion tag for voice direction or UI styling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion: Option<DialogEmotion>,
    
    /// Optional timing hint (seconds to display).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_hint_s: Option<f32>,
    
    /// Optional registry reference for voice line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sfx_override: Option<conker_registry::SfxId>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DialogEmotion {
    Neutral,
    Sarcastic,
    Angry,
    Drunk,
    Fearful,
    Triumphant,
    Confused,
    Flirtatious,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BeatCondition {
    /// Condition type: what game state to check.
    #[serde(rename = "type")]
    pub condition_type: ConditionType,
    
    /// Parameter for the condition (e.g., item ID, zone ID, health threshold).
    pub parameter: String,
    
    /// Comparison operator.
    #[serde(default)]
    pub operator: ConditionOperator,
    
    /// Value to compare against.
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    /// Check if player has an item.
    HasItem,
    /// Check player health/armor threshold.
    HealthThreshold,
    /// Check if a zone is active.
    ZoneActive,
    /// Check if an NPC is alive.
    NpcAlive,
    /// Check game mode or rule state.
    GameRule,
    /// Check random chance (for variability).
    RandomChance,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BeatEffect {
    /// Effect type: what action to perform.
    #[serde(rename = "type")]
    pub effect_type: EffectType,
    
    /// Target identifier (entity, zone, player, etc.).
    pub target: String,
    
    /// Effect parameters.
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    /// Spawn an entity or NPC.
    Spawn,
    /// Despawn or kill an entity.
    Despawn,
    /// Grant an item to player.
    GrantItem,
    /// Trigger a VFX or SFX.
    PlayEffect,
    /// Change game state or flag.
    SetFlag,
    /// Modify player stats.
    ModifyStat,
    /// Trigger a camera or cinematic.
    TriggerCinematic,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BeatBranch {
    /// Unique identifier for this branch path.
    pub id: String,
    
    /// Label for designers/AI.
    pub label: String,
    
    /// The beat ID this branch leads to.
    pub next_beat_id: String,
    
    /// Optional condition for this branch to be available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<BeatCondition>,
    
    /// Player-facing choice text (if interactive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct AiBeatMetadata {
    /// Tags for AI-Chat to filter/search beats.
    #[serde(default)]
    pub tags: Vec<String>,
    
    /// Suggested prompt context for generating similar beats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_hint: Option<String>,
    
    /// Difficulty rating (1-5) for pacing control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<u8>,
    
    /// Estimated duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_duration_s: Option<f32>,
    
    /// Content warnings or tone indicators.
    #[serde(default)]
    pub content_notes: Vec<String>,
}

/// A complete story sequence: ordered beats with entry/exit points.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StorySequence {
    /// Unique sequence identifier.
    pub id: String,
    
    /// Human-readable title.
    pub title: String,
    
    /// Entry beat ID (where the sequence starts).
    pub entry_beat_id: String,
    
    /// Map of beat IDs to beat definitions.
    pub beats: HashMap<String, NarrativeBeat>,
    
    /// Optional exit conditions (when the sequence is "complete").
    #[serde(default)]
    pub exit_conditions: Vec<BeatCondition>,
    
    /// Metadata for AI-Chat.
    #[serde(default)]
    pub ai_metadata: AiSequenceMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct AiSequenceMetadata {
    /// Genre tags (e.g., "comedy", "action", "puzzle").
    #[serde(default)]
    pub genre_tags: Vec<String>,
    
    /// Target player count (1 = solo, 2+ = multiplayer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_count: Option<u8>,
    
    /// Suggested game mode compatibility.
    #[serde(default)]
    pub compatible_modes: Vec<conker_registry::GmrId>,
    
    /// Estimated playtime in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_playtime_min: Option<u16>,
}

/// Helper trait for schema-backed narrative types.
pub trait NarrativeSchemaBacked: schemars::JsonSchema + serde::Serialize + serde::de::DeserializeOwned {
    fn schema_name() -> &'static str;
    fn schema_description() -> &'static str;
}

impl NarrativeSchemaBacked for NarrativeBeat {
    fn schema_name() -> &'static str { "conker_narrative_beat_v1" }
    fn schema_description() -> &'static str { "Conker narrative beat definition" }
}

impl NarrativeSchemaBacked for StorySequence {
    fn schema_name() -> &'static str { "conker_narrative_sequence_v1" }
    fn schema_description() -> &'static str { "Conker story sequence definition" }
}
