# AI-Chat Prompting Guide for Conker: Live & Uncut Development

This document provides structured prompting patterns for AI-Chat to generate high-quality, registry-compliant content for Conker: Live & Uncut fan projects. All examples assume access to the `conker_narrative` and `conker_kg_query` crates.

---

## Quick Reference: Query Types

| Goal | Recommended KgQuery | Example Prompt Snippet |
|------|---------------------|----------------------|
| Discover available SFX | `ListItems { item_type: Sfx }` | "List all Conker voice lines tagged 'taunt'" |
| Find valid VFX for an animation | `GetValidCombinations { context: CombatEffects { asid_id: "ASID_401" } }` | "What VFX pairs with the chainsaw execution ASID_401?" |
| Generate character dialog | Use `conker_narrative::generate_dialog` | "Generate 2 drunk CONKER lines for context 'idle'" |
| Get prompt conditioning hints | `GetPromptHints { for_system: "conker_narrative", detail_level: Detailed }` | "How should I prompt for narrative beat generation?" |

---

## Pattern 1: Registry-Aware Asset Selection

**Goal:** Ensure AI-generated content references valid, canonical IDs from `conker_registry`.

**Prompt Template:**
```
Before generating content, query the knowledge graph to validate asset IDs:

1. Use KgQuery::ListItems to discover available {item_type} with tags: [{tags}]
2. Filter results using KgQuery::SearchItems if needed
3. Reference the returned IDs in your output

Example:
Query: ListItems { item_type: Sfx, tags: ["vo", "conker"] }
Response: [ { "id": "SFX_501", "label": "VoConkPain01", ... }, ... ]

Now generate a NarrativeBeat that uses one of these SFX IDs.
```

**Why this works:** Grounds AI output in the canonical vocabulary, preventing invalid or invented IDs.

---

## Pattern 2: Contextual Dialog Generation

**Goal:** Produce character-appropriate, emotionally-tagged dialog lines.

**Prompt Template:**
```
Generate dialog using conker_narrative::DialogGenParams:

{
  "speaker": "{CHARACTER}",
  "context": "{DIALOG_CONTEXT}",
  "emotion": "{OPTIONAL_EMOTION}",
  "variables": { "target": "{TARGET_NAME}", "item": "{ITEM_NAME}" },
  "variant_count": {N}
}

Rules:
- Use only DialogContext values: taunt, pain, triumph, defeat, idle, objective, drunk, combat, discovery, transition
- Emotion must be from DialogEmotion enum if specified
- Interpolate {variables} into template text
- Return DialogGenResult with metadata

Example request:
{
  "speaker": "CONKER",
  "context": "drunk",
  "variables": {},
  "variant_count": 3
}
```

**Output Format:**
```json
{
  "lines": [
    {
      "speaker": "CONKER",
      "text": "*hic* Blimey, the room's gone all... spinny.",
      "emotion": "Confused",
      "duration_hint_s": null,
      "sfx_override": null
    }
  ],
  "metadata": {
    "seed": 12345,
    "applied_contexts": ["drunk"],
    "suggested_sfx": "SFX_503",
    "confidence": 0.85
  }
}
```

---

## Pattern 3: Narrative Beat Construction

**Goal:** Build complete, branching narrative beats with conditions and effects.

**Prompt Template:**
```
Construct a NarrativeBeat with the following structure:

{
  "id": "BEAT_UNIQUE_IDENTIFIER",
  "label": "Human-readable title",
  "type": "{BeatType}",
  "sfx_id": "OPTIONAL_REGISTRY_ID",
  "vfx_id": "OPTIONAL_REGISTRY_ID", 
  "asid_id": "OPTIONAL_REGISTRY_ID",
  "dialog_lines": [ /* DialogLine objects */ ],
  "conditions": [ /* BeatCondition objects */ ],
  "effects": [ /* BeatEffect objects */ ],
  "branches": [ /* BeatBranch objects for player choices */ ],
  "ai_metadata": {
    "tags": ["comedy", "combat", "conker"],
    "prompt_hint": "Optional hint for similar generation",
    "difficulty": 2,
    "estimated_duration_s": 15.0
  }
}

Constraints:
- BeatType must be: intro, trigger, action, objective, taunt, transition, outro
- All registry IDs (sfx_id, etc.) must be validated via KgQuery first
- Conditions use ConditionType and ConditionOperator enums
- Effects use EffectType enum
- Branches define player-choice paths

Example: Create a "taunt" beat where CONKER mocks GREGG before combat.
```

---

## Pattern 4: Machine-Readable Story Sequences

**Goal:** Output complete story sequences suitable for engine consumption.

**Prompt Template:**
```
Generate a StorySequence for {MAP_ID} in {GAME_MODE}:

{
  "id": "SEQ_{MAP}_{MODE}_{THEME}",
  "title": "Descriptive title",
  "entry_beat_id": "BEAT_START_ID",
  "beats": {
    "BEAT_START_ID": { /* NarrativeBeat */ },
    "BEAT_NEXT_ID": { /* NarrativeBeat */ },
    // ... more beats
  },
  "exit_conditions": [ /* When is the sequence complete? */ ],
  "ai_metadata": {
    "genre_tags": ["comedy", "action"],
    "player_count": 1,
    "compatible_modes": ["GMR_DEATHMATCH_CLASSIC"],
    "estimated_playtime_min": 5
  }
}

Requirements:
- Every beat ID referenced in branches must exist in the "beats" map
- Entry beat must have no incoming branches (or be marked as start)
- Exit conditions determine when the sequence is "done"
- Use KgQuery::GetValidCombinations to ensure beat assets are compatible

Example: A 3-beat intro sequence for Alien Base map in Deathmatch mode.
```

---

## Pattern 5: Prompt Conditioning with Metadata

**Goal:** Leverage `ai_metadata` fields to guide AI-Chat toward desired output styles.

**Key Metadata Fields:**
| Field | Purpose | Example Values |
|-------|---------|---------------|
| `tags` | Filter/search beats by theme | `["comedy", "gore", "stealth"]` |
| `prompt_hint` | Suggested phrasing for similar generation | `"Generate a drunk CONKER line targeting {target}"` |
| `difficulty` | Pacing control (1=easy, 5=hard) | `2` |
| `estimated_duration_s` | Timing for UI/cinematic sync | `12.5` |
| `content_notes` | Warnings for tone/sensitivity | `["mild language", "cartoon violence"]` |

**Prompt Template:**
```
When generating content, respect these ai_metadata constraints:

- If difficulty <= 2: keep dialog simple, avoid complex branching
- If tags include "drunk": use CONKER's drunk speech patterns (slurred, repetitive)
- If content_notes include "mild language": allow Conker-style cursing but avoid slurs
- Use prompt_hint as a template for your generation approach

Example request with metadata:
{
  "speaker": "CONKER",
  "context": "taunt",
  "ai_metadata": {
    "tags": ["drunk", "humor"],
    "prompt_hint": "Slurred insult targeting {target}",
    "difficulty": 1,
    "content_notes": ["mild language"]
  }
}
```

---

## Pattern 6: Batch Generation for Scenes

**Goal:** Produce multiple coordinated dialog lines or beats for a full scene.

**Prompt Template:**
```
Generate a batch of dialog lines for a multi-character scene:

{
  "requests": [
    {
      "speaker": "CONKER",
      "context": "taunt",
      "variables": {"target": "GREGG"},
      "variant_count": 2
    },
    {
      "speaker": "GREGG", 
      "context": "combat",
      "variables": {"target": "CONKER"},
      "variant_count": 2
    }
  ],
  "global_seed": 42
}

Rules:
- Use the same global_seed for reproducible results across requests
- Ensure character voices remain distinct (CONKER = sarcastic/drunk, GREGG = menacing)
- Return BatchDialogResult with per-request metadata

Post-processing:
- Use metadata.confidence to filter low-quality outputs
- Cross-reference suggested_sfx with conker_registry for asset validation
```

---

## Error Handling & Validation

**Common Errors and Fixes:**

| Error | Likely Cause | Fix |
|-------|-------------|-----|
| "Item 'XYZ' not found" | Invalid registry ID | Use KgQuery::ListItems to discover valid IDs first |
| "Invalid DialogContext" | Typo or invented value | Use only enum values: taunt, drunk, combat, etc. |
| "Branch references unknown beat" | Beat ID mismatch | Ensure all branch.next_beat_id values exist in beats map |
| "Schema validation failed" | Missing required field | Check NarrativeBeat/DialogLine schema for required fields |

**Validation Workflow:**
1. Generate content using patterns above
2. Serialize to JSON
3. Validate against schema using `schemaguard` or `conker_narrative::NarrativeSchemaBacked`
4. If validation fails, use error message to refine prompt

---

## Advanced: Chaining Queries for Complex Generation

**Example: Generate a complete combat encounter**

```
Step 1: Discover valid assets
Query: GetValidCombinations { context: CombatEffects { asid_id: "ASID_401" } }
→ Returns valid VFX_016, SFX_152 pairings

Step 2: Generate contextual dialog
Use conker_narrative::generate_dialog with:
{
  "speaker": "CONKER",
  "context": "combat",
  "variables": {"weapon": "chainsaw"},
  "variant_count": 3
}

Step 3: Construct the NarrativeBeat
{
  "id": "BEAT_CHAINSAW_EXECUTION",
  "type": "action",
  "asid_id": "ASID_401",
  "vfx_id": "VFX_016",
  "sfx_id": "SFX_152",
  "dialog_lines": [ /* from Step 2 */ ],
  "effects": [{ "type": "Spawn", "target": "gore_particles", ... }],
  "ai_metadata": { "tags": ["gore", "execution", "conker"], "difficulty": 3 }
}

Step 4: Validate
Serialize and validate against NarrativeBeat schema.
```

---

## Repository Integration Notes

- All generated JSON should be saved to `data/narrative/` with descriptive filenames
- Use `cargo run -p conker_kg_query -- --query-json '<query>'` to test queries locally
- CI will validate all `*.json` files in `data/narrative/` against schemas
- For AI-Chat sessions, include this guide's patterns in your system prompt for best results
