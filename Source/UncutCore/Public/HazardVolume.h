#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "HazardVolume.generated.h"

class UBoxComponent;
class UNiagaraSystem;
class USoundBase;

USTRUCT()
struct FHazardProfile
{
    GENERATED_BODY()

    UPROPERTY()
    FString Id;

    UPROPERTY()
    FString HazardType;

    UPROPERTY()
    float DamagePerSecond = 0.0f;

    UPROPERTY()
    float TickInterval = 0.1f;

    UPROPERTY()
    TArray<FString> ImmunityAsids;

    UPROPERTY()
    FName VisualEffectId;

    UPROPERTY()
    FName AudioCueId;

    UPROPERTY()
    FName ActivationController;
};

UCLASS()
class AHazardVolume : public AActor
{
    GENERATED_BODY()

public:
    AHazardVolume();

    UPROPERTY(EditAnywhere, Category="Hazard")
    FString HazardProfileId;

    UPROPERTY(EditAnywhere, Category="Hazard")
    bool bStartsEnabled = false;

    UPROPERTY(VisibleAnywhere, Category="Hazard")
    UBoxComponent* Volume;

    UFUNCTION(BlueprintCallable, Category="Hazard")
    void SetHazardEnabled(bool bEnabled);

    UFUNCTION(BlueprintCallable, Category="Hazard")
    bool IsHazardEnabled() const { return bIsEnabled; }

protected:
    virtual void BeginPlay() override;
    virtual void Tick(float DeltaSeconds) override;

private:
    FHazardProfile ActiveProfile;

    bool bIsEnabled = false;
    float TickAccumulator = 0.0f;

    void LoadProfile();
    void ApplyDamageTick();
    bool IsPawnImmune(class AActor* Pawn) const;
    void UpdateEffects(bool bEnable);
};
