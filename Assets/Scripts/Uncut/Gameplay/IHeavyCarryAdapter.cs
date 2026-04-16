namespace UncutMultiplayer.Gameplay
{
    public interface IHeavyCarryAdapter
    {
        // Called when ASID050 should be active
        void EnterHeavyCarry(WeaponStats stats);

        // Called when leaving ASID050
        void ClearHeavyCarry();
    }
}
