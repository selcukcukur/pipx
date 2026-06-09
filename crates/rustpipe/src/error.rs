#[derive(Debug)]
pub enum PipelineError {
    StepFailure(String),   // step içindeki hata
    InputMissing,          // send çağrılmadan then_return yapılırsa
    DispatchError(String), // via ile yanlış method seçimi
    RescueFailure(String), // rescue içinde fallback başarısız
}

pub type PipelineResult<T> = Result<T, PipelineError>;

