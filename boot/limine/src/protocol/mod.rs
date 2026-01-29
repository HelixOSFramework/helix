//! # Limine Protocol Definitions
//!
//! This module contains the raw protocol definitions for Limine boot protocol.
//! These are low-level FFI structures that match the C ABI exactly.
//!
//! ## Module Structure
//!
//! - [`magic`]: Magic numbers and revision constants
//! - [`raw`]: Raw C-compatible structure definitions
//! - [`request_ids`]: Request identifier arrays
//! - [`macros`]: Helper macros for request generation

pub mod magic;
pub mod raw;
pub mod request_ids;

// Re-exports
pub use magic::*;
pub use raw::*;
pub use request_ids::*;
