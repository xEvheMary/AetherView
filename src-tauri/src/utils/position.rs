use crate::commands::settings::{load_settings, save_settings};
use tauri::Manager;

#[tauri::command]
pub fn position_window_bottom_right(window: tauri::Window) -> Result<(), String> {
    position_window_bottom_right_internal(
        &window
            .get_webview_window("main")
            .ok_or("Main window not found")?,
    )
}

pub fn position_window_bottom_right_internal(window: &tauri::WebviewWindow) -> Result<(), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No monitor found".to_string())?;

    let work_area = monitor.work_area();

    let physical_bottom = work_area.position.y + (work_area.size.height as i32);
    let physical_right = work_area.position.x + (work_area.size.width as i32);

    let window_size = window.outer_size().map_err(|e| e.to_string())?;

    let margin = 5;

    let x = physical_right - window_size.width as i32 - margin;

    let y = physical_bottom - window_size.height as i32 - margin;

    window
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
            x, y,
        )))
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn position_window_internal(
    window: &tauri::WebviewWindow,
    x: Option<i32>,
    y: Option<i32>,
) -> Result<(), String> {
    let (overflow_x, overflow_y) = calculate_overflow_position(&window.app_handle())?;
    let x = x.unwrap_or(window.outer_position().map_err(|e| e.to_string())?.x);
    let y = y.unwrap_or(window.outer_position().map_err(|e| e.to_string())?.y);
    window
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
            x - overflow_x,
            y - overflow_y,
        )))
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn restore_window_position(
    app: &tauri::AppHandle,
    settings: &crate::models::settings::AppSettings,
) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;
    if let (Some(x), Some(y)) = (settings.general.window_x, settings.general.window_y) {
        position_window_internal(&window, Some(x), Some(y))?;
    } else {
        position_window_bottom_right_internal(&window)?;
    }
    Ok(())
}

pub fn save_window_position(app: &tauri::AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    let position = window
        .outer_position()
        .map_err(|e| format!("Failed to get window position: {}", e))?;

    let mut settings = load_settings(app)?;
    settings.general.window_x = Some(position.x);
    settings.general.window_y = Some(position.y);
    save_settings(app, &settings)?;

    Ok(())
}

pub fn calculate_overflow_position(app: &tauri::AppHandle) -> Result<(i32, i32), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No monitor found".to_string())?;

    let work_area = monitor.work_area();

    let overflow_x = {
        let window_position = window
            .outer_position()
            .map_err(|e| format!("Failed to get window position: {}", e))?;
        let window_size = window
            .outer_size()
            .map_err(|e| format!("Failed to get window size: {}", e))?;
        if window_position.x < work_area.position.x && work_area.position.x == 0 {
            work_area.position.x
        } else if work_area.position.x > window_position.x && work_area.position.x > 0 {
            //
            (window_position.x + window_size.width as i32) - work_area.position.x
        } else if window_position.x + window_size.width as i32
            > work_area.position.x + work_area.size.width as i32
        {
            (window_position.x + window_size.width as i32)
                - (work_area.position.x + work_area.size.width as i32)
        } else {
            0
        }
    };
    let overflow_y = {
        let window_position = window
            .outer_position()
            .map_err(|e| format!("Failed to get window position: {}", e))?;
        if window_position.y < work_area.position.y && work_area.position.y == 0 {
            work_area.position.y
        } else if work_area.position.y > window_position.y && work_area.position.y > 0 {
            (window_position.y
                + window
                    .outer_size()
                    .map_err(|e| format!("Failed to get window size: {}", e))?
                    .height as i32)
                - work_area.position.y
        } else if window_position.y
            + window
                .outer_size()
                .map_err(|e| format!("Failed to get window size: {}", e))?
                .height as i32
            > work_area.position.y + work_area.size.height as i32
        {
            (window_position.y
                + window
                    .outer_size()
                    .map_err(|e| format!("Failed to get window size: {}", e))?
                    .height as i32)
                - (work_area.position.y + work_area.size.height as i32)
        } else {
            0
        }
    };

    Ok((overflow_x, overflow_y))
}
