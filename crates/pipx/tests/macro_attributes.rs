#![cfg(feature = "macros")]

use pipx::{
    Next, PipeResult, Pipeline, TransformPipeResult, TransformPipeline, pipes, transform_pipe,
};

#[pipe(String, pipx::PipelineError)]
struct MacroMiddleware;

impl MacroMiddleware {
    fn handle(&self, passable: String, next: Next<'_, String>) -> PipeResult<String> {
        next.handle(format!("macro:{passable}"))
    }
}

#[transform_pipe(String, pipx::PipelineError)]
struct MacroTransform;

impl MacroTransform {
    fn handle(&self, passable: String) -> TransformPipeResult<String> {
        Ok(passable.to_uppercase())
    }
}

#[test]
fn pipe_macro_implements_middleware_trait() {
    let result = Pipeline::new()
        .send("hello".to_string())
        .when(true, std::sync::Arc::new(MacroMiddleware))
        .then_return()
        .unwrap();

    assert_eq!(result, "macro:hello");
}

#[test]
fn transform_pipe_macro_implements_transform_trait() {
    let result = TransformPipeline::new()
        .send("hello".to_string())
        .when(true, std::sync::Arc::new(MacroTransform))
        .then_return()
        .unwrap();

    assert_eq!(result, "HELLO");
}
