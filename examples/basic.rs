use rsflux::pipeline::{Pipeline, Pipe};

struct TrimStep;
impl Pipe<String> for TrimStep {
    fn handle(&self, input: String) -> String {
        input.trim().to_string()
    }
}

struct UpperStep;
impl Pipe<String> for UpperStep {
    fn handle(&self, input: String) -> String {
        input.to_uppercase()
    }
}

fn main() {
    let pipeline = Pipeline::new()
        .add(TrimStep)
        .add(UpperStep);

    let result = pipeline.execute("   hello rustpipe   ".to_string());
    println!("{}", result);
}
