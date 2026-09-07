use std::fs;
use std::path::PathBuf;
use serde_json;
use tauri::Manager;
use crate::models::settings::AppSettings;

fn get_config_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let config_dir = app
        .path().app_data_dir().map_err(|e| format!("Failed to get app data directory: {}", e))?;
    Ok(config_dir)
}

pub fn load_settings(app: &tauri::AppHandle) -> Result<AppSettings, String> {
    let settings_path = get_config_path(app)?.join("settings.json");

    if !settings_path.exists() {
        let settings = AppSettings::default();

        fs::create_dir_all(
            settings_path.parent().unwrap()
        ).map_err(|e| e.to_string())?;

        let json = serde_json::to_string_pretty(&settings)
            .map_err(|e| e.to_string())?;

        fs::write(&settings_path, json)
            .map_err(|e| e.to_string())?;

        return Ok(settings);
    }

    let content = fs::read_to_string(&settings_path)
        .map_err(|e| e.to_string())?;

    serde_json::from_str(&content)
        .map_err(|e| e.to_string())
}

pub fn save_settings(
    app: &tauri::AppHandle,
    settings: &AppSettings
) -> Result<(), String> {
    let settings_path = get_config_path(app)?.join("settings.json");

    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| e.to_string())?;

    fs::write(settings_path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}