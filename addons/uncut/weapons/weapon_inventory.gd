extends Node
class_name WeaponInventory

class WeaponStats:
    var id: String = ""
    var display_name: String = ""
    var weapon_type: String = ""
    var category: String = ""

    var damage_per_hit: float = 0.0
    var damage_per_pellet: float = 0.0
    var pellet_count: int = 0
    var headshot_multiplier: float = 1.0

    var fire_mode: String = ""
    var rate_of_fire: float = 1.0
    var clip_size: int = 0
    var max_reserve_ammo: int = 0
    var reload_time: float = 0.0

    var projectile_speed: float = 0.0
    var spread_degrees: float = 0.0

    var move_speed_multiplier: float = 1.0
    var no_jump: bool = false
    var is_heavy: bool = false
    var has_friendly_fire_aoe: bool = false

    var zombie_damage_profile_id: String = ""

    var sfx_fire: String = ""
    var sfx_hit: String = ""
    var vfx_muzzle: String = ""

class WeaponRuntimeState:
    var stats: WeaponStats
    var clip_ammo: int = 0
    var reserve_ammo: int = 0

signal weapon_changed(new_weapon: WeaponRuntimeState)

var _weapons: Dictionary = {}
var _current_weapon_id: String = ""

func _ready() -> void:
    pass

func initialize_from_stats(all_stats: Array) -> void:
    _weapons.clear()
    _current_weapon_id = ""

    for stats in all_stats:
        var state := WeaponRuntimeState.new()
        state.stats = stats
        state.clip_ammo = 0
        state.reserve_ammo = 0
        _weapons[stats.id] = state

func has_weapon(weapon_id: String) -> bool:
    if not _weapons.has(weapon_id):
        return false
    var state: WeaponRuntimeState = _weapons[weapon_id]
    return state.clip_ammo > 0 or state.reserve_ammo > 0

func give_weapon(weapon_id: String, clip_ammo: int, reserve_ammo: int, make_current: bool) -> void:
    if not _weapons.has(weapon_id):
        return

    var state: WeaponRuntimeState = _weapons[weapon_id]
    var stats: WeaponStats = state.stats

    if stats.clip_size > 0:
        state.clip_ammo = clamp(state.clip_ammo + clip_ammo, 0, stats.clip_size)

    state.reserve_ammo = clamp(state.reserve_ammo + reserve_ammo, 0, stats.max_reserve_ammo)

    if make_current:
        set_current_weapon(weapon_id)

func set_current_weapon(weapon_id: String) -> void:
    if not _weapons.has(weapon_id):
        return

    _current_weapon_id = weapon_id
    emit_signal("weapon_changed", _weapons[weapon_id])

func try_consume_ammo() -> bool:
    if not _weapons.has(_current_weapon_id):
        return false

    var state: WeaponRuntimeState = _weapons[_current_weapon_id]
    var stats: WeaponStats = state.stats

    if stats.clip_size <= 0:
        return true

    if state.clip_ammo > 0:
        state.clip_ammo -= 1
        return true

    return false

func can_reload_current() -> bool:
    if not _weapons.has(_current_weapon_id):
        return false

    var state: WeaponRuntimeState = _weapons[_current_weapon_id]
    var stats: WeaponStats = state.stats

    if stats.clip_size <= 0:
        return false

    if state.clip_ammo >= stats.clip_size:
        return false

    return state.reserve_ammo > 0

func reload_current() -> void:
    if not _weapons.has(_current_weapon_id):
        return

    var state: WeaponRuntimeState = _weapons[_current_weapon_id]
    var stats: WeaponStats = state.stats

    if stats.clip_size <= 0:
        return

    if state.reserve_ammo <= 0:
        return

    var needed := stats.clip_size - state.clip_ammo
    var taken := min(needed, state.reserve_ammo)

    state.clip_ammo += taken
    state.reserve_ammo -= taken

func get_current_move_speed_multiplier() -> float:
    if not _weapons.has(_current_weapon_id):
        return 1.0
    var state: WeaponRuntimeState = _weapons[_current_weapon_id]
    return state.stats.move_speed_multiplier

func get_current_no_jump() -> bool:
    if not _weapons.has(_current_weapon_id):
        return false
    var state: WeaponRuntimeState = _weapons[_current_weapon_id]
    return state.stats.no_jump

func get_current_is_heavy() -> bool:
    if not _weapons.has(_current_weapon_id):
        return false
    var state: WeaponRuntimeState = _weapons[_current_weapon_id]
    return state.stats.is_heavy
