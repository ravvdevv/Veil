use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

#[tauri::command]
pub fn update_toggle_key(app: tauri::AppHandle, new_key: String) {
    let _ = app.global_shortcut().unregister_all();
    setup_hotkeys(&app, &new_key);
}

pub fn setup_hotkeys(app: &AppHandle, toggle_key: &str) {
    let app_handle = app.clone();
    
    let shortcut = match toggle_key.parse::<Shortcut>() {
        Ok(s) => s,
        Err(_) => return,
    };
    
    let _ = app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
        if event.state() == ShortcutState::Pressed {
            if let Some(window) = app_handle.get_webview_window("main") {
                if window.is_visible().unwrap_or(false) {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
    });
}
