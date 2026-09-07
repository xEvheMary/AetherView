pub mod commands;
pub mod models;
pub mod utils;
pub mod services;

use tauri::Manager;
use std::sync::Mutex;

use crate::commands::metrics::{get_system_metrics};
use crate::commands::profile::{set_profile_mode};
use crate::commands::system_commands::{get_top_processes};
use crate::models::metrics::NetworkState;
use crate::utils::position::{position_window_bottom_right};
use crate::utils::tray::{setup_tray};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(NetworkState::default()));
            setup_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_system_metrics, position_window_bottom_right, get_top_processes, set_profile_mode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
