//! Raw UEFI Protocol Definitions
//!
//! This module contains the raw FFI definitions for UEFI protocols.
//! These are low-level structures matching the UEFI specification exactly.

pub mod gop;
pub mod file;
pub mod block;
pub mod pci;
pub mod serial;
pub mod loaded_image;
pub mod device_path;
pub mod rng;

// Re-export commonly used protocols
pub use gop::*;
pub use file::*;
pub use block::*;
pub use pci::*;
pub use serial::*;
pub use loaded_image::*;
pub use device_path::*;
pub use rng::*;
