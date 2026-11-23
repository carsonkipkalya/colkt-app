pub mod commands;
pub mod db;
pub mod error;

use tauri::Manager;

use crate::error::{AppError, ErrorCode};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::app::ping])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let data_dir = app.path().app_data_dir().map_err(|_| {
                log::error!("failed to resolve application data directory");
                AppError::new(ErrorCode::Internal, "failed to initialize application data")
            })?;
            let database = db::Database::open(&data_dir.join("colkt.db"))?;
            app.manage(database);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
