use crate::{AsyncNext, AsyncPipelineStep, PipelineError, PipelineResult};

pub struct AsyncPipeline<TPassable, TError = PipelineError> {
    passable: Option<TPassable>,
    pipes: Vec<AsyncPipelineStep<TPassable, TError>>,
}

impl<TPassable, TError> Default for AsyncPipeline<TPassable, TError> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TPassable, TError> AsyncPipeline<TPassable, TError> {
    /// Creates a new asynchronous middleware pipeline.
    pub fn new() -> Self {
        Self {
            passable: None,
            pipes: Vec::new(),
        }
    }

    /// Provides the initial passable value to the async middleware pipeline.
    pub fn send(mut self, passable: TPassable) -> Self {
        self.passable = Some(passable);
        self
    }

    /// Adds asynchronous middleware pipes to the pipeline.
    pub fn through(mut self, pipes: Vec<AsyncPipelineStep<TPassable, TError>>) -> Self {
        self.pipes.extend(pipes);
        self
    }
}

impl<TPassable, TError> AsyncPipeline<TPassable, TError>
where
    TPassable: Send,
    TError: Into<PipelineError> + Send,
{
    /// Finalizes the asynchronous middleware pipeline and returns the processed output.
    pub async fn then_return(self) -> PipelineResult<TPassable> {
        let passable = self.passable.ok_or(PipelineError::InputMissing)?;
        let destination = |passable| {
            Box::pin(async move { Ok(passable) })
                as crate::types::AsyncPipeFuture<'_, TPassable, TError>
        };
        let next = AsyncNext::new(&self.pipes, &destination);
        next.handle(passable).await.map_err(Into::into)
    }

    /// Executes async middleware and applies a final destination closure.
    pub async fn then<F, R>(self, destination: F) -> PipelineResult<R>
    where
        F: FnOnce(TPassable) -> R,
    {
        self.then_return().await.map(destination)
    }
}
