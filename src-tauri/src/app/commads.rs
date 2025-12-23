use crate::domain::services;
use crate::error::AppResult;

#[tauri::command]
pub fn sample(input: String) -> AppResult<String> {
    services::process_input(input)
}

use crate::infrastructure::db::SqliteAdapter;

pub fn get_greeting() -> String {
    let db = SqliteAdapter::new("my_db.sqlite").unwrap();
    db.execute("CREATE TABLE IF NOT EXISTS greetings (msg TEXT)")
        .unwrap();
    db.execute("INSERT INTO greetings (msg) VALUES ('Hello from Rust')")
        .unwrap();
    db.query_one("SELECT msg FROM greetings LIMIT 1").unwrap()
}
