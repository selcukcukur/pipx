use crate::{
    AsyncPipelineDestination,
    AsyncPipelineFuture,
    AsyncPipelineStep,
    PipelineError
};

/// Asynchronous pipeline continuation.
///
/// `AsyncNext` represents the remaining asynchronous execution chain after the
/// current step. A step can call [`AsyncNext::handle`] to continue the pipeline,
/// return early to stop execution, wrap the downstream result, or return an
/// error.
///
/// **Generics**
/// - `TPassable` - The value being passed through the pipeline.
/// - `TError` - The error type returned by asynchronous pipeline steps.
pub struct AsyncNext<'a, TPassable, TError = PipelineError> {
    /// The remaining asynchronous pipeline steps.
    pipes: &'a [AsyncPipelineStep<TPassable, TError>],

    /// The destination executed after all steps have completed.
    destination: &'a AsyncPipelineDestination<'a, TPassable, TError>,
}

impl<'a, TPassable, TError> AsyncNext<'a, TPassable, TError>
where
    TPassable: Send + 'a,
    TError: Send + 'a,
{
    /// Creates an asynchronous pipeline continuation.
    ///
    /// **Parameters**
    /// - `pipes` - The remaining asynchronous pipeline steps.
    /// - `destination` - The final callback executed after all steps have completed.
    ///
    /// **Returns**
    /// - [`AsyncNext`] - A continuation for the remaining asynchronous pipeline execution.
    pub(crate) fn new(
        pipes: &'a [AsyncPipelineStep<TPassable, TError>],
        destination: &'a AsyncPipelineDestination<'a, TPassable, TError>,
    ) -> Self {
        Self { pipes, destination }
    }

    /// Continues the asynchronous pipeline execution.
    ///
    /// **Parameters**
    /// - `passable` - The value that should be passed to the next asynchronous step.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The value produced after the remaining asynchronous pipeline steps have completed successfully.
    /// - `Err(TError)` - The asynchronous pipeline execution failed before reaching the destination.
    pub fn handle(self, passable: TPassable) -> AsyncPipelineFuture<'a, TPassable, TError> {
        Box::pin(async move {
            if let Some((pipe, rest)) = self.pipes.split_first() {
                let next = AsyncNext::new(rest, self.destination);
                pipe.handle(passable, next).await
            } else {
                (self.destination)(passable).await
            }
        })
    }
}