using UnityEngine;
using Uncut.Multiplayer.Systems;

[RequireComponent(typeof(Collider))]
public class HazardVolume : MonoBehaviour
{
	[SerializeField] private float damagePerSecond = 60.0f;
	[SerializeField] private bool isActive = false;

	private readonly System.Collections.Generic.HashSet<IHazardVictim> victims =
		new System.Collections.Generic.HashSet<IHazardVictim>();

	private void OnTriggerEnter(Collider other)
	{
		var victim = other.GetComponent<IHazardVictim>();
		if (victim != null)
		{
			victims.Add(victim);
		}
	}

	private void OnTriggerExit(Collider other)
	{
		var victim = other.GetComponent<IHazardVictim>();
		if (victim != null)
		{
			victims.Remove(victim);
		}
	}

	private void Update()
	{
		if (!isActive || victims.Count == 0)
			return;

		float delta = Time.deltaTime;
		float damageThisTick = damagePerSecond * delta;

		foreach (var victim in victims)
		{
			if (victim == null)
				continue;

			int asid = victim.GetCurrentASID();
			if (ASIDHelpers.ShouldIgnoreHazardDamage(asid))
				continue;

			victim.ApplyHazardDamage(damageThisTick);
		}
	}
}

public interface IHazardVictim
{
	int GetCurrentASID();
	void ApplyHazardDamage(float amount);
}
