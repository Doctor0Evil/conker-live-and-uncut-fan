#pragma once

#include "CoreMinimal.h"

enum class EASIDLockType : uint8
{
	None,
	SoftLock,      // e.g., stun; damped movement, input limited.
	MovementMode,  // e.g., heavy carry; locomotion override.
	HardLock       // e.g., executions; full lock while active.
};

FORCEINLINE bool IsExecutionASID(int32 Asid)
{
	switch (Asid)
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

FORCEINLINE bool IsSoftLockASID(int32 Asid)
{
	switch (Asid)
	{
	case 12:  // HIT_STUN_DAZE
		return true;
	default:
		return false;
	}
}

FORCEINLINE bool IsMovementModeASID(int32 Asid)
{
	switch (Asid)
	{
	case 50:  // HLD_HEAVY_WALK
	case 920: // ZMB_CRAWL_MOVE
		return true;
	default:
		return false;
	}
}

FORCEINLINE EASIDLockType GetASIDLockType(int32 Asid)
{
	if (IsExecutionASID(Asid))
	{
		return EASIDLockType::HardLock;
	}
	if (IsSoftLockASID(Asid))
	{
		return EASIDLockType::SoftLock;
	}
	if (IsMovementModeASID(Asid))
	{
		return EASIDLockType::MovementMode;
	}
	return EASIDLockType::None;
}

/// Hazard volumes call this to decide whether to skip map damage.
FORCEINLINE bool ShouldIgnoreHazardDamageForASID(int32 Asid)
{
	// Executions ignore hazard tick damage while locked.
	return IsExecutionASID(Asid);
}

/// Movement/input systems can query these to centralize behavior.
FORCEINLINE bool IsJumpDisabledForASID(int32 Asid)
{
	// Heavy carry and zombie crawl both disable jumping.
	return IsMovementModeASID(Asid);
}

FORCEINLINE float GetMoveSpeedMultiplierForASID(int32 Asid)
{
	switch (Asid)
	{
	case 50:  // Heavy carry
		return 0.6f;
	case 920: // Zombie crawl
		return 0.25f;
	default:
		return 1.0f;
	}
}
