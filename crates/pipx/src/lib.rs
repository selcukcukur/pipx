/// Error types and utilities.
pub mod errors;

/// Declarative macros for pipeline construction.
pub mod macros;

/// Pipeline step traits and continuations.
pub mod pipe;

/// Pipeline implementations and helpers.
pub mod pipeline;

/// Shared pipeline type aliases.
pub mod types;

/// Re-export error types and utilities.
pub use crate::errors::*;

/// Re-export pipeline step traits and continuations.
pub use crate::pipe::*;

/// Re-export pipeline implementations and helpers.
pub use crate::pipeline::*;

/// Re-export shared pipeline type aliases.
pub use crate::types::*;

#[cfg(feature = "macros")]
pub use pipx_macros::pipe;

/// Creates a pipeline with an initial value.
///
/// **Parameters**
/// - `passable` - The value that should be processed by the pipeline.
///
/// **Returns**
/// - [`Pipeline`] - A pipeline instance with the provided value set.
pub fn pipeline<TPassable>(passable: TPassable) -> Pipeline<TPassable> {
    Pipeline::new().send(passable)
}

/// Creates an asynchronous pipeline with an initial value.
///
/// **Parameters**
/// - `passable` - The value that should be processed by the asynchronous pipeline.
///
/// **Returns**
/// - [`AsyncPipeline`] - An asynchronous pipeline instance with the provided value set.
#[cfg(feature = "async")]
pub fn async_pipeline<TPassable>(passable: TPassable) -> AsyncPipeline<TPassable> {
    AsyncPipeline::new().send(passable)
}