#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "AlienBase_AirlockController.generated.h"

class AAlienBase_Volume_HubFloorGas;
class AAlienBase_Volume_SublevelAcid;

/**
 * AlienBase_AirlockController
 *
 * Map-level Airlock/Gas state machine for 04_Multiplayer_Alien_Base.
 * Controls hazard volumes and responds to trigger consoles.
 */
UCLASS()
class AAlienBase_AirlockController : public AActor
{
    GENERATED_BODY()

public:
    AAlienBase_AirlockController();

    virtual void Tick(float DeltaSeconds) override;

    UFUNCTION(BlueprintCallable, Category = "AlienBase|Airlock")
    void RequestTriggerActivation(FName TriggerId, int32 InstigatorTeam);

protected:
    virtual void BeginPlay() override;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "AlienBase|Airlock")
    AAlienBase_Volume_HubFloorGas* HubFloorGasVolume;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "AlienBase|Airlock")
    AAlienBase_Volume_SublevelAcid* SublevelAcidVolume;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "AlienBase|Airlock|Timing")
    float ArmingDurationSec;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "AlienBase|Airlock|Timing")
    float ActiveDurationSec;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "AlienBase|Airlock|Timing")
    float CooldownDurationSec;

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "AlienBase|Airlock")
    int32 EventInstigatorTeam;

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "AlienBase|Airlock")
    float TimeInState;

    UENUM(BlueprintType)
    enum class EAirlockState : uint8
    {
        Idle,
        Arming,
        Active,
        Cooldown
    };

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "AlienBase|Airlock")
    EAirlockState CurrentState;

private:
    void EnterState(EAirlockState NewState);
    void UpdateState(float DeltaSeconds);

    void OnEnteredIdle();
    void OnEnteredArming();
    void OnEnteredActive();
    void OnEnteredCooldown();

    void SetHazardVolumesActive(bool bActive);
};
