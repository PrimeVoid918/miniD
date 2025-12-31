use crate::domain::medias::Media;
use crate::error::app_error::AppError;
use crate::infrastructure::yt_dlp;

pub fn get_media(url: &str) -> Result<Media, AppError> {
  let raw_json = yt_dlp::YtDLPAdapter::fetch_metadata(url)?;
  let media = Media::from_yt_dlp(&raw_json)?;
  Ok(media)
}

pub fn download(format_id: &str, url: &str) -> Result<Media, AppError> {
  let raw_json = yt_dlp::YtDLPAdapter::download(url, format_id)?;
  let media = Media::from_yt_dlp(&raw_json)?;
  Ok(media)
}
