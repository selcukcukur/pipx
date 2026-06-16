use crate::{Next, PipelineError, PipelineResult};

/// Defines a pipeline step.
///
/// A pipe receives the current value and a [`Next`] continuation.
/// It may continue the pipeline, stop execution early, wrap the downstream
/// result, or return an error.
///
/// **Generics**
/// - `TPassable` - The value processed by the pipeline.
/// - `TError` - The error type returned by the pipe.
pub trait Pipe<TPassable, TError = PipelineError> {
    /// Handles the current pipeline value.
    ///
    /// **Parameters**
    /// - `passable` - The current value being passed through the pipeline.
    /// - `next` - The continuation for the remaining pipeline steps.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The value produced by the pipe or the remaining pipeline steps.
    /// - `Err(TError)` - The pipe failed and stopped pipeline execution.
    fn handle(
        &self,
        passable: TPassable,
        next: Next<'_, TPassable, TError>,
    ) -> PipelineResult<TPassable, TError>;
}