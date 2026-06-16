#![allow(clippy::module_inception)]
mod next;
mod pipe;

#[cfg(feature = "async")]
mod async_pipe;

#[cfg(feature = "async")]
mod async_next;

/// Pipeline continuation types.
pub use next::*;

/// Pipeline step traits.
pub use pipe::*;

#[cfg(feature = "async")]
/// Asynchronous pipeline continuation types.
pub use async_next::*;

#[cfg(feature = "async")]
/// Asynchronous pipeline step traits.
pub use async_pipe::*;
