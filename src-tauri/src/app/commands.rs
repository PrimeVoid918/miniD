use crate::app::media_service;
use crate::domain::medias::Media;
use crate::error::app_error::AppError;
use crate::utils::AppResult;

#[tauri::command]
pub fn fetch_media(url: String) -> Result<AppResult<Media>, AppError> {
  let media = media_service::get_media(&url)?;
  Ok(AppResult::new(media, 1))
}
