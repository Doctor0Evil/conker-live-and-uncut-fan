#include "WeaponInventoryComponent.h"

UWeaponInventoryComponent::UWeaponInventoryComponent()
{
    PrimaryComponentTick.bCanEverTick = false;
}

void UWeaponInventoryComponent::BeginPlay()
{
    Super::BeginPlay();
}

void UWeaponInventoryComponent::InitializeFromStats(const TArray<FWeaponStats>& AllStats)
{
    Weapons.Empty();
    CurrentWeaponId.Empty();

    for (const FWeaponStats& Stats : AllStats)
    {
        FWeaponRuntimeState State;
        State.Stats = Stats;
        State.ClipAmmo = 0;
        State.ReserveAmmo = 0;
        Weapons.Add(Stats.Id, State);
    }
}

bool UWeaponInventoryComponent::HasWeapon(const FString& WeaponId) const
{
    if (const FWeaponRuntimeState* State = Weapons.Find(WeaponId))
    {
        return State->ClipAmmo > 0 || State->ReserveAmmo > 0;
    }
    return false;
}

void UWeaponInventoryComponent::GiveWeapon(const FString& WeaponId, int32 ClipAmmo, int32 ReserveAmmo, bool bMakeCurrent)
{
    if (FWeaponRuntimeState* State = Weapons.Find(WeaponId))
    {
        const int32 ClipSize = State->Stats.ClipSize;
        if (ClipSize > 0)
        {
            State->ClipAmmo = FMath::Clamp(State->ClipAmmo + ClipAmmo, 0, ClipSize);
        }

        const int32 MaxReserve = State->Stats.MaxReserveAmmo;
        State->ReserveAmmo = FMath::Clamp(State->ReserveAmmo + ReserveAmmo, 0, MaxReserve);

        if (bMakeCurrent)
        {
            SetCurrentWeapon(WeaponId);
        }
    }
}

void UWeaponInventoryComponent::SetCurrentWeapon(const FString& WeaponId)
{
    if (!Weapons.Contains(WeaponId))
    {
        return;
    }

    CurrentWeaponId = WeaponId;
    OnWeaponChanged.Broadcast(*Weapons.Find(CurrentWeaponId));
}

bool UWeaponInventoryComponent::TryConsumeAmmo()
{
    if (!Weapons.Contains(CurrentWeaponId))
    {
        return false;
    }

    FWeaponRuntimeState& State = Weapons.FindChecked(CurrentWeaponId);
    const FWeaponStats& Stats = State.Stats;

    if (Stats.ClipSize <= 0)
    {
        return true;
    }

    if (State.ClipAmmo > 0)
    {
        State.ClipAmmo--;
        return true;
    }

    return false;
}

bool UWeaponInventoryComponent::CanReloadCurrent() const
{
    if (!Weapons.Contains(CurrentWeaponId))
    {
        return false;
    }

    const FWeaponRuntimeState& State = Weapons.FindChecked(CurrentWeaponId);
    const FWeaponStats& Stats = State.Stats;

    if (Stats.ClipSize <= 0)
    {
        return false;
    }

    if (State.ClipAmmo >= Stats.ClipSize)
    {
        return false;
    }

    return State.ReserveAmmo > 0;
}

void UWeaponInventoryComponent::ReloadCurrent()
{
    if (!Weapons.Contains(CurrentWeaponId))
    {
        return;
    }

    FWeaponRuntimeState& State = Weapons.FindChecked(CurrentWeaponId);
    const FWeaponStats& Stats = State.Stats;

    if (Stats.ClipSize <= 0)
    {
        return;
    }

    if (State.ReserveAmmo <= 0)
    {
        return;
    }

    const int32 Needed = Stats.ClipSize - State.ClipAmmo;
    const int32 Taken = FMath::Min(Needed, State.ReserveAmmo);

    State.ClipAmmo += Taken;
    State.ReserveAmmo -= Taken;
}

float UWeaponInventoryComponent::GetCurrentMoveSpeedMultiplier() const
{
    if (const FWeaponRuntimeState* State = Weapons.Find(CurrentWeaponId))
    {
        return State->Stats.MoveSpeedMultiplier;
    }
    return 1.0f;
}

bool UWeaponInventoryComponent::GetCurrentNoJump() const
{
    if (const FWeaponRuntimeState* State = Weapons.Find(CurrentWeaponId))
    {
        return State->Stats.bNoJump;
    }
    return false;
}

bool UWeaponInventoryComponent::GetCurrentIsHeavy() const
{
    if (const FWeaponRuntimeState* State = Weapons.Find(CurrentWeaponId))
    {
        return State->Stats.bIsHeavy;
    }
    return false;
}
