use crate::app::media_service;
use crate::domain::medias::Media;
use crate::error::app_error::AppError;
use crate::utils::AppResult;

use std::process::Command;
use std::io::{BufRead, BufReader};
use tauri::Window;

#[tauri::command]
pub fn fetch_media(url: String) -> Result<AppResult<Media>, AppError> {
  let media = media_service::get_media(&url)?;
  Ok(AppResult::new(media, 1))
}

#[tauri::command]
pub fn download_media(format_id: String, url: String) -> Result<AppResult<Media>, AppError> {
  let media = media_service::get_media(&url)?;
  Ok(AppResult::new(media, 1))
}

#[tuari::command]
pub async fn download_with_progress(
  window: Window,
  url: String,
  format_id: String,
) -> Result<String, String> {
  let mut child = Command::new("yt-dlp").args([
    "_f", &format_id,
    "--newline", // makes the ouput progress in new line 
    "-o", "%(title)s.%(ext)s", // Standard output template
    &url,
  ])
  .stdout(Stdio::pipe()) // pipe the output
  .spawn()
  .map_err(|e| e.to_string())?;
}
