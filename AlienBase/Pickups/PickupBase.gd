extends Area3D
class_name PickupBase

@export_category("Pickup")
@export var weapon_id: StringName = &"Unset"        # e.g. &"Chainsaw"
@export var is_heavy_carry: bool = false           # maps to ASID050
@export var respawn_time_sec: float = 0.0          # 0 = no respawn
@export var interaction_radius: float = 2.0        # meters

var _state: int = 0 # 0 = available, 1 = taken, 2 = respawning
var _respawn_timer: float = 0.0

@onready var _collision_shape: CollisionShape3D = $CollisionShape3D
@onready var _mesh_instance: Node3D = $MeshInstance3D

func _ready() -> void:
	var sphere_shape := SphereShape3D.new()
	sphere_shape.radius = interaction_radius
	_collision_shape.shape = sphere_shape

	body_entered.connect(_on_body_entered)
	_set_state(0)

func _process(delta: float) -> void:
	if _state == 2 and respawn_time_sec > 0.0:
		_respawn_timer -= delta
		if _respawn_timer <= 0.0:
			_set_state(0)

func _on_body_entered(body: Node) -> void:
	if _state != 0:
		return

	if not body.has_method("give_weapon_pickup"):
		return

	var consumed: bool = body.give_weapon_pickup(weapon_id, is_heavy_carry)
	if not consumed:
		return

	_set_state(1)

	if respawn_time_sec > 0.0:
		_state = 2
		_respawn_timer = respawn_time_sec

func _set_state(new_state: int) -> void:
	_state = new_state
	var visible := (_state == 0)
	if _mesh_instance:
		_mesh_instance.visible = visible
	visible = (_state == 0)
	monitoring = visible
	monitorable = visible

func get_weapon_id() -> StringName:
	return weapon_id

func get_is_heavy_carry() -> bool:
	return is_heavy_carry

func get_state() -> int:
	return _state
