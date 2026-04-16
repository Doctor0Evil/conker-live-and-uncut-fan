#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "BeachFenceObjective.generated.h"

DECLARE_DYNAMIC_MULTICAST_DELEGATE_TwoParams(
	FOnFenceStateChanged,
	int32, FenceIndex,
	FName, NewState);

UENUM(BlueprintType)
enum class EFenceState : uint8
{
	Intact,
	Damaged,
	Destroyed
};

UCLASS()
class ABeachFenceObjective : public AActor
{
	GENERATED_BODY()

public:
	ABeachFenceObjective();

	virtual void Tick(float DeltaSeconds) override;

	UFUNCTION(BlueprintCallable, Category="Fence")
	void ApplyDamage(float Amount);

	UFUNCTION(BlueprintCallable, Category="Fence")
	void SetFenceIndex(int32 InIndex);

	UFUNCTION(BlueprintCallable, Category="Fence")
	EFenceState GetState() const { return CurrentState; }

	// Broadcast when state changes (used by spawn manager / audio / FX).
	UPROPERTY(BlueprintAssignable, Category="Fence")
	FOnFenceStateChanged OnFenceStateChanged;

protected:
	virtual void BeginPlay() override;

	UPROPERTY(EditAnywhere, BlueprintReadOnly, Category="Fence")
	int32 FenceIndex;

	UPROPERTY(EditAnywhere, BlueprintReadOnly, Category="Fence")
	float MaxHealth;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category="Fence")
	float CurrentHealth;

	UPROPERTY(EditAnywhere, BlueprintReadOnly, Category="Fence")
	float DamagedThreshold;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category="Fence")
	EFenceState CurrentState;

	// Optional static mesh components for different states.
	UPROPERTY(VisibleAnywhere, Category="Fence")
	UStaticMeshComponent* IntactMesh;

	UPROPERTY(VisibleAnywhere, Category="Fence")
	UStaticMeshComponent* DamagedMesh;

	UPROPERTY(VisibleAnywhere, Category="Fence")
	UStaticMeshComponent* DestroyedMesh;

private:
	void EvaluateState();
	void UpdateVisuals();
	void BroadcastStateChange(EFenceState NewState);
};
