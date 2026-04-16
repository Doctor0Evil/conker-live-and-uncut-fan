# godot/Systems/ASID/ASIDExecutionHelpers.gd
# Shared helpers for reasoning about ASIDs on the Godot side.

extends Node
class_name ASIDExecutionHelpers

# Set of ASIDs that represent hard-locked execution states.
const EXECUTION_ASIDS: = {
	"400": true, # FIN_CHAINSAW_V
	"405": true, # FIN_SABRE_H
	"666": true, # SPEC_GREGG_REAP
	"901": true  # ALN_BITE_EXEC
}


static func is_execution_asid(asid_code: String) -> bool:
	return EXECUTION_ASIDS.has(asid_code)


static func should_ignore_hazard_damage(asid_code: String) -> bool:
	# Mirror Unreal: executions are hazard-immune while locked.
	return is_execution_asid(asid_code)
