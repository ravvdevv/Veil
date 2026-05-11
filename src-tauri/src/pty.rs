use parking_lot::Mutex;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::sync::Arc;
use std::thread;
use tauri::{AppHandle, Emitter};

pub struct PtyState {
    pub writer: Mutex<Box<dyn Write + Send>>,
    pub master: Mutex<Box<dyn portable_pty::MasterPty + Send>>,
}

#[tauri::command]
pub fn write_to_pty(state: tauri::State<'_, Arc<PtyState>>, data: String) {
    let mut writer = state.writer.lock();
    let _ = writer.write_all(data.as_bytes());
    let _ = writer.flush();
}

#[tauri::command]
pub fn resize_pty(state: tauri::State<'_, Arc<PtyState>>, rows: u16, cols: u16) {
    let master = state.master.lock();
    let _ = master.resize(PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
    });
}

pub fn setup_pty(app: &AppHandle, cwd: &str) -> Arc<PtyState> {
    let pty_system = native_pty_system();
    
    // Determine default shell
    let shell_string;
    let shell = if cfg!(target_os = "windows") {
        "cmd.exe"
    } else {
        shell_string = std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string());
        &shell_string
    };

    let pty_pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("failed to open pty");

    let mut cmd = CommandBuilder::new(shell);
    cmd.cwd(cwd);
    let mut child = pty_pair.slave.spawn_command(cmd).expect("failed to spawn shell");

    let master = pty_pair.master;
    let writer = master.take_writer().expect("failed to get writer");
    let mut reader = master.try_clone_reader().expect("failed to get reader");

    let app_handle_output = app.clone();
    thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = app_handle_output.emit("pty-data", data);
                }
                Ok(_) => break,
                Err(_) => break,
            }
        }
    });

    let app_handle_exit = app.clone();
    thread::spawn(move || {
        let _ = child.wait();
        app_handle_exit.exit(0);
    });

    Arc::new(PtyState {
        writer: Mutex::new(writer),
        master: Mutex::new(master),
    })
}
