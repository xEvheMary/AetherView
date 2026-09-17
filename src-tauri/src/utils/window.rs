use tauri::{Manager, WebviewUrl};

#[tauri::command]
pub fn open_settings(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        open_settings_window(&app);
    });
}

pub fn open_settings_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    let url = WebviewUrl::App("settings.html".into());
    let _ = tauri::WebviewWindowBuilder::new(app, "settings", url)
        .title("Settings")
        .inner_size(500.0, 350.0)
        .shadow(false)
        .build();
}