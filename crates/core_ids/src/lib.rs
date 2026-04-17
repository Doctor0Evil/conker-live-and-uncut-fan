//! Core ID enums for Uncut Multiplayer
//! 
//! This crate provides strongly-typed enums for all game IDs:
//! - SfxId: Sound effects (SFX_XXX)
//! - VfxId: Visual effects (VFX_XXX)
//! - PrtId: Particle decals (PRT_XXX)
//! - EnvId: Environmental hazards (ENV_XXX)
//! - HudId: HUD elements (HUD_XXX)
//! - SeqId: Cinematic sequences (SEQ_XXX)
//! - Asid: Animation states (ASID_XXX)
//! - ObjId: Game objects (OBJ_XXX)
//!
//! All enums implement TryFrom<&str> and Display for string conversion.

use strum_macros::{Display, EnumString, EnumIter};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdParseError {
    #[error("Invalid ID format: {0}")]
    InvalidFormat(String),
    #[error("Unknown ID value: {0}")]
    UnknownValue(String),
}

// ============================================================================
// SFX IDs
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SfxId {
    #[strum(serialize = "SFX_150")]
    Sfx150_WpnFlamerPilot,
    #[strum(serialize = "SFX_151")]
    Sfx151_WpnFlamerLoop,
    #[strum(serialize = "SFX_152")]
    Sfx152_WpnChainsawIdleRev,
    #[strum(serialize = "SFX_153")]
    Sfx153_WpnChainsawGoreImpact,
    #[strum(serialize = "SFX_301")]
    Sfx301_HazAirlockDoorClose,
    #[strum(serialize = "SFX_302")]
    Sfx302_HazGasChamberFill,
    #[strum(serialize = "SFX_303")]
    Sfx303_HazElectricSparkBurst,
    #[strum(serialize = "SFX_304")]
    Sfx304_EnvEggPulseLoop,
    #[strum(serialize = "SFX_501")]
    Sfx501_VoShcPainShort01,
    #[strum(serialize = "SFX_502")]
    Sfx502_VoShcDeath01,
    #[strum(serialize = "SFX_503")]
    Sfx503_VoShcTaunt01,
    #[strum(serialize = "SFX_667")]
    Sfx667_VoConkPain01,
    #[strum(serialize = "SFX_668")]
    Sfx668_VoConkDeath01,
    #[strum(serialize = "SFX_701")]
    Sfx701_VoAlienScreech01,
    #[strum(serialize = "SFX_702")]
    Sfx702_VoAlienDeath01,
    #[strum(serialize = "SFX_901")]
    Sfx901_VoFrenchieYell01,
    #[strum(serialize = "SFX_902")]
    Sfx902_VoFrenchieDeath01,
}

// ============================================================================
// VFX IDs
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VfxId {
    #[strum(serialize = "VFX_012")]
    Vfx012_PrtGoreLimeSpurt,
    #[strum(serialize = "VFX_013")]
    Vfx013_PrtGoreBrainMatter,
    #[strum(serialize = "VFX_014")]
    Vfx014_PrtGoreLimbDetach,
    #[strum(serialize = "VFX_015")]
    Vfx015_PrtGoreBoneChip,
    #[strum(serialize = "VFX_016")]
    Vfx016_PrtGoreHeavySlice,
    #[strum(serialize = "VFX_017")]
    Vfx017_PrtGoreExplosiveDismember,
    #[strum(serialize = "VFX_018")]
    Vfx018_PrtGoreBoneShard,
    #[strum(serialize = "VFX_020")]
    Vfx020_SparkMetalImpact,
    #[strum(serialize = "VFX_080")]
    Vfx080_HazSteamVent,
    #[strum(serialize = "VFX_110")]
    Vfx110_WfxMuzzleFlamethrower,
    #[strum(serialize = "VFX_111")]
    Vfx111_WpnImpactAcidSplash,
    #[strum(serialize = "VFX_112")]
    Vfx112_WpnMuzzleKatanaSwing,
    #[strum(serialize = "VFX_113")]
    Vfx113_WpnShellCasingEject,
}

// ============================================================================
// PRT IDs (Decals)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrtId {
    #[strum(serialize = "PRT_101")]
    Prt101_DecalBloodPoolLime,
    #[strum(serialize = "PRT_102")]
    Prt102_DecalDeepCutSlice,
    #[strum(serialize = "PRT_103")]
    Prt103_DecalScorchMark,
}

// ============================================================================
// ENV IDs
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnvId {
    #[strum(serialize = "ENV_050")]
    Env050_HazAirlockDoorClose,
    #[strum(serialize = "ENV_051")]
    Env051_HazGasChamberFill,
    #[strum(serialize = "ENV_052")]
    Env052_HazElectricSpark,
    #[strum(serialize = "ENV_304")]
    Env304_EnvEggPulse,
}

// ============================================================================
// HUD IDs
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HudId {
    #[strum(serialize = "HUD_001")]
    Hud001_ReticlePistol,
    #[strum(serialize = "HUD_002")]
    Hud002_ReticleShotgun,
    #[strum(serialize = "HUD_003")]
    Hud003_ReticleChaingun,
    #[strum(serialize = "HUD_010")]
    Hud010_CounterHealth,
    #[strum(serialize = "HUD_011")]
    Hud011_CounterArmor,
    #[strum(serialize = "HUD_012")]
    Hud012_CounterAmmo,
}

// ============================================================================
// SEQ IDs
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeqId {
    #[strum(serialize = "SEQ_001")]
    Seq001_SeqMatchStart,
    #[strum(serialize = "SEQ_002")]
    Seq002_SeqSuddenDeath,
    #[strum(serialize = "SEQ_003")]
    Seq003_SeqInstantWinGas,
    #[strum(serialize = "SEQ_004")]
    Seq004_SeqPostMatch,
}

// ============================================================================
// ASID (Animation State IDs)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Asid {
    #[strum(serialize = "ASID_050")]
    Asid050_HeavyCarry,
    #[strum(serialize = "ASID_401")]
    Asid401_FinChainsawH,
    #[strum(serialize = "ASID_406")]
    Asid406_FinKatanaStab,
    #[strum(serialize = "ASID_410")]
    Asid410_FinNeckSnap,
    #[strum(serialize = "ASID_411")]
    Asid411_FinAlienTailwhip,
    #[strum(serialize = "ASID_450")]
    Asid450_MovCrouchWalk,
    #[strum(serialize = "ASID_451")]
    Asid451_MovSlide,
    #[strum(serialize = "ASID_500")]
    Asid500_IdleDrunk,
    #[strum(serialize = "ASID_501")]
    Asid501_IdlePee,
    #[strum(serialize = "ASID_900")]
    Asid900_AlienPounce,
    #[strum(serialize = "ASID_901")]
    Asid901_AlienFacebite,
}

// ============================================================================
// OBJ IDs
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObjId {
    #[strum(serialize = "OBJ_010")]
    Obj010_WpnPickupChainsaw,
    #[strum(serialize = "OBJ_011")]
    Obj011_WpnPickupKatana,
    #[strum(serialize = "OBJ_012")]
    Obj012_WpnPickupFlamethrower,
    #[strum(serialize = "OBJ_013")]
    Obj013_WpnPickupChaingun,
    #[strum(serialize = "OBJ_014")]
    Obj014_WpnPickupBazooka,
    #[strum(serialize = "OBJ_020")]
    Obj020_AmmoPickupRocket,
    #[strum(serialize = "OBJ_021")]
    Obj021_AmmoPickupChaingun,
    #[strum(serialize = "OBJ_030")]
    Obj030_HealthPickupChocolate,
    #[strum(serialize = "OBJ_031")]
    Obj031_HealthPickupMedkit,
    #[strum(serialize = "OBJ_032")]
    Obj032_ArmorPickupVest,
    #[strum(serialize = "OBJ_040")]
    Obj040_InteractDoorSliding,
    #[strum(serialize = "OBJ_041")]
    Obj041_InteractTurretMg,
    #[strum(serialize = "OBJ_050")]
    Obj050_ObjAlienEgg,
    #[strum(serialize = "OBJ_051")]
    Obj051_ObjHeistVaultKeycard,
    #[strum(serialize = "OBJ_060")]
    Obj060_VehicleSpawnerBoat,
}

// ============================================================================
// Conversion traits
// ============================================================================

impl SfxId {
    pub fn from_str_id(s: &str) -> Result<Self, IdParseError> {
        s.parse().map_err(|_| IdParseError::UnknownValue(s.to_string()))
    }
}

impl VfxId {
    pub fn from_str_id(s: &str) -> Result<Self, IdParseError> {
        s.parse().map_err(|_| IdParseError::UnknownValue(s.to_string()))
    }
}

impl PrtId {
    pub fn from_str_id(s: &str) -> Result<Self, IdParseError> {
        s.parse().map_err(|_| IdParseError::UnknownValue(s.to_string()))
    }
}

impl EnvId {
    pub fn from_str_id(s: &str) -> Result<Self, IdParseError> {
        s.parse().map_err(|_| IdParseError::UnknownValue(s.to_string()))
    }
}

impl HudId {
    pub fn from_str_id(s: &str) -> Result<Self, IdParseError> {
        s.parse().map_err(|_| IdParseError::UnknownValue(s.to_string()))
    }
}

impl SeqId {
    pub fn from_str_id(s: &str) -> Result<Self, IdParseError> {
        s.parse().map_err(|_| IdParseError::UnknownValue(s.to_string()))
    }
}

impl Asid {
    pub fn from_str_id(s: &str) -> Result<Self, IdParseError> {
        s.parse().map_err(|_| IdParseError::UnknownValue(s.to_string()))
    }
}

impl ObjId {
    pub fn from_str_id(s: &str) -> Result<Self, IdParseError> {
        s.parse().map_err(|_| IdParseError::UnknownValue(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sfx_parse() {
        assert_eq!(SfxId::from_str_id("SFX_150").unwrap(), SfxId::Sfx150_WpnFlamerPilot);
        assert_eq!(SfxId::from_str_id("SFX_501").unwrap(), SfxId::Sfx501_VoShcPainShort01);
    }

    #[test]
    fn test_vfx_display() {
        assert_eq!(format!("{}", VfxId::Vfx012_PrtGoreLimeSpurt), "VFX_012");
        assert_eq!(format!("{}", VfxId::Vfx016_PrtGoreHeavySlice), "VFX_016");
    }

    #[test]
    fn test_asid_parse() {
        assert_eq!(Asid::from_str_id("ASID_401").unwrap(), Asid::Asid401_FinChainsawH);
        assert_eq!(Asid::from_str_id("ASID_050").unwrap(), Asid::Asid050_HeavyCarry);
    }

    #[test]
    fn test_obj_parse() {
        assert_eq!(ObjId::from_str_id("OBJ_010").unwrap(), ObjId::Obj010_WpnPickupChainsaw);
        assert_eq!(ObjId::from_str_id("OBJ_050").unwrap(), ObjId::Obj050_ObjAlienEgg);
    }
}
