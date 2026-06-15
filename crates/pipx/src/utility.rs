use crate::errors::PipelineError;

/// Requires a pipeline input value before execution starts.
///
/// **Parameters**
/// - `passable` - The optional passable value stored by the pipeline.
///
/// **Returns**
/// - `Ok(T)` - The passable value exists.
/// - `Err(PipelineError::InputMissing)` - The pipeline was executed without `send`.
pub fn require_passable<T>(passable: Option<T>) -> Result<T, PipelineError> {
    passable.ok_or(PipelineError::InputMissing)
}
