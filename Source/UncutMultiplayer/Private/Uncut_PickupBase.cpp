#include "Uncut_PickupBase.h"

#include "Components/SphereComponent.h"
#include "Components/StaticMeshComponent.h"
#include "GameFramework/Character.h"
#include "TimerManager.h"
#include "UncutCharacterInterface.h"
#include "UncutWeaponRegistry.h"

AUncut_PickupBase::AUncut_PickupBase()
{
	PrimaryActorTick.bCanEverTick = true;

	InteractionSphere = CreateDefaultSubobject<USphereComponent>(TEXT("InteractionSphere"));
	RootComponent = InteractionSphere;
	InteractionSphere->InitSphereRadius(200.0f);
	InteractionSphere->SetCollisionEnabled(ECollisionEnabled::QueryOnly);
	InteractionSphere->SetCollisionResponseToAllChannels(ECR_Ignore);
	InteractionSphere->SetCollisionResponseToChannel(ECC_Pawn, ECR_Overlap);

	PickupMesh = CreateDefaultSubobject<UStaticMeshComponent>(TEXT("PickupMesh"));
	PickupMesh->SetupAttachment(RootComponent);

	PickupState = EUncutPickupState::Available;
}

void AUncut_PickupBase::BeginPlay()
{
	Super::BeginPlay();

	InteractionSphere->OnComponentBeginOverlap.AddDynamic(
		this, &AUncut_PickupBase::OnInteractionSphereBeginOverlap);

	InteractionSphere->SetSphereRadius(PickupSpec.InteractionRadius);
	MakeAvailable();
}

void AUncut_PickupBase::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);
}

void AUncut_PickupBase::OnInteractionSphereBeginOverlap(
	UPrimitiveComponent* OverlappedComp,
	AActor* OtherActor, UPrimitiveComponent* OtherComp,
	int32 OtherBodyIndex, bool bFromSweep, const FHitResult& SweepResult)
{
	if (PickupState != EUncut_PickupState::Available)
	{
		return;
	}

	if (!OtherActor || OtherActor == this)
	{
		return;
	}

	ACharacter* Character = Cast<ACharacter>(OtherActor);
	if (!Character)
	{
		return;
	}

	MakeTaken(OtherActor);

	ApplyPickupToCharacter(OtherActor, PickupSpec.WeaponId, PickupSpec.bIsHeavyCarry);

	if (PickupSpec.RespawnTimeSeconds > 0.0f)
	{
		StartRespawnTimer();
	}
}

void AUncut_PickupBase::MakeAvailable()
{
	PickupState = EUncut_PickupState::Available;
	SetActorHiddenInGame(false);
	SetActorEnableCollision(true);
}

void AUncut_PickupBase::MakeTaken(AActor* Picker)
{
	PickupState = EUncut_PickupState::Taken;
	SetActorHiddenInGame(true);
	SetActorEnableCollision(false);
}

void AUncut_PickupBase::StartRespawnTimer()
{
	PickupState = EUncut_PickupState::Respawning;

	if (UWorld* World = GetWorld())
	{
		World->GetTimerManager().SetTimer(
			RespawnTimerHandle,
			this,
			&AUncut_PickupBase::HandleRespawn,
			PickupSpec.RespawnTimeSeconds,
			false);
	}
}

void AUncut_PickupBase::HandleRespawn()
{
	MakeAvailable();
}

void AUncut_PickupBase::ApplyPickupToCharacter_Implementation(
	AActor* Picker, const FName& WeaponId, bool bIsHeavyCarry)
{
	if (!Picker)
	{
		return;
	}

	UUncutWeaponRegistry* Registry = UUncutWeaponRegistry::Get(this);
	if (!Registry)
	{
		return;
	}

	FUncutWeaponStats Stats;
	if (!Registry->GetWeaponStats(WeaponId, Stats))
	{
		return;
	}

	if (!Picker->GetClass()->ImplementsInterface(UUncutCharacterInterface::StaticClass()))
	{
		return;
	}

	if (IUncutCharacterInterface::Execute_GiveWeaponFromStats(Picker, Stats))
	{
		if (Stats.bIsHeavyCarry)
		{
			IUncutCharacterInterface::Execute_EnterHeavyCarry(Picker);
		}
		else
		{
			IUncutCharacterInterface::Execute_ClearHeavyCarry(Picker);
		}
	}
}
