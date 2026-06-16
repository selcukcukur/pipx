use crate::{PipelineError, PipelineResult, PipelineStep};

/// Pipeline continuation.
///
/// `Next` represents the remaining execution chain after the current step.
/// A step can call [`Next::handle`] to continue the pipeline, return early to
/// stop execution, wrap the downstream result, or return an error.
///
/// **Generics**
/// - `TPassable` - The value being passed through the pipeline.
/// - `TError` - The error type returned by pipeline steps.
pub struct Next<'a, TPassable, TError = PipelineError> {
    /// The remaining pipeline steps.
    pipes: &'a [PipelineStep<TPassable, TError>],

    /// The destination executed after all steps have completed.
    destination: &'a dyn Fn(TPassable) -> PipelineResult<TPassable, TError>,
}

impl<'a, TPassable, TError> Next<'a, TPassable, TError> {
    /// Creates a pipeline continuation.
    ///
    /// **Parameters**
    /// - `pipes` - The remaining pipeline steps.
    /// - `destination` - The final callback executed after all steps have completed.
    ///
    /// **Returns**
    /// - [`Next`] - A continuation for the remaining pipeline execution.
    pub(crate) fn new(
        pipes: &'a [PipelineStep<TPassable, TError>],
        destination: &'a dyn Fn(TPassable) -> PipelineResult<TPassable, TError>,
    ) -> Self {
        Self { pipes, destination }
    }

    /// Continues the pipeline execution.
    ///
    /// **Parameters**
    /// - `passable` - The value that should be passed to the next pipeline step.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The value produced after the remaining pipeline steps have completed successfully.
    /// - `Err(TError)` - The pipeline execution failed before reaching the destination.
    pub fn handle(self, passable: TPassable) -> PipelineResult<TPassable, TError> {
        if let Some((pipe, rest)) = self.pipes.split_first() {
            let next = Next::new(rest, self.destination);
            pipe.handle(passable, next)
        } else {
            (self.destination)(passable)
        }
    }
}
