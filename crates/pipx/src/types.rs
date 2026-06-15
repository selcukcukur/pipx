use std::pin::Pin;
use std::sync::Arc;

use crate::{pipe::Pipe, errors::PipelineError, AsyncPipe};

/// Result type used by pipelines and pipeline steps.
///
/// This alias is intentionally shared by both pipeline execution and individual
/// pipe implementations. The first generic parameter is the successful output
/// value, while the second generic parameter is the concrete error type.
///
/// By default, public pipeline execution returns [`PipelineError`].
pub type PipelineResult<TPassable, TError = PipelineError> = Result<TPassable, TError>;

/// A thread-safe, shareable pipe unit.
///
/// **Generics**
/// - `TPassable` - The type of the value flowing through the pipeline.
/// - `TError` - The error type returned when the pipe fails.
pub type PipeType<TPassable, TError = PipelineError> = Arc<dyn Pipe<TPassable, TError> + Send + Sync>;

/// Boxed finalizer callback used by pipeline implementations.
pub type Finalizer<TPassable> = Box<dyn Fn(&PipelineResult<TPassable>) + Send + Sync>;

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
