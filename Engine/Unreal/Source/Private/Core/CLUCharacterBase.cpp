// CLUCharacterBase.cpp
// Destination: conker-live-and-uncut-fan/Engine/Unreal/Source/Private/Core/CLUCharacterBase.cpp

#include "Core/CLUCharacterBase.h"

#include "Camera/CameraComponent.h"
#include "Components/CapsuleComponent.h"
#include "Components/SkeletalMeshComponent.h"
#include "GameFramework/CharacterMovementComponent.h"
#include "GameFramework/Controller.h"
#include "GameFramework/SpringArmComponent.h"
#include "Net/UnrealNetwork.h"

// SystemNode ID: systems.ue5.conker.core.character
// Implements deterministic step functions for movement and state updates.
// State transitions (Idle->Crouch, Healthy->Damaged) follow schema-defined invariants.

ACLUCharacterBase::ACLUCharacterBase(const FObjectInitializer& ObjectInitializer)
	: Super(ObjectInitializer)
{
	PrimaryActorTick.bCanEverTick = true;

	// Networking / replication
	bReplicates = true;
	bAlwaysRelevant = true;
	NetUpdateFrequency = 60.0f; // Fixed frequency for consistent state sync

	// Health and emote state
	MaxHealth = 100.0f;
	Health = MaxHealth;
	LastEmote = ECLUEmoteType::None;

	// Movement speeds
	WalkSpeed   = 600.0f;
	SprintSpeed = 900.0f;

	// Capsule / movement defaults similar to a standard UE template
	GetCapsuleComponent()->InitCapsuleSize(42.0f, 96.0f);

	// Rotate character to direction of movement
	if (UCharacterMovementComponent* MoveComp = GetCharacterMovement())
	{
		MoveComp->bOrientRotationToMovement = true;
		MoveComp->RotationRate              = FRotator(0.0f, 540.0f, 0.0f);
		MoveComp->JumpZVelocity             = 600.0f;
		MoveComp->AirControl                = 0.2f;
	}

	// We control rotation via controller yaw, camera pitch
	bUseControllerRotationPitch = false;
	bUseControllerRotationYaw   = false;
	bUseControllerRotationRoll  = false;

	// Camera boom
	CameraBoom = ObjectInitializer.CreateDefaultSubobject<USpringArmComponent>(this, TEXT("CameraBoom"));
	CameraBoom->SetupAttachment(RootComponent);
	CameraBoom->TargetArmLength        = 350.0f;
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
		Health = FMath::Clamp(Health, 0.0f, MaxHealth);
	}
}

void ACLUCharacterBase::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
	Super::SetupPlayerInputComponent(PlayerInputComponent);

	check(PlayerInputComponent);

	PlayerInputComponent->BindAxis("MoveForward", this, &ACLUCharacterBase::MoveForward);
	PlayerInputComponent->BindAxis("MoveRight",   this, &ACLUCharacterBase::MoveRight);
	PlayerInputComponent->BindAxis("Turn",        this, &ACLUCharacterBase::Turn);
	PlayerInputComponent->BindAxis("LookUp",      this, &ACLUCharacterBase::LookUp);

	PlayerInputComponent->BindAction("Jump", IE_Pressed,  this, &ACLUCharacterBase::StartJump);
	PlayerInputComponent->BindAction("Jump", IE_Released, this, &ACLUCharacterBase::StopJump);
}

void ACLUCharacterBase::MoveForward(float Value)
{
	if (Controller && Value != 0.0f)
	{
		const FRotator YawRotation(0.0f, Controller->GetControlRotation().Yaw, 0.0f);
		const FVector  Direction   = FRotationMatrix(YawRotation).GetUnitAxis(EAxis::X);
		AddMovementInput(Direction, Value);
	}
}

void ACLUCharacterBase::MoveRight(float Value)
{
	if (Controller && Value != 0.0f)
	{
		const FRotator YawRotation(0.0f, Controller->GetControlRotation().Yaw, 0.0f);
		const FVector  Direction   = FRotationMatrix(YawRotation).GetUnitAxis(EAxis::Y);
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

bool ACLUCharacterBase::IsAlive() const
{
	return Health > 0.0f;
}

void ACLUCharacterBase::ApplyDamage(float DamageAmount)
{
	if (!HasAuthority())
	{
		ServerApplyDamage(DamageAmount);
		return;
	}

	if (DamageAmount <= 0.0f || !IsAlive())
	{
		return;
	}

	const float NewHealth = FMath::Clamp(Health - DamageAmount, 0.0f, MaxHealth);
	SetHealthInternal(NewHealth);
}

void ACLUCharacterBase::Heal(float HealAmount)
{
	if (!HasAuthority())
	{
		ServerHeal(HealAmount);
		return;
	}

	if (HealAmount <= 0.0f || !IsAlive())
	{
		return;
	}

	const float NewHealth = FMath::Clamp(Health + HealAmount, 0.0f, MaxHealth);
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
	return DamageAmount >= 0.0f && DamageAmount < 10000.0f;
}

void ACLUCharacterBase::ServerHeal_Implementation(float HealAmount)
{
	Heal(HealAmount);
}

bool ACLUCharacterBase::ServerHeal_Validate(float HealAmount)
{
	return HealAmount >= 0.0f && HealAmount < 10000.0f;
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
