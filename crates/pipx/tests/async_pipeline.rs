#![cfg(feature = "async")]

use std::pin::Pin;
use std::sync::Arc;

use pipx::{AsyncNext, AsyncPipe, AsyncPipeline, PipelineResult};

struct AsyncPrefix(&'static str);

impl AsyncPipe<String> for AsyncPrefix {
    fn handle<'a>(
        &'a self,
        passable: String,
        next: AsyncNext<'a, String>,
    ) -> Pin<Box<dyn std::future::Future<Output = PipelineResult<String>> + Send + 'a>> {
        Box::pin(async move { next.handle(format!("{}{}", self.0, passable)).await })
    }
}

struct AsyncStop;

impl AsyncPipe<String> for AsyncStop {
    fn handle<'a>(
        &'a self,
        passable: String,
        _next: AsyncNext<'a, String>,
    ) -> Pin<Box<dyn std::future::Future<Output = PipelineResult<String>> + Send + 'a>> {
        Box::pin(async move { Ok(format!("{passable}:stopped")) })
    }
}

struct AsyncAddSuffix(&'static str);

impl AsyncPipe<String> for AsyncAddSuffix {
    fn handle<'a>(
        &'a self,
        passable: String,
        next: AsyncNext<'a, String>,
    ) -> Pin<Box<dyn std::future::Future<Output = PipelineResult<String>> + Send + 'a>> {
        Box::pin(async move { next.handle(format!("{}{}", passable, self.0)).await })
    }
}

struct AsyncBatchAdd(u64);

impl AsyncPipe<Vec<u64>> for AsyncBatchAdd {
    fn handle<'a>(
        &'a self,
        mut passable: Vec<u64>,
        next: AsyncNext<'a, Vec<u64>>,
    ) -> Pin<Box<dyn std::future::Future<Output = PipelineResult<Vec<u64>>> + Send + 'a>> {
        Box::pin(async move {
            for value in &mut passable {
                *value += self.0;
            }

            next.handle(passable).await
        })
    }
}

#[tokio::test]
async fn async_pipeline_runs_next_chain() {
    let result = AsyncPipeline::new()
        .send("core".to_string())
        .through(vec![Arc::new(AsyncPrefix("app:"))])
        .then_return()
        .await
        .unwrap();

    assert_eq!(result, "app:core");
}

#[tokio::test]
async fn async_pipeline_runs_multiple_steps_in_order() {
    let result = AsyncPipeline::new()
        .send("core".to_string())
        .through(vec![
            Arc::new(AsyncPrefix("app:")),
            Arc::new(AsyncAddSuffix(":done")),
        ])
        .then_return()
        .await
        .unwrap();

    assert_eq!(result, "app:core:done");
}

#[tokio::test]
async fn async_pipeline_can_short_circuit() {
    let result = AsyncPipeline::new()
        .send("core".to_string())
        .through(vec![Arc::new(AsyncStop), Arc::new(AsyncPrefix("never:"))])
        .then_return()
        .await
        .unwrap();

    assert_eq!(result, "core:stopped");
}

#[tokio::test]
async fn async_pipeline_processes_batches() {
    let result = AsyncPipeline::new()
        .send((0_u64..1_000).collect::<Vec<_>>())
        .through(vec![
            Arc::new(AsyncBatchAdd(10)),
            Arc::new(AsyncBatchAdd(5)),
        ])
        .then(|values| values.into_iter().sum::<u64>())
        .await
        .unwrap();

    assert_eq!(result, 514_500);
}
