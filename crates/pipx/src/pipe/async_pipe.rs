use crate::{AsyncNext, PipelineError, PipelineResult};
use async_trait::async_trait;

/// Defines an asynchronous pipeline step.
///
/// An async pipe receives the current value and an [`AsyncNext`] continuation.
/// It may continue the pipeline, stop execution early, wrap the downstream
/// result, or return an error.
///
/// **Generics**
/// - `TPassable` - The value processed by the pipeline.
/// - `TError` - The error type returned by the pipe.
#[async_trait]
pub trait AsyncPipe<TPassable, TError = PipelineError>
where
    TPassable: Send,
    TError: Send,
{
    /// Handles the current pipeline value asynchronously.
    ///
    /// **Parameters**
    /// - `passable` - The current value being passed through the pipeline.
    /// - `next` - The continuation for the remaining asynchronous pipeline steps.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The value produced by the pipe or the remaining asynchronous pipeline steps.
    /// - `Err(TError)` - The pipe failed and stopped pipeline execution.
    async fn handle(
        &self,
        passable: TPassable,
        next: AsyncNext<'_, TPassable, TError>,
    ) -> PipelineResult<TPassable, TError>;
}
