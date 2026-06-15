#![cfg(feature = "async")]

use std::hint::black_box;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use pipx::{AsyncNext, AsyncPipe, AsyncPipeType, AsyncPipeline, PipelineResult};

struct AsyncAdd(u64);

impl AsyncPipe<Vec<u64>> for AsyncAdd {
    fn handle<'a>(
        &'a self,
        mut passable: Vec<u64>,
        next: AsyncNext<'a, Vec<u64>>,
    ) -> Pin<Box<dyn std::future::Future<Output = PipelineResult<Vec<u64>>> + Send + 'a>> {
        Box::pin(async move {
            for value in &mut passable {
                *value = value.wrapping_add(self.0);
            }

            next.handle(passable).await
        })
    }
}

fn values(count: usize) -> Vec<u64> {
    (0..count as u64).collect()
}

fn pipes(count: usize) -> Vec<AsyncPipeType<Vec<u64>>> {
    (0..count)
        .map(|index| Arc::new(AsyncAdd(index as u64 + 1)) as AsyncPipeType<Vec<u64>>)
        .collect()
}

fn bench_async_pipeline_value_steps(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("async_pipeline/value_steps");

    for pipe_count in [1usize, 10, 100] {
        let stack = pipes(pipe_count);

        group.throughput(Throughput::Elements(pipe_count as u64));

        group.bench_with_input(
            BenchmarkId::new("1000_values", pipe_count),
            &stack,
            |b, stack| {
                b.iter(|| {
                    runtime.block_on(async {
                        let output = AsyncPipeline::new()
                            .send(black_box(values(1_000)))
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

fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(20)
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(8))
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = bench_async_pipeline_value_steps
}

criterion_main!(benches);
