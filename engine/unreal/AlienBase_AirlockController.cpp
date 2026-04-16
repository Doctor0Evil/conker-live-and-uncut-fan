#include "AlienBase_AirlockController.h"
#include "Engine/World.h"
#include "GameFramework/Actor.h"
#include "Kismet/GameplayStatics.h"

#include "AlienBase_Volume_HubFloorGas.h"
#include "AlienBase_Volume_SublevelAcid.h"

AAlienBase_AirlockController::AAlienBase_AirlockController()
{
    PrimaryActorTick.bCanEverTick = true;

    ArmingDurationSec = 5.0f;
    ActiveDurationSec = 12.0f;
    CooldownDurationSec = 30.0f;

    EventInstigatorTeam = INDEX_NONE;
    TimeInState = 0.0f;
    CurrentState = EAirlockState::Idle;
}

void AAlienBase_AirlockController::BeginPlay()
{
    Super::BeginPlay();
    EnterState(EAirlockState::Idle);
}

void AAlienBase_AirlockController::Tick(float DeltaSeconds)
{
    Super::Tick(DeltaSeconds);
    UpdateState(DeltaSeconds);
}

void AAlienBase_AirlockController::RequestTriggerActivation(FName TriggerId, int32 InstigatorTeam)
{
    if (CurrentState != EAirlockState::Idle)
    {
        return;
    }

    EventInstigatorTeam = InstigatorTeam;
    EnterState(EAirlockState::Arming);

    // TODO: Play global arming VO/sfx and start warning lights.
}

void AAlienBase_AirlockController::EnterState(EAirlockState NewState)
{
    CurrentState = NewState;
    TimeInState = 0.0f;

    switch (CurrentState)
    {
    case EAirlockState::Idle:
        OnEnteredIdle();
        break;
    case EAirlockState::Arming:
        OnEnteredArming();
        break;
    case EAirlockState::Active:
        OnEnteredActive();
        break;
    case EAirlockState::Cooldown:
        OnEnteredCooldown();
        break;
    default:
        break;
    }
}

void AAlienBase_AirlockController::UpdateState(float DeltaSeconds)
{
    TimeInState += DeltaSeconds;

    switch (CurrentState)
    {
    case EAirlockState::Idle:
        break;

    case EAirlockState::Arming:
        if (TimeInState >= ArmingDurationSec)
        {
            EnterState(EAirlockState::Active);
        }
        break;

    case EAirlockState::Active:
        if (TimeInState >= ActiveDurationSec)
        {
            EnterState(EAirlockState::Cooldown);
        }
        break;

    case EAirlockState::Cooldown:
        if (TimeInState >= CooldownDurationSec)
        {
            EnterState(EAirlockState::Idle);
        }
        break;

    default:
        break;
    }
}

void AAlienBase_AirlockController::OnEnteredIdle()
{
    SetHazardVolumesActive(false);
    EventInstigatorTeam = INDEX_NONE;
}

void AAlienBase_AirlockController::OnEnteredArming()
{
    SetHazardVolumesActive(false);
    // TODO: Start sirens, flicker lights, pre-gas vent FX.
}

void AAlienBase_AirlockController::OnEnteredActive()
{
    SetHazardVolumesActive(true);
    // TODO: Update VO/UI to "Airlock sealed / gas released".
}

void AAlienBase_AirlockController::OnEnteredCooldown()
{
    SetHazardVolumesActive(false);
    // TODO: Venting VO/sfx, restore normal ambience.
}

void AAlienBase_AirlockController::SetHazardVolumesActive(bool bActive)
{
    if (HubFloorGasVolume)
    {
        HubFloorGasVolume->SetVolumeActive(bActive);
    }

    if (SublevelAcidVolume)
    {
        SublevelAcidVolume->SetVolumeActive(bActive);
    }
}
