use std::sync::Arc;

use pipx::{Next, Pipe, Pipeline, PipelineResult};

#[derive(Debug)]
struct RenderFrame {
    label: String,
    command_log: Vec<String>,
    vertex_count: u32,
}

struct UploadBuffers;

impl Pipe<RenderFrame> for UploadBuffers {
    fn handle(
        &self,
        mut passable: RenderFrame,
        next: Next<'_, RenderFrame>,
    ) -> PipelineResult<RenderFrame> {
        passable.command_log.push("wgpu:upload-buffers".to_string());
        next.handle(passable)
    }
}

struct EncodeRenderPass;

impl Pipe<RenderFrame> for EncodeRenderPass {
    fn handle(
        &self,
        mut passable: RenderFrame,
        next: Next<'_, RenderFrame>,
    ) -> PipelineResult<RenderFrame> {
        passable.command_log.push(format!(
            "wgpu:encode-render-pass vertices={}",
            passable.vertex_count
        ));

        next.handle(passable)
    }
}

struct SubmitQueue;

impl Pipe<RenderFrame> for SubmitQueue {
    fn handle(
        &self,
        mut passable: RenderFrame,
        next: Next<'_, RenderFrame>,
    ) -> PipelineResult<RenderFrame> {
        passable.command_log.push("wgpu:queue-submit".to_string());
        next.handle(passable)
    }
}

fn main() -> pipx::PipelineResult<()> {
    let frame = RenderFrame {
        label: "main-pass".to_string(),
        command_log: Vec::new(),
        vertex_count: 36,
    };

    let frame = Pipeline::new()
        .send(frame)
        .through(vec![
            Arc::new(UploadBuffers),
            Arc::new(EncodeRenderPass),
            Arc::new(SubmitQueue),
        ])
        .then_return()?;

    println!("{} {:?}", frame.label, frame.command_log);
    Ok(())
}
