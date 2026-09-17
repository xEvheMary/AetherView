use std::sync::{Arc, Mutex};
use tauri::{
    App, Emitter, Manager, image::Image, include_image, menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem}, tray::{TrayIconBuilder, TrayIconEvent},
};

use crate::utils::window::open_settings_window;

pub fn setup_tray(app: &mut App) -> tauri::Result<()> {
    let click_through_state = Arc::new(Mutex::new(false));
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
    let click_through = CheckMenuItem::with_id(
        app,
        "click_through",
        "Click Through",
        true,
        false,
        None::<&str>,
    )?;

    let _separator = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&show, &click_through, &settings, &_separator, &quit])?;

    const ICON: Image = include_image!("./icons/icon.png");
    let click_through_state = click_through_state.clone();
    let click_through_menu = click_through.clone();
    TrayIconBuilder::new()
        .icon(ICON)
        .menu(&menu)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::DoubleClick { .. } = event {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "click_through" => {
                if let Some(window) = app.get_webview_window("main") {
                    let mut enabled = click_through_state.lock().unwrap();
                    *enabled = !*enabled;
                    if let Err(e) = window.set_ignore_cursor_events(*enabled) {
                        eprint!("Failed to set ignore cursor events: {:?}", e);
                        *enabled = !*enabled;
                    } else {
                        window.set_always_on_top(*enabled).unwrap();
                        let _ = click_through_menu.set_checked(*enabled);
                    }
                }
            }
            "settings" => {
                open_settings_window(app);
            }
            "quit" => {
                app.emit("save_settings_before_close", ()).unwrap();
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
