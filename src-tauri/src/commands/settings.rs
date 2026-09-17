use crate::models::settings::AppSettings;
use crate::utils::{data::get_config_path, app::apply_autostart};
use serde_json;
use tauri::Emitter;
use std::fs;

#[tauri::command]
pub fn get_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    load_settings(&app)
}

#[tauri::command]
pub fn store_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    save_settings(&app, &settings)?;
    apply_autostart(&app, settings.general.run_on_startup)?;
    app.emit("setting_saved", "").unwrap();
    Ok(())
}

#[tauri::command]
pub fn get_theme(app: tauri::AppHandle) -> Result<String, String> {
    let settings = load_settings(&app)?;
    Ok(settings.appearance.theme)
}

#[tauri::command]
pub fn set_theme(app: tauri::AppHandle, theme: String) -> Result<(), String> {
    let mut settings = load_settings(&app)?;
    settings.appearance.theme = theme;
    save_settings(&app, &settings)
}

pub fn load_settings(app: &tauri::AppHandle) -> Result<AppSettings, String> {
    let settings_path = get_config_path(app)?.join("settings.json");

    if !settings_path.exists() {
        let settings = AppSettings::default();

        fs::create_dir_all(settings_path.parent().unwrap()).map_err(|e| e.to_string())?;

        let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;

        fs::write(&settings_path, json).map_err(|e| e.to_string())?;

        return Ok(settings);
    }

    let content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;

    serde_json::from_str(&content).map_err(|e| e.to_string())
}

pub fn save_settings(app: &tauri::AppHandle, settings: &AppSettings) -> Result<(), String> {
    let settings_path = get_config_path(app)?.join("settings.json");

    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;

    fs::write(settings_path, json).map_err(|e| e.to_string())?;

    Ok(())
}
