#include "AlienBaseVolumeHubFloorGas.h"
#include "ASIDHelpers.h"
#include "GameFramework/Actor.h"
#include "UncutCharacterInterface.h"

void AAlienBaseVolumeHubFloorGas::Tick(float DeltaSeconds)
{
	Super::Tick(DeltaSeconds);

	for (AActor* Overlapping : OverlappingActors)
	{
		if (!IsValid(Overlapping))
		{
			continue;
		}

		int32 CurrentAsid = 0;
		if (Overlapping->GetClass()->ImplementsInterface(UUncutCharacterInterface::StaticClass()))
		{
			CurrentAsid = IUncutCharacterInterface::Execute_GetCurrentASID(Overlapping);
		}

		if (ShouldIgnoreHazardDamageForASID(CurrentAsid))
		{
			// Actor is in a hard-locked execution; skip hazard damage this tick.
			continue;
		}

		ApplyHazardDamage(Overlapping, DeltaSeconds);
	}
}
