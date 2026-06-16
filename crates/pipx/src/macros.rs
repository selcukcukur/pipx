/// Creates a list of pipeline steps.
///
/// **Parameters**
/// - `$pipe` - One or more pipe expressions that should be stored as pipeline steps.
///
/// **Returns**
/// - [`Vec`] - A vector containing the provided steps as [`PipelineStep`] values.
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
