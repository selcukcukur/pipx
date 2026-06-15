use std::sync::Arc;

use pipx::{Next, Pipe, Pipeline, PipelineResult};

struct Trim;

impl Pipe<String> for Trim {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(passable.trim().to_string())
    }
}

struct Slugify;

impl Pipe<String> for Slugify {
    fn handle(
        &self,
        passable: String,
        next: Next<'_, String>,
    ) -> PipelineResult<String> {
        next.handle(
            passable
                .to_lowercase()
                .replace(' ', "-"),
        )
    }
}

fn main() -> pipx::PipelineResult<()> {
    let slug = Pipeline::new()
        .send(" Rust Pipeline Example ".to_string())
        .through(vec![
            Arc::new(Trim),
            Arc::new(Slugify),
        ])
        .then_return()?;

    println!("{slug}");

    Ok(())
}