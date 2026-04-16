# godot/Systems/ASID/ASIDMappings.gd
# Engine-side ASID mapping table for Godot 4.x.
# This file is read by tools and CI (uncut_ci_checks) to verify that all ASIDs
# defined in docs/systems/AnimationStateRegistry.md are declared here.
#
# The keys must be three-digit string codes, e.g. "050", "400", "405", "012".
# Values are engine-local animation/state identifiers that your character
# state machines and AnimationTree/StateMachine nodes will use.

class_name ASIDMappings
extends Node

# ASID → animation/state name mapping for player characters and key NPCs.
# You can expand this over time, but do not remove or renumber existing keys
# without updating docs/systems/AnimationStateRegistry.md and CI.
var ASID_TO_STATE: Dictionary = {
	# Heavy carry locomotion (no jump, ~0.6x move speed, reduced turn rate).
	"050": "hld_heavy_walk",

	# Hit stun / daze lock.
	"012": "hit_stun_daze",

	# Chainsaw vertical execution (decap/bisect).
	"400": "fin_chainsaw_vertical",

	# Katana 360° sweep execution.
	"405": "fin_katana_sweep",

	# Gregg scythe reap execution (unique to Gregg).
	"666": "fin_gregg_reap",

	# Alien pounce (movement override that leads into facebite on hit).
	"900": "aln_pounce_strike",

	# Alien facebite execution (hard lock, gore trigger).
	"901": "aln_bite_execution",

	# Zombie crawl locomotion (post-maim state).
	"920": "zmb_crawl_move"
}

# Optional reverse lookup if you ever need STATE → ASID at runtime.
var STATE_TO_ASID: Dictionary = {}


func _ready() -> void:
	# Build reverse lookup at runtime.
	for asid_code: String in ASID_TO_STATE.keys():
		var state_name := ASID_TO_STATE[asid_code]
		STATE_TO_ASID[state_name] = asid_code


func get_state_name(asid_code: String) -> String:
	# Returns engine-local state/animation name for a given ASID code,
	# or an empty string if the ASID is unknown.
	if ASID_TO_STATE.has(asid_code):
		return ASID_TO_STATE[asid_code]
	return ""


func get_asid_code(state_name: String) -> String:
	# Returns three-digit ASID code for a given engine-local state name,
	# or an empty string if the state is unmapped.
	if STATE_TO_ASID.has(state_name):
		return STATE_TO_ASID[state_name]
	return ""
