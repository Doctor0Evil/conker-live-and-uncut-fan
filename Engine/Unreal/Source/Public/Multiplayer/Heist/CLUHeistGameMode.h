#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "CLUHeistGameMode.generated.h"

class APlayerController;
class APlayerState;
class AGameStateBase;

UENUM(BlueprintType)
enum class ECLUHeistPhase : uint8
{
    Warmup     UMETA(DisplayName = "Warmup"),
    Setup      UMETA(DisplayName = "Setup"),
    Breach     UMETA(DisplayName = "Breach"),
    Escape     UMETA(DisplayName = "Escape"),
    PostMatch  UMETA(DisplayName = "Post Match")
};

/**
 * CLUHeistGameMode
 *
 * Server-authoritative rule set for the Heist multiplayer mode.
 * Responsibilities:
 * - Team assignment and spawn selection.
 * - Match phase state machine (Warmup → Setup → Breach → Escape → PostMatch).
 * - Tracking team scores via GameState / PlayerState for replication.
 * - Blueprint events for vault state, announcements, and end-of-match.
 */
UCLASS()
class CONKERLIVEUNCUT_API ACLUHeistGameMode : public AGameModeBase
{
    GENERATED_BODY()

public:
    ACLUHeistGameMode(const FObjectInitializer& ObjectInitializer = FObjectInitializer::Get());

    /** Team index for robbers (bank attackers). */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "CLU|Heist")
    int32 RobberTeamId;

    /** Team index for defenders (bank guards). */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "CLU|Heist")
    int32 GuardTeamId;

    /** Target score or objective count required to win the match. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "CLU|Heist")
    int32 TargetScoreToWin;

    /** Duration (in seconds) for the Setup phase. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "CLU|Heist")
    float SetupPhaseDuration;

    /** Duration (in seconds) for the Breach phase. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "CLU|Heist")
    float BreachPhaseDuration;

    /** Duration (in seconds) for the Escape phase. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "CLU|Heist")
    float EscapePhaseDuration;

    /** Current phase of the match (server-only, mirror to GameState if you need it in Blueprints). */
    UPROPERTY(BlueprintReadOnly, Category = "CLU|Heist")
    ECLUHeistPhase CurrentPhase;

    /** Returns current score for the robber team (from GameState or cached). */
    UFUNCTION(BlueprintCallable, Category = "CLU|Heist")
    int32 GetRobberTeamScore() const;

    /** Returns current score for the guard team (from GameState or cached). */
    UFUNCTION(BlueprintCallable, Category = "CLU|Heist")
    int32 GetGuardTeamScore() const;

    /** Adds score to a team and checks for win conditions. */
    UFUNCTION(BlueprintCallable, Category = "CLU|Heist")
    void AddTeamScore(int32 TeamId, int32 Delta);

    /** Notify the GameMode that the vault door has been opened successfully. */
    UFUNCTION(BlueprintCallable, Category = "CLU|Heist")
    void NotifyVaultOpened(int32 TeamId);

    /** Notify the GameMode that the robbers have escaped with the loot. */
    UFUNCTION(BlueprintCallable, Category = "CLU|Heist")
    void NotifyLootEscaped(int32 TeamId);

    /** Get whether the match is currently active (not in Warmup or PostMatch). */
    UFUNCTION(BlueprintPure, Category = "CLU|Heist")
    bool IsMatchInProgressHeist() const;

protected:
    virtual void BeginPlay() override;

    virtual void HandleMatchHasStarted() override;
    virtual void HandleMatchHasEnded() override;

    virtual void PostLogin(APlayerController* NewPlayer) override;

    /** Advance to the next logical phase in the heist state machine. */
    void AdvancePhase();

    /** Set the current phase and broadcast to Blueprints. */
    void SetPhase(ECLUHeistPhase NewPhase);

    /** Internal helper to update team scores on GameState. */
    void SetTeamScoreInternal(int32 TeamId, int32 NewScore);

    /** Timer handles for phase transitions. */
    FTimerHandle TimerHandle_SetupPhase;
    FTimerHandle TimerHandle_BreachPhase;
    FTimerHandle TimerHandle_EscapePhase;

    /** Cached scores (mirrored to GameState for replication). */
    int32 RobberTeamScore;
    int32 GuardTeamScore;

    /** Whether the vault has been opened this round. */
    bool bVaultOpened;

    /** Whether the loot has been successfully extracted this round. */
    bool bLootEscaped;

    /** Blueprint event: phase changed on the server. */
    UFUNCTION(BlueprintImplementableEvent, Category = "CLU|Heist")
    void BP_OnPhaseChanged(ECLUHeistPhase NewPhase);

    /** Blueprint event: vault opened (e.g., for sounds, VFX). */
    UFUNCTION(BlueprintImplementableEvent, Category = "CLU|Heist")
    void BP_OnVaultOpened(int32 ByTeamId);

    /** Blueprint event: loot escaped (e.g., end-of-round sequence). */
    UFUNCTION(BlueprintImplementableEvent, Category = "CLU|Heist")
    void BP_OnLootEscaped(int32 ByTeamId);

    /** Blueprint event: match finished, with winning team ID (-1 for draw). */
    UFUNCTION(BlueprintImplementableEvent, Category = "CLU|Heist")
    void BP_OnMatchFinished(int32 WinningTeamId);

    /** Assign the new player to a team (simple alternating or balanced assignment). */
    void AssignPlayerToTeam(APlayerState* PlayerState);

    /** Check win conditions after a scoring event. */
    void CheckWinConditions();
};
