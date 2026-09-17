use tauri_plugin_autostart::ManagerExt;

pub fn apply_autostart(
    app: &tauri::AppHandle,
    enabled: bool,
) -> Result<(), String> {
    let manager = app.autolaunch();
    let current_status = manager.is_enabled().map_err(|e| format!("Failed to get autostart status: {e}"))?;
    let status_changed = enabled != current_status;
    if status_changed {
        if enabled {
            manager
                .enable()
                .map_err(|e| format!("Failed to enable autostart: {e}"))?;
        } else {
            manager
                .disable()
                .map_err(|e| format!("Failed to disable autostart: {e}"))?;
        }
    }
    Ok(())
}