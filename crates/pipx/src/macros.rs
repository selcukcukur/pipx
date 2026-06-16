#[macro_export]
macro_rules! steps {
    ($($pipe:expr),+ $(,)?) => {{
        vec![
            $(
                std::sync::Arc::new($pipe) as $crate::PipelineStep<_, _>
            ),+
        ]
    }};
}
