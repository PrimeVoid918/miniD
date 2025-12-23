use crate::error::domain_error::DomainError;
use std::fmt;
use tauri::ipc::InvokeError;

#[derive(Debug)]
pub enum AppError {
  Domain(DomainError),
  Infra(String), // wrap other infra errors (e.g., IO, yt-dlp)
  Unknown(String),
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AppError::Domain(msg) => write!(f, "Domain error: {}", msg),
      AppError::Infra(msg) => write!(f, "Infra error: {}", msg),
      AppError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
    }
  }
}

impl std::error::Error for AppError {}

impl From<AppError> for InvokeError {
  fn from(err: AppError) -> Self {
    InvokeError::from(err.to_string())
  }
}

// Optional: allow easy conversion from DomainError to AppError
impl From<DomainError> for AppError {
  fn from(err: DomainError) -> Self {
    AppError::Domain(err)
  }
}

// Optional: allow easy conversion from std::io::Error to AppError
impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    AppError::Infra(err.to_string())
  }
}
