use std::sync::Arc;

use pipx::{TransformPipe, PipelineResult, TransformPipeline};

struct Trim;

impl TransformPipe<String> for Trim {
    fn handle(&self, passable: String) -> PipelineResult<String> {
        Ok(passable.trim().to_string())
    }
}

struct Slugify;

impl TransformPipe<String> for Slugify {
    fn handle(&self, passable: String) -> PipelineResult<String> {
        Ok(passable.to_lowercase().replace(' ', "-"))
    }
}

fn main() -> pipx::PipelineResult<()> {
    let slug = TransformPipeline::new()
        .send(" Rust Pipeline Example ".to_string())
        .through(vec![Arc::new(Trim), Arc::new(Slugify)])
        .then_return()?;

    println!("{slug}");
    Ok(())
}
