pub mod errors;
pub mod macros;
pub mod pipe;
pub mod pipeline;
pub mod types;

pub use crate::errors::*;
pub use crate::pipe::*;
pub use crate::pipeline::*;
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