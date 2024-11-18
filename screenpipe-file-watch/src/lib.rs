use anyhow::Result;
use chrono::{DateTime, Utc};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use tracing::{info, error, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileWatchConfig {
    pub watch_directories: Vec<String>,
    pub file_patterns: Vec<String>,
}

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: std::sync::mpsc::Receiver<notify::Result<notify::Event>>,
    patterns: Vec<Regex>,
    db: SqlitePool,
}

#[derive(Debug)]
pub struct FileEvent {
    pub path: PathBuf,
    pub timestamp: DateTime<Utc>,
    pub event_type: FileEventType,
}

#[derive(Debug)]
pub enum FileEventType {
    Screenshot,
    OcrText,
}

impl FileWatcher {
    pub async fn new(config: &FileWatchConfig, db: SqlitePool) -> Result<Self> {
        let (tx, rx) = channel();
        let watcher = RecommendedWatcher::new(
            move |res| {
                if let Err(e) = tx.send(res) {
                    error!("Failed to send file event: {}", e);
                }
            },
            Config::default(),
        )?;

        let patterns = config.file_patterns
            .iter()
            .map(|p| {
                Regex::new(&p.replace("*", ".*"))
                    .map_err(|e| anyhow::anyhow!("Invalid pattern {}: {}", p, e))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(FileWatcher { watcher, rx, patterns, db })
    }

    pub fn start_watching(&mut self, config: &FileWatchConfig) -> Result<()> {
        for dir in &config.watch_directories {
            info!("Starting to watch directory: {}", dir);
            self.watcher.watch(Path::new(dir), RecursiveMode::Recursive)?;
        }
        Ok(())
    }

    pub async fn process_events(&self) -> Result<Vec<FileEvent>> {
        let mut events = Vec::new();
        
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Ok(event) => {
                    if let notify::EventKind::Create(_) = event.kind {
                        for path in event.paths {
                            if let Some(event) = self.process_file(&path).await? {
                                events.push(event);
                            }
                        }
                    }
                }
                Err(e) => warn!("Error receiving file event: {}", e),
            }
        }

        Ok(events)
    }

    async fn process_file(&self, path: &Path) -> Result<Option<FileEvent>> {
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => return Ok(None),
        };

        if !self.patterns.iter().any(|p| p.is_match(file_name)) {
            return Ok(None);
        }

        let event_type = if file_name.ends_with(".png") {
            FileEventType::Screenshot
        } else if file_name.ends_with(".txt") {
            FileEventType::OcrText
        } else {
            return Ok(None);
        };

        // Extract timestamp from filename (assuming format: scnpip_YYYYMMDDHHMMSS)
        let timestamp = chrono::Utc::now(); // You might want to parse from filename instead

        let event = FileEvent {
            path: path.to_path_buf(),
            timestamp,
            event_type,
        };

        self.store_event(&event).await?;

        Ok(Some(event))
    }

    async fn store_event(&self, event: &FileEvent) -> Result<()> {
        match event.event_type {
            FileEventType::Screenshot => {
                sqlx::query(
                    "INSERT INTO frames (timestamp, file_path) VALUES (?, ?)"
                )
                .bind(event.timestamp)
                .bind(event.path.to_string_lossy().as_ref())
                .execute(&self.db)
                .await?;
            }
            FileEventType::OcrText => {
                // Assuming the OCR text file corresponds to a PNG with the same base name
                let png_path = event.path.with_extension("png");
                if let Ok(text) = std::fs::read_to_string(&event.path) {
                    if let Some(frame_id) = self.get_frame_id(&png_path).await? {
                        sqlx::query(
                            "INSERT INTO ocr_text (frame_id, text) VALUES (?, ?)"
                        )
                        .bind(frame_id)
                        .bind(text)
                        .execute(&self.db)
                        .await?;
                    }
                }
            }
        }

        Ok(())
    }

    async fn get_frame_id(&self, png_path: &Path) -> Result<Option<i64>> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT id FROM frames WHERE file_path = ?"
        )
        .bind(png_path.to_string_lossy().as_ref())
        .fetch_optional(&self.db)
        .await?;

        Ok(result)
    }
}