#![cfg(feature = "macros")]

use pipx::{Next, Pipeline, PipelineError, PipelineResult, pipe};

#[pipe(String, PipelineError)]
struct MacroMiddleware;

impl MacroMiddleware {
    fn handle(&self, passable: String, next: Next<'_, String>) -> PipelineResult<String> {
        next.handle(format!("macro:{passable}"))
    }
}

#[test]
fn pipe_macro_implements_pipe_trait() {
    let result = Pipeline::new()
        .send("hello".to_string())
        .when(true, MacroMiddleware)
        .then_return()
        .unwrap();

    assert_eq!(result, "macro:hello");
}
