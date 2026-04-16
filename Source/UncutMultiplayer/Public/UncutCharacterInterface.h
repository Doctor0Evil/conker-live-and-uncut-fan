#pragma once

#include "UObject/Interface.h"
#include "UncutWeaponRegistry.h"
#include "UncutCharacterInterface.generated.h"

UINTERFACE(Blueprintable)
class UUncutCharacterInterface : public UInterface
{
	GENERATED_BODY()
};

class IUncutCharacterInterface
{
	GENERATED_BODY()

public:
	// Grants weapon & ammo using stats from registry
	UFUNCTION(BlueprintNativeEvent, BlueprintCallable, Category = "Uncut")
	bool GiveWeaponFromStats(const FUncutWeaponStats& Stats);

	// Heavy carry ASID050
	UFUNCTION(BlueprintNativeEvent, BlueprintCallable, Category = "Uncut")
	void EnterHeavyCarry();   // sets ASID050, no jump, slower move

	UFUNCTION(BlueprintNativeEvent, BlueprintCallable, Category = "Uncut")
	void ClearHeavyCarry();   // leaves ASID050 back to normal locomotion
};
