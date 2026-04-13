#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Character.h"
#include "CLUCharacterBase.generated.h"

UENUM(BlueprintType)
enum class ECLUEmoteType : uint8
{
    None        UMETA(DisplayName = "None"),
    Laugh       UMETA(DisplayName = "Laugh"),
    Taunt       UMETA(DisplayName = "Taunt"),
    Cheer       UMETA(DisplayName = "Cheer"),
    Confused    UMETA(DisplayName = "Confused"),
};

DECLARE_DYNAMIC_MULTICAST_DELEGATE_OneParam(FCLUOnHealthChangedSignature, float, NewHealth);
DECLARE_DYNAMIC_MULTICAST_DELEGATE_OneParam(FCLUOnEmoteSignature, ECLUEmoteType, EmoteType);

/**
 * CLUCharacterBase
 *
 * Base character class for Conker: Live & Uncut.
 * Provides:
 * - Standard WASD + mouse movement.
 * - Jumping.
 * - Basic third-person camera boom + follow camera.
 * - Replicated health with OnRep notifications.
 * - Simple emote triggers that can drive audio/animation in Blueprints.
 */
UCLASS()
class CONKERLIVEUNCUT_API ACLUCharacterBase : public ACharacter
{
    GENERATED_BODY()

public:
    ACLUCharacterBase(const FObjectInitializer& ObjectInitializer = FObjectInitializer::Get());

    /** Max health for this character (non-replicated, design-time constant). */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "CLU|Health")
    float MaxHealth;

    /** Current health, replicated to clients. */
    UPROPERTY(ReplicatedUsing = OnRep_Health, VisibleAnywhere, BlueprintReadOnly, Category = "CLU|Health")
    float Health;

    /** How fast the character moves when walking. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "CLU|Movement")
    float WalkSpeed;

    /** How fast the character moves when sprinting (if you add sprint logic later). */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "CLU|Movement")
    float SprintSpeed;

    /** Camera boom that positions the camera behind the character. */
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "CLU|Camera")
    class USpringArmComponent* CameraBoom;

    /** Follow camera. */
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "CLU|Camera")
    class UCameraComponent* FollowCamera;

    /** Event fired whenever Health changes on a client. */
    UPROPERTY(BlueprintAssignable, Category = "CLU|Events")
    FCLUOnHealthChangedSignature OnHealthChanged;

    /** Event fired when an emote is triggered. */
    UPROPERTY(BlueprintAssignable, Category = "CLU|Events")
    FCLUOnEmoteSignature OnEmote;

    /** Returns true if the character is alive (Health > 0). */
    UFUNCTION(BlueprintPure, Category = "CLU|Health")
    bool IsAlive() const { return Health > 0.f; }

    /** Apply damage to this character (server-authoritative). */
    UFUNCTION(BlueprintCallable, Category = "CLU|Health")
    void ApplyDamage(float DamageAmount);

    /** Heal this character (clamped to MaxHealth, server-authoritative). */
    UFUNCTION(BlueprintCallable, Category = "CLU|Health")
    void Heal(float HealAmount);

    /** Trigger an emote on this character (server will replicate to clients). */
    UFUNCTION(BlueprintCallable, Category = "CLU|Emote")
    void TriggerEmote(ECLUEmoteType EmoteType);

protected:
    virtual void BeginPlay() override;

    virtual void SetupPlayerInputComponent(class UInputComponent* PlayerInputComponent) override;

    /** Move input along the forward axis. */
    void MoveForward(float Value);

    /** Move input along the right axis. */
    void MoveRight(float Value);

    /** Look input turning yaw. */
    void Turn(float Value);

    /** Look input pitching up/down. */
    void LookUp(float Value);

    /** Jump pressed. */
    void StartJump();

    /** Jump released. */
    void StopJump();

    /** Server-side version of ApplyDamage. */
    UFUNCTION(Server, Reliable, WithValidation)
    void ServerApplyDamage(float DamageAmount);
    void ServerApplyDamage_Implementation(float DamageAmount);
    bool ServerApplyDamage_Validate(float DamageAmount);

    /** Server-side version of Heal. */
    UFUNCTION(Server, Reliable, WithValidation)
    void ServerHeal(float HealAmount);
    void ServerHeal_Implementation(float HealAmount);
    bool ServerHeal_Validate(float HealAmount);

    /** Server-side version of TriggerEmote. */
    UFUNCTION(Server, Reliable, WithValidation)
    void ServerTriggerEmote(ECLUEmoteType EmoteType);
    void ServerTriggerEmote_Implementation(ECLUEmoteType EmoteType);
    bool ServerTriggerEmote_Validate(ECLUEmoteType EmoteType);

    /** Called when Health is replicated to a client. */
    UFUNCTION()
    void OnRep_Health();

    /** Internal helper to set health on server and fire notifications. */
    void SetHealthInternal(float NewHealth);

    /** Last emote triggered, replicated so clients can react. */
    UPROPERTY(ReplicatedUsing = OnRep_Emote)
    ECLUEmoteType LastEmote;

    UFUNCTION()
    void OnRep_Emote();

    /** Configure default movement and camera settings. */
    void InitializeCharacterDefaults();

public:
    virtual void GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const override;
};
