using System;
using System.Collections.Generic;

namespace Uncut.Weapons
{
    [Serializable]
    public class WeaponStats
    {
        public string Id;
        public string DisplayName;
        public string WeaponType;
        public string Category;

        public float DamagePerHit;
        public float DamagePerPellet;
        public int PelletCount;
        public float HeadshotMultiplier;

        public string FireMode;
        public float RateOfFire;
        public int ClipSize;
        public int MaxReserveAmmo;
        public float ReloadTime;

        public float ProjectileSpeed;
        public float SpreadDegrees;

        public float MoveSpeedMultiplier;
        public bool NoJump;
        public bool IsHeavy;
        public bool HasFriendlyFireAoe;

        public string ZombieDamageProfileId;

        public string SfxFire;
        public string SfxHit;
        public string VfxMuzzle;
    }

    [Serializable]
    public class WeaponRuntimeState
    {
        public WeaponStats Stats;
        public int ClipAmmo;
        public int ReserveAmmo;
    }

    public class WeaponInventory
    {
        private readonly Dictionary<string, WeaponRuntimeState> _weapons =
            new Dictionary<string, WeaponRuntimeState>();

        public WeaponRuntimeState CurrentWeapon { get; private set; }

        public event Action<WeaponRuntimeState> OnWeaponChanged;

        public void InitializeFromStats(IEnumerable<WeaponStats> allStats)
        {
            _weapons.Clear();
            CurrentWeapon = null;

            foreach (var stats in allStats)
            {
                var state = new WeaponRuntimeState
                {
                    Stats = stats,
                    ClipAmmo = 0,
                    ReserveAmmo = 0
                };
                _weapons[stats.Id] = state;
            }
        }

        public bool HasWeapon(string weaponId)
        {
            return _weapons.TryGetValue(weaponId, out var state) &&
                   (state.ClipAmmo > 0 || state.ReserveAmmo > 0);
        }

        public void GiveWeapon(string weaponId, int clipAmmo, int reserveAmmo, bool makeCurrent)
        {
            if (!_weapons.TryGetValue(weaponId, out var state))
            {
                return;
            }

            if (state.Stats.ClipSize > 0)
            {
                state.ClipAmmo = Math.Min(state.Stats.ClipSize, state.ClipAmmo + clipAmmo);
            }

            state.ReserveAmmo = Math.Min(state.Stats.MaxReserveAmmo, state.ReserveAmmo + reserveAmmo);

            if (makeCurrent)
            {
                SetCurrentWeapon(weaponId);
            }
        }

        public void SetCurrentWeapon(string weaponId)
        {
            if (!_weapons.TryGetValue(weaponId, out var state))
            {
                return;
            }

            CurrentWeapon = state;
            OnWeaponChanged?.Invoke(CurrentWeapon);
        }

        public bool TryConsumeAmmo()
        {
            if (CurrentWeapon == null)
            {
                return false;
            }

            var stats = CurrentWeapon.Stats;
            if (stats.ClipSize <= 0)
            {
                return true;
            }

            if (CurrentWeapon.ClipAmmo > 0)
            {
                CurrentWeapon.ClipAmmo--;
                return true;
            }

            return false;
        }

        public bool CanReloadCurrent()
        {
            if (CurrentWeapon == null)
            {
                return false;
            }

            var stats = CurrentWeapon.Stats;
            if (stats.ClipSize <= 0)
            {
                return false;
            }

            if (CurrentWeapon.ClipAmmo >= stats.ClipSize)
            {
                return false;
            }

            return CurrentWeapon.ReserveAmmo > 0;
        }

        public void ReloadCurrent()
        {
            if (!CanReloadCurrent())
            {
                return;
            }

            var stats = CurrentWeapon.Stats;
            var needed = stats.ClipSize - CurrentWeapon.ClipAmmo;
            var taken = Math.Min(needed, CurrentWeapon.ReserveAmmo);

            CurrentWeapon.ClipAmmo += taken;
            CurrentWeapon.ReserveAmmo -= taken;
        }

        public float GetCurrentMoveSpeedMultiplier()
        {
            if (CurrentWeapon == null)
            {
                return 1.0f;
            }

            return CurrentWeapon.Stats.MoveSpeedMultiplier;
        }

        public bool GetCurrentNoJump()
        {
            if (CurrentWeapon == null)
            {
                return false;
            }

            return CurrentWeapon.Stats.NoJump;
        }

        public bool GetCurrentIsHeavy()
        {
            if (CurrentWeapon == null)
            {
                return false;
            }

            return CurrentWeapon.Stats.IsHeavy;
        }
    }
}
