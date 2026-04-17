use serde::Deserialize;

/// HazardProfile defines the damage and immunity characteristics of a hazard volume.
#[derive(Debug, Clone, Deserialize)]
pub struct HazardProfile {
    /// Unique identifier for this profile (e.g., "hub_gas_v1").
    pub id: String,
    /// Damage dealt per second to entities without immunity.
    pub damage_per_second: f32,
    /// List of ASIDs (Actor Security IDs) that are immune to this hazard.
    #[serde(default)]
    pub immunity_asids: Vec<u16>,
    /// Optional visual effect asset path.
    #[serde(default)]
    pub vfx_asset: Option<String>,
    /// Optional sound effect asset path.
    #[serde(default)]
    pub sfx_asset: Option<String>,
}

impl HazardProfile {
    /// Validates the HazardProfile:
    /// - damage_per_second must be > 0.0
    /// - All immunity_asids must be valid u16 values (always true by type, but we check for duplicates)
    pub fn validate(&self) -> Result<(), String> {
        if self.damage_per_second <= 0.0 {
            return Err(format!(
                "HazardProfile '{}': damage_per_second ({}) must be > 0.0",
                self.id, self.damage_per_second
            ));
        }

        // Check for duplicate ASIDs
        let mut seen = std::collections::HashSet::new();
        for &asid in &self.immunity_asids {
            if !seen.insert(asid) {
                return Err(format!(
                    "HazardProfile '{}': duplicate immunity_asid {}",
                    self.id, asid
                ));
            }
        }

        Ok(())
    }
}

/// DamageRule defines conditional damage behavior for specific entity types.
#[derive(Debug, Clone, Deserialize)]
pub struct DamageRule {
    /// Unique identifier for this rule.
    pub id: String,
    /// The hazard profile ID this rule applies to.
    pub hazard_profile_id: String,
    /// Target entity type or tag this rule affects.
    pub target_entity_type: String,
    /// Damage multiplier for this entity type (1.0 = normal damage).
    #[serde(default = "default_damage_multiplier")]
    pub damage_multiplier: f32,
    /// Whether this entity type is completely immune (overrides damage_multiplier).
    #[serde(default)]
    pub is_immune: bool,
}

fn default_damage_multiplier() -> f32 {
    1.0
}

impl DamageRule {
    /// Validates the DamageRule:
    /// - damage_multiplier must be >= 0.0
    pub fn validate(&self) -> Result<(), String> {
        if self.damage_multiplier < 0.0 {
            return Err(format!(
                "DamageRule '{}': damage_multiplier ({}) must be >= 0.0",
                self.id, self.damage_multiplier
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hazard_profile() {
        let profile = HazardProfile {
            id: "test_gas".to_string(),
            damage_per_second: 10.0,
            immunity_asids: vec![1, 2, 3],
            vfx_asset: None,
            sfx_asset: None,
        };
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn test_invalid_damage_per_second() {
        let profile = HazardProfile {
            id: "test_gas".to_string(),
            damage_per_second: 0.0,
            immunity_asids: vec![],
            vfx_asset: None,
            sfx_asset: None,
        };
        assert!(profile.validate().is_err());
    }

    #[test]
    fn test_duplicate_asid() {
        let profile = HazardProfile {
            id: "test_gas".to_string(),
            damage_per_second: 10.0,
            immunity_asids: vec![1, 1, 2],
            vfx_asset: None,
            sfx_asset: None,
        };
        assert!(profile.validate().is_err());
    }

    #[test]
    fn test_valid_damage_rule() {
        let rule = DamageRule {
            id: "marine_weakness".to_string(),
            hazard_profile_id: "hub_gas_v1".to_string(),
            target_entity_type: "marine".to_string(),
            damage_multiplier: 1.5,
            is_immune: false,
        };
        assert!(rule.validate().is_ok());
    }

    #[test]
    fn test_invalid_damage_multiplier() {
        let rule = DamageRule {
            id: "bad_rule".to_string(),
            hazard_profile_id: "hub_gas_v1".to_string(),
            target_entity_type: "marine".to_string(),
            damage_multiplier: -0.5,
            is_immune: false,
        };
        assert!(rule.validate().is_err());
    }
}
