using System.Linq;
using UnityEngine;
using UnityEditor;

namespace UncutMultiplayer.Gameplay
{
    [ExecuteInEditMode]
    public class BeachDeadLevelBuilder : MonoBehaviour
    {
        [Header("JSON Data")]
        [SerializeField] private TextAsset gridJson;
        [SerializeField] private TextAsset entitiesJson;
        [SerializeField] private TextAsset modeProfilesJson;

        [Header("Build Settings")]
        [SerializeField] private string modeId = "BeachFrenchiesDay";
        [SerializeField] private string generatedTag = "Grid2SceneGenerated";

        private MapGrid grid;
        private MapEntities entities;
        private ModeProfilesFile modeProfiles;

        [ContextMenu("Build Level For Mode")]
        public void BuildLevelForMode()
        {
            if (!LoadAllJson())
                return;

            var mode = modeProfiles.modes.FirstOrDefault(m => m.id == modeId);
            if (mode == null)
            {
                Debug.LogWarning($"BeachDeadLevelBuilder: Unknown mode '{modeId}'.");
                return;
            }

            ClearPrevious();

            BuildTiles(); // same as AlienBase pattern

            BuildSpawnsFiltered(mode);
            BuildObjectivesFiltered(mode);
            BuildHazardsFiltered(mode);

            ApplyLightingProfile(mode.lightingprofile);
        }

        private bool LoadAllJson()
        {
            if (gridJson == null || entitiesJson == null || modeProfilesJson == null)
            {
                Debug.LogError("BeachDeadLevelBuilder: Missing JSON assets.");
                return false;
            }

            grid = JsonUtility.FromJson<MapGrid>(gridJson.text);
            entities = JsonUtility.FromJson<MapEntities>(entitiesJson.text);
            modeProfiles = JsonUtility.FromJson<ModeProfilesFile>(modeProfilesJson.text);

            if (grid == null || entities == null || modeProfiles == null)
            {
                Debug.LogError("BeachDeadLevelBuilder: Failed to parse JSON.");
                return false;
            }

            return true;
        }

        private void ClearPrevious()
        {
            var toDestroy = GetComponentsInChildren<Transform>(true)
                .Where(t => t != transform && t.CompareTag(generatedTag))
                .Select(t => t.gameObject)
                .ToList();

            foreach (var go in toDestroy)
            {
                Undo.DestroyObjectImmediate(go);
            }
        }

        private void BuildSpawnsFiltered(ModeProfile mode)
        {
            var parent = new GameObject("Spawns");
            parent.transform.SetParent(transform, false);
            parent.tag = generatedTag;

            var spawnPrefab = /* resolve from tileset mapping, as in AlienBase builder */ default(GameObject);
            if (spawnPrefab == null)
            {
                Debug.LogWarning("BeachDeadLevelBuilder: spawnPrefab not configured.");
                return;
            }

            foreach (var sp in entities.spawnpoints)
            {
                if (!ModeFilter.IsAllowed(sp.roletags, mode.enabledspawnroletags))
                    continue;

                var pos = grid.CellToWorld(sp.col, sp.row, sp.yoffset);
                var instance = (GameObject)PrefabUtility.InstantiatePrefab(spawnPrefab);
                Undo.RegisterCreatedObjectUndo(instance, "Create spawn");
                instance.name = $"Spawn_{sp.id}";
                instance.transform.position = pos;
                instance.transform.SetParent(parent.transform, true);
                instance.tag = generatedTag;
            }
        }

        private void BuildObjectivesFiltered(ModeProfile mode)
        {
            var parent = new GameObject("Objectives");
            parent.transform.SetParent(transform, false);
            parent.tag = generatedTag;

            // Use tileset.entitymappings.objective to resolve per-objective-type prefab, as in your existing emitter.

            foreach (var obj in entities.objectives)
            {
                if (!ModeFilter.IsAllowed(obj.roletags, mode.enabledobjectiveroletags))
                    continue;

                var prefab = ResolveObjectivePrefab(obj.objectivetype);
                if (prefab == null)
                    continue;

                var pos = grid.CellToWorld(obj.col, obj.row, obj.yoffset);
                var instance = (GameObject)PrefabUtility.InstantiatePrefab(prefab);
                Undo.RegisterCreatedObjectUndo(instance, "Create objective");
                instance.name = $"Objective_{obj.id}";
                instance.transform.position = pos;
                instance.transform.SetParent(parent.transform, true);
                instance.tag = generatedTag;
            }
        }

        private void BuildHazardsFiltered(ModeProfile mode)
        {
            var parent = new GameObject("Hazards");
            parent.transform.SetParent(transform, false);
            parent.tag = generatedTag;

            foreach (var hz in entities.hazardvolumes)
            {
                if (!ModeFilter.IsAllowed(hz.roletags, mode.enabledhazardroletags))
                    continue;

                var prefab = ResolveHazardPrefab(hz.hazardtype);
                if (prefab == null)
                    continue;

                var center = grid.CellToWorld(hz.centercol, hz.centerrow, 0.0f);
                var instance = (GameObject)PrefabUtility.InstantiatePrefab(prefab);
                Undo.RegisterCreatedObjectUndo(instance, "Create hazard");
                instance.name = $"Hazard_{hz.id}";
                instance.transform.position = center;
                instance.transform.SetParent(parent.transform, true);
                instance.tag = generatedTag;

                // Optionally configure collider radius and height based on hz.radiuscells, hz.minyoffset, hz.maxyoffset.
            }
        }

        private GameObject ResolveObjectivePrefab(string objectiveType)
        {
            // Hook into your tileset.entitymappings.objective table.
            return null;
        }

        private GameObject ResolveHazardPrefab(string hazardType)
        {
            // Hook into your tileset.entitymappings.hazardvolume table.
            return null;
        }

        private void ApplyLightingProfile(string lightingProfileId)
        {
            // Switch a ScriptableRenderSettings asset, post-processing volume, or light setup
            // based on lightingProfileId (BeachDay, BeachNightStealth, WarOvercast).
        }
    }
}
