use crate::{PipelineError, PipelineResult, PipelineStep};

/// The continuation object passed to middleware pipes.
///
/// `Next` is inspired by Laravel's pipeline middleware flow. A middleware may call
/// [`Next::handle`] to continue, return early to short-circuit, or modify the
/// returned value after the rest of the stack has completed.
///
/// **Generics**
/// - `TPassable` - The type of the value flowing through the middleware pipeline.
/// - `TError` - The error type returned when any pipe fails.
pub struct Next<'a, TPassable, TError = PipelineError> {
    pipes: &'a [PipelineStep<TPassable, TError>],
    destination: &'a dyn Fn(TPassable) -> PipelineResult<TPassable, TError>,
}

impl<'a, TPassable, TError> Next<'a, TPassable, TError> {
    /// Creates a continuation for the remaining middleware stack.
    ///
    /// **Parameters**
    /// - `pipes` - The remaining middleware pipes to execute.
    /// - `destination` - The final closure called after all middleware has run.
    ///
    /// **Returns**
    /// - A `Next` value that can continue the middleware chain.
    pub(crate) fn new(
        pipes: &'a [PipelineStep<TPassable, TError>],
        destination: &'a dyn Fn(TPassable) -> PipelineResult<TPassable, TError>,
    ) -> Self {
        Self { pipes, destination }
    }

    /// Continues the middleware chain with the given passable value.
    ///
    /// **Parameters**
    /// - `passable` - The value that should be passed to the next middleware.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The remaining chain completed successfully.
    /// - `Err(TError)` - A later middleware or destination failed.
    pub fn handle(self, passable: TPassable) -> PipelineResult<TPassable, TError> {
        if let Some((pipe, rest)) = self.pipes.split_first() {
            let next = Next::new(rest, self.destination);
            pipe.handle(passable, next)
        } else {
            (self.destination)(passable)
        }
    }
}
