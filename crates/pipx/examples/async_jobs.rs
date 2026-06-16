#![cfg(feature = "async")]

use async_trait::async_trait;
use pipx::{AsyncNext, AsyncPipe, AsyncPipeline, async_steps};

#[derive(Debug)]
struct Job {
    id: u64,
    attempts: u8,
    events: Vec<String>,
}

struct LoadFromQueue;

#[async_trait]
impl AsyncPipe<Job> for LoadFromQueue {
    async fn handle(
        &self,
        mut passable: Job,
        next: AsyncNext<'_, Job>,
    ) -> pipx::PipelineResult<Job> {
        passable.events.push("queue:loaded".to_string());

        next.handle(passable).await
    }
}

struct ExecuteJob;

#[async_trait]
impl AsyncPipe<Job> for ExecuteJob {
    async fn handle(
        &self,
        mut passable: Job,
        next: AsyncNext<'_, Job>,
    ) -> pipx::PipelineResult<Job> {
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

    let job = AsyncPipeline::new()
        .send(job)
        .through(async_steps![LoadFromQueue, ExecuteJob])
        .then_return()
        .await?;

    println!("job={} attempts={} {:?}", job.id, job.attempts, job.events);

    Ok(())
}
