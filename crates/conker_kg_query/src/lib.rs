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
        KgQuery::ListItems { item_type } => {
            let items = list_items_stub(item_type);
            KgResponse::ListItems {
                items,
                total_count: items.len(),
            }
        }
        KgQuery::GetItem { item_type, item_id } => {
            match get_item_stub(item_type, &item_id) {
                Some(item) => KgResponse::GetItem { item },
                None => KgResponse::Error {
                    message: format!("Item '{}' of type '{:?}' not found", item_id, item_type),
                    suggestion: Some("Try KgQuery::ListItems to see available IDs".into()),
                },
            }
        }
        KgQuery::SearchItems { item_type, tags, exclude_tags } => {
            let items = search_items_stub(item_type, &tags, &exclude_tags);
            KgResponse::SearchItems {
                items,
                total_count: items.len(),
                matched_tags: tags,
            }
        }
        KgQuery::GetValidCombinations { context, constraints } => {
            get_valid_combinations_stub(context, constraints)
        }
        KgQuery::GetPromptHints { for_system, detail_level } => {
            get_prompt_hints_stub(&for_system, detail_level)
        }
    }
}

// Stub implementations - replace with real knowledge graph loading

fn list_items_stub(item_type: KgItemType) -> Vec<KgItemSummary> {
    match item_type {
        KgItemType::Sfx => vec![
            KgItemSummary {
                id: "SFX_501".into(),
                label: "VoConkPain01".into(),
                description: Some("Conker pain VO: 'Bloody hell!'".into()),
                tags: vec!["vo".into(), "conker".into(), "pain".into()],
            },
            KgItemSummary {
                id: "SFX_502".into(),
                label: "VoConkTaunt01".into(),
                description: Some("Conker taunt VO: 'I'm gonna gut you like a fish!'".into()),
                tags: vec!["vo".into(), "conker".into(), "taunt".into()],
            },
        ],
        KgItemType::DialogContext => vec![
            KgItemSummary {
                id: "taunt".into(),
                label: "Taunt".into(),
                description: Some("Opening taunt or greeting".into()),
                tags: vec!["combat".into(), "intro".into()],
            },
            KgItemSummary {
                id: "drunk".into(),
                label: "Drunk".into(),
                description: Some("Humor/Conker-specific drunk dialog".into()),
                tags: vec!["humor".into(), "conker".into()],
            },
        ],
        _ => vec![],
    }
}

fn get_item_stub(item_type: KgItemType, item_id: &str) -> Option<KgItemDetail> {
    if item_type == KgItemType::Sfx && item_id == "SFX_501" {
        Some(KgItemDetail {
            id: "SFX_501".into(),
            item_type: KgItemType::Sfx,
            label: "VoConkPain01".into(),
            description: Some("Conker pain VO: 'Bloody hell!'".into()),
            tags: vec!["vo".into(), "conker".into(), "pain".into()],
            metadata: {
                let mut m = HashMap::new();
                m.insert("duration_ms".into(), 1200.into());
                m.insert("priority".into(), "high".into());
                m
            },
            related_items: vec![
                KgRelatedItem {
                    id: "ASID_450".into(),
                    item_type: KgItemType::Asid,
                    relationship: "often_played_with".into(),
                },
            ],
        })
    } else {
        None
    }
}

fn search_items_stub(item_type: KgItemType, tags: &[String], exclude: &[String]) -> Vec<KgItemSummary> {
    // Very basic stub: return items that have all requested tags and none of the excluded
    list_items_stub(item_type)
        .into_iter()
        .filter(|item| {
            tags.iter().all(|t| item.tags.contains(t)) &&
            exclude.iter().all(|t| !item.tags.contains(t))
        })
        .collect()
}

fn get_valid_combinations_stub(context: KgCombinationContext, _constraints: HashMap<String, serde_json::Value>) -> KgResponse {
    match context {
        KgCombinationContext::CombatEffects { asid_id } => {
            let combos = match asid_id.as_str() {
                "ASID_401" => vec![ // FIN_CHAINSAW_H
                    KgCombination {
                        parameters: [("vfx_id".into(), "VFX_016".into())].into_iter().collect(),
                        description: "Vertical torso slice gore effect".into(),
                        validity: 1.0,
                    },
                    KgCombination {
                        parameters: [("sfx_id".into(), "SFX_152".into())].into_iter().collect(),
                        description: "Chainsaw engine idle sputter".into(),
                        validity: 0.9,
                    },
                ],
                _ => vec![],
            };
            KgResponse::GetValidCombinations {
                combinations: combos,
                explanation: "Valid VFX/SFX pairings for the given ASID based on canonical Conker registry".into(),
            }
        }
        _ => KgResponse::GetValidCombinations {
            combinations: vec![],
            explanation: "Combination context not yet implemented".into(),
        },
    }
}

fn get_prompt_hints_stub(for_system: &str, detail: PromptDetailLevel) -> KgResponse {
    let hints = match for_system {
        "conker_narrative" => vec![
            PromptHint {
                include: "Specify speaker, context, and any template variables".into(),
                avoid: Some("Avoid mixing character voices or anachronistic language".into()),
                example: Some("Generate a drunk CONKER taunt for target 'GREGG'".into()),
            },
            PromptHint {
                include: "Reference registry IDs (SFX_, VFX_, ASID_) when suggesting assets".into(),
                avoid: None,
                example: Some("Use SFX_501 for Conker pain lines".into()),
            },
        ],
        "dialog_generation" => vec![
            PromptHint {
                include: "Use DialogContext enum values: taunt, drunk, combat, etc.".into(),
                avoid: Some("Do not invent new context types without updating the schema".into()),
                example: Some("Context: DialogContext::Drunk for Conker humor".into()),
            },
        ],
        _ => vec![
            PromptHint {
                include: "Start with KgQuery::ListItems to discover available options".into(),
                avoid: None,
                example: None,
            },
        ],
    };
    
    let example_prompt = match detail {
        PromptDetailLevel::Minimal => format!("Query: ListItems {{ item_type: {:?} }}", KgItemType::Sfx),
        PromptDetailLevel::Standard => "Generate 3 taunt lines for CONKER targeting GREGG, with SFX suggestions".into(),
        PromptDetailLevel::Detailed => r#"
Example: Generate contextual dialog for a Conker narrative beat.

Input:
{
  "speaker": "CONKER",
  "context": "taunt",
  "variables": {"target": "GREGG"},
  "emotion": "Sarcastic",
  "variant_count": 2
}

Expected output includes:
- DialogLine objects with speaker, text, emotion
- Suggested SfxId from registry
- Confidence metadata for filtering
"#.into(),
    };
    
    KgResponse::GetPromptHints {
        system: for_system.to_string(),
        hints,
        example_prompt: example_prompt.to_string(),
    }
}

/// Convenience function for AI-Chat: execute query and return JSON string.
pub fn execute_query_json(query: KgQuery) -> Result<String, serde_json::Error> {
    let response = execute_query(query);
    serde_json::to_string_pretty(&response)
}
