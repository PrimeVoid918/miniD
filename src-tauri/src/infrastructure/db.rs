use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite://my_db.sqlite").await.unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS history (id INTEGER PRIMARY KEY, url TEXT, downloaded_at TEXT)",
    )
    .execute(&pool)
    .await
    .unwrap();
    pool
}
