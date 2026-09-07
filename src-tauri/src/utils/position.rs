
#[tauri::command]
pub fn position_window_bottom_right(window: tauri::Window) -> Result<(), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No monitor found".to_string())?;

    let work_area = monitor.work_area();

    let physical_bottom = work_area.position.y + (work_area.size.height as i32);
    let physical_right = work_area.position.x + (work_area.size.width as i32);

    let window_size = window.outer_size().map_err(|e| e.to_string())?;

    let margin = 20;

    let x = physical_right
        - window_size.width as i32
        - margin;

    let y = physical_bottom
        - window_size.height as i32
        - (margin * 2) as i32;

    window
        .set_position(tauri::Position::Physical(
            tauri::PhysicalPosition::new(x, y),
        ))
        .map_err(|e| e.to_string())?;

    Ok(())
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

    let margin = 20;

    let x = physical_right
        - window_size.width as i32
        - margin;

    let y = physical_bottom
        - window_size.height as i32
        - (margin * 2) as i32;

    window
        .set_position(tauri::Position::Physical(
            tauri::PhysicalPosition::new(x, y),
        ))
        .map_err(|e| e.to_string())?;

    Ok(())
}