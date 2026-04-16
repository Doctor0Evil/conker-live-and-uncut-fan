// Gas Canister Logic Implementation for Fortress Total War Mode
// Implements Objective 36: Gas canister pickup, Heavy Carry penalty, and base arming

#include "GasCanister.h"
#include "Components/SphereComponent.h"
#include "Components/StaticMeshComponent.h"
#include "GameFramework/Character.h"
#include "GameFramework/CharacterMovementComponent.h"
#include "Net/UnrealNetwork.h"
#include "Kismet/GameplayStatics.h"
#include "DeliveryZone.h"
#include "GasHazardVolume.h"
#include "WarGameMode.h"

AGasCanister::AGasCanister()
{
    PrimaryActorTick.bCanEverTick = true;
    bReplicates = true;

    // Collision sphere for pickup detection
    PickupSphere = CreateDefaultSubobject<USphereComponent>(TEXT("PickupSphere"));
    PickupSphere->InitSphereRadius(100.0f);
    PickupSphere->SetCollisionEnabled(ECollisionEnabled::QueryOnly);
    PickupSphere->SetCollisionObjectType(ECC_Pawn);
    PickupSphere->SetCollisionResponseToAllChannels(ECR_Ignore);
    PickupSphere->SetCollisionResponseToChannel(ECC_Pawn, ECR_Overlap);
    RootComponent = PickupSphere;

    // Visual mesh
    MeshComponent = CreateDefaultSubobject<UStaticMeshComponent>(TEXT("MeshComponent"));
    MeshComponent->SetupAttachment(RootComponent);
    MeshComponent->SetCollisionEnabled(ECollisionEnabled::NoCollision);

    // Default configuration values (override via Lua/DataTable)
    CarrySpeedMultiplier = 0.5f;        // 50% movement speed when carrying
    RespawnTimeSeconds = 60.0f;         // 60 second respawn timer
    DeliveryBonusTickets = 50;          // 50 tickets on successful delivery
    WarningTimeSeconds = 5.0f;          // 5 second warning before gas activates
    GasDurationSeconds = 20.0f;         // 20 seconds of gas damage
    DamagePerSecond = 15.0f;            // 15 DPS (lethal in ~7 seconds)

    CurrentCarrier = nullptr;
    CurrentState = EGasCanisterState::WaitingForPickup;
    GasProgress = 0.0f;
    OriginalCarrierSpeed = 0.0f;
}

void AGasCanister::BeginPlay()
{
    Super::BeginPlay();

    // Bind overlap events for pickup detection
    PickupSphere->OnComponentBeginOverlap.AddDynamic(this, &AGasCanister::OnPickupSphereOverlap);
}

void AGasCanister::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    // State machine progression
    switch (CurrentState)
    {
        case EGasCanisterState::Arming:
            // Progress through warning phase
            GasProgress += (1.0f / WarningTimeSeconds) * DeltaTime;
            if (GasProgress >= 1.0f)
            {
                // Transition to gas active phase
                CurrentState = EGasCanisterState::GasActive;
                GasProgress = 0.0f;
                OnRep_State(); // Trigger visual/audio feedback
            }
            break;

        case EGasCanisterState::GasActive:
            // Progress through gas damage phase
            GasProgress += (1.0f / GasDurationSeconds) * DeltaTime;
            if (GasProgress >= 1.0f)
            {
                // Gas expired - start cooldown
                CurrentState = EGasCanisterState::Cooldown;
                StartRespawnTimer();
            }
            break;

        default:
            break;
    }
}

void AGasCanister::OnPickup(ACharacter* Picker)
{
    if (!Picker || CurrentCarrier != nullptr)
    {
        return;
    }

    // Server-authoritative carrier assignment
    if (HasAuthority())
    {
        Server_SetCarrier(Picker);
    }
    else
    {
        // Client requests pickup
        // (Server_SetCarrier will be called via RPC)
    }
}

void AGasCanister::OnDrop(ACharacter* DroppedBy)
{
    if (!DroppedBy || CurrentCarrier != DroppedBy)
    {
        return;
    }

    if (HasAuthority())
    {
        // Restore carrier's original movement speed
        if (OriginalCarrierSpeed > 0.0f)
        {
            DroppedBy->GetCharacterMovement()->MaxWalkSpeed = OriginalCarrierSpeed;
        }

        CurrentCarrier = nullptr;
        CurrentState = EGasCanisterState::WaitingForPickup;
        OnRep_Carrier();
        OnRep_State();
    }
}

void AGasCanister::OnArmAtDeliveryPoint(ADeliveryZone* Zone)
{
    if (!Zone || CurrentCarrier == nullptr)
    {
        return;
    }

    if (HasAuthority())
    {
        // Begin arming sequence
        CurrentState = EGasCanisterState::Arming;
        GasProgress = 0.0f;
        OnRep_State();

        // Notify game mode for ticket bonus tracking
        AWarGameMode* GameMode = Cast<AWarGameMode>(UGameplayStatics::GetGameMode(GetWorld()));
        if (GameMode)
        {
            GameMode->OnGasCanisterArmed(Zone->OwningTeam, DeliveryBonusTickets);
        }
    }
}

void AGasCanister::Server_SetCarrier_Implementation(ACharacter* NewCarrier)
{
    if (!NewCarrier)
    {
        return;
    }

    // Store original speed and apply penalty
    OriginalCarrierSpeed = NewCarrier->GetCharacterMovement()->MaxWalkSpeed;
    NewCarrier->GetCharacterMovement()->MaxWalkSpeed = OriginalCarrierSpeed * CarrySpeedMultiplier;

    CurrentCarrier = NewCarrier;
    CurrentState = EGasCanisterState::BeingCarried;
    
    // Attach canister to carrier (visual only - server doesn't need this)
    // Client-side attachment handled separately
    
    OnRep_Carrier();
    OnRep_State();
}

bool AGasCanister::Server_SetCarrier_Validate(ACharacter* NewCarrier)
{
    return NewCarrier != nullptr && CurrentCarrier == nullptr;
}

void AGasCanister::OnRep_Carrier()
{
    // Client-side: attach/detach canister mesh to carrier
    if (CurrentCarrier)
    {
        // Attach to carrier's hand/socket
        MeshComponent->AttachToComponent(
            CurrentCarrier->GetRootComponent(),
            FAttachmentTransformRules::SnapToTargetNotIncludingScale,
            TEXT("Hand_RSocket")
        );
    }
    else
    {
        // Detach and drop to ground
        MeshComponent->DetachFromComponent(FDetachmentTransformRules::KeepWorldTransform);
    }

    // Update UI indicator above carrier's head
    // (Blueprint implementation)
}

void AGasCanister::OnRep_State()
{
    // Client-side state visualization
    switch (CurrentState)
    {
        case EGasCanisterState::WaitingForPickup:
            // Neutral glow, no audio
            break;

        case EGasCanisterState::BeingCarried:
            // Pulsing icon above carrier, team-colored glow
            break;

        case EGasCanisterState::Arming:
            // Urgent red pulsing, warning siren audio
            // UI countdown overlay
            break;

        case EGasCanisterState::GasActive:
            // Green/yellow particle effects, hissing audio
            break;

        case EGasCanisterState::Cooldown:
            // Dimmed, respawning soon indicator
            break;
    }
}

void AGasCanister::TriggerGasEffect_Implementation(ADeliveryZone* TargetZone)
{
    if (!TargetZone)
    {
        return;
    }

    // Spawn gas hazard volume in target zone
    FActorSpawnParameters SpawnParams;
    SpawnParams.SpawnCollisionHandlingOverride = ESpawnActorCollisionHandlingMethod::AlwaysSpawn;

    AGasHazardVolume* GasVolume = GetWorld()->SpawnActor<AGasHazardVolume>(
        TargetZone->GasVolumeClass,
        TargetZone->GetActorLocation(),
        TargetZone->GetActorRotation(),
        SpawnParams
    );

    if (GasVolume)
    {
        GasVolume->DamagePerSecond = DamagePerSecond;
        GasVolume->DurationSeconds = GasDurationSeconds;
        GasVolume->ActivateGas();
    }
}

void AGasCanister::ApplyGasDamage_Implementation(AActor* DamagedActor)
{
    if (!DamagedActor)
    {
        return;
    }

    // Apply damage over time to actor in gas volume
    UGameplayStatics::ApplyDamage(
        DamagedActor,
        DamagePerSecond * 0.5f,  // Damage per 0.5s tick
        nullptr,
        this,
        UDamageType::StaticClass()
    );
}

void AGasCanister::StartRespawnTimer()
{
    if (!HasAuthority())
    {
        return;
    }

    GetWorldTimerManager().SetTimer(
        RespawnTimerHandle,
        this,
        &AGasCanister::RespawnCanister,
        RespawnTimeSeconds,
        false
    );
}

void AGasCanister::RespawnCanister()
{
    if (!HasAuthority())
    {
        return;
    }

    // Reset state
    CurrentState = EGasCanisterState::WaitingForPickup;
    CurrentCarrier = nullptr;
    GasProgress = 0.0f;

    // Teleport back to spawn location
    // (Spawn location defined in map grid JSON)
    SetActorLocation(FVector(1400.0f, 1000.0f, 100.0f)); // Example coords from grid

    OnRep_Carrier();
    OnRep_State();
}

void AGasCanister::GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const
{
    Super::GetLifetimeReplicatedProps(OutLifetimeProps);

    DOREPLIFETIME(AGasCanister, CurrentCarrier);
    DOREPLIFETIME(AGasCanister, CurrentState);
    DOREPLIFETIME(AGasCanister, GasProgress);
}
