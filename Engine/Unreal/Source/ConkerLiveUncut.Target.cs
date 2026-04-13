using UnrealBuildTool;
using System.Collections.Generic;

[SupportedPlatforms(UnrealPlatformClass.Desktop)]
public class ConkerLiveUncutTarget : TargetRules
{
	public ConkerLiveUncutTarget(TargetInfo Target) : base(Target)
	{
		Type = TargetType.Game;
		DefaultBuildSettings = BuildSettingsVersion.V5;
		IncludeOrderVersion = EngineIncludeOrderVersion.Unreal5_3;
		ExtraModuleNames.AddRange(new string[] { "ConkerLiveUncut" });
		
		// Enforce strict compilation standards for determinism and safety
		bEnableExceptions = false;
		bUseUnity = true; // Enable unity builds for performance
		bUseFastPacer = true; // Enable fast linker where supported
		bOptimizeCode = true;
		
		// Define project macros for engine selection and feature toggles
		Definitions.Add("CLU_PROJECT_VERSION=\"0.1.0\"");
		Definitions.Add("CLU_ENABLE_DETERMINISM=1");
		Definitions.Add("CLU_ENABLE_AI_CHAT_HOOKS=1");
		
		// Disable engine plugins that are not required to reduce build size
		bBuildEditor = false; // Runtime build does not need editor support
		bBuildDeveloperTools = false;
	}
}
