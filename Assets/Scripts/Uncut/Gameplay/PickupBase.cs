using UnityEngine;

namespace UncutMultiplayer.Gameplay
{
    public enum PickupState
    {
        Available,
        Taken,
        Respawning
    }

    [System.Serializable]
    public struct WeaponPickupSpec
    {
        public string WeaponId;           // e.g. "Chainsaw", "Bazooka"
        public bool IsHeavyCarry;         // maps to ASID050 (can mirror WeaponStats.IsHeavyCarry)
        public float RespawnTimeSeconds;  // 0 = no respawn
        public float InteractionRadius;   // ≈ 2.0f in Alien Base hub
    }

    // Scriptable registry + JSON were defined previously; we just consume it here.
    //  - WeaponRegistry.TryGet(string id, out WeaponStats stats)
    //  - WeaponStats has IsHeavyCarry, MovementSpeedMult, etc.

    [RequireComponent(typeof(SphereCollider))]
    public class PickupBase : MonoBehaviour
    {
        [SerializeField]
        private WeaponPickupSpec pickupSpec = new WeaponPickupSpec
        {
            WeaponId = "Unset",
            IsHeavyCarry = false,
            RespawnTimeSeconds = 0.0f,
            InteractionRadius = 2.0f
        };

        [SerializeField]
        private MeshRenderer pickupRenderer;

        [SerializeField]
        private Collider pickupCollider;

        [SerializeField]
        private LayerMask characterLayerMask = ~0; // filter if needed

        [SerializeField]
        private WeaponRegistry weaponRegistry;

        private PickupState state = PickupState.Available;
        private float respawnTimer;

        private SphereCollider interactionCollider;

        protected virtual void Awake()
        {
            interactionCollider = GetComponent<SphereCollider>();
            interactionCollider.isTrigger = true;
            interactionCollider.radius = pickupSpec.InteractionRadius;

            if (pickupCollider == null)
            {
                pickupCollider = interactionCollider;
            }

            if (pickupRenderer == null)
            {
                pickupRenderer = GetComponentInChildren<MeshRenderer>();
            }

            SetState(PickupState.Available);
        }

        protected virtual void Update()
        {
            if (state == PickupState.Respawning && pickupSpec.RespawnTimeSeconds > 0.0f)
            {
                respawnTimer -= Time.deltaTime;
                if (respawnTimer <= 0.0f)
                {
                    SetState(PickupState.Available);
                }
            }
        }

        private void OnTriggerEnter(Collider other)
        {
            if (state != PickupState.Available)
                return;

            if (((1 << other.gameObject.layer) & characterLayerMask) == 0)
                return;

            var character = other.GetComponent<IUncutCharacter>();
            if (character == null)
                return;

            if (!ApplyPickupToCharacter(character, pickupSpec))
                return;

            SetState(PickupState.Taken);

            if (pickupSpec.RespawnTimeSeconds > 0.0f)
            {
                state = PickupState.Respawning;
                respawnTimer = pickupSpec.RespawnTimeSeconds;
            }
        }

        protected virtual bool ApplyPickupToCharacter(IUncutCharacter character, WeaponPickupSpec spec)
        {
            if (weaponRegistry == null)
            {
                Debug.LogError("PickupBase: WeaponRegistry is not assigned.");
                return false;
            }

            if (!weaponRegistry.TryGet(spec.WeaponId, out var stats))
            {
                Debug.LogWarning($"PickupBase: No stats for weapon '{spec.WeaponId}'.");
                return false;
            }

            if (!character.GiveWeapon(stats))
            {
                return false;
            }

            var heavyCarry = (character as Component)?.GetComponent<IHeavyCarryAdapter>();
            if (heavyCarry != null)
            {
                // Prefer stats.IsHeavyCarry from JSON; spec.IsHeavyCarry can act as a local override
                bool isHeavy = stats.IsHeavyCarry || spec.IsHeavyCarry;
                if (isHeavy)
                {
                    heavyCarry.EnterHeavyCarry(stats);
                }
                else
                {
                    heavyCarry.ClearHeavyCarry();
                }
            }

            return true;
        }

        private void SetState(PickupState newState)
        {
            state = newState;

            bool visible = (state == PickupState.Available);
            if (pickupRenderer != null)
                pickupRenderer.enabled = visible;

            if (pickupCollider != null)
                pickupCollider.enabled = visible;
        }

        public WeaponPickupSpec Spec => pickupSpec;
        public PickupState State => state;
    }

    // Character side: used by PickupBase
    public interface IUncutCharacter
    {
        // Gives weapon using stats from WeaponRegistry / weaponstatsv1.json
        bool GiveWeapon(WeaponStats stats);
    }

    // Heavy-carry / ASID050 adapter on the character controller
    public interface IHeavyCarryAdapter
    {
        void EnterHeavyCarry(WeaponStats stats);
        void ClearHeavyCarry();
    }
}
