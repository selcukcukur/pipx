#[cfg(feature = "async")]
use std::future::Future;
#[cfg(feature = "async")]
use std::pin::Pin;

use crate::{AsyncDestination, AsyncPipeType, PipeType, PipelineError, PipelineResult};

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
    pipes: &'a [PipeType<TPassable, TError>],
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
        pipes: &'a [PipeType<TPassable, TError>],
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

/// An asynchronous middleware pipe that can decide whether to call the next step.
#[cfg(feature = "async")]
pub trait AsyncPipe<TPassable, TError = PipelineError> {
    /// Handles a passable value and optionally continues the async chain.
    fn handle<'a>(
        &'a self,
        passable: TPassable,
        next: AsyncNext<'a, TPassable, TError>,
    ) -> Pin<Box<dyn Future<Output = PipelineResult<TPassable, TError>> + Send + 'a>>;
}

/// The asynchronous continuation object passed to async middleware pipes.
#[cfg(feature = "async")]
pub struct AsyncNext<'a, TPassable, TError = PipelineError> {
    pipes: &'a [AsyncPipeType<TPassable, TError>],
    destination: &'a AsyncDestination<'a, TPassable, TError>,
}

#[cfg(feature = "async")]
impl<'a, TPassable, TError> AsyncNext<'a, TPassable, TError>
where
    TPassable: Send + 'a,
    TError: Send + 'a,
{
    /// Creates an asynchronous continuation for the remaining middleware stack.
    pub(crate) fn new(
        pipes: &'a [AsyncPipeType<TPassable, TError>],
        destination: &'a AsyncDestination<'a, TPassable, TError>,
    ) -> Self {
        Self { pipes, destination }
    }

    /// Continues the asynchronous middleware chain with the given passable value.
    pub fn handle(
        self,
        passable: TPassable,
    ) -> Pin<Box<dyn Future<Output = PipelineResult<TPassable, TError>> + Send + 'a>> {
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
