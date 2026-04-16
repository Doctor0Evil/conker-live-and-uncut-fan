using UnityEngine;

/// <summary>
/// AlienBaseAirlockController
/// 
/// Map-level Airlock/Gas state machine for 04_Multiplayer_Alien_Base.
/// Controls hazard volumes and responds to trigger consoles.
/// </summary>
public class AlienBaseAirlockController : MonoBehaviour
{
    public enum AirlockState
    {
        Idle,
        Arming,
        Active,
        Cooldown
        }

    [Header("Hazard Volumes")]
    public HazardVolume hubFloorGasVolume;
    public HazardVolume sublevelAcidVolume;

    [Header("Timing (seconds)")]
    public float armingDurationSec = 5.0f;
    public float activeDurationSec = 12.0f;
    public float cooldownDurationSec = 30.0f;

    [Header("Debug")]
    public AirlockState currentState = AirlockState.Idle;
    public float timeInState = 0.0f;
    public int eventInstigatorTeam = -1;

    void Start()
    {
        EnterState(AirlockState.Idle);
    }

    void Update()
    {
        UpdateState(Time.deltaTime);
    }

    public void RequestTriggerActivation(string triggerId, int instigatorTeam)
    {
        if (currentState != AirlockState.Idle)
        {
            return;
        }

        eventInstigatorTeam = instigatorTeam;
        EnterState(AirlockState.Arming);

        // TODO: Play global arming VO/sfx and start warning lights.
    }

    void EnterState(AirlockState newState)
    {
        currentState = newState;
        timeInState = 0.0f;

        switch (currentState)
        {
            case AirlockState.Idle:
                OnEnteredIdle();
                break;
            case AirlockState.Arming:
                OnEnteredArming();
                break;
            case AirlockState.Active:
                OnEnteredActive();
                break;
            case AirlockState.Cooldown:
                OnEnteredCooldown();
                break;
        }
    }

    void UpdateState(float deltaTime)
    {
        timeInState += deltaTime;

        switch (currentState)
        {
            case AirlockState.Idle:
                break;

            case AirlockState.Arming:
                if (timeInState >= armingDurationSec)
                {
                    EnterState(AirlockState.Active);
                }
                break;

            case AirlockState.Active:
                if (timeInState >= activeDurationSec)
                {
                    EnterState(AirlockState.Cooldown);
                }
                break;

            case AirlockState.Cooldown:
                if (timeInState >= cooldownDurationSec)
                {
                    EnterState(AirlockState.Idle);
                }
                break;
        }
    }

    void OnEnteredIdle()
    {
        SetHazardVolumesActive(false);
        eventInstigatorTeam = -1;
    }

    void OnEnteredArming()
    {
        SetHazardVolumesActive(false);
        // TODO: Sirens, lights, pre-gas FX.
    }

    void OnEnteredActive()
    {
        SetHazardVolumesActive(true);
        // TODO: Update VO/UI to "Airlock sealed / gas released".
    }

    void OnEnteredCooldown()
    {
        SetHazardVolumesActive(false);
        // TODO: Venting FX and ambience restore.
    }

    void SetHazardVolumesActive(bool active)
    {
        if (hubFloorGasVolume != null)
        {
            hubFloorGasVolume.SetActive(active);
        }

        if (sublevelAcidVolume != null)
        {
            sublevelAcidVolume.SetActive(active);
        }
    }
}
