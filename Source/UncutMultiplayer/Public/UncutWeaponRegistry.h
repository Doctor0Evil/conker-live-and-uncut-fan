#pragma once

#include "CoreMinimal.h"
#include "UncutWeaponRegistry.generated.h"

USTRUCT(BlueprintType)
struct FUncutWeaponStats
{
	GENERATED_BODY()

	UPROPERTY(EditAnywhere, BlueprintReadWrite)
	FName Id;

	UPROPERTY(EditAnywhere, BlueprintReadWrite)
	FName Category;

	UPROPERTY(EditAnywhere, BlueprintReadWrite)
	float Damage = 0.0f;

	UPROPERTY(EditAnywhere, BlueprintReadWrite)
	float RangeMeters = 0.0f;

	UPROPERTY(EditAnywhere, BlueprintReadWrite)
	float FireRateHz = 0.0f;

	UPROPERTY(EditAnywhere, BlueprintReadWrite)
	bool bIsHeavyCarry = false;

	UPROPERTY(EditAnywhere, BlueprintReadWrite)
	int32 ClipSize = 0;

	UPROPERTY(EditAnywhere, BlueprintReadWrite)
	int32 ReservedAmmo = 0;

	UPROPERTY(EditAnywhere, BlueprintReadWrite)
	float MovementSpeedMult = 1.0f;
};

UCLASS(BlueprintType)
class UUncutWeaponRegistry : public UObject
{
	GENERATED_BODY()

public:
	UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Weapons")
	TMap<FName, FUncutWeaponStats> WeaponTable;

	UFUNCTION(BlueprintCallable, Category = "Weapons")
	bool GetWeaponStats(FName WeaponId, FUncutWeaponStats& OutStats) const
	{
		if (const FUncutWeaponStats* Found = WeaponTable.Find(WeaponId))
		{
			OutStats = *Found;
			return true;
		}
		return false;
	}

	// Implementation lives in UncutWeaponRegistry.cpp
	static UUncutWeaponRegistry* Get(const UObject* WorldContext);
};
