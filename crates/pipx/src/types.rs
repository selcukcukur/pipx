#[cfg(feature = "async")]
use crate::AsyncPipe;
use crate::{Pipe, PipelineError};
#[cfg(feature = "async")]
use std::pin::Pin;
use std::sync::Arc;

/// Shared pipeline execution result.
///
/// A pipeline result represents either a successfully processed value
/// or an error returned by a pipeline step during execution.
///
/// **Generics**
/// - `TPassable` - The successful value produced by the pipeline.
/// - `TError` - The error type returned when execution fails.
pub type PipelineResult<TPassable, TError = PipelineError> = Result<TPassable, TError>;

/// Shared pipeline step definition.
///
/// A pipeline step receives the current passable value and may either
/// continue execution, modify the value, stop the chain, or return an error.
///
/// **Generics**
/// - `TPassable` - The value flowing through the pipeline.
/// - `TError` - The error type returned when the step fails.
pub type PipelineStep<TPassable, TError = PipelineError> =
    Arc<dyn Pipe<TPassable, TError> + Send + Sync>;

/// Shared asynchronous pipeline step definition.
///
/// An asynchronous pipeline step receives the current passable value and may
/// either continue execution, modify the value, stop the chain, or return an
/// error after awaiting asynchronous work.
///
/// **Generics**
/// - `TPassable` - The value flowing through the pipeline.
/// - `TError` - The error type returned when the step fails.
#[cfg(feature = "async")]
pub type AsyncPipelineStep<TPassable, TError = PipelineError> =
    Arc<dyn AsyncPipe<TPassable, TError> + Send + Sync>;

/// Boxed future returned by asynchronous pipeline operations.
///
/// **Generics**
/// - `TPassable` - The value produced by the asynchronous operation.
/// - `TError` - The error type returned when the operation fails.
#[cfg(feature = "async")]
pub type AsyncPipelineFuture<'a, TPassable, TError = PipelineError> =
    Pin<Box<dyn Future<Output = PipelineResult<TPassable, TError>> + Send + 'a>>;

/// Final callback used by asynchronous pipeline continuations.
///
/// **Generics**
/// - `TPassable` - The value passed to the destination callback.
/// - `TError` - The error type returned when the destination fails.
#[cfg(feature = "async")]
pub type AsyncPipelineDestination<'a, TPassable, TError = PipelineError> =
    dyn Fn(TPassable) -> AsyncPipelineFuture<'a, TPassable, TError> + Sync + 'a;


pub type FinallyCallback<TPassable> = Box<dyn Fn(&PipelineResult<TPassable>) + Send + Sync>;
