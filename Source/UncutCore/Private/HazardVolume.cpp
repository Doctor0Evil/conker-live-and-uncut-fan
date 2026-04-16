#include "HazardVolume.h"
#include "Components/BoxComponent.h"
#include "GameFramework/Actor.h"
#include "Kismet/GameplayStatics.h"
#include "JsonObjectConverter.h"

AHazardVolume::AHazardVolume()
{
    PrimaryActorTick.bCanEverTick = true;

    Volume = CreateDefaultSubobject<UBoxComponent>(TEXT("Volume"));
    RootComponent = Volume;
    Volume->SetCollisionEnabled(ECollisionEnabled::QueryOnly);
    Volume->SetCollisionResponseToAllChannels(ECR_Overlap);
}

void AHazardVolume::BeginPlay()
{
    Super::BeginPlay();

    LoadProfile();
    bIsEnabled = bStartsEnabled;
    UpdateEffects(bIsEnabled);
}

void AHazardVolume::Tick(float DeltaSeconds)
{
    Super::Tick(DeltaSeconds);

    if (!bIsEnabled || ActiveProfile.DamagePerSecond <= 0.0f)
    {
        return;
    }

    TickAccumulator += DeltaSeconds;
    if (TickAccumulator < ActiveProfile.TickInterval)
    {
        return;
    }

    TickAccumulator = 0.0f;
    ApplyDamageTick();
}

void AHazardVolume::SetHazardEnabled(bool bEnabled)
{
    if (bIsEnabled == bEnabled)
    {
        return;
    }

    bIsEnabled = bEnabled;
    UpdateEffects(bIsEnabled);
}

void AHazardVolume::LoadProfile()
{
    if (HazardProfileId.IsEmpty())
    {
        return;
    }

    FString JsonPath = FPaths::ProjectConfigDir() / TEXT("hazards/hazard_profiles_v1.json");
    FString JsonContent;
    if (!FFileHelper::LoadFileToString(JsonContent, *JsonPath))
    {
        UE_LOG(LogTemp, Warning, TEXT("HazardVolume: Failed to load %s"), *JsonPath);
        return;
    }

    TSharedPtr<FJsonObject> RootObj;
    TSharedRef<TJsonReader<>> Reader = TJsonReaderFactory<>::Create(JsonContent);
    if (!FJsonSerializer::Deserialize(Reader, RootObj) || !RootObj.IsValid())
    {
        UE_LOG(LogTemp, Warning, TEXT("HazardVolume: Failed to parse hazard profiles JSON"));
        return;
    }

    const TArray<TSharedPtr<FJsonValue>>* ProfilesArray;
    if (!RootObj->TryGetArrayField(TEXT("profiles"), ProfilesArray))
    {
        return;
    }

    for (const TSharedPtr<FJsonValue>& Value : *ProfilesArray)
    {
        const TSharedPtr<FJsonObject>* ObjPtr;
        if (!Value->TryGetObject(ObjPtr))
        {
            continue;
        }

        FHazardProfile Candidate;
        if (!FJsonObjectConverter::JsonObjectToUStruct(ObjPtr->ToSharedRef(), &Candidate))
        {
            continue;
        }

        if (Candidate.Id == HazardProfileId)
        {
            ActiveProfile = Candidate;
            UE_LOG(LogTemp, Log, TEXT("HazardVolume: Loaded profile %s"), *HazardProfileId);
            return;
        }
    }

    UE_LOG(LogTemp, Warning, TEXT("HazardVolume: Profile %s not found"), *HazardProfileId);
}

void AHazardVolume::ApplyDamageTick()
{
    TArray<AActor*> OverlappingActors;
    Volume->GetOverlappingActors(OverlappingActors);

    for (AActor* Actor : OverlappingActors)
    {
        if (!Actor || IsPawnImmune(Actor))
        {
            continue;
        }

        const float Damage = ActiveProfile.DamagePerSecond * ActiveProfile.TickInterval;
        UGameplayStatics::ApplyDamage(
            Actor,
            Damage,
            nullptr,
            this,
            nullptr
        );
    }
}

bool AHazardVolume::IsPawnImmune(AActor* Pawn) const
{
    // Pseudocode: ask the ASID component for active states and check against ImmunityAsids.
    // auto* AsidComponent = Pawn->FindComponentByClass<UAsidComponent>();
    // if (!AsidComponent) { return false; }
    // const TArray<FString> ActiveAsids = AsidComponent->GetActiveAsids();
    // for (const FString& Asid : ActiveAsids)
    // {
    //     if (ActiveProfile.ImmunityAsids.Contains(Asid))
    //     {
    //         return true;
    //     }
    // }
    return false;
}

void AHazardVolume::UpdateEffects(bool bEnable)
{
    // Hook up Niagara and audio here based on ActiveProfile.VisualEffectId / AudioCueId.
    // Designers can wire this in Blueprint, using ActiveProfile.HazardType as a switch.
}
