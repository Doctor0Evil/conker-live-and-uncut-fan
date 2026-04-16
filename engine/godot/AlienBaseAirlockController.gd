extends Node

##
## AlienBaseAirlockController
##
## Map-level Airlock/Gas state machine for 04_Multiplayer_Alien_Base.
## Controls hazard Area3D nodes and responds to trigger consoles.
##

enum AirlockState {
    IDLE,
    ARMING,
    ACTIVE,
    COOLDOWN,
}

@export var hub_floor_gas_volume: NodePath
@export var sublevel_acid_volume: NodePath

@export var arming_duration_sec: float = 5.0
@export var active_duration_sec: float = 12.0
@export var cooldown_duration_sec: float = 30.0

var current_state: AirlockState = AirlockState.IDLE
var time_in_state: float = 0.0
var event_instigator_team: int = -1

var _hub_volume_ref: Node = null
var _sublevel_volume_ref: Node = null

func _ready() -> void:
    if hub_floor_gas_volume != NodePath(""):
        _hub_volume_ref = get_node(hub_floor_gas_volume)
    if sublevel_acid_volume != NodePath(""):
        _sublevel_volume_ref = get_node(sublevel_acid_volume)
    _enter_state(AirlockState.IDLE)

func _process(delta: float) -> void:
    _update_state(delta)

func request_trigger_activation(trigger_id: String, instigator_team: int) -> void:
    if current_state != AirlockState.IDLE:
        return

    event_instigator_team = instigator_team
    _enter_state(AirlockState.ARMING)
    # TODO: Play arming VO/sfx and warning lights.

func _enter_state(new_state: AirlockState) -> void:
    current_state = new_state
    time_in_state = 0.0

    match current_state:
        AirlockState.IDLE:
            _on_entered_idle()
        AirlockState.ARMING:
            _on_entered_arming()
        AirlockState.ACTIVE:
            _on_entered_active()
        AirlockState.COOLDOWN:
            _on_entered_cooldown()

func _update_state(delta: float) -> void:
    time_in_state += delta

    match current_state:
        AirlockState.IDLE:
            pass

        AirlockState.ARMING:
            if time_in_state >= arming_duration_sec:
                _enter_state(AirlockState.ACTIVE)

        AirlockState.ACTIVE:
            if time_in_state >= active_duration_sec:
                _enter_state(AirlockState.COOLDOWN)

        AirlockState.COOLDOWN:
            if time_in_state >= cooldown_duration_sec:
                _enter_state(AirlockState.IDLE)

func _on_entered_idle() -> void:
    _set_hazard_volumes_active(false)
    event_instigator_team = -1

func _on_entered_arming() -> void:
    _set_hazard_volumes_active(false)
    # TODO: Sirens, flicker lights, pre-gas FX.

func _on_entered_active() -> void:
    _set_hazard_volumes_active(true)
    # TODO: Update VO/UI to "Airlock sealed / gas released".

func _on_entered_cooldown() -> void:
    _set_hazard_volumes_active(false)
    # TODO: Venting FX, ambience restore.

func _set_hazard_volumes_active(active: bool) -> void:
    if _hub_volume_ref and _hub_volume_ref.has_method("set_active"):
        _hub_volume_ref.call("set_active", active)
    if _sublevel_volume_ref and _sublevel_volume_ref.has_method("set_active"):
        _sublevel_volume_ref.call("set_active", active)
