use crate::{Next, PipelineError, PipelineResult};

/// A middleware pipe that can decide whether and when to call the next step.
pub trait Pipe<TPassable, TError = PipelineError> {
    /// Handles a passable value and optionally continues the middleware chain.
    ///
    /// **Parameters**
    /// - `passable` - The current value flowing through the pipeline.
    /// - `next` - The continuation for the remaining middleware stack.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The middleware completed successfully.
    /// - `Err(TError)` - The middleware failed and stopped execution.
    fn handle(
        &self,
        passable: TPassable,
        next: Next<'_, TPassable, TError>,
    ) -> PipelineResult<TPassable, TError>;
}
