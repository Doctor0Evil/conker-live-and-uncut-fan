using System.Collections.Generic;
using UnityEngine;

namespace UncutMultiplayer.Gameplay
{
    [CreateAssetMenu(
        fileName = "WeaponRegistry",
        menuName = "Uncut/Weapon Registry")]
    public class WeaponRegistry : ScriptableObject
    {
        [SerializeField]
        private TextAsset weaponStatsJson;

        private readonly Dictionary<string, WeaponStats> _table =
            new Dictionary<string, WeaponStats>();

        private bool _initialized;

        public void InitializeIfNeeded()
        {
            if (_initialized)
                return;

            if (weaponStatsJson == null)
            {
                Debug.LogError("WeaponRegistry: weaponStatsJson is not set.");
                return;
            }

            var file = JsonUtility.FromJson<WeaponStatsFile>(
                weaponStatsJson.text);

            if (file != null && file.weapons != null)
            {
                _table.Clear();
                foreach (var w in file.weapons)
                {
                    if (!string.IsNullOrEmpty(w.Id))
                    {
                        _table[w.Id] = w;
                    }
                }
            }

            _initialized = true;
        }

        public bool TryGet(string weaponId, out WeaponStats stats)
        {
            InitializeIfNeeded();
            return _table.TryGetValue(weaponId, out stats);
        }
    }
}
