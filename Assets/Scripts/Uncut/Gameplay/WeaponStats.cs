using UnityEngine;

namespace UncutMultiplayer.Gameplay
{
    [System.Serializable]
    public class WeaponStats
    {
        public string Id;
        public string Category;
        public float Damage;
        public float RangeMeters;
        public float FireRateHz;
        public bool IsHeavyCarry;
        public int ClipSize;
        public int ReservedAmmo;
        public float MovementSpeedMult;
    }

    [System.Serializable]
    public class WeaponStatsFile
    {
        public string version;
        public string schemaversion;
        public WeaponStats[] weapons;
    }
}
