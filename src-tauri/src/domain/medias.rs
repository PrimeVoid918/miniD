use crate::domain::formats::{AudioFormat, BaseFormat, MediaFormat, VideoFormat};
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

    let formats_json = data
      .get("formats")
      .and_then(|v| v.as_array())
      .ok_or_else(|| DomainError::MissingField("formats".into()))?;

    let formats = formats_json
      .iter()
      .filter_map(|f| {
        // !weird edge case for other websites
        let format_id = f.get("format_id")?.as_str()?.to_string();
        let ext = f.get("ext")?.as_str()?.to_string();

        let has_video = f.get("width").and_then(|v| v.as_u64()).is_some();
        let has_audio = f
          .get("acodec")
          .and_then(|v| v.as_str())
          .is_some_and(|v| v != "none");

        let base = BaseFormat {
          format_id,
          ext,
          protocol: f.get("protocol").and_then(|v| v.as_str()).map(String::from),
          filesize: f.get("filesize").and_then(|v| v.as_u64()),
          filesize_approx: f.get("filesize_approx").and_then(|v| v.as_u64()),
          tbr: f.get("tbr").and_then(|v| v.as_f64()).map(|v| v as f64),
          video_ext: f
            .get("video_ext")
            .and_then(|v| v.as_str())
            .map(String::from),
          audio_ext: f
            .get("audio_ext")
            .and_then(|v| v.as_str())
            .map(String::from),
        };

        //
        if has_video {
          println!("some videos deteced!: ");
          Some(MediaFormat::Video(VideoFormat {
            base,
            width: f.get("width")?.as_u64()? as u32,
            height: f.get("height")?.as_u64()? as u32,
            fps: f.get("fps").and_then(|v| v.as_f64()).map(|v| v as f32),
          }))
        } else if has_audio {
          println!("some audios deteced!: ");
          // println!("{}", serde_json::to_string_pretty(&base).unwrap());
          Some(MediaFormat::Audio(AudioFormat {
            base,
            abr: f.get("abr").and_then(|v| v.as_u64()).map(|v| v as u32),
          }))
        } else {
          None
        }
      })
      .collect::<Vec<_>>();

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
      formats: formats,             // simplify for now
    })
  }

  pub fn get_download_status(&self) -> Vec<MediaFormat> {
    
  }
}
