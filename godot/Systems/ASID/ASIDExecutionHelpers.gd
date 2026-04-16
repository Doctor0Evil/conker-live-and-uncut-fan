extends Node
class_name ASIDExecutionHelpers

const EXECUTION_ASIDS := {
	"400": true, # FIN_CHAINSAW_V
	"405": true, # FIN_SABRE_H
	"666": true, # SPEC_GREGG_REAP
	"901": true, # ALN_BITE_EXEC
}

const SOFT_LOCK_ASIDS := {
	"012": true, # HIT_STUN_DAZE
}

const MOVEMENT_MODE_ASIDS := {
	"050": true, # HLD_HEAVY_WALK
	"920": true, # ZMB_CRAWL_MOVE
}


static func is_execution_asid(asid_code: String) -> bool:
	return EXECUTION_ASIDS.has(asid_code)


static func is_soft_lock_asid(asid_code: String) -> bool:
	return SOFT_LOCK_ASIDS.has(asid_code)


static func is_movement_mode_asid(asid_code: String) -> bool:
	return MOVEMENT_MODE_ASIDS.has(asid_code)


static func get_lock_type(asid_code: String) -> String:
	if is_execution_asid(asid_code):
		return "hard_lock"
	if is_soft_lock_asid(asid_code):
		return "soft_lock"
	if is_movement_mode_asid(asid_code):
		return "movement_mode"
	return "none"


static func should_ignore_hazard_damage(asid_code: String) -> bool:
	return is_execution_asid(asid_code)


static func is_jump_disabled(asid_code: String) -> bool:
	# Heavy carry + zombie crawl.
	return is_movement_mode_asid(asid_code)


static func get_move_speed_multiplier(asid_code: String) -> float:
	match asid_code:
		"050":
			return 0.6
		"920":
			return 0.25
		_:
			return 1.0
