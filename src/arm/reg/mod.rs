//! `Register` structs for various ARM architectures.

/// `RegId` definitions for ARM architectures.
pub mod id;

mod arm64_core;
mod arm_core;

pub use arm64_core::Aarch64CoreRegs;
pub use arm_core::ArmCoreRegs;
