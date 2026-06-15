use crate::{FinallyCallback, Next, PipelineStep, PipelineError, PipelineResult};

/// A Laravel-inspired middleware pipeline.
///
/// Each [`Pipe`](crate::Pipe) receives the current value and a [`Next`]
/// continuation. Middleware can call `next.handle(passable)` to continue,
/// return early to stop the chain, or wrap the downstream result.
///
/// **Generics**
/// - `TPassable` - The type of the value that flows through the pipeline.
/// - `TError` - The error type returned by middleware pipes.
pub struct Pipeline<TPassable, TError = PipelineError> {
    passable: Option<TPassable>,
    pipes: Vec<PipelineStep<TPassable, TError>>,
    finally: Option<FinallyCallback<TPassable>>,
}

impl<TPassable, TError> Default for Pipeline<TPassable, TError> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TPassable, TError> Pipeline<TPassable, TError> {
    /// Create a new empty pipeline.
    pub fn new() -> Self {
        Self {
            passable: None,
            pipes: Vec::new(),
            finally: None,
        }
    }

    /// Provides the initial passable value to the middleware pipeline.
    ///
    /// **Parameters**
    /// - `passable` - The initial value that will flow through the middleware chain.
    ///
    /// **Returns**
    /// - The pipeline instance with the initial passable value set.
    pub fn send(mut self, passable: TPassable) -> Self {
        self.passable = Some(passable);
        self
    }

    /// Adds a sequence of middleware pipes to the pipeline.
    ///
    /// **Parameters**
    /// - `pipes` - Middleware pipes executed in the order they are provided.
    ///
    /// **Returns**
    /// - The pipeline instance with the provided middleware appended.
    pub fn through(mut self, pipes: Vec<PipelineStep<TPassable, TError>>) -> Self {
        self.pipes.extend(pipes);
        self
    }

    /// Adds a middleware pipe when the condition is `true`.
    ///
    /// **Parameters**
    /// - `condition` - Controls whether the pipe is appended.
    /// - `pipe` - The middleware pipe to append when the condition matches.
    pub fn when(mut self, condition: bool, pipe: PipelineStep<TPassable, TError>) -> Self {
        if condition {
            self.pipes.push(pipe);
        }
        self
    }

    /// Adds the pipe when `condition` is `false`.
    ///
    /// **Parameters**
    /// - `condition` - Controls whether the pipe is skipped.
    /// - `pipe` - The middleware pipe to append when the condition is false.
    pub fn unless(mut self, condition: bool, pipe: PipelineStep<TPassable, TError>) -> Self {
        if !condition {
            self.pipes.push(pipe);
        }
        self
    }

    /// Set a final callback to be executed after the pipeline ends regardless of the outcome.
    ///
    /// **Parameters**
    /// - `callback` - A closure that receives the final pipeline result.
    pub fn finally<F>(mut self, callback: F) -> Self
    where
        F: Fn(&PipelineResult<TPassable>) + Send + Sync + 'static,
    {
        self.finally = Some(Box::new(callback));
        self
    }
}

impl<TPassable, TError> Pipeline<TPassable, TError>
where
    TError: Into<PipelineError>,
{
    /// Run the pipeline with a final destination callback.
    ///
    /// **Parameters**
    /// - `destination` - A closure that maps the post-middleware value into `R`.
    ///
    /// **Returns**
    /// - `Ok(R)` - The middleware chain completed and the destination ran.
    /// - `Err(PipelineError)` - Input was missing or middleware failed.
    pub fn then<F, R>(self, destination: F) -> PipelineResult<R>
    where
        F: FnOnce(TPassable) -> R,
    {
        self.run(|passable| Ok(passable)).map(destination)
    }

    /// Run the pipeline and return the result.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The middleware chain completed successfully.
    /// - `Err(PipelineError)` - Input was missing or middleware failed.
    pub fn then_return(self) -> PipelineResult<TPassable> {
        self.run(|passable| Ok(passable))
    }

    /// Intercepts middleware errors and allows recovery with a fallback value.
    ///
    /// **Parameters**
    /// - `recovery` - A closure that maps [`PipelineError`] into `TPassable`.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The successful or recovered pipeline value.
    /// - `Err(PipelineError)` - Input was missing before any recovery was possible.
    pub fn rescue<F>(self, recovery: F) -> PipelineResult<TPassable>
    where
        F: FnOnce(PipelineError) -> TPassable,
    {
        match self.then_return() {
            | Ok(passable) => Ok(passable),
            | Err(PipelineError::InputMissing) => Err(PipelineError::InputMissing),
            | Err(err) => Ok(recovery(err)),
        }
    }

    fn run<F>(self, destination: F) -> PipelineResult<TPassable>
    where
        F: Fn(TPassable) -> PipelineResult<TPassable, TError>,
    {
        let passable = self.passable.ok_or(PipelineError::InputMissing)?;

        // Build the first continuation from the whole middleware slice. Every
        // middleware receives a shorter slice through `Next`, so execution stays
        // stack-safe for typical pipeline sizes without heap-building closures.
        let next = Next::new(&self.pipes, &destination);
        let result = next.handle(passable).map_err(Into::into);

        if let Some(finally) = &self.finally {
            finally(&result);
        }

        result
    }
}
