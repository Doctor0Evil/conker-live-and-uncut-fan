#include "UncutWeaponRegistry.h"
#include "Engine/World.h"
#include "Engine/GameInstance.h"

UUncutWeaponRegistry* UUncutWeaponRegistry::Get(const UObject* WorldContext)
{
	if (!WorldContext)
	{
		return nullptr;
	}

	if (const UWorld* World = WorldContext->GetWorld())
	{
		if (UGameInstance* GI = World->GetGameInstance())
		{
			// Simplest pattern: registry is a UObject created and stored on the GameInstance
			return GI->GetSubsystem<UUncutWeaponRegistry>();
		}
	}

	return nullptr;
}
