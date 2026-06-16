#![cfg(feature = "async")]

use async_trait::async_trait;
use pipx::{AsyncNext, AsyncPipe, AsyncPipeline, PipelineResult, async_steps};

struct AsyncPrefix(&'static str);

#[async_trait]
impl AsyncPipe<String> for AsyncPrefix {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(format!("{}{}", self.0, passable)).await
    }
}

struct AsyncStop;

#[async_trait]
impl AsyncPipe<String> for AsyncStop {
    async fn handle(
        &self,
        passable: String,
        _next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        Ok(format!("{passable}:stopped"))
    }
}

struct AsyncAddSuffix(&'static str);

#[async_trait]
impl AsyncPipe<String> for AsyncAddSuffix {
    async fn handle(
        &self,
        passable: String,
        next: AsyncNext<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(format!("{}{}", passable, self.0)).await
    }
}

struct AsyncBatchAdd(u64);

#[async_trait]
impl AsyncPipe<Vec<u64>> for AsyncBatchAdd {
    async fn handle(
        &self,
        mut passable: Vec<u64>,
        next: AsyncNext<'_, Vec<u64>>,
    ) -> PipelineResult<Vec<u64>> {
        for value in &mut passable {
            *value += self.0;
        }

        next.handle(passable).await
    }
}

#[tokio::test]
async fn async_pipeline_runs_next_chain() {
    let result = AsyncPipeline::new()
        .send("core".to_string())
        .through(async_steps![AsyncPrefix("app:")])
        .then_return()
        .await
        .unwrap();

    assert_eq!(result, "app:core");
}

#[tokio::test]
async fn async_pipeline_runs_multiple_steps_in_order() {
    let result = AsyncPipeline::new()
        .send("core".to_string())
        .through(async_steps![AsyncPrefix("app:"), AsyncAddSuffix(":done")])
        .then_return()
        .await
        .unwrap();

    assert_eq!(result, "app:core:done");
}

#[tokio::test]
async fn async_pipeline_can_short_circuit() {
    let result = AsyncPipeline::new()
        .send("core".to_string())
        .through(async_steps![AsyncStop, AsyncPrefix("never:")])
        .then_return()
        .await
        .unwrap();

    assert_eq!(result, "core:stopped");
}

#[tokio::test]
async fn async_pipeline_processes_batches() {
    let result = AsyncPipeline::new()
        .send((0_u64..1_000).collect::<Vec<_>>())
        .through(async_steps![AsyncBatchAdd(10), AsyncBatchAdd(5)])
        .then(|values| values.into_iter().sum::<u64>())
        .await
        .unwrap();

    assert_eq!(result, 514_500);
}
