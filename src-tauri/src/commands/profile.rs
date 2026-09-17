use tauri::Manager;
use crate::commands::settings;
use crate::utils::position::{position_window_internal};

#[tauri::command]
pub fn get_profile_mode(app: tauri::AppHandle) -> Result<String, String> {
    let settings = settings::load_settings(&app)?;
    Ok(settings.general.active_profile)
}

#[tauri::command]
pub fn set_profile_mode(
    app: tauri::AppHandle,
    mode: String,
) -> Result<(), String> {
    set_profile_mode_internal(&app, mode)
}

pub fn set_profile_mode_internal(
    app: &tauri::AppHandle,
    mode: String,
) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Window not found")?;

    match mode.as_str() {
        "minimal" => {
            let _ = window.set_size(
                tauri::Size::Logical(
                    tauri::LogicalSize::new(500.0, 80.0)
                )
            );
            window.set_resizable(false).unwrap();
        }
        // "form" => {
        //     let _ = window.set_size(
        //         tauri::Size::Logical(
        //             tauri::LogicalSize::new(300.0, 360.0)
        //         )
        //     );
        //     window.set_resizable(true).unwrap();
        // }
        _ => {
            let _ = window.set_size(
                tauri::Size::Logical(
                    tauri::LogicalSize::new(240.0, 320.0)
                )
            );
            window.set_resizable(true).unwrap();
        }
    }
    save_profile_mode(app, mode)?;
    position_window_internal(&window, None, None)?;
    Ok(())
}

pub fn save_profile_mode(app: &tauri::AppHandle, mode: String) -> Result<(), String> {
    let mut settings = settings::load_settings(app)?;
    if mode != "form" {
        settings.general.active_profile = mode;
    }
    settings::save_settings(app, &settings)?;
    Ok(())
}