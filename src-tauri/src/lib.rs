pub mod commands;
pub mod models;
pub mod utils;
pub mod services;

use tauri::Manager;
use std::sync::Mutex;

use crate::commands::metrics::{get_system_metrics};
use crate::commands::profile::{get_profile_mode, set_profile_mode, set_profile_mode_internal};
use crate::commands::system_commands::{get_top_processes};
use crate::commands::monitor_commands::{get_monitor_state, add_monitor_targets,load_monitor, initialize_monitor_thread};
use crate::commands::settings::{get_settings, store_settings,get_theme, load_settings};
use crate::models::{metrics::NetworkState, monitor::MonitorState};
use crate::utils::position::{position_window_bottom_right, restore_window_position, save_window_position};
use crate::utils::tray::{setup_tray};
use crate::utils::window::{open_settings};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(NetworkState::default()));
            app.manage(Mutex::new(MonitorState::default()));
            let settings = load_settings(app.handle()).map_err(|e| e.to_string())?;
            set_profile_mode_internal(app.handle(), settings.general.active_profile.clone())?;
            restore_window_position(app.handle(), &settings)?;
            let _ = load_monitor(app.handle()).map_err(|e| e.to_string())?;
            initialize_monitor_thread(app.handle().clone());
            setup_tray(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    match window.label() {
                        "main" => {
                            let _ = save_window_position(&window.app_handle());
                        }
                        "settings" => {
                            api.prevent_close();
                            let _ = window.hide();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![get_system_metrics, position_window_bottom_right, get_top_processes, get_profile_mode, set_profile_mode, get_monitor_state, add_monitor_targets, get_settings, store_settings, get_theme, open_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
