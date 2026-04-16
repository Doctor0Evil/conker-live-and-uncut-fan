extends Node
class_name WeaponRegistry

var _table: Dictionary = {}
var _initialized: bool = false

@export_file("*.json")
var weapon_stats_path: String = "res://data/weapons/weaponstatsv1.json"

func _ready() -> void:
	_initialize_if_needed()

func _initialize_if_needed() -> void:
	if _initialized:
		return

	var file := FileAccess.open(weapon_stats_path, FileAccess.READ)
	if file == null:
		push_error("WeaponRegistry: Failed to open %s" % weapon_stats_path)
		return

	var text := file.get_as_text()
	file.close()

	var parsed := JSON.parse_string(text)
	if typeof(parsed) != TYPE_DICTIONARY:
		push_error("WeaponRegistry: Invalid JSON structure.")
		return

	_table.clear()
	if parsed.has("weapons") and typeof(parsed["weapons"]) == TYPE_ARRAY:
		for w in parsed["weapons"]:
			if typeof(w) == TYPE_DICTIONARY and w.has("id"):
				_table[w["id"]] = w

	_initialized = true

func get_weapon_stats(weapon_id: StringName) -> Dictionary:
	_initialize_if_needed()
	if _table.has(String(weapon_id)):
		return _table[String(weapon_id)]
	return {}
