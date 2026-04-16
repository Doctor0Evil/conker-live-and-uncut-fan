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
        public bool IsHeavyCarry;         // maps to ASID050
        public float RespawnTimeSeconds;  // 0 = no respawn
        public float InteractionRadius;   // ≈ 2.0f in Alien Base hub
    }

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

            // Apply pickup via interface: weapon registry + ASID050 heavy carry
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
            // Expected to:
            //  1. Look up spec.WeaponId in your weaponstatsv1.json-backed table
            //  2. Give weapon / ammo to character
            //  3. If spec.IsHeavyCarry, enable HeavyCarry (ASID050) on character
            // Return true if pickup consumed, false to leave it
            return character.GiveWeaponPickup(spec.WeaponId, spec.IsHeavyCarry);
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

    // Example character interface that your player controller implements
    public interface IUncutCharacter
    {
        // Applies weapon + heavy-carry (ASID050) from your weapon registry
        bool GiveWeaponPickup(string weaponId, bool isHeavyCarry);
    }
}
