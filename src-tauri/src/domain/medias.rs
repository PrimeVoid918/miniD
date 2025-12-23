use crate::domain::formats::MediaFormat;
use crate::error::domain_error::DomainError;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Media {
  pub id: String,
  pub title: String,
  pub thumbnail: String,
  pub description: Option<String>,
  pub url: String,
  pub media_type: MediaType,
  pub formats: Vec<MediaFormat>,
}

#[derive(Serialize)]
pub enum MediaType {
  Video,
  Audio,
}

// !
impl Media {
  pub fn from_yt_dlp(json: &str) -> Result<Self, DomainError> {
    // Parse JSON
    let data: Value =
      serde_json::from_str(json).map_err(|e| DomainError::ParseError(e.to_string()))?;

    // Map fields, basic validation
    let id = data
      .get("id")
      .and_then(|v| v.as_str())
      .ok_or_else(|| DomainError::MissingField("id".to_string()))?;

    let title = data
      .get("title")
      .and_then(|v| v.as_str())
      .ok_or_else(|| DomainError::MissingField("title".to_string()))?;

    // Build Media instance
    Ok(Media {
      id: id.to_string(),
      title: title.to_string(),
      thumbnail: data
        .get("thumbnail")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string(),
      description: data
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()),
      url: data
        .get("url")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string(),
      media_type: MediaType::Video, // simplify for now
      formats: vec![],              // simplify for now
    })
  }
}
