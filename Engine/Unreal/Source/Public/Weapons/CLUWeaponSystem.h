// CLUWeaponSystem.h
// Conker: Live & Uncut — Weapon System Header
// Implements the weapon types and behaviors described in E3 2003 previews.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "CLUWeaponSystem.generated.h"

UENUM(BlueprintType)
enum class ECLUWeaponType : uint8
{
    Melee_Sword        UMETA(DisplayName = "Sword"),
    Melee_DualSwords   UMETA(DisplayName = "Dual Swords"),
    Melee_Knife        UMETA(DisplayName = "Knife"),
    Pistol             UMETA(DisplayName = "Pistol"),
    SMG_Uzi            UMETA(DisplayName = "Dual Uzis"),
    AssaultRifle       UMETA(DisplayName = "Assault Rifle"),
    Shotgun            UMETA(DisplayName = "Shotgun"),
    SniperRifle        UMETA(DisplayName = "Sniper Rifle"),
    Heavy_RocketLauncher UMETA(DisplayName = "Rocket Launcher"),
    Heavy_GrenadeLauncher UMETA(DisplayName = "Grenade Launcher"),
    Explosive_Grenade  UMETA(DisplayName = "Grenade"),
    Explosive_C4       UMETA(DisplayName = "C4 Explosive"),
    Special_Sentry     UMETA(DisplayName = "Deployable Sentry"),
    Special_Flamethrower UMETA(DisplayName = "Flamethrower")
};

UENUM(BlueprintType)
enum class ECLUWeaponSlot : uint8
{
    Primary   UMETA(DisplayName = "Primary"),
    Secondary UMETA(DisplayName = "Secondary"),
    Melee     UMETA(DisplayName = "Melee"),
    Explosive UMETA(DisplayName = "Explosive"),
    Special   UMETA(DisplayName = "Special")
};

USTRUCT(BlueprintType)
struct FCLUWeaponStats
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Stats")
    float Damage = 25.0f;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Stats")
    float FireRate = 0.1f;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Stats")
    float ReloadTime = 1.5f;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Stats")
    int32 MagazineSize = 30;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Stats")
    int32 MaxAmmo = 180;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Stats")
    float Range = 5000.0f;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Stats")
    float Spread = 0.0f;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Stats")
    bool bIsAutomatic = true;
};

UCLASS(Blueprintable, BlueprintType)
class CONKERLIVEUNCUT_API ACLUWeaponBase : public AActor
{
    GENERATED_BODY()

public:
    ACLUWeaponBase();

protected:
    virtual void BeginPlay() override;

public:
    virtual void Tick(float DeltaTime) override;

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    virtual void Fire();

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    virtual void Reload();

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    virtual void AltFire();

    UFUNCTION(BlueprintPure, Category = "Weapon")
    bool CanFire() const;

    UFUNCTION(BlueprintPure, Category = "Weapon")
    bool NeedsReload() const;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Weapon")
    ECLUWeaponType WeaponType;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Weapon")
    ECLUWeaponSlot WeaponSlot;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Weapon")
    FCLUWeaponStats WeaponStats;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Weapon")
    UAnimMontage* FireAnimation;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Weapon")
    UAnimMontage* ReloadAnimation;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Weapon")
    USoundBase* FireSound;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Weapon")
    USoundBase* ReloadSound;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Weapon")
    UParticleSystem* MuzzleFlash;

    UPROPERTY(Replicated)
    int32 CurrentAmmo;

    UPROPERTY(Replicated)
    int32 ReserveAmmo;

    UPROPERTY(Replicated)
    bool bIsReloading;

protected:
    FTimerHandle ReloadTimerHandle;
    FTimerHandle FireTimerHandle;

    virtual void GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const override;
    virtual void FinishReload();
    virtual void ResetFireCooldown();
};

UCLASS(Blueprintable, BlueprintType)
class CONKERLIVEUNCUT_API ACLUMeleeWeapon : public ACLUWeaponBase
{
    GENERATED_BODY()

public:
    virtual void Fire() override;

    UFUNCTION(BlueprintCallable, Category = "Melee")
    void PerformTrace();

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Melee")
    float TraceLength = 200.0f;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Melee")
    float TraceRadius = 50.0f;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Melee")
    bool bIsDualWield = false;
};

UCLASS(Blueprintable, BlueprintType)
class CONKERLIVEUNCUT_API ACLUProjectileWeapon : public ACLUWeaponBase
{
    GENERATED_BODY()

public:
    virtual void Fire() override;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Projectile")
    TSubclassOf<class ACLUProjectileBase> ProjectileClass;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Projectile")
    float ProjectileSpeed = 10000.0f;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Projectile")
    int32 ProjectilesPerShot = 1;
};

UCLASS(Blueprintable, BlueprintType)
class CONKERLIVEUNCUT_API ACLUInventoryComponent : public UActorComponent
{
    GENERATED_BODY()

public:
    ACLUInventoryComponent();

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    bool AddWeapon(TSubclassOf<ACLUWeaponBase> WeaponClass, ECLUWeaponSlot Slot);

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    void SwitchToSlot(ECLUWeaponSlot Slot);

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    ACLUWeaponBase* GetCurrentWeapon() const;

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    void AddAmmo(ECLUWeaponType WeaponType, int32 Amount);

    UPROPERTY(Replicated)
    TMap<ECLUWeaponSlot, ACLUWeaponBase*> EquippedWeapons;

    UPROPERTY(Replicated)
    ECLUWeaponSlot CurrentSlot;

    UPROPERTY(Replicated)
    TMap<ECLUWeaponType, int32> AmmoReserves;

protected:
    virtual void GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const override;
};
