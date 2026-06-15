#![doc = include_str!("../readme.md")]

//! Type-safe and composable pipelines for Rust.
//!
//! **pipx** provides a Laravel-inspired middleware pipeline model built around
//! explicit execution flow, predictable composition, and reusable pipeline steps.
//!
//! Core concepts:
//! - [`Pipeline`] for pipeline execution.
//! - [`Pipe`] for reusable middleware steps.
//! - [`Next`] for continuation-based flow control.
//! - [`PipelineResult`] for unified result handling.

/// Centralized error definitions and handling utilities.
pub mod errors;

/// Pipeline implementations and builders.
pub mod pipeline;

/// Shared result types and callback definitions.
pub mod types;

/// Core pipe traits and continuation types.
pub mod pipe;

pub use crate::errors::*;
pub use crate::pipe::*;
pub use crate::pipeline::*;
pub use crate::types::*;

#[cfg(feature = "macros")]
pub use pipx_macros::pipe;