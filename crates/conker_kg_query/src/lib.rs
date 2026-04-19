//! # Conker Knowledge Graph Query Utility
//!
//! This crate provides AI-Chat with a structured interface to query
//! the Conker knowledge graph: discovering available assets, systems,
//! narrative beats, and valid parameter combinations.
//!
//! All queries and responses are JSON-serializable for machine consumption.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use conker_registry::{SfxId, VfxId, AsidId, MapId, ZoneId, RoleId, GmrId, RulId};

/// A query to the knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "query_type", rename_all = "snake_case")]
pub enum KgQuery {
    /// List all available items of a given type.
    ListItems { item_type: KgItemType },
    
    /// Get details about a specific item by ID.
    GetItem { item_type: KgItemType, item_id: String },
    
    /// Find items matching tags or filters.
    SearchItems {
        item_type: KgItemType,
        tags: Vec<String>,
        #[serde(default)]
        exclude_tags: Vec<String>,
    },
    
    /// Get valid combinations for a given context.
    GetValidCombinations {
        context: KgCombinationContext,
        constraints: HashMap<String, serde_json::Value>,
    },
    
    /// Get prompt conditioning hints for AI-Chat.
    GetPromptHints {
        for_system: String,
        #[serde(default)]
        detail_level: PromptDetailLevel,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KgItemType {
    /// Sound effect IDs from conker_registry.
    Sfx,
    /// Visual effect IDs.
    Vfx,
    /// Animation state IDs.
    Asid,
    /// Map identifiers.
    Map,
    /// Zone identifiers.
    Zone,
    /// Role identifiers.
    Role,
    /// Game mode IDs.
    GameMode,
    /// Rule toggle IDs.
    Rule,
    /// Narrative beat IDs.
    NarrativeBeat,
    /// Story sequence IDs.
    StorySequence,
    /// Dialog context types.
    DialogContext,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "context_type", rename_all = "snake_case")]
pub enum KgCombinationContext {
    /// What SFX/VFX pairs well with a given ASID?
    CombatEffects { asid_id: String },
    
    /// What zones are valid for a given map?
    MapZones { map_id: String },
    
    /// What roles can be placed in a given zone?
    ZoneRoles { zone_id: String },
    
    /// What rules are compatible with a game mode?
    ModeRules { gmr_id: String },
    
    /// What dialog contexts suit a given character?
    CharacterDialog { speaker: String },
    
    /// What narrative beats fit a given game mode?
    ModeNarrative { gmr_id: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PromptDetailLevel {
    /// Just item names/IDs.
    Minimal,
    /// Names + brief descriptions.
    Standard,
    /// Full metadata, examples, and usage hints.
    Detailed,
}

/// Response from a knowledge graph query.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "response_type", rename_all = "snake_case")]
pub enum KgResponse {
    ListItems {
        items: Vec<KgItemSummary>,
        total_count: usize,
    },
    
    GetItem {
        item: KgItemDetail,
    },
    
    SearchItems {
        items: Vec<KgItemSummary>,
        total_count: usize,
        matched_tags: Vec<String>,
    },
    
    GetValidCombinations {
        combinations: Vec<KgCombination>,
        explanation: String,
    },
    
    GetPromptHints {
        system: String,
        hints: Vec<PromptHint>,
        example_prompt: String,
    },
    
    Error {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        suggestion: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct KgItemSummary {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct KgItemDetail {
    pub id: String,
    pub item_type: KgItemType,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub related_items: Vec<KgRelatedItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct KgRelatedItem {
    pub id: String,
    pub item_type: KgItemType,
    pub relationship: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct KgCombination {
    /// The combination as key-value pairs.
    pub parameters: HashMap<String, String>,
    /// Human-readable explanation.
    pub description: String,
    /// Confidence or validity score.
    pub validity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptHint {
    /// What to include in the prompt.
    pub include: String,
    /// What to avoid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid: Option<String>,
    /// Example phrasing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
}

/// Main query executor: processes KgQuery and returns KgResponse.
///
/// In production, this would load data from knowledge_graph/ files,
/// registry crates, and narrative definitions. This stub demonstrates
/// the interface AI-Chat will use.
pub fn execute_query(query: KgQuery) -> KgResponse {
    match query {
