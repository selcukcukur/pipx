use std::sync::Arc;

use pipx::{Next, Pipe, Pipeline, PipelineError, PipelineResult, StepFailure};

#[derive(Debug)]
struct Signup {
    email: String,
    password: String,
    normalized: bool,
}

struct NormalizeEmail;

impl Pipe<Signup> for NormalizeEmail {
    fn handle(
        &self,
        mut passable: Signup,
        next: Next<'_, Signup>,
    ) -> PipelineResult<Signup> {
        passable.email = passable.email.trim().to_lowercase();
        passable.normalized = true;

        next.handle(passable)
    }
}

struct ValidateEmail;

impl Pipe<Signup> for ValidateEmail {
    fn handle(
        &self,
        passable: Signup,
        next: Next<'_, Signup>,
    ) -> PipelineResult<Signup> {
        if passable.email.contains('@') {
            next.handle(passable)
        } else {
            Err(PipelineError::StepFailure(StepFailure {
                step: "ValidateEmail",
                message: "email must contain @".to_string(),
            }))
        }
    }
}

struct ValidatePassword;

impl Pipe<Signup> for ValidatePassword {
    fn handle(
        &self,
        passable: Signup,
        next: Next<'_, Signup>,
    ) -> PipelineResult<Signup> {
        if passable.password.len() >= 12 {
            next.handle(passable)
        } else {
            Err(PipelineError::StepFailure(StepFailure {
                step: "ValidatePassword",
                message: "password must be at least 12 characters".to_string(),
            }))
        }
    }
}

fn main() -> pipx::PipelineResult<()> {
    let signup = Signup {
        email: " USER@example.COM ".to_string(),
        password: "correct horse".to_string(),
        normalized: false,
    };

    let signup = Pipeline::new()
        .send(signup)
        .through(vec![
            Arc::new(NormalizeEmail),
            Arc::new(ValidateEmail),
            Arc::new(ValidatePassword),
        ])
        .then_return()?;

    println!("{signup:?}");
    Ok(())
}