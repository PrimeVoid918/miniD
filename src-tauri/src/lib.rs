#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod app;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod utils;

use crate::app::commands::fetch_media;

pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fetch_media])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
