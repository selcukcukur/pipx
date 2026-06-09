use std::future::Future;
use std::pin::Pin;

pub trait AsyncPipe<T, E> {
    fn handle<'a>(&'a self, input: T) -> Pin<Box<dyn Future<Output = Result<T, E>> + 'a>>;
}

pub struct AsyncPipeline<T, E> {
    steps: Vec<Box<dyn AsyncPipe<T, E>>>,
}

impl<T, E> AsyncPipeline<T, E> {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add<P: AsyncPipe<T, E> + 'static>(mut self, step: P) -> Self {
        self.steps.push(Box::new(step));
        self
    }

    pub async fn execute(&self, mut input: T) -> Result<T, E> {
        for step in &self.steps {
            input = step.handle(input).await?;
        }
        Ok(input)
    }
}