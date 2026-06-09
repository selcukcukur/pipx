use rustpipe::pipeline::{Pipeline, Pipe};

struct TrimStep;
impl Pipe<String, String> for TrimStep {
    fn handle(&self, input: String) -> Result<String, String> {
        Ok(input.trim().to_string())
    }
}

struct UpperStep;
impl Pipe<String, String> for UpperStep {
    fn handle(&self, input: String) -> Result<String, String> {
        Ok(input.to_uppercase())
    }
}

fn main() {
    let pipeline = Pipeline::new()
        .add(TrimStep)
        .add(UpperStep);

    match pipeline.execute("   hello rustpipe   ".to_string()) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Pipeline error: {}", e),
    }
}