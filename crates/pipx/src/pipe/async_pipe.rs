use std::future::Future;
use std::pin::Pin;

use crate::{AsyncDestination, AsyncPipeType, PipelineError, PipelineResult};
use crate::pipe::AsyncNext;

/// An asynchronous middleware pipe that can decide whether to call the next step.
pub trait AsyncPipe<TPassable, TError = PipelineError> {
    /// Handles a passable value and optionally continues the async chain.
    fn handle<'a>(
        &'a self,
        passable: TPassable,
        next: AsyncNext<'a, TPassable, TError>,
    ) -> Pin<Box<dyn Future<Output = PipelineResult<TPassable, TError>> + Send + 'a>>;
}
