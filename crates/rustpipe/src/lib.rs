pub mod error;
mod utility;

use std::any::type_name;
#[cfg(feature = "async")]
use std::future::Future;
#[cfg(feature = "async")]
use std::pin::Pin;
use crate::error::{PipelineError, PipelineResult, StepFailure};

pub trait Pipe<T, E> {
    fn handle(&self, passable: T) -> Result<T, E>;
}

pub struct Pipeline<T, E> {
    passable: Option<T>,
    pipes: Vec<Box<dyn Pipe<T, E>>>,
    taps: Vec<Box<dyn Fn(&T)>>
}

impl<T, E: std::fmt::Debug> Pipeline<T, E> where PipelineError: From<E> {
    /// Creates a new, empty pipeline instance.
    pub fn new() -> Self {
        Self {
            passable: None,
            pipes: Vec::new(),
            taps: Vec::new(),
        }
    }

    /// Provides the initial passable value to the pipeline.
    pub fn send(mut self, passable: T) -> Self {
        self.passable = Some(passable);
        self
    }

    /// Intercepts errors and allows recovery via a closure.
    pub fn rescue<F>(self, f: F) -> PipelineResult<T>
    where
        F: FnOnce(PipelineError) -> T,
    {
        let mut passable = self.passable.ok_or(PipelineError::InputMissing)?;
        for step in &self.pipes {
            match step.handle(passable) {
                Ok(val) => passable = val,
                Err(err) => {
                    return Err(utility::step_failure_from::<E, T>(err).into())
                }
            }
        }
        Ok(passable)
    }

    /// Observes the current pipeline passable without modifying it.
    pub fn tap<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) + 'static,
    {
        self.taps.push(Box::new(f));
        self
    }

    /// Adds a sequence of pipes to the pipeline.
    pub fn through(mut self, pipes: Vec<Box<dyn Pipe<T, E>>>) -> Self {
        for step in pipes {
            self.pipes.push(step);
        }
        self
    }

    /// Executes the pipeline and applies a final transformation closure to the result.
    pub fn then<F, R>(self, f: F) -> PipelineResult<R>
    where
        F: FnOnce(T) -> R,
    {
        let mut passable = self.passable.ok_or(PipelineError::InputMissing)?;
        for step in &self.pipes {
            match step.handle(passable) {
                Ok(val) => passable = val,
                Err(err) => {
                    return Err(utility::step_failure_from::<E, T>(err).into())
                }
            }
        }
        Ok(f(passable))
    }

    /// Finalizes the pipeline and returns the processed output.
    pub fn then_return(self) -> PipelineResult<T> {
        let mut passable = utility::require_passable(self.passable)?;
        for step in &self.pipes {
            match step.handle(passable) {
                Ok(val) => {
                    passable = val;
                    utility::run_taps(&self.taps, &passable);
                }
                Err(err) => {
                    return Err(utility::step_failure_from::<E, T>(err).into());
                }
            }
        }
        Ok(passable)
    }

    /// Adds a step that runs only if condition is true.
    pub fn when(mut self, condition: bool, step: Box<dyn Pipe<T, E>>) -> Self {
        if condition {
            self.pipes.push(step);
        }
        self
    }

    /// Adds a step that runs only if condition is false.
    pub fn unless(mut self, condition: bool, step: Box<dyn Pipe<T, E>>) -> Self {
        if !condition {
            self.pipes.push(step);
        }
        self
    }
}

#[cfg(feature = "async")]
pub trait AsyncPipe<T, E> {
    fn handle<'a>(&'a self, passable: T) -> Pin<Box<dyn Future<Output = Result<T, E>> + 'a>>;
}

#[cfg(feature = "async")]
pub struct AsyncPipeline<T, E> {
    pipes: Vec<Box<dyn AsyncPipe<T, E>>>,
}

#[cfg(feature = "async")]
impl<T, E> AsyncPipeline<T, E> {
    pub fn new() -> Self {
        Self { pipes: Vec::new() }
    }

    pub fn add<P: AsyncPipe<T, E> + 'static>(mut self, step: P) -> Self {
        self.pipes.push(Box::new(step));
        self
    }

    pub async fn execute(&self, mut passable: T) -> Result<T, E> {
        for step in &self.pipes {
            passable = step.handle(passable).await?;
        }
        Ok(passable)
    }
}