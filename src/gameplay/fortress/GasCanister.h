// Gas Canister Logic for Fortress Total War Mode
// Implements Objective 36: Gas canister pickup, Heavy Carry penalty, and base arming

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "GasCanister.generated.h"

UCLASS()
class CONKERLIVEUNCUT_API AGasCanister : public AActor
{
    GENERATED_BODY()

public:
    AGasCanister();

protected:
    virtual void BeginPlay() override;
    virtual void Tick(float DeltaTime) override;

public:
    // Pickup interaction
    UFUNCTION(BlueprintCallable, Category = "Interaction")
    void OnPickup(ACharacter* Picker);

    UFUNCTION(BlueprintCallable, Category = "Interaction")
    void OnDrop(ACharacter* DroppedBy);

    UFUNCTION(BlueprintCallable, Category = "Interaction")
    void OnArmAtDeliveryPoint(ADeliveryZone* Zone);

    // Carrier management
    UFUNCTION(Server, Reliable, WithValidation)
    void Server_SetCarrier(ACharacter* NewCarrier);
    void Server_SetCarrier_Implementation(ACharacter* NewCarrier);
    bool Server_SetCarrier_Validate(ACharacter* NewCarrier);

    // Properties - Configurable via Lua/DataTable
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GasCanister|Config")
    float CarrySpeedMultiplier;  // Default: 0.5 (50% movement speed)

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GasCanister|Config")
    float RespawnTimeSeconds;  // Default: 60.0

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GasCanister|Config")
    int32 DeliveryBonusTickets;  // Default: 50

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GasCanister|Config")
    float WarningTimeSeconds;  // Default: 5.0

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GasCanister|Config")
    float GasDurationSeconds;  // Default: 20.0

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GasCanister|Config")
    float DamagePerSecond;  // Default: 15.0

    // State replication
    UPROPERTY(ReplicatedUsing = OnRep_Carrier)
    ACharacter* CurrentCarrier;

    UPROPERTY(ReplicatedUsing = OnRep_State)
    EGasCanisterState CurrentState;

    UPROPERTY(Replicated)
    float GasProgress;  // 0.0 to 1.0 during warning/gas phases

    UFUNCTION()
    void OnRep_Carrier();

    UFUNCTION()
    void OnRep_State();

    // Delivery zone references
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "GasCanister")
    class USphereComponent* PickupSphere;

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "GasCanister")
    class UStaticMeshComponent* MeshComponent;

    // Gas effect system
    UFUNCTION(BlueprintNativeEvent, Category = "GasCanister|Effects")
    void TriggerGasEffect(ADeliveryZone* TargetZone);
    void TriggerGasEffect_Implementation(ADeliveryZone* TargetZone);

    UFUNCTION(BlueprintNativeEvent, Category = "GasCanister|Effects")
    void ApplyGasDamage(AActor* DamagedActor);
    void ApplyGasDamage_Implementation(AActor* DamagedActor);

    // Respawn logic
    UFUNCTION()
    void StartRespawnTimer();

    UFUNCTION()
    void RespawnCanister();

private:
    // Internal state tracking
    FTimerHandle RespawnTimerHandle;
    FTimerHandle GasDamageTimerHandle;
    
    // Original carrier speed (to restore on drop)
    float OriginalCarrierSpeed;
};

// Enum for canister state machine
UENUM(BlueprintType)
enum class EGasCanisterState : uint8
{
    WaitingForPickup    UMETA(DisplayName = "Waiting For Pickup"),
    BeingCarried        UMETA(DisplayName = "Being Carried"),
    Arming              UMETA(DisplayName = "Arming (Warning Phase)"),
    GasActive           UMETA(DisplayName = "Gas Active (Damage Phase)"),
    Cooldown            UMETA(DisplayName = "Cooldown (Respawn Wait)")
};

// Delivery zone actor definition
UCLASS()
class CONKERLIVEUNCUT_API ADeliveryZone : public AActor
{
    GENERATED_BODY()

public:
    ADeliveryZone();

protected:
    virtual void BeginPlay() override;
    virtual void Tick(float DeltaTime) override;

public:
    // Zone configuration
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "DeliveryZone|Config")
    FString InteractionPrompt;  // "Arm Gas Canister at SHC Base?"

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "DeliveryZone|Config")
    TEnumAsByte<ETeamAffiliation> OwningTeam;  // SHC or Tediz

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "DeliveryZone")
    class USphereComponent* TriggerSphere;

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "DeliveryZone")
    class UWidgetComponent* InteractionWidget;

    // Gas hazard volume (spawns when canister armed)
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "DeliveryZone")
    class AGasHazardVolume* GasVolume;

    // Interaction handling
    UFUNCTION(BlueprintCallable, Category = "Interaction")
    void OnCarrierEntered(AGasCanister* Canister, ACharacter* Carrier);

    UFUNCTION(BlueprintCallable, Category = "Interaction")
    void ShowInteractionPrompt(ACharacter* Player);

    UFUNCTION(BlueprintCallable, Category = "Interaction")
    void HideInteractionPrompt(ACharacter* Player);

    // State replication
    UPROPERTY(ReplicatedUsing = OnRep_IsArmed)
    bool bIsArmed;

    UPROPERTY(Replicated)
    float WarningProgress;  // 0.0 to 1.0 during warning phase

    UFUNCTION()
    void OnRep_IsArmed();

private:
    AGasCanister* ArmedCanister;
};

// Gas hazard volume for area damage
UCLASS()
class CONKERLIVEUNCUT_API AGasHazardVolume : public AVolume
{
    GENERATED_BODY()

public:
    AGasHazardVolume();

protected:
    virtual void BeginPlay() override;
    virtual void Tick(float DeltaTime) override;

public:
    // Damage configuration
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GasHazard|Config")
    float DamagePerSecond;  // Default: 15.0

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GasHazard|Config")
    float DurationSeconds;  // Default: 20.0

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "GasHazard|Config")
    float DamageInterval;  // Default: 0.5 (apply damage twice per second)

    // Visual effects
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "GasHazard|VFX")
    class UNiagaraComponent* GasParticleEffect;

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "GasHazard|VFX")
    class UBoxComponent* CollisionBox;

    // Lifecycle management
    UFUNCTION(BlueprintCallable, Category = "GasHazard")
    void ActivateGas();

    UFUNCTION(BlueprintCallable, Category = "GasHazard")
    void DeactivateGas();

    // Damage application
    UFUNCTION()
    void OnOverlapBegin(AActor* OverlappedActor, AActor* OtherActor);

    UFUNCTION()
    void OnOverlapEnd(AActor* OverlappedActor, AActor* OtherActor);

    UFUNCTION()
    void ApplyDamageToOverlappingActors();

private:
    FTimerHandle DamageTimerHandle;
    FTimerHandle DeactivationTimerHandle;
    TArray<AActor*> OverlappingActors;
};
