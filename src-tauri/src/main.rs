// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pty;
mod config;
mod hotkeys;

use tauri::Manager;

#[tauri::command]
fn restart_app(app_handle: tauri::AppHandle) {
    app_handle.restart();
}

#[tauri::command]
async fn snap_window(window: tauri::Window, side: String) {
    let monitor = match window.current_monitor() {
        Ok(Some(m)) => m,
        _ => return,
    };
    
    let m_size = monitor.size();
    let m_pos = monitor.position();
    
    // Use work_area to avoid taskbar interference
    let _work_area = monitor.size(); // Fallback
    
    let mut new_x = m_pos.x;
    let mut new_y = m_pos.y;
    let new_w;
    let new_h;

    match side.as_str() {
        "left" => {
            new_w = m_size.width / 2;
            new_h = m_size.height;
        }
        "right" => {
            new_x = m_pos.x + (m_size.width / 2) as i32;
            new_w = m_size.width / 2;
            new_h = m_size.height;
        }
        "top" => {
            new_w = 800; // Default width for top snap
            new_h = 500; // Default height
            new_x = m_pos.x + (m_size.width / 2) as i32 - (new_w / 2) as i32;
            new_y = m_pos.y;
        }
        "center" => {
            new_w = 800;
            new_h = 500;
            new_x = m_pos.x + (m_size.width / 2) as i32 - (new_w / 2) as i32;
            new_y = m_pos.y + (m_size.height / 2) as i32 - (new_h / 2) as i32;
        }
        _ => {
            new_w = 800;
            new_h = 500;
        }
    }

    let _ = window.set_size(tauri::PhysicalSize { width: new_w, height: new_h });
    let _ = window.set_position(tauri::PhysicalPosition { x: new_x, y: new_y });
}

#[tauri::command]
async fn resize_window_to_scale(window: tauri::Window, font_size: u32) {
    let monitor = match window.current_monitor() {
        Ok(Some(m)) => m,
        _ => return,
    };
    
    // Base size for 14px font
    let base_w = 700.0;
    let base_h = 400.0;
    
    let scale = font_size as f32 / 14.0;
    
    let new_w = (base_w * scale) as u32;
    let new_h = (base_h * scale) as u32;
    
    // Ensure we don't exceed monitor size
    let m_size = monitor.size();
    let final_w = new_w.min(m_size.width);
    let final_h = new_h.min(m_size.height);

    let _ = window.set_size(tauri::PhysicalSize { width: final_w, height: final_h });
}

#[tauri::command]
async fn move_window(window: tauri::Window, dx: i32, dy: i32) {
    let pos = window.outer_position().unwrap();
    let _ = window.set_position(tauri::PhysicalPosition {
        x: pos.x + dx,
        y: pos.y + dy,
    });
}

#[tauri::command]
async fn resize_window(window: tauri::Window, dw: i32, dh: i32) {
    let size = window.outer_size().unwrap();
    let _ = window.set_size(tauri::PhysicalSize {
        width: (size.width as i32 + dw) as u32,
        height: (size.height as i32 + dh) as u32,
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // Load config
            let config = config::load_config(&app.handle());
            let pty_state = pty::setup_pty(&app.handle(), &config.last_dir);
            app.manage(pty_state);
            
            // Setup Hotkeys
            hotkeys::setup_hotkeys(&app.handle(), &config.toggle_key);
            
            // Window setup (ensure frameless and transparent via tauri.conf.json)
            // But we can also do some adjustments here if needed.
            let window = app.get_webview_window("main").unwrap();
            
            // Apply vibrancy for a sleek look
            #[cfg(target_os = "windows")]
            {
                use window_vibrancy::{apply_mica, apply_acrylic};
                // Try Mica first (Windows 11), fallback to Acrylic
                if let Err(_) = apply_mica(&window, None) {
                    let _ = apply_acrylic(&window, Some((0, 0, 0, 0)));
                }
            }
            
            // Set initial state from config
            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: config.window.width as u32,
                height: config.window.height as u32,
            }));
            let _ = window.set_always_on_top(config.window.always_on_top);
            
            // Auto-hide logic
            let app_handle = app.handle().clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(false) = event {
                    let config = config::load_config(&app_handle);
                    if config.auto_hide {
                        let win = app_handle.get_webview_window("main").unwrap();
                        let _ = win.hide();
                    }
                }
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            pty::write_to_pty,
            pty::resize_pty,
            config::get_settings,
            config::save_settings,
            hotkeys::update_toggle_key,
            move_window,
            resize_window,
            resize_window_to_scale,
            snap_window,
            restart_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
