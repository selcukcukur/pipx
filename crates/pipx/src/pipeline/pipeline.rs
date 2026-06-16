use crate::{FinallyCallback, Next, Pipe, PipelineError, PipelineResult, PipelineStep};
use std::sync::Arc;

/// Composable pipeline executor.
///
/// A pipeline stores an optional input value, an ordered list of steps,
/// and an optional callback that runs after execution finishes.
///
/// Each [`Pipe`] receives the current value and a [`Next`] continuation.
/// A step may continue the chain, stop execution early, wrap the downstream
/// result, or return an error.
///
/// **Generics**
/// - `TPassable` - The value processed by the pipeline.
/// - `TError` - The error type returned by pipeline steps.
pub struct Pipeline<TPassable, TError = PipelineError> {
    /// The value being passed through the pipeline.
    passable: Option<TPassable>,

    /// The ordered steps that make up the pipeline.
    pipes: Vec<PipelineStep<TPassable, TError>>,

    /// The callback executed after the pipeline finishes.
    finally: Option<FinallyCallback<TPassable>>,
}

impl<TPassable, TError> Pipeline<TPassable, TError> {
    /// Creates an empty pipeline.
    ///
    /// **Returns**
    /// - [`Pipeline`] - A pipeline instance without an initial value or steps.
    pub fn new() -> Self {
        Self {
            passable: None,
            pipes: Vec::new(),
            finally: None,
        }
    }

    /// Provides the initial value to the pipeline.
    ///
    /// **Parameters**
    /// - `passable` - The value that should be processed by the pipeline.
    ///
    /// **Returns**
    /// - [`Pipeline`] - The pipeline instance with the provided value set.
    pub fn send(mut self, passable: TPassable) -> Self {
        self.passable = Some(passable);
        self
    }

    /// Adds a pipeline step to the execution chain.
    ///
    /// **Parameters**
    /// - `pipe` - The step that should be executed as part of the pipeline.
    ///
    /// **Returns**
    /// - [`Pipeline`] - The pipeline instance with the provided step appended.
    pub fn pipe<TPipe>(mut self, pipe: TPipe) -> Self
    where
        TPipe: Pipe<TPassable, TError> + Send + Sync + 'static,
    {
        self.pipes.push(Arc::new(pipe));
        self
    }

    /// Sets the pipeline steps.
    ///
    /// **Parameters**
    /// - `pipes` - The steps that should define the pipeline execution chain.
    ///
    /// **Returns**
    /// - [`Pipeline`] - The pipeline instance with the provided steps set.
    pub fn through(mut self, pipes: Vec<PipelineStep<TPassable, TError>>) -> Self {
        self.pipes = pipes;
        self
    }

    /// Adds a pipeline step when the condition is `true`.
    ///
    /// **Parameters**
    /// - `condition` - Determines whether the step should be appended.
    /// - `pipe` - The step that should be executed as part of the pipeline.
    ///
    /// **Returns**
    /// - [`Pipeline`] - The pipeline instance with the provided step conditionally appended.
    pub fn when<P>(mut self, condition: bool, pipe: P) -> Self
    where
        P: Pipe<TPassable, TError> + Send + Sync + 'static,
    {
        if condition {
            self.pipes.push(Arc::new(pipe));
        }

        self
    }

    /// Adds a pipeline step when the condition is `false`.
    ///
    /// **Parameters**
    /// - `condition` - Determines whether the step should be skipped.
    /// - `pipe` - The step that should be executed as part of the pipeline.
    ///
    /// **Returns**
    /// - [`Pipeline`] - The pipeline instance with the provided step conditionally appended.
    pub fn unless<P>(mut self, condition: bool, pipe: P) -> Self
    where
        P: Pipe<TPassable, TError> + Send + Sync + 'static,
    {
        if !condition {
            self.pipes.push(Arc::new(pipe));
        }

        self
    }

    /// Sets a final callback to be executed after the pipeline finishes.
    ///
    /// **Parameters**
    /// - `callback` - A closure that receives the final pipeline result.
    ///
    /// **Returns**
    /// - [`Pipeline`] - The pipeline instance with the final callback registered.
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
    /// Executes the pipeline and applies a final transformation.
    ///
    /// Pipe errors are converted into [`PipelineError`] before the result is
    /// returned to the caller.
    ///
    /// **Parameters**
    /// - `destination` - The callback used to transform the final pipeline value.
    ///
    /// **Returns**
    /// - `Ok(R)` - The value returned by the destination callback after the pipeline has completed successfully.
    /// - `Err(PipelineError)` - The pipeline execution could not be completed.
    pub fn then<F, R>(self, destination: F) -> PipelineResult<R>
    where
        F: FnOnce(TPassable) -> R,
    {
        self.run(|passable| Ok(passable)).map(destination)
    }

    /// Executes the pipeline and returns the final value.
    ///
    /// Pipe errors are converted into [`PipelineError`] before the result is
    /// returned to the caller.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The value produced after all pipeline steps have been executed successfully.
    /// - `Err(PipelineError)` - The pipeline execution could not be completed.
    pub fn then_return(self) -> PipelineResult<TPassable> {
        self.run(|passable| Ok(passable))
    }

    /// Recovers from pipeline errors using a fallback value.
    ///
    /// **Parameters**
    /// - `recovery` - The callback used to produce a fallback value from a [`PipelineError`].
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The value produced by the pipeline or the fallback value returned by the recovery callback.
    /// - `Err(PipelineError)` - The pipeline was executed without an initial value.
    pub fn rescue<F>(self, recovery: F) -> PipelineResult<TPassable>
    where
        F: FnOnce(PipelineError) -> TPassable,
    {
        match self.then_return() {
            | Ok(passable) => Ok(passable),

            // A pipeline cannot execute without an initial value.
            //
            // `InputMissing` indicates a pipeline configuration error rather than
            // a failure that occurred while processing a value. Recovery callbacks
            // are only intended to handle execution failures, so this error is
            // returned unchanged to the caller.
            | Err(PipelineError::InputMissing) => Err(PipelineError::InputMissing),

            | Err(err) => Ok(recovery(err)),
        }
    }

    /// Executes the configured pipeline steps.
    ///
    /// **Parameters**
    /// - `destination` - The final callback executed after all pipeline steps have completed.
    ///
    /// **Returns**
    /// - `Ok(TPassable)` - The value produced after all pipeline steps and the destination callback have completed successfully.
    /// - `Err(PipelineError)` - The pipeline execution could not be completed.
    fn run<F>(self, destination: F) -> PipelineResult<TPassable>
    where
        F: Fn(TPassable) -> PipelineResult<TPassable, TError>,
    {
        // A pipeline must have an initial value before execution can start.
        let passable = self.passable.ok_or(PipelineError::InputMissing)?;

        // `Next` represents the remaining execution chain.
        //
        // It receives the full step list at the beginning. Each call to `handle`
        // creates a new `Next` with a shorter remaining slice, so every step only
        // sees the part of the pipeline that comes after it.
        let next = Next::new(&self.pipes, &destination);

        // Step errors use the concrete `TError` type internally.
        //
        // Public pipeline execution returns `PipelineError`, so the step error is
        // normalized before leaving the pipeline boundary.
        let result = next.handle(passable).map_err(Into::into);

        // The final callback always runs after execution completes, whether the
        // pipeline completed successfully or returned an error.
        if let Some(finally) = &self.finally {
            finally(&result);
        }

        result
    }
}

impl<TPassable, TError> Default for Pipeline<TPassable, TError> {
    fn default() -> Self {
        Self::new()
    }
}
