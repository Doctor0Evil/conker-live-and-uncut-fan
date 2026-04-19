//! Contextual dialog generation utilities for Conker: Live & Uncut.
//!
//! This module provides AI-Chat-accessible functions for generating
//! character-appropriate dialog lines with templating, emotion tagging,
//! and registry integration.

use crate::{DialogLine, DialogEmotion, AiBeatMetadata};
use conker_registry::SfxId;
use rand::seq::SliceRandom;
use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;

/// Input parameters for dialog generation.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DialogGenParams {
    /// Character speaking (CONKER, GREGG, BERRI, etc.).
    pub speaker: String,
    
    /// Context tag: what situation is this dialog for?
    pub context: DialogContext,
    
    /// Optional emotion override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion: Option<DialogEmotion>,
    
    /// Optional template variables for interpolation.
    #[serde(default)]
    pub variables: HashMap<String, String>,
    
    /// Optional seed for deterministic generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    
    /// Number of variants to generate.
    #[serde(default = "default_variant_count")]
    pub variant_count: usize,
}

fn default_variant_count() -> usize { 1 }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DialogContext {
    /// Opening taunt or greeting.
    Taunt,
    /// Reacting to player damage.
    Pain,
    /// Reacting to player success.
    Triumph,
    /// Reacting to player failure.
    Defeat,
    /// Idle banter or ambient line.
    Idle,
    /// Objective-related instruction or hint.
    Objective,
    /// Humor/Conker-specific drunk dialog.
    Drunk,
    /// Combat bark during fight.
    Combat,
    /// Discovery of secret or item.
    Discovery,
    /// Transition between areas.
    Transition,
}

/// Output from dialog generation.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DialogGenResult {
    /// The generated dialog lines.
    pub lines: Vec<DialogLine>,
    
    /// Metadata about generation (for AI-Chat feedback).
    pub metadata: DialogGenMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct DialogGenMetadata {
    /// Seed used for generation (for reproducibility).
    pub seed: u64,
    
    /// Context tags that influenced generation.
    pub applied_contexts: Vec<String>,
    
    /// Optional SFX suggestion from registry.
    pub suggested_sfx: Option<SfxId>,
    
    /// Confidence score (0.0-1.0) for AI-Chat to filter results.
    pub confidence: f32,
}

/// Main dialog generator: produces context-appropriate lines.
pub fn generate_dialog(params: DialogGenParams) -> DialogGenResult {
    let seed = params.seed.unwrap_or_else(|| rand::random());
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    
    let mut lines = Vec::with_capacity(params.variant_count);
    
    for _ in 0..params.variant_count {
        let line = generate_single_line(&params, &mut rng);
        lines.push(line);
    }
    
    DialogGenResult {
        lines,
        metadata: DialogGenMetadata {
            seed,
            applied_contexts: vec![format!("{:?}", params.context)],
            suggested_sfx: suggest_sfx_for_context(&params.speaker, params.context),
            confidence: 0.85, // Placeholder; could be computed from template match quality
        },
    }
}

fn generate_single_line(params: &DialogGenParams, rng: &mut ChaCha8Rng) -> DialogLine {
    let templates = get_templates_for_context(params.speaker.as_str(), params.context);
    
    // Select template with weighted randomness
    let template = templates.choose_weighted(rng, |t| t.weight).unwrap().text.clone();
    
    // Interpolate variables
    let text = interpolate_template(&template, &params.variables);
    
    DialogLine {
        speaker: params.speaker.clone(),
        text,
        emotion: params.emotion.or_else(|| default_emotion_for_context(params.context)),
        duration_hint_s: None,
        sfx_override: None,
    }
}

/// Template definition with weight for selection.
#[derive(Debug, Clone)]
struct DialogTemplate {
    text: String,
    weight: f32,
    required_vars: Vec<String>,
}

fn get_templates_for_context(speaker: &str, context: DialogContext) -> Vec<DialogTemplate> {
    // This is a minimal example; in production, load from data files or knowledge graph
    match (speaker, context) {
        ("CONKER", DialogContext::Taunt) => vec![
            DialogTemplate { text: "Right then, {target}, let's see what you're made of!".into(), weight: 1.0, required_vars: vec!["target".into()] },
            DialogTemplate { text: "Fancy a go, do ya? Come on then!".into(), weight: 0.8, required_vars: vec![] },
            DialogTemplate { text: "I've had worse from me nan, {target}.".into(), weight: 0.6, required_vars: vec!["target".into()] },
        ],
        ("CONKER", DialogContext::Drunk) => vec![
            DialogTemplate { text: "*hic* Blimey, the room's gone all... spinny.".into(), weight: 1.0, required_vars: vec![] },
            DialogTemplate { text: "Just... just gotta find the... the thing. Yeah.".into(), weight: 0.9, required_vars: vec![] },
            DialogTemplate { text: "*burp* Excuse me. Right, where was I?".into(), weight: 0.7, required_vars: vec![] },
        ],
        ("GREGG", DialogContext::Taunt) => vec![
            DialogTemplate { text: "Your soul is mine, furry!".into(), weight: 1.0, required_vars: vec![] },
            DialogTemplate { text: "You cannot escape the darkness, {target}!".into(), weight: 0.8, required_vars: vec!["target".into()] },
        ],
        // Add more speaker/context combinations as needed
        _ => vec![
            DialogTemplate { text: "{speaker} says something appropriate.".into(), weight: 1.0, required_vars: vec!["speaker".into()] },
        ],
    }
}

fn interpolate_template(template: &str, vars: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}

fn default_emotion_for_context(context: DialogContext) -> Option<DialogEmotion> {
    match context {
        DialogContext::Taunt => Some(DialogEmotion::Sarcastic),
        DialogContext::Pain => Some(DialogEmotion::Angry),
        DialogContext::Triumph => Some(DialogEmotion::Triumphant),
        DialogContext::Defeat => Some(DialogEmotion::Fearful),
        DialogContext::Drunk => Some(DialogEmotion::Confused),
        _ => Some(DialogEmotion::Neutral),
    }
}

fn suggest_sfx_for_context(speaker: &str, context: DialogContext) -> Option<SfxId> {
    // Map context + speaker to registry SFX IDs
    // This is a stub; in production, query conker_registry or a config file
    match (speaker, context) {
        ("CONKER", DialogContext::Pain) => Some(conker_registry::SfxId::VoConkPain01),
        ("CONKER", DialogContext::Taunt) => Some(conker_registry::SfxId::VoConkTaunt01),
        ("CONKER", DialogContext::Drunk) => Some(conker_registry::SfxId::VoConkBurp),
        ("GREGG", DialogContext::Taunt) => Some(conker_registry::SfxId::VoGreggTaunt),
        _ => None,
    }
}

/// Batch generation: produce multiple contextual lines for a scene.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchDialogParams {
    /// List of individual dialog generation requests.
    pub requests: Vec<DialogGenParams>,
    
    /// Optional global seed for consistent batch generation.
    #[serde(skip_serializing_if = "Option::is_none
