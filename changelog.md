# Changelog

## Unreleased

We're excited to announce the first public release of pipx — now available on crates.io! 🎉

pipx is a framework-agnostic pipeline library for Rust that focuses on composability, predictability, and type safety. It provides a consistent way to build sequential processing flows using reusable pipeline steps, making complex execution chains easier to organize, test, and maintain.

Whether you're implementing middleware systems, validation pipelines, request processing, data transformation workflows, background jobs, or application-specific execution chains, pipx aims to provide a clean and ergonomic developer experience without sacrificing flexibility.

### Philosophy

pipx is built around a few core principles:

* Explicit execution over hidden magic.
* Predictable and deterministic pipeline behavior.
* Strong type safety without macros being required.
* Minimal abstractions that remain easy to understand.
* Framework independence and ecosystem compatibility.
* Consistent APIs across synchronous and asynchronous execution.

The goal is to provide a pipeline abstraction that feels natural in Rust while remaining flexible enough to support a wide variety of use cases.

### Architecture

The initial release establishes the core architecture of the library:

* Fully generic over passable values and error types.
* Shared error handling through `PipelineError`.
* Deterministic sequential execution model.
* Reusable pipeline steps through trait-based composition.
* Unified synchronous and asynchronous APIs.
* Optional async support through the `async` feature flag.
* Helper functions for ergonomic pipeline creation.
* Consistent documentation and API conventions across the crate.

### Core Components

#### Synchronous Pipelines

The synchronous API provides the foundation of the library:

* `Pipeline`
* `Pipe`
* `Next`
* `PipelineStep`
* `PipelineResult`
* `PipelineError`

These components allow developers to compose reusable processing steps while maintaining full control over execution flow.

#### Asynchronous Pipelines

pipx also provides asynchronous execution support:

* `AsyncPipeline`
* `AsyncPipe`
* `AsyncNext`
* `AsyncPipelineStep`
* `AsyncPipelineFuture`

The asynchronous API mirrors the synchronous API as closely as possible to reduce cognitive overhead when switching between execution models.

### Features

#### Pipeline Construction

* Pipeline builder API.
* Pipeline helper functions (`pipeline`, `async_pipeline`).
* Reusable step collections through the `steps!` macro.
* Generic passable values.
* Generic error types.

#### Conditional Composition

Pipelines can be composed dynamically using:

* `when`
* `unless`

These helpers make it easy to enable or disable pipeline steps based on runtime conditions.

#### Error Handling

pipx includes built-in mechanisms for handling failures:

* Structured pipeline errors.
* Error propagation.
* Error recovery with `rescue`.
* Completion hooks with `finally`.

This allows applications to centralize failure handling while keeping execution logic focused and readable.

#### Async Support

Optional asynchronous support includes:

* Async pipeline execution.
* Async pipeline steps.
* Async continuation handling.
* Consistent APIs between sync and async pipelines.

### Tooling

The project includes a modern development workflow:

* Cargo workspace configuration.
* Shared linting and formatting rules.
* Automated testing workflows.
* Benchmark suites.
* Example applications.
* CI/CD automation.
* Release automation.
* Code coverage reporting through Codecov.

### Examples

The repository contains examples demonstrating real-world usage patterns, including:

* Request processing pipelines.
* Validation chains.
* Middleware-style execution.
* Background job workflows.
* Asynchronous execution flows.
* Custom error handling strategies.

### Benchmarks

Benchmark suites are included to measure:

* Synchronous pipeline performance.
* Asynchronous pipeline performance.
* Large pipeline execution scenarios.
* General execution overhead.

These benchmarks help validate performance characteristics and track regressions over time.

### Future Direction

This release establishes the foundation of the pipx ecosystem.

Future releases will continue to focus on:

* API refinement.
* Performance improvements.
* Additional examples and integrations.
* Expanded ecosystem support.
* Developer experience enhancements.
* Long-term stability and maintainability.

Thank you to everyone following the project and providing feedback during development. 🚀
