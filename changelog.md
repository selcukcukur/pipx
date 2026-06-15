# Changelog

## Unreleased

We’re excited to announce the very first release of pipx — now live on [crates.io](https://crates.io/crates/pipx)! 🎉

### Philosophy

pipx was built to make type‑safe pipelines in Rust simple yet powerful. The crate is 
framework‑agnostic, designed to let developers compose clear data flows without losing 
control over execution. It offers both middleware‑style and transform‑style pipelines, 
so you can choose the model that best fits your problem domain.

### Architecture

- Fully generic over passable values and error types.
- Centralized error handling via `PipelineError`
- Async support available behind the `async` feature
- Workspace setup with shared dependencies and formatting rules
- CI workflows for linting, tests, benches, examples, and publishing

### Highlights

- Type‑safe sync middleware pipelines (`Pipe`, `Next`, `Pipeline`)
- Type‑safe sync transform pipelines (`TransformPipe`, `TransformPipeline`)
- Async pipelines enabled via feature flag
- Conditional composition with `when` and `unless`
- Error recovery and finalization with `rescue` and `finally`
- Optional observation hooks (`taps`)
- Proc macros for boilerplate‑free pipe implementations (`#[pipe]`, `#[transform_pipe]`)
- Benchmarks covering sync/async middleware and transforms
- Runnable examples for web adapters (Axum, Actix), validation flows, async jobs, and GPU pipelines
- Coverage reporting integrated with Codecov
