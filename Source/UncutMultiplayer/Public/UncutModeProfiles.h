#pragma once

#include "CoreMinimal.h"
#include "UncutModeProfiles.generated.h"

USTRUCT(BlueprintType)
struct FUncutModeProfile
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FName Id;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString DisplayName;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    TArray<FName> EnabledZones;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    TArray<FName> EnabledSpawnRoleTags;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    TArray<FName> EnabledObjectiveRoleTags;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    TArray<FName> EnabledHazardRoleTags;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FName LightingProfile;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString Notes;
};

USTRUCT(BlueprintType)
struct FUncutModeProfilesFile
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString Version;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FString SchemaVersion;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FName MapId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    TArray<FUncutModeProfile> Modes;
};
