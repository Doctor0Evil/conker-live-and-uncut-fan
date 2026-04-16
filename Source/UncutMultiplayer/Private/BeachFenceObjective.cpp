#include "BeachFenceObjective.h"
#include "Net/UnrealNetwork.h"

ABeachFenceObjective::ABeachFenceObjective()
{
	PrimaryActorTick.bCanEverTick = false;

	FenceIndex = 1;
	MaxHealth = 1000.f;
	CurrentHealth = MaxHealth;
	DamagedThreshold = 0.5f; // 50%

	CurrentState = EFenceState::Intact;

	IntactMesh = CreateDefaultSubobject<UStaticMeshComponent>(TEXT("IntactMesh"));
	RootComponent = IntactMesh;

	DamagedMesh = CreateDefaultSubobject<UStaticMeshComponent>(TEXT("DamagedMesh"));
	DamagedMesh->SetupAttachment(RootComponent);

	DestroyedMesh = CreateDefaultSubobject<UStaticMeshComponent>(TEXT("DestroyedMesh"));
	DestroyedMesh->SetupAttachment(RootComponent);
}

void ABeachFenceObjective::BeginPlay()
{
	Super::BeginPlay();
	UpdateVisuals();
}

void ABeachFenceObjective::Tick(float DeltaSeconds)
{
	Super::Tick(DeltaSeconds);
}

void ABeachFenceObjective::ApplyDamage(float Amount)
{
	if (CurrentState == EFenceState::Destroyed || Amount <= 0.f)
	{
		return;
	}

	CurrentHealth = FMath::Clamp(CurrentHealth - Amount, 0.f, MaxHealth);
	EvaluateState();
}

void ABeachFenceObjective::SetFenceIndex(int32 InIndex)
{
	FenceIndex = InIndex;
}

void ABeachFenceObjective::EvaluateState()
{
	float HealthRatio = (MaxHealth > 0.f) ? (CurrentHealth / MaxHealth) : 0.f;
	EFenceState NewState = CurrentState;

	if (CurrentHealth <= 0.f)
	{
		NewState = EFenceState::Destroyed;
	}
	else if (HealthRatio <= DamagedThreshold)
	{
		NewState = EFenceState::Damaged;
	}
	else
	{
		NewState = EFenceState::Intact;
	}

	if (NewState != CurrentState)
	{
		CurrentState = NewState;
		UpdateVisuals();
		BroadcastStateChange(NewState);
	}
}

void ABeachFenceObjective::UpdateVisuals()
{
	if (IntactMesh)
	{
		IntactMesh->SetVisibility(CurrentState == EFenceState::Intact);
	}
	if (DamagedMesh)
	{
		DamagedMesh->SetVisibility(CurrentState == EFenceState::Damaged);
	}
	if (DestroyedMesh)
	{
		DestroyedMesh->SetVisibility(CurrentState == EFenceState::Destroyed);
	}
}

void ABeachFenceObjective::BroadcastStateChange(EFenceState NewState)
{
	FName StateName = NAME_None;
	switch (NewState)
	{
	case EFenceState::Intact:    StateName = FName(TEXT("Intact"));    break;
	case EFenceState::Damaged:   StateName = FName(TEXT("Damaged"));   break;
	case EFenceState::Destroyed: StateName = FName(TEXT("Destroyed")); break;
	default: break;
	}

	OnFenceStateChanged.Broadcast(FenceIndex, StateName);
}
