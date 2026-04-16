#pragma once

#include "CoreMinimal.h"

// Returns true if the given ASID represents a hard-locked execution state.
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

// Returns true if the actor should ignore environmental hazard damage
// for this frame, based on its current ASID.
FORCEINLINE bool ShouldIgnoreHazardDamageForASID(int32 Asid)
{
	// For now, we treat execution ASIDs as hazard-immune during their lock.
	return IsExecutionASID(Asid);
}
