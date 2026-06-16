![linting](https://github.com/selcukcukur/pipx/actions/workflows/linting.yml/badge.svg)
![tests](https://github.com/selcukcukur/pipx/actions/workflows/tests.yml/badge.svg)
![examples](https://github.com/selcukcukur/pipx/actions/workflows/examples.yml/badge.svg)
![benches](https://github.com/selcukcukur/pipx/actions/workflows/benches.yml/badge.svg)
[![coverage](https://codecov.io/gh/selcukcukur/pipx/graph/badge.svg)](https://codecov.io/gh/selcukcukur/pipx)
[![crates.io](https://img.shields.io/crates/v/pipx.svg)](https://crates.io/crates/pipx)
[![docs.rs](https://docs.rs/pipx/badge.svg)](https://docs.rs/pipx)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](license.md)

> Procedural macros for reducing boilerplate when implementing pipelines with **pipx**.

The **pipx-macros** crate provides a collection of procedural macros that simplify
the implementation of synchronous and asynchronous pipeline steps.

Instead of manually implementing the underlying traits, the provided macros
generate the required trait implementations automatically, allowing you to focus
on pipeline behavior rather than boilerplate.

The crate is intended to be used alongside **pipx** and is automatically enabled
through the `macros` or `full` feature flags on the main crate.

## Installation

Most users should enable the `macros` feature on the main **pipx** crate.

#### Cargo

```bash
cargo add pipx --features macros
```

#### Manual

```toml
[dependencies]
pipx = { version = "0.1.0", features = ["macros"] }
```

### Everything

#### Cargo

```bash
cargo add pipx --features full
```

#### Manual

```toml
[dependencies]
pipx = { version = "0.1.0", features = ["full"] }
```

## Quick Start

### `#[pipe]`

Use `#[pipe]` to automatically implement the `Pipe` trait for a type.

```rust
use pipx::{pipe, pipeline, Next, PipelineResult};

/// Automatically implements Pipe<String>.
#[pipe(String)]
struct Uppercase;

impl Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        // Transform the incoming value.
        let passable = passable.to_uppercase();

        // Continue the pipeline with the updated value.
        next.handle(passable)
    }
}

fn main() -> pipx::PipelineResult<()> {
    // Create a pipeline with an initial value.
    let output = pipeline("hello".to_string())
        // Append a pipeline step.
        .pipe(Uppercase)
        // Execute the pipeline and return the final value.
        .then_return()?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### `#[pipe]` with custom error

Use `#[pipe]` with a custom error type when a pipeline step should return domain-specific errors.

```rust
use pipx::{
    pipe,
    pipeline,
    Next,
    PipelineError,
    PipelineResult,
};

#[derive(Debug)]
enum UserError {
    UsernameTooShort,
}

// Convert the custom error into PipelineError.
impl From<UserError> for PipelineError {
    fn from(error: UserError) -> Self {
        PipelineError::Message(format!("{error:?}"))
    }
}

/// Automatically implements Pipe<String, UserError>.
#[pipe(String, UserError)]
struct ValidateUsername;

impl ValidateUsername {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String, UserError>,
    ) -> PipelineResult<String, UserError> {
        // Stop execution when validation fails.
        if passable.len() < 3 {
            return Err(UserError::UsernameTooShort);
        }

        // Continue execution when validation succeeds.
        next.handle(passable)
    }
}

fn main() -> pipx::PipelineResult<()> {
    let username = pipeline("selcuk".to_string())
        .pipe(ValidateUsername)
        .then_return()?;

    assert_eq!(username, "selcuk");

    Ok(())
}
```

### `#[async_pipe]`

Use `#[async_pipe]` to automatically implement the `AsyncPipe` trait for a type.

```rust
use async_trait::async_trait;
use pipx::{
    async_pipe,
    async_pipeline,
    AsyncNext,
    PipelineResult,
};

/// Automatically implements AsyncPipe<String>.
#[async_pipe(String)]
struct AsyncUppercase;

#[async_trait]
impl AsyncUppercase {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        // Transform the incoming value.
        let passable = passable.to_uppercase();

        // Continue the pipeline with the updated value.
        next.handle(passable).await
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    // Create a pipeline with an initial value.
    let output = async_pipeline("hello".to_string())
        // Append a pipeline step.
        .pipe(AsyncUppercase)
        // Execute the pipeline and return the final value.
        .then_return()
        .await?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### `#[async_pipe]` with custom error

Use `#[async_pipe]` with a custom error type when an asynchronous pipeline step should return domain-specific errors.

```rust
use async_trait::async_trait;
use pipx::{
    async_pipe,
    async_pipeline,
    AsyncNext,
    PipelineError,
    PipelineResult,
};

#[derive(Debug)]
enum UserError {
    UsernameTooShort,
}

// Convert the custom error into PipelineError.
impl From<UserError> for PipelineError {
    fn from(error: UserError) -> Self {
        PipelineError::Message(format!("{error:?}"))
    }
}

/// Automatically implements AsyncPipe<String, UserError>.
#[async_pipe(String, UserError)]
struct ValidateUsername;

#[async_trait]
impl ValidateUsername {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String, UserError>,
    ) -> PipelineResult<String, UserError> {
        // Stop execution when validation fails.
        if passable.len() < 3 {
            return Err(UserError::UsernameTooShort);
        }

        // Continue execution when validation succeeds.
        next.handle(passable).await
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    let username = async_pipeline("selcuk".to_string())
        .pipe(ValidateUsername)
        .then_return()
        .await?;

    assert_eq!(username, "selcuk");

    Ok(())
}
```

## Contributing

The **pipx** project welcomes contributions from the community.

Whether you want to report a bug, suggest a new feature, improve the documentation,
or submit code changes, your contributions are greatly appreciated.

You can find detailed information about the contribution process by visiting the link below.

- **[Contributing Guide](contributing.md)**

## Security

The **pipx** project takes security vulnerabilities seriously.

If you believe you have discovered a security vulnerability, please report it
responsibly by contacting **Selçuk Çukur** at **[hello@selcukcukur.me](mailto:hello@selcukcukur.me)**.

Please do not disclose security vulnerabilities publicly until they have been
reviewed and addressed.

You can find detailed information about the security policy by visiting the link below.

- **[Security Policy](security.md)**

## License

The **pipx** project is published as open source software under the **[MIT License](license.md)**,
which is one of the most widely used open source licenses.

You can find detailed information about the license terms by visiting the link below.

- **[MIT License](license.md)**
