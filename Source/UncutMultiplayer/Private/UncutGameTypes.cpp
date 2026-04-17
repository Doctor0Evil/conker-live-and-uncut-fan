#include "UncutGameTypes.h"
#include "Components/StaticMeshComponent.h"
#include "GameFramework/ProjectileMovementComponent.h"

// =============================================================================
// UGoreChunkComponent Implementation
// =============================================================================

UGoreChunkComponent::UGoreChunkComponent()
{
	PrimaryComponentTick.bCanEverTick = true;
	
	// Create mesh component
	GoreMesh = CreateDefaultSubobject<UStaticMeshComponent>(TEXT("GoreMesh"));
	GoreMesh->SetupAttachment(this);
	GoreMesh->SetCollisionEnabled(ECollisionEnabled::PhysicsOnly);
	GoreMesh->SetSimulatePhysics(true);
	
	// Create movement component for physics impulse
	MovementComp = CreateDefaultSubobject<UProjectileMovementComponent>(TEXT("GoreMovement"));
	MovementComp->InitialSpeed = 0.0f;
	MovementComp->MaxSpeed = 5000.0f;
	MovementComp->bRotationFollowsVelocity = true;
	MovementComp->bShouldBounce = true;
	MovementComp->Bounciness = 0.3f;
	MovementComp->ProjectileGravityScale = 1.0f;
	
	RemainingLifetime = 0.0f;
}

void UGoreChunkComponent::BeginPlay()
{
	Super::BeginPlay();
	
	RemainingLifetime = GoreSpec.LifetimeSeconds;
}

void UGoreChunkComponent::TickComponent(float DeltaTime, ELevelTick TickType, FActorComponentTickFunction* ThisTickFunction)
{
	Super::TickComponent(DeltaTime, TickType, ThisTickFunction);
	
	// Update lifetime
	if (RemainingLifetime > 0.0f)
	{
		RemainingLifetime -= DeltaTime;
		
		// Auto-destruct when lifetime expires
		if (RemainingLifetime <= 0.0f && GetOwner())
		{
			GetOwner()->Destroy();
		}
	}
}

void UGoreChunkComponent::InitializeFromSpec(const FGoreChunkSpec& Spec)
{
	GoreSpec = Spec;
	RemainingLifetime = Spec.LifetimeSeconds;
	
	// Apply initial impulse if specified
	if (!Spec.PhysicsImpulse.IsZero() && MovementComp)
	{
		MovementComp->Velocity = Spec.PhysicsImpulse;
	}
}

void UGoreChunkComponent::ApplyImpulse(const FVector& Impulse)
{
	if (MovementComp && GoreMesh)
	{
		// Add impulse in world space
		MovementComp->Velocity += Impulse;
		
		// Also apply direct physics impulse to mesh
		if (GoreMesh->IsSimulatingPhysics())
		{
			GoreMesh->AddImpulse(Impulse, NAME_None, true);
		}
	}
}

// =============================================================================
// UMatchStateResource Implementation
// =============================================================================

UMatchStateResource::UMatchStateResource()
{
	CurrentState = EMatchState::WaitingForPlayers;
	TimeInState = 0.0f;
	TotalMatchTime = 0.0f;
	SuddenDeathTimer = 0.0f;
}

void UMatchStateResource::TransitionToState(EMatchState NewState)
{
	CurrentState = NewState;
	TimeInState = 0.0f;
	
	// Initialize sudden death timer when entering that state
	if (NewState == EMatchState::SuddenDeath)
	{
		SuddenDeathTimer = 60.0f; // Default 60 seconds, can be configured
	}
}

void UMatchStateResource::UpdateState(float DeltaTime)
{
	TotalMatchTime += DeltaTime;
	TimeInState += DeltaTime;
	
	// Update sudden death timer if active
	if (CurrentState == EMatchState::SuddenDeath && SuddenDeathTimer > 0.0f)
	{
		SuddenDeathTimer -= DeltaTime;
		
		// End match if sudden death timer expires
		if (SuddenDeathTimer <= 0.0f)
		{
			TransitionToState(EMatchState::PostMatch);
		}
	}
}

bool UMatchStateResource::IsMatchActive() const
{
	return CurrentState == EMatchState::InProgress || 
		   CurrentState == EMatchState::SuddenDeath;
}

bool UMatchStateResource::IsSuddenDeath() const
{
	return CurrentState == EMatchState::SuddenDeath;
}

FString UMatchStateResource::GetStateAsString() const
{
	switch (CurrentState)
	{
		case EMatchState::WaitingForPlayers:
			return TEXT("Waiting For Players");
		case EMatchState::InProgress:
			return TEXT("In Progress");
		case EMatchState::SuddenDeath:
			return TEXT("Sudden Death");
		case EMatchState::PostMatch:
			return TEXT("Post Match");
		default:
			return TEXT("Unknown");
	}
}
