#[cfg(feature = "async")]
use std::pin::Pin;
use std::sync::Arc;
#[cfg(feature = "async")]
use crate::{AsyncPipe};
use crate::{Pipe, PipelineError};

pub type PipelineResult<TPassable, TError = PipelineError> = Result<TPassable, TError>;

pub type PipelineStep<TPassable, TError = PipelineError> =
    Arc<dyn Pipe<TPassable, TError> + Send + Sync>;

pub type FinallyCallback<TPassable> = Box<dyn Fn(&PipelineResult<TPassable>) + Send + Sync>;

#[cfg(feature = "async")]
pub type AsyncPipelineStep<TPassable, TError = PipelineError> =
    Arc<dyn AsyncPipe<TPassable, TError> + Send + Sync>;

#[cfg(feature = "async")]
pub type AsyncPipeFuture<'a, TPassable, TError = PipelineError> =
    Pin<Box<dyn Future<Output = PipelineResult<TPassable, TError>> + Send + 'a>>;

#[cfg(feature = "async")]
pub type AsyncDestination<'a, TPassable, TError = PipelineError> =
    dyn Fn(TPassable) -> AsyncPipeFuture<'a, TPassable, TError> + Sync + 'a;
