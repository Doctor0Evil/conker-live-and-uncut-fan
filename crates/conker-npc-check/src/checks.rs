// Destination: crates/conker-npc-check/src/checks.rs

use crate::model::{CheckReport, SessionProfile};
use anyhow::Result;
use conker_schema::{ConkerMapRecipe, NpcContract};
use std::collections::{BTreeMap, BTreeSet};

pub fn run_all_checks(
    maps: &[(String, ConkerMapRecipe)],
    npc_contracts: &[(String, NpcContract)],
    session: &SessionProfile,
) -> Result<CheckReport> {
    let mut report = CheckReport::default();

    let npc_by_id: BTreeMap<String, &NpcContract> =
        npc_contracts.iter().map(|(id, npc)| (id.clone(), npc)).collect();

    if npc_by_id.len() != npc_contracts.len() {
        report.push("duplicate NPC IDs found in npc contracts");
    }

    check_maps_reference_existing_npcs(maps, &npc_by_id, &mut report);

    if session.enforce_zombie_headshot_rule {
        check_zombie_headshot_rule(&npc_by_id, &mut report);
    }

    if session.enforce_pickup_only_arsenal {
        check_no_class_or_loadout_fields(&npc_by_id, &mut report);
    }

    // New: enforce Fire Imp invariants unconditionally for Conker sessions.
    check_fire_imp_blood_vial_invariants(&npc_by_id, &mut report);

    Ok(report)
}

fn check_maps_reference_existing_npcs(
    maps: &[(String, ConkerMapRecipe)],
    npc_by_id: &BTreeMap<String, &NpcContract>,
    report: &mut CheckReport,
) {
    // We assume ConkerMapRecipe has a field:
    //   npc_refs: Vec<NpcRef> where NpcRef { npc_id: String }
    for (map_id, map) in maps {
        for npc_ref in &map.npc_refs {
            if !npc_by_id.contains_key(&npc_ref.npc_id) {
                report.push(format!(
                    "map '{}' references unknown NPC contract '{}'",
                    map_id, npc_ref.npc_id
                ));
            }
        }
    }
}

/// Enforce that zombie contracts have headshot-style kill tags and ignore
/// bodyshot tags, matching the N64 graveyard behavior.
fn check_zombie_headshot_rule(
    npc_by_id: &BTreeMap<String, &NpcContract>,
    report: &mut CheckReport,
) {
    for (id, npc) in npc_by_id {
        let is_zombie = npc.kind.to_string().to_uppercase() == "ZOMBIE"
            || id.to_lowercase().contains("zombie");

        if !is_zombie {
            continue;
        }

        let kill_tags: BTreeSet<_> = npc.damageable.kill_tags.iter().collect();
        let has_headshot = kill_tags
            .iter()
            .any(|tag| tag.contains("headshot") || tag.contains("HEADSHOT"));

        if !has_headshot {
            report.push(format!(
                "NPC '{}' (kind ZOMBIE) is missing a headshot kill tag in damageable.killTags",
                id
            ));
        }

        let ignore_tags: BTreeSet<_> = npc.damageable.ignore_tags.iter().collect();
        let ignores_body = ignore_tags
            .iter()
            .any(|tag| tag.contains("bodyshot") || tag.contains("BODYSHOT"));

        if !ignores_body {
            report.push(format!(
                "NPC '{}' (kind ZOMBIE) should ignore at least one bodyshot tag in damageable.ignoreTags",
                id
            ));
        }
    }
}

/// Guardrail: reject class/loadout-like naming in NPC contracts.
fn check_no_class_or_loadout_fields(
    npc_by_id: &BTreeMap<String, &NpcContract>,
    report: &mut CheckReport,
) {
    for (id, npc) in npc_by_id {
        let lower_id = id.to_lowercase();
        let lower_title = npc.title.to_lowercase();

        let banned_markers = [
            "class.",
            ".assault",
            ".sniper",
            ".medic",
            ".engineer",
            "loadout",
            "perk",
        ];

        if banned_markers
            .iter()
            .any(|m| lower_id.contains(m) || lower_title.contains(m))
        {
            report.push(format!(
                "NPC '{}' uses class/loadout-like naming ('{}'); this violates the pickup-only arsenal rule",
                id, npc.title
            ));
        }
    }
}

/// Fire Imp invariant:
/// - Must only become active when a blood-vial-related condition is present.
/// - Must have a relatively long respawn cooldown to give players breathing room.
fn check_fire_imp_blood_vial_invariants(
    npc_by_id: &BTreeMap<String, &NpcContract>,
    report: &mut CheckReport,
) {
    const MIN_FIRE_IMP_COOLDOWN: f32 = 30.0;

    for (id, npc) in npc_by_id {
        let is_fire_imp = npc.kind.to_string().to_uppercase() == "FIRE_IMP"
            || id.to_lowercase().contains("fire_imp");

        if !is_fire_imp {
            continue;
        }

        // 1. Activation condition must reference a blood-vial-style condition.
        let mut has_blood_vial_condition = false;
        for cond in &npc.spawn.activation_conditions {
            let cid = cond.condition_id.to_lowercase();
            if cid.contains("blood_vial") || cid.contains("bloodvial") {
                has_blood_vial_condition = true;
                break;
            }
        }

        if !has_blood_vial_condition {
            report.push(format!(
                "NPC '{}' (Fire Imp) must have an activation condition tied to a blood-vial carrier or objective (conditionId containing 'blood_vial')",
                id
            ));
        }

        // 2. Respawn cooldown must be >= MIN_FIRE_IMP_COOLDOWN.
        let cd = npc.spawn.respawn.cooldown_seconds;
        if cd < MIN_FIRE_IMP_COOLDOWN {
            report.push(format!(
                "NPC '{}' (Fire Imp) respawn.cooldownSeconds={} is too short; expected >= {} seconds to provide breathing room",
                id, cd, MIN_FIRE_IMP_COOLDOWN
            ));
        }
    }
}
