use tauri::Manager;

use crate::utils::position::position_window_bottom_right_internal;

#[tauri::command]
pub fn set_profile_mode(
    app: tauri::AppHandle,
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
        _ => {
            let _ = window.set_size(
                tauri::Size::Logical(
                    tauri::LogicalSize::new(240.0, 320.0)
                )
            );
            window.set_resizable(true).unwrap();
        }
    }
    position_window_bottom_right_internal(&window)?;
    Ok(())
}