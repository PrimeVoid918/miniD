use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct AppResult<T> {
  pub success: bool,
  pub results: T,
  pub stats: Option<Stats>,
  pub timestamp: String,
}

#[derive(Serialize)]
pub struct Stats {
  pub status_code: usize,
}

impl<T> AppResult<T> {
  pub fn new(results: T, status_code: usize) -> Self {
    AppResult {
      success: true,
      results,
      stats: Some(Stats { status_code }),
      timestamp: Utc::now().to_rfc3339(),
    }
  }
}
