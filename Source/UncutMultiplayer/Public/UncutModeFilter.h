#pragma once

#include "CoreMinimal.h"
#include "UncutModeProfiles.h"
#include "UncutModeFilter.generated.h"

USTRUCT(BlueprintType)
struct FUncutRoleTaggedEntity
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    FName Id;

    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    TArray<FName> RoleTags;
};

UCLASS()
class UUncutModeFilter : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "Uncut|Mode")
    static bool IsEntityAllowedInMode(
        const FUncutModeProfile& Mode,
        const TArray<FName>& EntityRoleTags,
        const TArray<FName>& AllowedRoleTags
    )
    {
        for (const FName& Tag : EntityRoleTags)
        {
            if (AllowedRoleTags.Contains(Tag))
            {
                return true;
            }
        }
        return AllowedRoleTags.Num() == 0;
    }
};
