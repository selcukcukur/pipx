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

/// Creates a list of asynchronous pipeline steps.
///
/// **Parameters**
/// - `$pipe` - One or more async pipe expressions that should be stored as asynchronous pipeline steps.
///
/// **Returns**
/// - [`Vec`] - A vector containing the provided steps as [`AsyncPipelineStep`] values.
#[cfg(feature = "async")]
#[macro_export]
macro_rules! async_steps {
    ($($pipe:expr),+ $(,)?) => {{
        vec![
            $(
                std::sync::Arc::new($pipe) as $crate::AsyncPipelineStep<_, _>
            ),+
        ]
    }};
}
