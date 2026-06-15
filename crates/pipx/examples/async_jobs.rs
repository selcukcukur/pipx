#![cfg(feature = "async")]

use std::pin::Pin;
use std::sync::Arc;

use pipx::{AsyncNext, AsyncPipe, AsyncPipeline, PipelineResult};

#[derive(Debug)]
struct Job {
    id: u64,
    attempts: u8,
    events: Vec<String>,
}

struct LoadFromQueue;

impl AsyncPipe<Job> for LoadFromQueue {
    fn handle<'a>(
        &'a self,
        mut passable: Job,
        next: AsyncNext<'a, Job>,
    ) -> Pin<Box<dyn std::future::Future<Output = PipelineResult<Job>> + Send + 'a>> {
        Box::pin(async move {
            passable.events.push("queue:loaded".to_string());
            next.handle(passable).await
        })
    }
}

struct ExecuteJob;

impl AsyncPipe<Job> for ExecuteJob {
    fn handle<'a>(
        &'a self,
        mut passable: Job,
        next: AsyncNext<'a, Job>,
    ) -> Pin<Box<dyn std::future::Future<Output = PipelineResult<Job>> + Send + 'a>> {
        Box::pin(async move {
            passable.attempts += 1;
            passable.events.push("job:executed".to_string());
            next.handle(passable).await
        })
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
        .through(vec![Arc::new(LoadFromQueue), Arc::new(ExecuteJob)])
        .then_return()
        .await?;

    println!("job={} attempts={} {:?}", job.id, job.attempts, job.events);
    Ok(())
}
