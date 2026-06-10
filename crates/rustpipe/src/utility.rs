use std::any::type_name;
use crate::errors::StepFailure;
use crate::errors::PipelineError;

/// Tekrarlayan StepFailure oluşturma
pub fn step_failure_from<E: std::fmt::Debug, T>(err: E) -> StepFailure {
    StepFailure {
        step: type_name::<E>(), // error tipinin adı
        message: format!("{:?}", err),
    }
}

/// Input kontrolünü kısaltmak için helper
pub fn require_passable<T>(passable: Option<T>) -> Result<T, PipelineError> {
    passable.ok_or(PipelineError::InputMissing)
}

/// Tap çağrılarını çalıştıran helper
pub fn run_taps<T>(taps: &Vec<Box<dyn Fn(&T)>>, passable: &T) {
    for tap in taps {
        tap(passable);
    }
}
