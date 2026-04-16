#pragma once

#include "CoreMinimal.h"
#include "Components/ActorComponent.h"
#include "WeaponInventoryComponent.generated.h"

USTRUCT(BlueprintType)
struct FWeaponStats
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString Id;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString DisplayName;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString WeaponType;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString Category;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    float DamagePerHit = 0.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    float DamagePerPellet = 0.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    int32 PelletCount = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    float HeadshotMultiplier = 1.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString FireMode;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    float RateOfFire = 1.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    int32 ClipSize = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    int32 MaxReserveAmmo = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    float ReloadTime = 0.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    float ProjectileSpeed = 0.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    float SpreadDegrees = 0.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    float MoveSpeedMultiplier = 1.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    bool bNoJump = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    bool bIsHeavy = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    bool bHasFriendlyFireAoe = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString ZombieDamageProfileId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FName SfxFire;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FName SfxHit;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FName VfxMuzzle;
};

USTRUCT(BlueprintType)
struct FWeaponRuntimeState
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FWeaponStats Stats;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    int32 ClipAmmo = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    int32 ReserveAmmo = 0;
};

DECLARE_DYNAMIC_MULTICAST_DELEGATE_OneParam(FOnWeaponChanged, const FWeaponRuntimeState&, NewWeapon);

UCLASS(ClassGroup=(Custom), meta=(BlueprintSpawnableComponent))
class UNCUTCORE_API UWeaponInventoryComponent : public UActorComponent
{
    GENERATED_BODY()

public:
    UWeaponInventoryComponent();

    UPROPERTY(BlueprintAssignable, Category="Weapon")
    FOnWeaponChanged OnWeaponChanged;

    UFUNCTION(BlueprintCallable, Category="Weapon")
    void InitializeFromStats(const TArray<FWeaponStats>& AllStats);

    UFUNCTION(BlueprintCallable, Category="Weapon")
    bool HasWeapon(const FString& WeaponId) const;

    UFUNCTION(BlueprintCallable, Category="Weapon")
    void GiveWeapon(const FString& WeaponId, int32 ClipAmmo, int32 ReserveAmmo, bool bMakeCurrent);

    UFUNCTION(BlueprintCallable, Category="Weapon")
    void SetCurrentWeapon(const FString& WeaponId);

    UFUNCTION(BlueprintCallable, Category="Weapon")
    bool TryConsumeAmmo();

    UFUNCTION(BlueprintCallable, Category="Weapon")
    bool CanReloadCurrent() const;

    UFUNCTION(BlueprintCallable, Category="Weapon")
    void ReloadCurrent();

    UFUNCTION(BlueprintCallable, Category="Weapon")
    float GetCurrentMoveSpeedMultiplier() const;

    UFUNCTION(BlueprintCallable, Category="Weapon")
    bool GetCurrentNoJump() const;

    UFUNCTION(BlueprintCallable, Category="Weapon")
    bool GetCurrentIsHeavy() const;

    UFUNCTION(BlueprintCallable, Category="Weapon")
    bool HasCurrentWeapon() const { return CurrentWeaponId.Len() > 0; }

protected:
    virtual void BeginPlay() override;

private:
    UPROPERTY()
    TMap<FString, FWeaponRuntimeState> Weapons;

    UPROPERTY()
    FString CurrentWeaponId;
};
