use std::pin::Pin;
use crate::{AsyncDestination, AsyncPipeType, PipelineError, PipelineResult};

/// The asynchronous continuation object passed to async middleware pipes.
pub struct AsyncNext<'a, TPassable, TError = PipelineError> {
    pipes: &'a [AsyncPipeType<TPassable, TError>],
    destination: &'a AsyncDestination<'a, TPassable, TError>,
}

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
