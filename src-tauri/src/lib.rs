pub mod commands;
pub mod models;
pub mod utils;
pub mod services;

use tauri::Manager;
use std::sync::Mutex;

use crate::commands::metrics::{get_system_metrics};
use crate::commands::profile::{get_profile_mode, set_profile_mode, set_profile_mode_internal};
use crate::commands::system_commands::{get_top_processes};
use crate::commands::settings::{load_settings};
use crate::models::metrics::NetworkState;
use crate::utils::position::{position_window_bottom_right, restore_window_position, save_window_position};
use crate::utils::tray::{setup_tray};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(NetworkState::default()));
            let settings = load_settings(app.handle()).map_err(|e| e.to_string())?;
            set_profile_mode_internal(app.handle(), settings.active_profile.clone())?;
            restore_window_position(app.handle(), &settings)?;
            setup_tray(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let _ = save_window_position(&window.app_handle());
            }
        })
        .invoke_handler(tauri::generate_handler![get_system_metrics, position_window_bottom_right, get_top_processes, get_profile_mode, set_profile_mode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
