//! Type-safe middleware and transform pipelines for Rust.
//!
//! **pipx** provides two pipeline styles:
//! - [`Pipeline`] for Laravel-inspired middleware that can call `Next`.
//! - [`TransformPipeline`] for direct sequential value transformations.

/// Centralized error definitions and handling utilities.
pub mod errors;

pub mod pipeline;

/// Core trait and type definitions used across the crate.
pub mod types;

/// Internal helper functions used by pipeline implementations.
pub mod utility;

pub use crate::errors::*;
pub use crate::pipeline::*;
pub use crate::types::*;

#[cfg(feature = "macros")]
pub use pipx_macros::{pipe, transform_pipe};
