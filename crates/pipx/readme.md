![linting](https://github.com/selcukcukur/pipx/actions/workflows/linting.yml/badge.svg)
![tests](https://github.com/selcukcukur/pipx/actions/workflows/tests.yml/badge.svg)
![examples](https://github.com/selcukcukur/pipx/actions/workflows/examples.yml/badge.svg)
![benches](https://github.com/selcukcukur/pipx/actions/workflows/benches.yml/badge.svg)
[![coverage](https://codecov.io/gh/selcukcukur/pipx/graph/badge.svg)](https://codecov.io/gh/selcukcukur/pipx)
[![crates.io](https://img.shields.io/crates/v/pipx.svg)](https://crates.io/crates/pipx)
[![docs.rs](https://docs.rs/pipx/badge.svg)](https://docs.rs/pipx)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](license.md)

> Built around the idea that every pipeline should remain explicit, predictable, and easy to reason about.

**pipx** is a small, framework-agnostic pipeline crate for building clear, composable, and predictable data flows in Rust.

The crate is built around a continuation-based pipeline model that gives each pipe full 
control over execution. A pipe may continue the chain, wrap the downstream result, 
short-circuit execution, recover from failures, or return an error.

**pipx** is fully generic over both the passable value and error type, making it suitable 
for application pipelines, middleware systems, validation flows, request processing, 
background jobs, and other data transformation workflows.

Individual pipes may use their own error types as long as they can be converted into 
`PipelineError`, allowing applications to maintain domain-specific errors internally
while exposing a consistent public execution API.

## Features

* Type-safe pipelines with `Pipe`, `Next`, and `Pipeline`
* Optional asynchronous pipeline support through the `async` feature
* Conditional pipeline composition with `when` and `unless`
* Error recovery with `rescue`
* Finalization hooks with `finally`
* Helper macros with `steps!` and `async_steps!`
* Optional procedural macros through the `macros` feature
* Fully generic passable values and error types
* Centralized error handling through `PipelineError`
* Support for synchronous and asynchronous execution models
* Suitable for middleware, validation, processing, and workflow pipelines

## Installation

The **pipx** crate can be installed with the default feature set or with optional features depending on your use case.

### With default

Use the default installation if you only need synchronous pipelines.

#### Cargo
```bash
cargo add pipx
```

#### Manual
```toml
[dependencies]
pipx = "0.1.0"
```

### Async Feature

Enable the `async` feature if you want to use asynchronous pipelines.

#### Cargo
```bash
cargo add pipx --features async
```

#### Manual
```toml
[dependencies]
pipx = { version = "0.1.0", features = ["async"] }
```

### Macros Feature

Enable the `macros` feature if you want to use procedural macros for pipe implementations.

#### Cargo
```bash
cargo add pipx --features macros
```

#### Manual
```toml
[dependencies]
pipx = { version = "0.1.0", features = ["macros"] }
```

### Full feature

Enable the `full` feature if you want to use all available pipx features.

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

Each example demonstrates one behavior at a time so you can quickly understand how pipelines 
are created, how steps are attached, how execution is finalized, and how synchronous and 
asynchronous pipelines differ.

### Pipeline

Use `pipeline` when you want to create a synchronous pipeline with an initial value.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Trim;

impl Pipe<String> for Trim {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        // Modify the incoming value before passing it to the next step.
        let passable = passable.trim().to_string();

        // Continue the pipeline with the updated value.
        next.handle(passable)
    }
}

fn main() -> pipx::PipelineResult<()> {
    // Create a pipeline with an initial value.
    let output = pipeline("  hello  ".to_string())
        // Append a single pipeline step.
        .pipe(Trim)
        // Execute the pipeline and return the final value.
        .then_return()?;

    assert_eq!(output, "hello");

    Ok(())
}
```

### Pipeline with `then_return`

Use `then_return` when you want the pipeline to return the final processed value without applying an additional transformation.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Uppercase;

impl Pipe<String> for Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        // Convert the value before continuing the pipeline.
        next.handle(passable.to_uppercase())
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("hello".to_string())
        .pipe(Uppercase)
        .then_return()?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Pipeline with `then`

Use `then` when you want to execute the pipeline and transform the final value into another type.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Uppercase;

impl Pipe<String> for Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase())
    }
}

fn main() -> pipx::PipelineResult<()> {
    let length = pipeline("hello".to_string())
        .pipe(Uppercase)
        // Execute the pipeline, then map the final String into its length.
        .then(|value| value.len())?;

    assert_eq!(length, 5);

    Ok(())
}
```

### Pipeline with `through`

Use `through` when you want to set the full list of pipeline steps at once.

```rust
use pipx::{pipeline, steps, Next, Pipe, PipelineResult};

struct Trim;
struct Uppercase;

impl Pipe<String> for Trim {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.trim().to_string())
    }
}

impl Pipe<String> for Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase())
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("  hello  ".to_string())
        // Replace the current pipeline steps with the provided step list.
        .through(steps![Trim, Uppercase])
        .then_return()?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Pipeline with `when`

Use `when` when you want to append a step only if a condition is `true`.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Uppercase;

impl Pipe<String> for Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase())
    }
}

fn main() -> pipx::PipelineResult<()> {
    let should_uppercase = true;

    let output = pipeline("hello".to_string())
        // Append Uppercase only when the condition is true.
        .when(should_uppercase, Uppercase)
        .then_return()?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Pipeline with `unless`

Use `unless` when you want to append a step only if a condition is `false`.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Uppercase;

impl Pipe<String> for Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase())
    }
}

fn main() -> pipx::PipelineResult<()> {
    let skip_uppercase = false;

    let output = pipeline("hello".to_string())
        // Append Uppercase only when the condition is false.
        .unless(skip_uppercase, Uppercase)
        .then_return()?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Pipeline with `finally`

Use `finally` when you want to run a callback after pipeline execution completes.

The callback runs whether the pipeline succeeds or fails.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Uppercase;

impl Pipe<String> for Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase())
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("hello".to_string())
        .pipe(Uppercase)
        // Run after the pipeline finishes.
        .finally(|result| {
            println!("Pipeline finished: {result:?}");
        })
        .then_return()?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Pipeline with `rescue`

Use `rescue` when you want to recover from pipeline execution errors with a fallback value.

```rust
use pipx::{pipeline, Next, Pipe, PipelineError, PipelineResult};

struct Fail;

impl Pipe<String> for Fail {
    fn handle(
        &self,
        _passable: String,
        _next: Next<'_, String>,
    ) -> PipelineResult<String> {
        // Stop the pipeline by returning an error.
        Err(PipelineError::Message("pipeline failed".to_string()))
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("hello".to_string())
        .pipe(Fail)
        // Convert the error into a fallback value.
        .rescue(|_| "fallback".to_string())?;

    assert_eq!(output, "fallback");

    Ok(())
}
```

### Pipeline Short-Circuiting

A pipe can stop the chain by not calling `next.handle`.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Stop;

impl Pipe<String> for Stop {
    fn handle(
        &self,
        passable: String,
        _next: Next<'_, String>,
    ) -> PipelineResult<String> {
        // Return early without continuing to the remaining steps.
        Ok(format!("{passable}:stopped"))
    }
}

struct NeverRuns;

impl Pipe<String> for NeverRuns {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(format!("never:{passable}"))
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("hello".to_string())
        .pipe(Stop)
        .pipe(NeverRuns)
        .then_return()?;

    assert_eq!(output, "hello:stopped");

    Ok(())
}
```

### Pipeline Wrapping

A pipe can call `next.handle` and then wrap or modify the downstream result.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Wrap;

impl Pipe<String> for Wrap {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        // Continue the pipeline first.
        let result = next.handle(passable)?;

        // Modify the value after downstream steps have completed.
        Ok(format!("[{result}]"))
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("hello".to_string())
        .pipe(Wrap)
        .then_return()?;

    assert_eq!(output, "[hello]");

    Ok(())
}
```

### Async Pipeline

Use `async_pipeline` when you want to create an asynchronous pipeline.

This requires the `async` feature.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, AsyncNext, AsyncPipe, PipelineResult};

struct AsyncUppercase;

#[async_trait]
impl AsyncPipe<String> for AsyncUppercase {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        // Modify the value asynchronously before continuing.
        next.handle(passable.to_uppercase()).await
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    let output = async_pipeline("hello".to_string())
        .pipe(AsyncUppercase)
        .then_return()
        .await?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Async Pipeline with `then_return`

Use `then_return` when you want the asynchronous pipeline to return the final processed value.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, AsyncNext, AsyncPipe, PipelineResult};

struct AsyncTrim;

#[async_trait]
impl AsyncPipe<String> for AsyncTrim {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.trim().to_string()).await
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    let output = async_pipeline("  hello  ".to_string())
        .pipe(AsyncTrim)
        .then_return()
        .await?;

    assert_eq!(output, "hello");

    Ok(())
}
```

### Async Pipeline with `then`

Use `then` when you want to execute an asynchronous pipeline and transform the final value.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, AsyncNext, AsyncPipe, PipelineResult};

struct AsyncTrim;

#[async_trait]
impl AsyncPipe<String> for AsyncTrim {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.trim().to_string()).await
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    let length = async_pipeline("  hello  ".to_string())
        .pipe(AsyncTrim)
        // Execute the async pipeline, then map the final value into another type.
        .then(|value| value.len())
        .await?;

    assert_eq!(length, 5);

    Ok(())
}
```

### Async Pipeline with `through`

Use `through` with `async_steps!` when you want to set multiple asynchronous steps at once.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, async_steps, AsyncNext, AsyncPipe, PipelineResult};

struct AsyncTrim;
struct AsyncUppercase;

#[async_trait]
impl AsyncPipe<String> for AsyncTrim {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.trim().to_string()).await
    }
}

#[async_trait]
impl AsyncPipe<String> for AsyncUppercase {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase()).await
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    let output = async_pipeline("  hello  ".to_string())
        // Replace the current async pipeline steps with the provided step list.
        .through(async_steps![AsyncTrim, AsyncUppercase])
        .then_return()
        .await?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Async Pipeline with `when`

Use `when` when you want to append an asynchronous step only if a condition is `true`.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, AsyncNext, AsyncPipe, PipelineResult};

struct AsyncUppercase;

#[async_trait]
impl AsyncPipe<String> for AsyncUppercase {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase()).await
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    let should_uppercase = true;

    let output = async_pipeline("hello".to_string())
        .when(should_uppercase, AsyncUppercase)
        .then_return()
        .await?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Async Pipeline with `unless`

Use `unless` when you want to append an asynchronous step only if a condition is `false`.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, AsyncNext, AsyncPipe, PipelineResult};

struct AsyncUppercase;

#[async_trait]
impl AsyncPipe<String> for AsyncUppercase {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase()).await
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    let skip_uppercase = false;

    let output = async_pipeline("hello".to_string())
        .unless(skip_uppercase, AsyncUppercase)
        .then_return()
        .await?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Async Pipeline with `finally`

Use `finally` when you want to run a callback after asynchronous pipeline execution completes.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, AsyncNext, AsyncPipe, PipelineResult};

struct AsyncUppercase;

#[async_trait]
impl AsyncPipe<String> for AsyncUppercase {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase()).await
    }
}

#[tokio::main]
async fn main() -> PipelineResult<()> {
    let output = async_pipeline("hello".to_string())
        .pipe(AsyncUppercase)
        .finally(|result| {
            println!("Async pipeline finished: {result:?}");
        })
        .then_return()
        .await?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Async Pipeline with `rescue`

Use `rescue` when you want to recover from asynchronous pipeline errors with a fallback value.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, AsyncNext, AsyncPipe, PipelineError, PipelineResult};

struct AsyncFail;

#[async_trait]
impl AsyncPipe<String> for AsyncFail {
    async fn handle(
        &self,
        _passable: String,
        _next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        Err(PipelineError::Message("async pipeline failed".to_string()))
    }
}

#[tokio::main]
async fn main() -> PipelineResult<()> {
    let output = async_pipeline("hello".to_string())
        .pipe(AsyncFail)
        .rescue(|_| "fallback".to_string())
        .await?;

    assert_eq!(output, "fallback");

    Ok(())
}
```

## Advanced

The examples below focus on composition, short-circuiting, wrapping downstream results, 
custom errors, shared step collections, and asynchronous job-style workflows.

### Wrapping Downstream Results

A pipe can call `next.handle` first and then modify the value returned by the remaining pipeline.

This is useful for response wrapping, instrumentation, logging, tracing, or post-processing.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Wrap;

impl Pipe<String> for Wrap {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        // Continue the rest of the pipeline first.
        let result = next.handle(passable)?;

        // Wrap the downstream result after all following steps have completed.
        Ok(format!("[{result}]"))
    }
}

struct Uppercase;

impl Pipe<String> for Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase())
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("hello".to_string())
        .pipe(Wrap)
        .pipe(Uppercase)
        .then_return()?;

    assert_eq!(output, "[HELLO]");

    Ok(())
}
```

### Short-Circuiting Execution

A pipe can stop execution by returning a value without calling `next.handle`.

This is useful for cache hits, authorization failures, validation exits, or fallback behavior.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Stop;

impl Pipe<String> for Stop {
    fn handle(
        &self,
        passable: String,
        _next: Next<'_, String>,
    ) -> PipelineResult<String> {
        // Do not continue to the remaining steps.
        Ok(format!("{passable}:stopped"))
    }
}

struct NeverRuns;

impl Pipe<String> for NeverRuns {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(format!("never:{passable}"))
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("job".to_string())
        .pipe(Stop)
        .pipe(NeverRuns)
        .then_return()?;

    assert_eq!(output, "job:stopped");

    Ok(())
}
```

### Validation Pipeline

Pipelines work well for validation flows because each pipe can either continue with the value or stop with an error.

```rust
use pipx::{pipeline, Next, Pipe, PipelineError, PipelineResult};

struct UserInput {
    username: String,
    password: String,
}

struct ValidateUsername;

impl Pipe<UserInput> for ValidateUsername {
    fn handle(
        &self,
        passable: UserInput,
        next: Next<'_, UserInput>,
    ) -> PipelineResult<UserInput> {
        if passable.username.trim().is_empty() {
            return Err(PipelineError::Message("username is required".to_string()));
        }

        next.handle(passable)
    }
}

struct ValidatePassword;

impl Pipe<UserInput> for ValidatePassword {
    fn handle(
        &self,
        passable: UserInput,
        next: Next<'_, UserInput>,
    ) -> PipelineResult<UserInput> {
        if passable.password.len() < 8 {
            return Err(PipelineError::Message("password is too short".to_string()));
        }

        next.handle(passable)
    }
}

fn main() -> pipx::PipelineResult<()> {
    let input = UserInput {
        username: "selcuk".to_string(),
        password: "secret-password".to_string(),
    };

    let validated = pipeline(input)
        .pipe(ValidateUsername)
        .pipe(ValidatePassword)
        .then_return()?;

    assert_eq!(validated.username, "selcuk");

    Ok(())
}
```

### Shared Step Collections

Use `steps!` when you want to define reusable groups of pipeline steps.

```rust
use pipx::{pipeline, steps, Next, Pipe, PipelineResult};

struct Trim;
struct Uppercase;
struct Prefix(&'static str);

impl Pipe<String> for Trim {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.trim().to_string())
    }
}

impl Pipe<String> for Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase())
    }
}

impl Pipe<String> for Prefix {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(format!("{}{}", self.0, passable))
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("  admin  ".to_string())
        .through(steps![
            Trim,
            Uppercase,
            Prefix("USER:"),
        ])
        .then_return()?;

    assert_eq!(output, "USER:ADMIN");

    Ok(())
}
```

### Conditional Runtime Composition

Use `when` and `unless` when the pipeline should be assembled based on runtime state.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Trim;
struct DebugPrefix;

impl Pipe<String> for Trim {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.trim().to_string())
    }
}

impl Pipe<String> for DebugPrefix {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(format!("debug:{passable}"))
    }
}

fn main() -> pipx::PipelineResult<()> {
    let debug_enabled = true;
    let skip_trim = false;

    let output = pipeline("  request  ".to_string())
        .unless(skip_trim, Trim)
        .when(debug_enabled, DebugPrefix)
        .then_return()?;

    assert_eq!(output, "debug:request");

    Ok(())
}
```

### Error Recovery

Use `rescue` when pipeline errors should be converted into a fallback value.

`InputMissing` is not recovered because it represents an invalid pipeline configuration rather than a step execution failure.

```rust
use pipx::{pipeline, Next, Pipe, PipelineError, PipelineResult};

struct FailingStep;

impl Pipe<String> for FailingStep {
    fn handle(
        &self,
        _passable: String,
        _next: Next<'_, String>,
    ) -> PipelineResult<String> {
        Err(PipelineError::Message("step failed".to_string()))
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("hello".to_string())
        .pipe(FailingStep)
        .rescue(|error| {
            format!("fallback because: {error}")
        })?;

    assert!(output.starts_with("fallback because:"));

    Ok(())
}
```

### Finalization Hooks

Use `finally` when you need to observe the final result without changing it.

This is useful for logging, metrics, tracing, cleanup, or debugging.

```rust
use pipx::{pipeline, Next, Pipe, PipelineResult};

struct Uppercase;

impl Pipe<String> for Uppercase {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.to_uppercase())
    }
}

fn main() -> pipx::PipelineResult<()> {
    let output = pipeline("hello".to_string())
        .pipe(Uppercase)
        .finally(|result| {
            println!("Final pipeline result: {result:?}");
        })
        .then_return()?;

    assert_eq!(output, "HELLO");

    Ok(())
}
```

### Async Background Job Pipeline

Asynchronous pipelines are useful for job processing, queue workflows, I/O-heavy steps, and other async workloads.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, async_steps, AsyncNext, AsyncPipe, PipelineResult};

#[derive(Debug)]
struct Job {
    id: u64,
    attempts: u8,
    events: Vec<String>,
}

struct LoadFromQueue;
struct ExecuteJob;

#[async_trait]
impl AsyncPipe<Job> for LoadFromQueue {
    async fn handle(
        &self,
        mut passable: Job,
        next: AsyncNext<'_, Job>,
    ) -> PipelineResult<Job> {
        passable.events.push("queue:loaded".to_string());

        next.handle(passable).await
    }
}

#[async_trait]
impl AsyncPipe<Job> for ExecuteJob {
    async fn handle(
        &self,
        mut passable: Job,
        next: AsyncNext<'_, Job>,
    ) -> PipelineResult<Job> {
        passable.attempts += 1;
        passable.events.push("job:executed".to_string());

        next.handle(passable).await
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    let job = Job {
        id: 100,
        attempts: 0,
        events: Vec::new(),
    };

    let job = async_pipeline(job)
        .through(async_steps![
            LoadFromQueue,
            ExecuteJob,
        ])
        .then_return()
        .await?;

    assert_eq!(job.attempts, 1);
    assert_eq!(job.events, vec!["queue:loaded", "job:executed"]);

    Ok(())
}
```

### Async Error Recovery

Asynchronous pipelines can also recover from execution errors with `rescue`.

```rust
use async_trait::async_trait;
use pipx::{async_pipeline, AsyncNext, AsyncPipe, PipelineError, PipelineResult};

struct AsyncFail;

#[async_trait]
impl AsyncPipe<String> for AsyncFail {
    async fn handle(
        &self,
        _passable: String,
        _next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        Err(PipelineError::Message("async step failed".to_string()))
    }
}

#[tokio::main]
async fn main() -> pipx::PipelineResult<()> {
    let output = async_pipeline("job".to_string())
        .pipe(AsyncFail)
        .rescue(|error| {
            format!("recovered from: {error}")
        })
        .await?;

    assert!(output.starts_with("recovered from:"));

    Ok(())
}
```

## Benchmark

The repository includes Criterion benchmarks for measuring pipeline execution performance under different execution models and workloads.

### All Benchmarks

Runs every available benchmark suite.

```bash
cargo bench -p pipx
```

### Pipeline Benchmarks

Measures execution throughput, step traversal, and short-circuit behavior for standard pipelines.

```bash
cargo bench -p pipx --bench pipeline
```

### Transform Pipeline Benchmarks

Measures throughput for transform pipelines that process values sequentially without pipeline continuations.

```bash
cargo bench -p pipx --bench pipeline_transform
```

### Async Pipeline Benchmarks

Measures execution throughput, step traversal, and short-circuit behavior for asynchronous pipelines.

```bash
cargo bench -p pipx --features async --bench async_pipeline
```

### Async Transform Pipeline Benchmarks

Measures throughput for asynchronous transform pipelines operating on asynchronous workloads.

```bash
cargo bench -p pipx --features async --bench async_pipeline_transform
```

## Examples

Runnable examples live under `examples/*`.

```bash
cargo run -p pipx --example basic_transform
cargo run -p pipx --example middleware_auth
cargo run -p pipx --example axum_adapter
cargo run -p pipx --example actix_web_adapter
cargo run -p pipx --example data_validation
cargo run -p pipx --example gpu_wgpu_pipeline
cargo run -p pipx --features async --example async_jobs
```

The web and GPU examples use framework-shaped adapter types instead of forcing heavy framework or
GPU dependencies into the crate. They show how to place pipx around Axum-like handlers,
Actix-like service requests, validation flows, async jobs, and wgpu-style render command pipelines.

## Contributing

The **pipx** project welcomes contributions from the community.

Whether you want to report a bug, suggest a new feature, improve the
documentation, or submit code changes, your contributions are greatly appreciated.

You can find detailed information about the contribution process by visiting the link below.

- **[Contributing Guide](contributing.md)**

## Security

The **pipx** project takes security vulnerabilities seriously.

If you believe you have discovered a security vulnerability, please report it
responsibly by contacting **Selçuk Çukur** at **<hello@selcukcukur.me>**.

Please do not disclose security vulnerabilities publicly until they have been
reviewed and addressed.

You can find detailed information about the security policy by visiting the link below.

- **[Security Policy](security.md)**

## License

The **pipx** project is published as open source software under the **[MIT License](license.md)**,
which is one of the most widely used open source licenses. 

You can find detailed information about the license terms by visiting the link below.

- **[MIT License](license.md)**
