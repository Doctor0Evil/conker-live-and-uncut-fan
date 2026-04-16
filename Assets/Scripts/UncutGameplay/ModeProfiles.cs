using System;
using UnityEngine;

namespace UncutMultiplayer.Gameplay
{
    [Serializable]
    public class ModeProfile
    {
        public string id;
        public string displayname;
        public string[] enabledzones;
        public string[] enabledspawnroletags;
        public string[] enabledobjectiveroletags;
        public string[] enabledhazardroletags;
        public string lightingprofile;
        public string notes;
    }

    [Serializable]
    public class ModeProfilesFile
    {
        public string version;
        public string schemaversion;
        public string mapid;
        public ModeProfile[] modes;
    }

    public static class ModeFilter
    {
        public static bool IsAllowed(string[] entityTags, string[] allowedTags)
        {
            if (allowedTags == null || allowedTags.Length == 0)
                return true;

            if (entityTags == null)
                return false;

            foreach (var tag in entityTags)
            {
                foreach (var allowed in allowedTags)
                {
                    if (string.Equals(tag, allowed, StringComparison.OrdinalIgnoreCase))
                        return true;
                }
            }

            return false;
        }
    }
}
