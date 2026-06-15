use std::pin::Pin;
use std::sync::Arc;

use crate::{AsyncPipe, errors::PipelineError, pipe::Pipe};

/// Result type shared by pipeline execution and pipeline steps.
///
/// `TPassable` is the value flowing through the pipeline.
/// `TError` is the concrete error type returned by a step.
///
/// Public pipeline execution uses [`PipelineError`] by default.
pub type PipelineResult<TPassable, TError = PipelineError> = Result<TPassable, TError>;

/// A thread-safe, shareable pipe unit.
///
/// **Generics**
/// - `TPassable` - The type of the value flowing through the pipeline.
/// - `TError` - The error type returned when the pipe fails.
pub type PipeType<TPassable, TError = PipelineError> =
    Arc<dyn Pipe<TPassable, TError> + Send + Sync>;

/// Callback registered through `.finally(...)`.
///
/// Runs after pipeline execution completes, regardless of success or failure,
/// and receives the final pipeline result.
pub type FinallyCallback<TPassable> = Box<dyn Fn(&PipelineResult<TPassable>) + Send + Sync>;

/// A thread-safe, shareable asynchronous middleware unit.
#[cfg(feature = "async")]
pub type AsyncPipeType<TPassable, TError = PipelineError> =
    Arc<dyn AsyncPipe<TPassable, TError> + Send + Sync>;

/// Boxed future returned by async pipe operations.
#[cfg(feature = "async")]
pub type AsyncPipeFuture<'a, TPassable, TError = PipelineError> =
    Pin<Box<dyn Future<Output = PipelineResult<TPassable, TError>> + Send + 'a>>;

/// Destination callback used by asynchronous middleware continuations.
#[cfg(feature = "async")]
pub type AsyncDestination<'a, TPassable, TError = PipelineError> =
    dyn Fn(TPassable) -> AsyncPipeFuture<'a, TPassable, TError> + Sync + 'a;
