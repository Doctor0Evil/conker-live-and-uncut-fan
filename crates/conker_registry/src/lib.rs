//! Canonical registry enums for Conker: Live & Uncut.
//!
//! This crate is the single source of truth for all registry IDs
//! (SFX, VFX, and later ASID, MAP, ZONE, ROLE, AI, SPN, GMR, RUL, ASP, BFL).
//!
//! Each enum derives `Serialize`, `Deserialize`, and `JsonSchema` so that:
//! - Rust code gets type safety.
//! - JSON configs use canonical string IDs (e.g. `"SFX_150"`, `"VFX_031"`).
//! - JSON Schema files can be generated automatically at build time.
//!
//! Initially, this minimal slice exposes only `SfxId` and `VfxId` to let the
//! workspace compile and validate schemas before the full registry is ported.

pub mod sfx;
pub mod vfx;

// Re-exports for ergonomic use in other crates:
//
//     use conker_registry::SfxId;
//     use conker_registry::VfxId;
//
pub use sfx::SfxId;
pub use vfx::VfxId;

// As you port additional registries, add modules and re-exports here:
//
// pub mod asid;
// pub mod map;
// pub mod zone;
// pub mod role;
// pub mod ai;
// pub mod spn;
// pub mod gmr;
// pub mod rul;
// pub mod asp;
// pub mod bfl;
//
// pub use asid::AsidId;
// pub use map::MapId;
// pub use zone::ZoneId;
// pub use role::RoleId;
// pub use ai::AiId;
// pub use spn::SpnId;
// pub use gmr::GmrId;
// pub use rul::RulId;
// pub use asp::AspId;
// pub use bfl::BflId;
