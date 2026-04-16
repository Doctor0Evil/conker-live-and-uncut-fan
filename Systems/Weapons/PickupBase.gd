extends Area3D
class_name PickupBase

@export_category("Pickup")
@export var weapon_id: StringName = &"Unset"        # e.g. &"Chainsaw"
@export var is_heavy_carry: bool = false           # fallback if stats JSON missing
@export var respawn_time_sec: float = 0.0          # 0 = no respawn
@export var interaction_radius: float = 2.0        # meters

var _state: int = 0                                # 0 = available, 1 = taken, 2 = respawning
var _respawn_timer: float = 0.0

@onready var _collision_shape: CollisionShape3D = $CollisionShape3D
@onready var _mesh_instance: Node3D = $MeshInstance3D
@onready var _weapon_registry: WeaponRegistry = (
	get_tree().get_first_node_in_group("weapon_registry") as WeaponRegistry
)

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

	# New ASID‑aware path: give_weapon_from_stats + heavy‑carry hooks.
	if not body.has_method("give_weapon_from_stats"):
		return

	var stats: Dictionary = {}

	if _weapon_registry:
		stats = _weapon_registry.get_weapon_stats(weapon_id)

	# Fallback: synthesize minimal stats if JSON entry is missing.
	if stats.is_empty():
		stats = {
			"id": String(weapon_id),
			"is_heavy_carry": is_heavy_carry
		}

	var consumed: bool = body.give_weapon_from_stats(stats)
	if not consumed:
		return

	# Drive heavy carry from stats + character adapter methods.
	var heavy_flag := stats.get("is_heavy_carry", is_heavy_carry)

	if heavy_flag and body.has_method("enter_heavy_carry"):
		body.enter_heavy_carry(stats)
	elif body.has_method("clear_heavy_carry"):
		body.clear_heavy_carry()

	_set_state(1)

	if respawn_time_sec > 0.0:
		_state = 2
		_respawn_timer = respawn_time_sec

func _set_state(new_state: int) -> void:
	_state = new_state
	var visible := (_state == 0)

	if _mesh_instance:
		_mesh_instance.visible = visible

	# Area3D visibility vs. collision.
	self.visible = visible
	monitoring = visible
	monitorable = visible

func get_weapon_id() -> StringName:
	return weapon_id

func get_is_heavy_carry() -> bool:
	return is_heavy_carry

func get_state() -> int:
	return _state
