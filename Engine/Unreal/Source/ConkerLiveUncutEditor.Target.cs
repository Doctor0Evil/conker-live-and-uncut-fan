using UnrealBuildTool;
using System.Collections.Generic;

[SupportedPlatforms(UnrealPlatformClass.Desktop)]
public class ConkerLiveUncutEditorTarget : TargetRules
{
	public ConkerLiveUncutEditorTarget(TargetInfo Target) : base(Target)
	{
		Type = TargetType.Editor;
		DefaultBuildSettings = BuildSettingsVersion.V5;
		IncludeOrderVersion = EngineIncludeOrderVersion.Unreal5_3;
		ExtraModuleNames.AddRange(new string[] { "ConkerLiveUncut" });
		
		// Enable editor-only tools for content creation and debugging
		bBuildEditor = true;
		bBuildDeveloperTools = true;
		bCompileAgainstEngine = true;
		bCompileAgainstCoreUObject = true;
		bUseUnity = true;
		
		// Define editor macros
		Definitions.Add("CLU_EDITOR_BUILD=1");
		Definitions.Add("CLU_ENABLE_DEBUG_TOOLS=1");
		
		// Enable determinism hooks in editor for testing
		Definitions.Add("CLU_ENABLE_DETERMINISM=1");
		
		// Include additional modules for editor integration (Knowledge Graph tools, etc.)
		ExtraModuleNames.Add("UnrealEd");
		ExtraModuleNames.Add("AssetTools");
	}
}
