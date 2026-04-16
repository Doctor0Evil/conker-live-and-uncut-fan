// Unity/Assets/Scripts/Systems/ASID/ASIDHelpers.cs
// Engine-side helpers for reasoning about Animation State IDs (ASIDs).
// Mirrors the lock categories defined in docs/systems/AnimationStateRegistry.md.

using UnityEngine;

namespace Uncut.Multiplayer.Systems
{
	public enum ASIDLockType
	{
		None = 0,
		SoftLock,      // e.g., stun; damped movement, limited input.
		MovementMode,  // e.g., heavy carry, zombie crawl; locomotion override.
		HardLock       // e.g., executions; full lock while active.
	}

	public static class ASIDHelpers
	{
		public static bool IsExecutionASID(int asid)
		{
			switch (asid)
			{
				case 400: // FIN_CHAINSAW_V
				case 405: // FIN_SABRE_H
				case 666: // SPEC_GREGG_REAP
				case 901: // ALN_BITE_EXEC
					return true;
				default:
					return false;
			}
		}

		public static bool IsSoftLockASID(int asid)
		{
			switch (asid)
			{
				case 12: // HIT_STUN_DAZE
					return true;
				default:
					return false;
			}
		}

		public static bool IsMovementModeASID(int asid)
		{
			switch (asid)
			{
				case 50:  // HLD_HEAVY_WALK (heavy carry)
				case 920: // ZMB_CRAWL_MOVE (zombie crawl locomotion)
					return true;
				default:
					return false;
			}
		}

		public static ASIDLockType GetLockType(int asid)
		{
			if (IsExecutionASID(asid))
				return ASIDLockType.HardLock;
			if (IsSoftLockASID(asid))
				return ASIDLockType.SoftLock;
			if (IsMovementModeASID(asid))
				return ASIDLockType.MovementMode;
			return ASIDLockType.None;
		}

		/// <summary>
		/// Hazard volumes call this to decide whether to skip map damage.
		/// </summary>
		public static bool ShouldIgnoreHazardDamage(int asid)
		{
			// Executions are hazard-immune while locked.
			return IsExecutionASID(asid);
		}

		/// <summary>
		/// Movement / input code can query this to centralize jump rules.
		/// </summary>
		public static bool IsJumpDisabled(int asid)
		{
			// Heavy carry and zombie crawl both disable jumping.
			return IsMovementModeASID(asid);
		}

		/// <summary>
		/// Returns a movement speed multiplier to apply on top of base speed.
		/// </summary>
		public static float GetMoveSpeedMultiplier(int asid)
		{
			switch (asid)
			{
				case 50:  // Heavy carry
					return 0.6f;
				case 920: // Zombie crawl
					return 0.25f;
				default:
					return 1.0f;
			}
		}
	}
}
