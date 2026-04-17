#pragma once

#include "CoreMinimal.h"
#include "Components/ActorComponent.h"
#include "GameFramework/Actor.h"
#include "UncutGameTypes.generated.h"

// =============================================================================
// 26. PlayerAction Enum - All possible inputs for replication
// =============================================================================
UENUM(BlueprintType)
enum class EPlayerAction : uint8
{
	None        UMETA(DisplayName = "None"),
	Move        UMETA(DisplayName = "Move"),
	Jump        UMETA(DisplayName = "Jump"),
	Attack      UMETA(DisplayName = "Attack"),
	Use         UMETA(DisplayName = "Use"),
	Taunt       UMETA(DisplayName = "Taunt")
};

// =============================================================================
// 27. GoreType Enum - For gore chunk classification
// =============================================================================
UENUM(BlueprintType)
enum class EGoreType : uint8
{
	LimeGreen   UMETA(DisplayName = "Lime Green"),
	BloodRed    UMETA(DisplayName = "Blood Red"),
	RustBrown   UMETA(DisplayName = "Rust Brown"),
	SlateGrey   UMETA(DisplayName = "Slate Grey")
};

// =============================================================================
// 27. GoreChunk Component - Properties of detached limbs/heads
// =============================================================================
USTRUCT(BlueprintType)
struct FGoreChunkSpec
{
	GENERATED_BODY()

	// Mesh variant identifier (e.g., "Arm_L", "Head_01", "Torso_Upper")
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Gore")
	FName MeshVariant;

	// Physics impulse vector applied on detachment
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Gore")
	FVector PhysicsImpulse = FVector::ZeroVector;

	// Type of gore (determines material/color)
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Gore")
	EGoreType GoreType = EGoreType::LimeGreen;

	// Lifetime before auto-destruct
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Gore")
	float LifetimeSeconds = 10.0f;

	// Scale multiplier for this chunk
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Gore")
	float ScaleMultiplier = 1.0f;
};

UCLASS(ClassGroup = (Custom), meta = (BlueprintSpawnableComponent))
class UGoreChunkComponent : public UActorComponent
{
	GENERATED_BODY()

public:
	UGoreChunkComponent();

protected:
	virtual void BeginPlay() override;

public:
	virtual void TickComponent(float DeltaTime, ELevelTick TickType, FActorComponentTickFunction* ThisTickFunction) override;

	// Configure the gore chunk with spec
	UFUNCTION(BlueprintCallable, Category = "Gore")
	void InitializeFromSpec(const FGoreChunkSpec& Spec);

	// Get current spec
	UFUNCTION(BlueprintCallable, Category = "Gore")
	const FGoreChunkSpec& GetGoreSpec() const { return GoreSpec; }

	// Apply additional impulse
	UFUNCTION(BlueprintCallable, Category = "Gore")
	void ApplyImpulse(const FVector& Impulse);

	// Get remaining lifetime
	UFUNCTION(BlueprintCallable, Category = "Gore")
	float GetRemainingLifetime() const { return RemainingLifetime; }

private:
	UPROPERTY(EditAnywhere, Category = "Gore")
	FGoreChunkSpec GoreSpec;

	UPROPERTY(VisibleAnywhere, Category = "Gore")
	float RemainingLifetime;

	UPROPERTY(VisibleAnywhere, Category = "Gore")
	class UStaticMeshComponent* GoreMesh;

	UPROPERTY(VisibleAnywhere, Category = "Gore")
	class UProjectileMovementComponent* MovementComp;
};

// =============================================================================
// 28. GameMode Enum - Match types (ExpGain explicitly excluded per requirement)
// =============================================================================
UENUM(BlueprintType)
enum class EGameMode : uint8
{
	TotalWar    UMETA(DisplayName = "Total War"),
	Invasion    UMETA(DisplayName = "Invasion"),
	Heist       UMETA(DisplayName = "Heist"),
	Deathmatch  UMETA(DisplayName = "Deathmatch")
	// Note: ExpGain is intentionally excluded per user requirement
};

// =============================================================================
// 29. Team Enum - Faction identifiers
// =============================================================================
UENUM(BlueprintType)
enum class ETeam : uint8
{
	SHC         UMETA(DisplayName = "SHC"),
	Tediz       UMETA(DisplayName = "Tediz"),
	Alien       UMETA(DisplayName = "Alien"),
	Frenchies   UMETA(DisplayName = "Frenchies"),
	Cavemen     UMETA(DisplayName = "Cavemen"),
	Raptors     UMETA(DisplayName = "Raptors")
};

// =============================================================================
// 30. MatchState Enum - Current match progression state
// =============================================================================
UENUM(BlueprintType)
enum class EMatchState : uint8
{
	WaitingForPlayers UMETA(DisplayName = "Waiting For Players"),
	InProgress        UMETA(DisplayName = "In Progress"),
	SuddenDeath       UMETA(DisplayName = "Sudden Death"),
	PostMatch         UMETA(DisplayName = "Post Match")
};

// =============================================================================
// 30. MatchState Resource - Tracks match progression
// =============================================================================
UCLASS()
class UMatchStateResource : public UObject
{
	GENERATED_BODY()

public:
	// Current state of the match
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Match")
	EMatchState CurrentState;

	// Time elapsed in current state
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Match")
	float TimeInState;

	// Total match time elapsed
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Match")
	float TotalMatchTime;

	// Sudden death timer (only relevant during SuddenDeath state)
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Match")
	float SuddenDeathTimer;

	// Constructor
	UMatchStateResource();

	// Transition to a new state
	UFUNCTION(BlueprintCallable, Category = "Match")
	void TransitionToState(EMatchState NewState);

	// Update match state (call every tick)
	UFUNCTION(BlueprintCallable, Category = "Match")
	void UpdateState(float DeltaTime);

	// Check if match is active
	UFUNCTION(BlueprintPure, Category = "Match")
	bool IsMatchActive() const;

	// Check if in sudden death
	UFUNCTION(BlueprintPure, Category = "Match")
	bool IsSuddenDeath() const;

	// Get current state as string
	UFUNCTION(BlueprintPure, Category = "Match")
	FString GetStateAsString() const;
};
