// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::load_pipe_config;
use commands::save_pipe_config;
use sidecar::SidecarManager;
use tauri::Config;
use tokio::sync::mpsc;
use serde_json::Value;
use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;
use tauri::Wry;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState},
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;
#[allow(unused_imports)]
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_store::{with_store, StoreCollection};
use tokio::sync::Mutex;
use tracing::{debug, error, info};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
mod analytics;

use crate::analytics::start_analytics;

mod commands;
mod sidecar;
mod server;
pub use commands::open_screen_capture_preferences;
pub use commands::reset_screen_permissions;
pub use commands::reset_all_pipes;
pub use sidecar::kill_all_sreenpipes;
pub use sidecar::spawn_screenpipe;
pub use server::spawn_server;

use screenpipe_file_watch::{FileWatcher, FileWatchConfig};
use sqlx::sqlite::SqlitePoolOptions;
use tauri::State;

pub struct SidecarState(Arc<tokio::sync::Mutex<Option<SidecarManager>>>);

fn get_base_dir(app: &tauri::AppHandle, custom_path: Option<String>) -> anyhow::Result<PathBuf> {
    let default_path = app.path().local_data_dir().unwrap().join("screenpipe");

    let local_data_dir = custom_path.map(PathBuf::from).unwrap_or(default_path);

    fs::create_dir_all(&local_data_dir.join("data"))?;
    Ok(local_data_dir)
}

fn show_main_window(app_handle: &tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        let _ = tauri::WebviewWindowBuilder::new(
            app_handle,
            "main",
            tauri::WebviewUrl::App("index.html".into())
        )
        .title("Screenpipe")
        .build();
    }
}

struct WatcherState(Arc<Mutex<Option<FileWatcher>>>);

#[tauri::command]
async fn start_watcher(
    state: State<'_, WatcherState>,
    watch_dir: String,
) -> Result<String, String> {
    let config = FileWatchConfig {
        watch_directories: vec![watch_dir],
        file_patterns: vec![
            "scnpip_*.png".to_string(),
            "scnpip_*.txt".to_string(),
        ],
    };

    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("screenpipe");
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;

    let database_url = format!("sqlite:{}/db.sqlite", data_dir.display());
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| e.to_string())?;

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
    .await
    .map_err(|e| e.to_string())?;

    let mut watcher = FileWatcher::new(&config, pool)
        .await
        .map_err(|e| e.to_string())?;
    
    watcher.start_watching(&config).map_err(|e| e.to_string())?;

    let mut state = state.0.lock().await;
    *state = Some(watcher);

    Ok("Watcher started successfully".to_string())
}

#[tauri::command]
async fn get_recent_files(
    state: State<'_, WatcherState>,
) -> Result<Vec<String>, String> {
    if let Some(watcher) = state.0.lock().await.as_ref() {
        let events = watcher.process_events().await.map_err(|e| e.to_string())?;
        Ok(events
            .into_iter()
            .map(|e| e.path.to_string_lossy().to_string())
            .collect())
    } else {
        Ok(vec![])
    }
}

#[tokio::main]
async fn main() {
    let _ = fix_path_env::fix();

    let sidecar_state = SidecarState(Arc::new(tokio::sync::Mutex::new(None)));

    let app = tauri::Builder::default()
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let windows = app.webview_windows();
            windows
                .values()
                .next()
                .expect("Sorry, no window found")
                .set_focus()
                .expect("Can't focus window!");
        }))
        .manage(sidecar_state)
        .manage(WatcherState(Arc::new(Mutex::new(None))))
        .invoke_handler(tauri::generate_handler![
            spawn_screenpipe,
            kill_all_sreenpipes,
            reset_screen_permissions,
            open_screen_capture_preferences,
            load_pipe_config,
            save_pipe_config,
            reset_all_pipes,
            start_watcher,
            get_recent_files
        ])
        .setup(|app| {
            // Logging setup
            let app_handle = app.handle();
            let base_dir = get_base_dir(&app_handle, None).expect("Failed to ensure local data directory");

            // Set up file appender
            let file_appender = RollingFileAppender::new(
                Rotation::NEVER,
                base_dir.clone(),
                "screenpipe-app.log",
            );

            // Create a custom layer for file logging
            let file_layer = tracing_subscriber::fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false)
                .with_filter(EnvFilter::new("info"));

            // Create a custom layer for console logging
            let console_layer = tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_filter(EnvFilter::new("debug"));

            // Initialize the tracing subscriber with both layers
            tracing_subscriber::registry()
                .with(sentry::integrations::tracing::layer())
                .with(file_layer)
                .with(console_layer)
                .init();

            // Initialize Sentry
            let _guard = sentry::init((
                "https://cf682877173997afc8463e5ca2fbe3c7@o4507617161314304.ingest.us.sentry.io/4507617170161664",
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    traces_sample_rate: 0.2,
                    ..Default::default()
                }
            ));

            // Windows-specific setup
            if cfg!(windows) {
                let exe_dir = env::current_exe()
                    .expect("Failed to get current executable path")
                    .parent()
                    .expect("Failed to get parent directory of executable")
                    .to_path_buf();
                let tessdata_path = exe_dir.join("tessdata");
                env::set_var("TESSDATA_PREFIX", tessdata_path);
            }

            // Autostart setup
            let autostart_manager = app.autolaunch();
            let _ = autostart_manager.enable();
            debug!(
                "registered for autostart? {}",
                autostart_manager.is_enabled().unwrap()
            );

            info!("Local data directory: {}", base_dir.display());

            // PostHog analytics setup
            let posthog_api_key = "phc_Bt8GoTBPgkCpDrbaIZzJIEYt0CrJjhBiuLaBck1clce".to_string();
            let interval_hours = 1;

            let path = base_dir.join("store.bin");
            if !path.exists() {
                let _ = File::create(path.clone()).unwrap();
            }

            // Tray setup
            if let Some(main_tray) = app.tray_by_id("screenpipe_main") {
                let show = MenuItemBuilder::with_id("show", "Show Screenpipe").build(app)?;
                let quit = MenuItemBuilder::with_id("quit", "Quit Screenpipe").build(app)?;
                let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;
                let _ = main_tray.set_menu(Some(menu));
                main_tray.on_menu_event(move |app_handle, event| match event.id().as_ref() {
                    "show" => {
                        show_main_window(&app_handle);
                    }
                    "quit" => {
                        println!("quit clicked");
                        app_handle.exit(0);
                    }
                    _ => (),
                });
                main_tray.on_tray_icon_event(move |tray, event| match event {
                    tauri::tray::TrayIconEvent::Click {
                        button,
                        button_state,
                        ..
                    } => {
                        if button == MouseButton::Left && button_state == MouseButtonState::Up {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            } else {
                                show_main_window(&app);
                            }
                        }
                    }
                    _ => {}
                });
            }

            // Store setup and analytics initialization
            let stores = app.state::<StoreCollection<Wry>>();
            let _ = with_store(app.handle().clone(), stores.clone(), path.clone(), |store| {
                if store.keys().count() == 0 {
                    store.insert("analyticsEnabled".to_string(), Value::Bool(true))?;
                    store.insert(
                        "config".to_string(),
                        serde_json::to_value(Config::default())?,
                    )?;
                    store.save()?;
                }
                Ok(())
            });

            let _ = with_store(app.handle().clone(), stores.clone(), path.clone(), |store| {
                store.save()?;

                let is_analytics_enabled = store
                    .get("analyticsEnabled")
                    .unwrap_or(&Value::Bool(true))
                    .as_bool()
                    .unwrap_or(true);

                let unique_id = store
                    .get("userId")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_else(|| {
                        let new_id = Uuid::new_v4().to_string();
                        store
                            .insert(
                                "userId".to_string(),
                                serde_json::Value::String(new_id.clone()),
                            )
                            .unwrap();
                        store.save().unwrap();
                        new_id
                    });

                if is_analytics_enabled {
                    match start_analytics(unique_id, posthog_api_key, interval_hours) {
                        Ok(analytics_manager) => {
                            app.manage(analytics_manager);
                        }
                        Err(e) => {
                            error!("Failed to start analytics: {}", e);
                        }
                    }
                }

                Ok(())
            });

            // Dev mode check and sidecar spawn
            let use_dev_mode = with_store(app.handle().clone(), stores.clone(), path.clone(), |store| {
                Ok(store
                    .get("devMode")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false))
            })
            .unwrap_or(false);

            let sidecar_manager = Arc::new(Mutex::new(SidecarManager::new()));
            app.manage(sidecar_manager.clone());

            let app_handle = app.handle().clone();

            if !use_dev_mode {
                tauri::async_runtime::spawn(async move {
                    let mut manager = sidecar_manager.lock().await;
                    if let Err(e) = manager.spawn(&app_handle).await {
                        error!("Failed to spawn initial sidecar: {}", e);
                    }

                    // Spawn a background task to check and restart periodically
                    let mut manager = sidecar_manager.lock().await;
                    if let Err(e) = manager.check_and_restart(&app_handle).await {
                        error!("Failed to restart sidecar: {}", e);
                    }
                });
            } else {
                debug!("Dev mode enabled, skipping sidecar spawn and restart");
            }

            // Inside the main function, after the `app.manage(port);` line, add:
            let server_shutdown_tx = spawn_server(app.handle().clone(), 11435);
            app.manage(server_shutdown_tx);

            // Add this custom activate handler
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Regular);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| match event {
        tauri::RunEvent::Ready { .. } => {
            debug!("Ready event");
        }
        tauri::RunEvent::ExitRequested { .. } => {
            debug!("ExitRequested event");
            
            // Add this to shut down the server
            if let Some(server_shutdown_tx) = app_handle.try_state::<mpsc::Sender<()>>() {
                let _ = server_shutdown_tx.send(());
            }
        }
        tauri::RunEvent::WindowEvent {
            label,
            event: tauri::WindowEvent::Focused(focused),
            ..
        } => {
            if label == "main" && focused {
                let window = app_handle.get_webview_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }
        #[cfg(target_os = "macos")]
        tauri::RunEvent::Reopen { has_visible_windows, .. } => {
            if !has_visible_windows {
                show_main_window(&app_handle);
            }
        }
        _ => {}
    });
}
