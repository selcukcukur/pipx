pub trait Pipe<T> {
    fn handle(&self, input: T) -> T;
}
