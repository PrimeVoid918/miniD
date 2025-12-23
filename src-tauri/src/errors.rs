use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid input")]
    InvalidInput,

    #[error("System error: {0}")]
    System(String),
}
