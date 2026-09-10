use std::path::PathBuf;
use tauri::Manager;

pub fn get_config_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let config_dir = app
        .path().app_data_dir().map_err(|e| format!("Failed to get app data directory: {}", e))?;
    Ok(config_dir)
}