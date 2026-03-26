//! SPDF Core: types, container I/O, and document object model.
//!
//! This crate provides the foundational types for the SPDF format:
//! - Element types (the 25 semantic elements)
//! - Container read/write (ZIP-based .spdf files)
//! - Manifest generation and validation
//! - Document state machine (DRAFT -> REVIEW -> SIGNED -> CERTIFIED)

pub mod container;
pub mod diff;
pub mod dom;
pub mod error;
pub mod manifest;
pub mod redaction;
pub mod signing;
pub mod types;

pub use error::SpdfError;
pub use types::*;
