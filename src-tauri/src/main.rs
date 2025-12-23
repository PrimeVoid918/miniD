// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod infrastructure;

use crate::infrastructure::init_db;

#[tokio::main]
async fn main() {
    app_lib::run();
    init_db().await;
}
