#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "Uncut_PickupBase.generated.h"

UENUM(BlueprintType)
enum class EUncutPickupState : uint8
{
	Available,
	Taken,
	Respawning
};

USTRUCT(BlueprintType)
struct FUncutWeaponPickupSpec
{
	GENERATED_BODY()

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Pickup")
	FName WeaponId; // e.g. "Chainsaw", "Bazooka"

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Pickup")
	bool bIsHeavyCarry = false; // maps to ASID050 heavy carry behavior

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Pickup")
	float RespawnTimeSeconds = 0.0f; // 0 = no respawn

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Pickup")
	float InteractionRadius = 200.0f; // 200cm ≈ 2m hub interaction band
};

UCLASS(Abstract)
class AUncut_PickupBase : public AActor
{
	GENERATED_BODY()

public:
	AUncut_PickupBase();

protected:
	UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Pickup")
	FUncutWeaponPickupSpec PickupSpec;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Pickup")
	class USphereComponent* InteractionSphere;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Pickup")
	class UStaticMeshComponent* PickupMesh;

	UPROPERTY(BlueprintReadOnly, Category = "Pickup")
	EUncutPickupState PickupState;

	FTimerHandle RespawnTimerHandle;

	virtual void BeginPlay() override;

	UFUNCTION()
	void OnInteractionSphereBeginOverlap(UPrimitiveComponent* OverlappedComp,
		AActor* OtherActor, UPrimitiveComponent* OtherComp, int32 OtherBodyIndex,
		bool bFromSweep, const FHitResult& SweepResult);

	void MakeAvailable();
	void MakeTaken(AActor* Picker);
	void StartRespawnTimer();
	void HandleRespawn();

	// ASID / weapon stats hook: implemented in BP or a derived C++ class
	UFUNCTION(BlueprintNativeEvent, Category = "Pickup")
	void ApplyPickupToCharacter(AActor* Picker, const FName& WeaponId, bool bIsHeavyCarry);
	virtual void ApplyPickupToCharacter_Implementation(AActor* Picker, const FName& WeaponId, bool bIsHeavyCarry);

public:
	virtual void Tick(float DeltaTime) override;

	UFUNCTION(BlueprintCallable, Category = "Pickup")
	const FUncutWeaponPickupSpec& GetPickupSpec() const { return PickupSpec; }

	UFUNCTION(BlueprintCallable, Category = "Pickup")
	EUncutPickupState GetPickupState() const { return PickupState; }
};
