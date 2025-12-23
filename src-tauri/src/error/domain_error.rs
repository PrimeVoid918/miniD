use std::fmt;

#[derive(Debug)]
pub enum DomainError {
  ParseError(String),
  ValidationError(String),
  MissingField(String),
}

impl std::error::Error for DomainError {}

impl fmt::Display for DomainError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DomainError::ParseError(msg) => write!(f, "Parse error: {}", msg),
      DomainError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
      DomainError::MissingField(field) => write!(f, "Missing field: {}", field),
    }
  }
}
