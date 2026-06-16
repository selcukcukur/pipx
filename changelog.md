# Changelog

## v0.1.0 - 2026-06-16

We're excited to announce the first public release of **pipx** — now available on crates.io. 🎉

**pipx** is a composable pipeline library for Rust focused on explicit execution, predictable behavior, and strong type safety.

* Synchronous pipelines through `Pipeline`, `Pipe`, and `Next`
* Asynchronous pipelines through `AsyncPipeline`, `AsyncPipe`, and `AsyncNext`
* Generic passable values and error types
* Conditional composition with `when` and `unless`
* Error recovery with `rescue`
* Completion hooks with `finally`
* Helper constructors through `pipeline` and `async_pipeline`
* Step collection macros through `steps!` and `async_steps!`
* Procedural macros through `#[pipe]` and `#[async_pipe]`
* Runnable examples
* Criterion benchmark suites
* Optional feature flags for async execution and macros

This release establishes the foundation of the pipx ecosystem. Future releases will continue to focus on API refinement, performance improvements, additional examples, and developer experience enhancements.

Thank you to everyone following the project and providing feedback during development. 🚀
