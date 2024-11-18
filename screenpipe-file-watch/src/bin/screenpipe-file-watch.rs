use screenpipe_file_watch::{FileWatcher, FileWatchConfig};
use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;
use std::path::PathBuf;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get default data directory
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("screenpipe");
    std::fs::create_dir_all(&data_dir)?;

    // Database setup
    let database_url = format!("sqlite:{}/db.sqlite", data_dir.display());
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Initialize database schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS frames (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME NOT NULL,
            file_path TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS ocr_text (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            frame_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            FOREIGN KEY (frame_id) REFERENCES frames(id)
        );

        CREATE INDEX IF NOT EXISTS idx_frames_timestamp ON frames(timestamp);
        CREATE INDEX IF NOT EXISTS idx_ocr_text_frame_id ON ocr_text(frame_id);
        "#,
    )
    .execute(&pool)
    .await?;

    let config = FileWatchConfig {
        watch_directories: vec![
            "C:\\Users\\user\\Pictures\\Screenshots\\scnpip".to_string()
        ],
        file_patterns: vec![
            "scnpip_*.png".to_string(),
            "scnpip_*.txt".to_string()
        ],
    };

    let mut watcher = FileWatcher::new(&config, pool).await?;
    watcher.start_watching(&config)?;

    info!("File watcher started. Press Ctrl+C to exit.");

    loop {
        let events = watcher.process_events().await?;
        for event in events {
            info!("Processed file: {:?}", event);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}