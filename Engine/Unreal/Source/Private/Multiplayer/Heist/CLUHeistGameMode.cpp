#include "Multiplayer/Heist/CLUHeistGameMode.h"

#include "GameFramework/GameStateBase.h"
#include "GameFramework/PlayerState.h"
#include "TimerManager.h"
#include "Engine/World.h"

ACLUHeistGameMode::ACLUHeistGameMode(const FObjectInitializer& ObjectInitializer)
    : Super(ObjectInitializer)
{
    RobberTeamId = 0;
    GuardTeamId = 1;

    TargetScoreToWin = 3;

    SetupPhaseDuration = 20.0f;
    BreachPhaseDuration = 120.0f;
    EscapePhaseDuration = 45.0f;

    CurrentPhase = ECLUHeistPhase::Warmup;

    RobberTeamScore = 0;
    GuardTeamScore = 0;

    bVaultOpened = false;
    bLootEscaped = false;
}

void ACLUHeistGameMode::BeginPlay()
{
    Super::BeginPlay();

    // You could start the match automatically or wait for players / external trigger.
    // For now, rely on the standard MatchState flow.
}

void ACLUHeistGameMode::HandleMatchHasStarted()
{
    Super::HandleMatchHasStarted();

    RobberTeamScore = 0;
    GuardTeamScore = 0;
    bVaultOpened = false;
    bLootEscaped = false;

    SetPhase(ECLUHeistPhase::Setup);

    // Schedule transition to Breach phase.
    if (UWorld* World = GetWorld())
    {
        World->GetTimerManager().SetTimer(
            TimerHandle_SetupPhase,
            this,
            &ACLUHeistGameMode::AdvancePhase,
            SetupPhaseDuration,
            false
        );
    }
}

void ACLUHeistGameMode::HandleMatchHasEnded()
{
    Super::HandleMatchHasEnded();

    // Clear timers to avoid stray callbacks after match end.
    if (UWorld* World = GetWorld())
    {
        World->GetTimerManager().ClearTimer(TimerHandle_SetupPhase);
        World->GetTimerManager().ClearTimer(TimerHandle_BreachPhase);
        World->GetTimerManager().ClearTimer(TimerHandle_EscapePhase);
    }

    SetPhase(ECLUHeistPhase::PostMatch);
}

void ACLUHeistGameMode::PostLogin(APlayerController* NewPlayer)
{
    Super::PostLogin(NewPlayer);

    if (!NewPlayer)
    {
        return;
    }

    if (APlayerState* PS = NewPlayer->PlayerState)
    {
        AssignPlayerToTeam(PS);
    }
}

void ACLUHeistGameMode::AssignPlayerToTeam(APlayerState* PlayerState)
{
    if (!PlayerState)
    {
        return;
    }

    // Simple balancing: count players per team and assign to the smaller team.
    int32 RobberCount = 0;
    int32 GuardCount = 0;

    if (AGameStateBase* GS = GameState)
    {
        for (APlayerState* PS : GS->PlayerArray)
        {
            if (!PS) continue;

            // Using Generic Team Id pattern; you can replace with your own team system.
            const int32 TeamId = PS->GetPlayerId() % 2; // placeholder logic
            if (TeamId == RobberTeamId)
            {
                ++RobberCount;
            }
            else if (TeamId == GuardTeamId)
            {
                ++GuardCount;
            }
        }
    }

    const int32 AssignedTeamId = (RobberCount <= GuardCount) ? RobberTeamId : GuardTeamId;
    PlayerState->SetPlayerId(AssignedTeamId);
}

void ACLUHeistGameMode::SetPhase(ECLUHeistPhase NewPhase)
{
    if (CurrentPhase == NewPhase)
    {
        return;
    }

    CurrentPhase = NewPhase;
    BP_OnPhaseChanged(NewPhase);
}

void ACLUHeistGameMode::AdvancePhase()
{
    UWorld* World = GetWorld();
    if (!World)
    {
        return;
    }

    switch (CurrentPhase)
    {
    case ECLUHeistPhase::Setup:
        SetPhase(ECLUHeistPhase::Breach);
        World->GetTimerManager().SetTimer(
            TimerHandle_BreachPhase,
            this,
            &ACLUHeistGameMode::AdvancePhase,
            BreachPhaseDuration,
            false
        );
        break;

    case ECLUHeistPhase::Breach:
        SetPhase(ECLUHeistPhase::Escape);
        World->GetTimerManager().SetTimer(
            TimerHandle_EscapePhase,
            this,
            &ACLUHeistGameMode::AdvancePhase,
            EscapePhaseDuration,
            false
        );
        break;

    case ECLUHeistPhase::Escape:
        // Time is up, end match and evaluate scores.
        EndMatch();
        break;

    default:
        break;
    }
}

bool ACLUHeistGameMode::IsMatchInProgressHeist() const
{
    return CurrentPhase == ECLUHeistPhase::Setup ||
           CurrentPhase == ECLUHeistPhase::Breach ||
           CurrentPhase == ECLUHeistPhase::Escape;
}

int32 ACLUHeistGameMode::GetRobberTeamScore() const
{
    return RobberTeamScore;
}

int32 ACLUHeistGameMode::GetGuardTeamScore() const
{
    return GuardTeamScore;
}

void ACLUHeistGameMode::AddTeamScore(int32 TeamId, int32 Delta)
{
    if (!HasAuthority())
    {
        return;
    }

    if (Delta == 0)
    {
        return;
    }

    if (TeamId == RobberTeamId)
    {
        RobberTeamScore = FMath::Max(0, RobberTeamScore + Delta);
        SetTeamScoreInternal(RobberTeamId, RobberTeamScore);
    }
    else if (TeamId == GuardTeamId)
    {
        GuardTeamScore = FMath::Max(0, GuardTeamScore + Delta);
        SetTeamScoreInternal(GuardTeamId, GuardTeamScore);
    }

    CheckWinConditions();
}

void ACLUHeistGameMode::SetTeamScoreInternal(int32 TeamId, int32 NewScore)
{
    // Minimal implementation: if you have a custom GameState subclass,
    // mirror scores into that class for replication and UI.
    if (AGameStateBase* GS = GameState)
    {
        // Placeholder: you can later add team-aware score tracking on GameState.
        GS->Score = RobberTeamScore - GuardTeamScore;
    }
}

void ACLUHeistGameMode::CheckWinConditions()
{
    if (!HasAuthority())
    {
        return;
    }

    int32 WinningTeamId = -1;

    if (RobberTeamScore >= TargetScoreToWin && GuardTeamScore >= TargetScoreToWin)
    {
        WinningTeamId = -1; // draw
    }
    else if (RobberTeamScore >= TargetScoreToWin)
    {
        WinningTeamId = RobberTeamId;
    }
    else if (GuardTeamScore >= TargetScoreToWin)
    {
        WinningTeamId = GuardTeamId;
    }

    if (WinningTeamId != -1)
    {
        BP_OnMatchFinished(WinningTeamId);
        EndMatch();
    }
}

void ACLUHeistGameMode::NotifyVaultOpened(int32 TeamId)
{
    if (!HasAuthority() || bVaultOpened)
    {
        return;
    }

    bVaultOpened = true;

    BP_OnVaultOpened(TeamId);

    // Example: award a point to the team that opened the vault.
    AddTeamScore(TeamId, 1);
}

void ACLUHeistGameMode::NotifyLootEscaped(int32 TeamId)
{
    if (!HasAuthority() || bLootEscaped)
    {
        return;
    }

    bLootEscaped = true;

    BP_OnLootEscaped(TeamId);

    // Example: award an additional point for successful escape.
    AddTeamScore(TeamId, 1);
}
