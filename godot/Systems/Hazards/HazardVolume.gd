extends Area3D

class_name HazardVolume

@export var damage_per_second: float = 60.0
@export var active: bool = false

var _bodies: = []
var _asid_helpers: ASIDExecutionHelpers


func _ready() -> void:
	asid_helpers_init()
	body_entered.connect(_on_body_entered)
	body_exited.connect(_on_body_exited)


func asid_helpers_init() -> void:
	_asid_helpers = ASIDExecutionHelpers.new()


func _on_body_entered(body: Node3D) -> void:
	if body not in _bodies:
		_bodies.append(body)


func _on_body_exited(body: Node3D) -> void:
	_bodies.erase(body)


func _process(delta: float) -> void:
	if not active:
		return

	for body in _bodies:
		if not is_instance_valid(body):
			continue

		var asid_code := ""
		if body.has_method("get_current_asid_code"):
			asid_code = body.get_current_asid_code()

		if _asid_helpers.should_ignore_hazard_damage(asid_code):
			continue

		if body.has_method("apply_hazard_damage"):
			body.apply_hazard_damage(damage_per_second * delta)
