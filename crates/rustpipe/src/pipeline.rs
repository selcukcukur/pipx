pub trait Pipe<T, E> {
    fn handle(&self, input: T) -> Result<T, E>;
}

pub struct Pipeline<T, E> {
    steps: Vec<Box<dyn Pipe<T, E>>>,
}

impl<T, E> Pipeline<T, E> {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add<P: Pipe<T, E> + 'static>(mut self, step: P) -> Self {
        self.steps.push(Box::new(step));
        self
    }

    pub fn execute(&self, mut input: T) -> Result<T, E> {
        for step in &self.steps {
            input = step.handle(input)?;
        }
        Ok(input)
    }
}
