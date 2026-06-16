#![cfg(feature = "async")]

use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use pipx::{AsyncNext, AsyncPipe, AsyncPipeline, AsyncPipelineStep, PipelineResult};

struct AsyncAdd(u64);

#[async_trait]
impl AsyncPipe<u64> for AsyncAdd {
    async fn handle(&self, passable: u64, next: AsyncNext<'_, u64>) -> PipelineResult<u64> {
        next.handle(passable.wrapping_add(self.0)).await
    }
}

struct AsyncStop(u64);

#[async_trait]
impl AsyncPipe<u64> for AsyncStop {
    async fn handle(&self, passable: u64, _next: AsyncNext<'_, u64>) -> PipelineResult<u64> {
        Ok(passable + self.0)
    }
}

fn pipes(count: usize) -> Vec<AsyncPipelineStep<u64>> {
    (0..count)
        .map(|index| Arc::new(AsyncAdd(index as u64 + 1)) as AsyncPipelineStep<u64>)
        .collect()
}

const PIPE_COUNTS: [usize; 4] = [1, 10, 100, 1_000];

fn bench_async_pipeline_by_pipe_count(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("async_pipeline/pipe_count");

    for pipe_count in PIPE_COUNTS {
        let stack = pipes(pipe_count);

        group.throughput(Throughput::Elements(pipe_count as u64));

        group.bench_with_input(
            BenchmarkId::new("u64_value", pipe_count),
            &stack,
            |bench, stack| {
                bench.iter(|| {
                    runtime.block_on(async {
                        let output = AsyncPipeline::new()
                            .send(black_box(0_u64))
                            .through(stack.clone())
                            .then_return()
                            .await
                            .unwrap();

                        black_box(output);
                    });
                });
            },
        );
    }

    group.finish();
}

fn bench_async_pipeline_short_circuit(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("async_pipeline/short_circuit");

    for tail_count in [100usize, 1_000, 10_000, 100_000] {
        let tail = pipes(tail_count);

        group.throughput(Throughput::Elements(tail_count as u64));

        group.bench_with_input(
            BenchmarkId::new("stop_before_tail", tail_count),
            &tail,
            |bench, tail| {
                bench.iter(|| {
                    runtime.block_on(async {
                        let mut stack: Vec<AsyncPipelineStep<u64>> = vec![Arc::new(AsyncStop(10))];

                        stack.extend(tail.clone());

                        let output = AsyncPipeline::new()
                            .send(black_box(1_u64))
                            .through(stack)
                            .then_return()
                            .await
                            .unwrap();

                        black_box(output);
                    });
                });
            },
        );
    }

    group.finish();
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(5))
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets =
        bench_async_pipeline_by_pipe_count,
        bench_async_pipeline_short_circuit
}

criterion_main!(benches);
