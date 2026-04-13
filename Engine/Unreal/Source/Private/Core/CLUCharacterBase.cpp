#include "Core/CLUCharacterBase.h"

#include "Camera/CameraComponent.h"
#include "Components/CapsuleComponent.h"
#include "GameFramework/CharacterMovementComponent.h"
#include "GameFramework/Controller.h"
#include "GameFramework/SpringArmComponent.h"
#include "Net/UnrealNetwork.h"

ACLUCharacterBase::ACLUCharacterBase(const FObjectInitializer& ObjectInitializer)
    : Super(ObjectInitializer)
{
    bReplicates = true;
    bAlwaysRelevant = true;

    MaxHealth = 100.f;
    Health = MaxHealth;
    WalkSpeed = 600.f;
    SprintSpeed = 900.f;
    LastEmote = ECLUEmoteType::None;

    // Capsule / movement defaults similar to a standard UE template
    GetCapsuleComponent()->InitCapsuleSize(42.f, 96.0f);

    // Rotate character to direction of movement
    GetCharacterMovement()->bOrientRotationToMovement = true;
    GetCharacterMovement()->RotationRate = FRotator(0.f, 540.f, 0.f);
    GetCharacterMovement()->JumpZVelocity = 600.f;
    GetCharacterMovement()->AirControl = 0.2f;

    // We control rotation via controller yaw, camera pitch
    bUseControllerRotationPitch = false;
    bUseControllerRotationYaw = false;
    bUseControllerRotationRoll = false;

    // Camera boom
    CameraBoom = ObjectInitializer.CreateDefaultSubobject<USpringArmComponent>(this, TEXT("CameraBoom"));
    CameraBoom->SetupAttachment(RootComponent);
    CameraBoom->TargetArmLength = 350.0f;
    CameraBoom->bUsePawnControlRotation = true;

    // Follow camera
    FollowCamera = ObjectInitializer.CreateDefaultSubobject<UCameraComponent>(this, TEXT("FollowCamera"));
    FollowCamera->SetupAttachment(CameraBoom, USpringArmComponent::SocketName);
    FollowCamera->bUsePawnControlRotation = false;

    InitializeCharacterDefaults();
}

void ACLUCharacterBase::InitializeCharacterDefaults()
{
    // Ensure movement speed starts at WalkSpeed
    if (UCharacterMovementComponent* MoveComp = GetCharacterMovement())
    {
        MoveComp->MaxWalkSpeed = WalkSpeed;
    }
}

void ACLUCharacterBase::BeginPlay()
{
    Super::BeginPlay();

    // Ensure health is valid on server start
    if (HasAuthority())
    {
        Health = FMath::Clamp(Health, 0.f, MaxHealth);
    }
}

void ACLUCharacterBase::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
    Super::SetupPlayerInputComponent(PlayerInputComponent);

    check(PlayerInputComponent);

    PlayerInputComponent->BindAxis("MoveForward", this, &ACLUCharacterBase::MoveForward);
    PlayerInputComponent->BindAxis("MoveRight", this, &ACLUCharacterBase::MoveRight);
    PlayerInputComponent->BindAxis("Turn", this, &ACLUCharacterBase::Turn);
    PlayerInputComponent->BindAxis("LookUp", this, &ACLUCharacterBase::LookUp);

    PlayerInputComponent->BindAction("Jump", IE_Pressed, this, &ACLUCharacterBase::StartJump);
    PlayerInputComponent->BindAction("Jump", IE_Released, this, &ACLUCharacterBase::StopJump);
}

void ACLUCharacterBase::MoveForward(float Value)
{
    if ((Controller != nullptr) && (Value != 0.0f))
    {
        const FRotator YawRotation(0, Controller->GetControlRotation().Yaw, 0);
        const FVector Direction = FRotationMatrix(YawRotation).GetUnitAxis(EAxis::X);
        AddMovementInput(Direction, Value);
    }
}

void ACLUCharacterBase::MoveRight(float Value)
{
    if ((Controller != nullptr) && (Value != 0.0f))
    {
        const FRotator YawRotation(0, Controller->GetControlRotation().Yaw, 0);
        const FVector Direction = FRotationMatrix(YawRotation).GetUnitAxis(EAxis::Y);
        AddMovementInput(Direction, Value);
    }
}

void ACLUCharacterBase::Turn(float Value)
{
    AddControllerYawInput(Value);
}

void ACLUCharacterBase::LookUp(float Value)
{
    AddControllerPitchInput(Value);
}

void ACLUCharacterBase::StartJump()
{
    Jump();
}

void ACLUCharacterBase::StopJump()
{
    StopJumping();
}

void ACLUCharacterBase::ApplyDamage(float DamageAmount)
{
    if (!HasAuthority())
    {
        ServerApplyDamage(DamageAmount);
        return;
    }

    if (DamageAmount <= 0.f || !IsAlive())
    {
        return;
    }

    const float NewHealth = FMath::Clamp(Health - DamageAmount, 0.f, MaxHealth);
    SetHealthInternal(NewHealth);
}

void ACLUCharacterBase::Heal(float HealAmount)
{
    if (!HasAuthority())
    {
        ServerHeal(HealAmount);
        return;
    }

    if (HealAmount <= 0.f || !IsAlive())
    {
        return;
    }

    const float NewHealth = FMath::Clamp(Health + HealAmount, 0.f, MaxHealth);
    SetHealthInternal(NewHealth);
}

void ACLUCharacterBase::TriggerEmote(ECLUEmoteType EmoteType)
{
    if (!HasAuthority())
    {
        ServerTriggerEmote(EmoteType);
        return;
    }

    LastEmote = EmoteType;
    OnEmote.Broadcast(EmoteType);
}

void ACLUCharacterBase::SetHealthInternal(float NewHealth)
{
    Health = NewHealth;
    OnHealthChanged.Broadcast(Health);

    if (!IsAlive())
    {
        // Simple death behavior can be extended here (ragdoll, respawn, etc.).
        // For now, disable movement as a basic signal.
        if (UCharacterMovementComponent* MoveComp = GetCharacterMovement())
        {
            MoveComp->DisableMovement();
        }
    }
}

void ACLUCharacterBase::OnRep_Health()
{
    OnHealthChanged.Broadcast(Health);
}

void ACLUCharacterBase::OnRep_Emote()
{
    OnEmote.Broadcast(LastEmote);
}

void ACLUCharacterBase::ServerApplyDamage_Implementation(float DamageAmount)
{
    ApplyDamage(DamageAmount);
}

bool ACLUCharacterBase::ServerApplyDamage_Validate(float DamageAmount)
{
    return DamageAmount >= 0.f && DamageAmount < 10000.f;
}

void ACLUCharacterBase::ServerHeal_Implementation(float HealAmount)
{
    Heal(HealAmount);
}

bool ACLUCharacterBase::ServerHeal_Validate(float HealAmount)
{
    return HealAmount >= 0.f && HealAmount < 10000.f;
}

void ACLUCharacterBase::ServerTriggerEmote_Implementation(ECLUEmoteType EmoteType)
{
    TriggerEmote(EmoteType);
}

bool ACLUCharacterBase::ServerTriggerEmote_Validate(ECLUEmoteType EmoteType)
{
    // All enum values are acceptable for now.
    return true;
}

void ACLUCharacterBase::GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const
{
    Super::GetLifetimeReplicatedProps(OutLifetimeProps);

    DOREPLIFETIME(ACLUCharacterBase, Health);
    DOREPLIFETIME(ACLUCharacterBase, LastEmote);
}
