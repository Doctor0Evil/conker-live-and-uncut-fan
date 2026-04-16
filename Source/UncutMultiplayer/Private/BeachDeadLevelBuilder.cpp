#include "BeachDeadLevelBuilder.h"
#include "UncutModeProfiles.h"
#include "UncutModeFilter.h"

void ABeachDeadLevelBuilder::BuildLevelForMode(const FName ModeId)
{
    // Assume Entities and ModeProfiles are already loaded from JSON.
    const FUncutModeProfile* Mode = ModeProfiles.Modes.FindByPredicate(
        [ModeId](const FUncutModeProfile& M) { return M.Id == ModeId; });

    if (!Mode)
    {
        UE_LOG(LogTemp, Warning, TEXT("BeachDead: Unknown mode '%s'"), *ModeId.ToString());
        return;
    }

    // Spawns
    for (const FUncutSpawnPoint& Spawn : Entities.SpawnPoints)
    {
        if (!UUncutModeFilter::IsEntityAllowedInMode(
                *Mode,
                Spawn.RoleTags,
                Mode->EnabledSpawnRoleTags))
        {
            continue;
        }

        SpawnPlayerStartActor(Spawn);
    }

    // Objectives
    for (const FUncutObjective& Obj : Entities.Objectives)
    {
        if (!UUncutModeFilter::IsEntityAllowedInMode(
                *Mode,
                Obj.RoleTags,
                Mode->EnabledObjectiveRoleTags))
        {
            continue;
        }

        SpawnObjectiveActor(Obj);
    }

    // Hazards
    for (const FUncutHazardVolume& Haz : Entities.HazardVolumes)
    {
        if (!UUncutModeFilter::IsEntityAllowedInMode(
                *Mode,
                Haz.RoleTags,
                Mode->EnabledHazardRoleTags))
        {
            continue;
        }

        SpawnHazardActor(Haz);
    }

    ApplyLightingProfile(Mode->LightingProfile);
}
