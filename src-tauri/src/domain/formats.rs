use serde::Serialize;

#[derive(Serialize)]
pub struct VideoFormat {
  #[serde(flatten)]
  pub base: BaseFormat,
  pub width: u32,
  pub height: u32,
  pub fps: Option<f32>,
}

#[derive(Serialize)]
pub struct AudioFormat {
  #[serde(flatten)]
  pub base: BaseFormat,
  pub abr: Option<u32>,
}

#[derive(Serialize)]
#[serde(tag = "kind")]
pub enum MediaFormat {
  Video(VideoFormat),
  Audio(AudioFormat),
}

#[derive(Serialize)]
pub struct BaseFormat {
  pub format_id: String,
  pub ext: String,
  pub protocol: Option<String>,
  pub filesize: Option<u64>,
  pub filesize_approx: Option<u64>,
  pub tbr: Option<f64>,
  pub video_ext: Option<String>,
  pub audio_ext: Option<String>,
}
