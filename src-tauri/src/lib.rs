mod commands;
mod db;
mod models;
mod scanner;
mod classifier;
mod cleanup;

use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("onesweep.db");
            let conn = rusqlite::Connection::open(&db_path)
                .expect("Failed to open database");
            db::run_migrations(&conn).expect("Failed to run migrations");
            app.manage(AppState { db: Mutex::new(conn) });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::scan::start_scan,
            commands::scan::get_cached_scan,
            commands::cleanup::start_cleanup,
            commands::audit::get_audit_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
