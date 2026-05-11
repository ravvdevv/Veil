use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub toggle_key: String,
    pub opacity: f32,
    pub blur: bool,
    pub font_size: u32,
    pub font_family: String,
    pub auto_hide: bool,
    pub auto_copy: bool,
    pub last_dir: String,
    pub current_theme: String,
    pub themes: Vec<Theme>,
    pub window: WindowConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub name: String,
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub accent: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    pub width: f64,
    pub height: f64,
    pub always_on_top: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            toggle_key: "Ctrl+`".to_string(),
            opacity: 0.85,
            blur: true,
            font_size: 14,
            font_family: "JetBrains Mono".to_string(),
            auto_hide: true,
            auto_copy: true,
            last_dir: std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\".to_string()),
            current_theme: "Minimal Dark".to_string(),
            themes: vec![
                Theme {
                    name: "Minimal Dark".to_string(),
                    background: "#0a0a0a".to_string(),
                    foreground: "#ededed".to_string(),
                    cursor: "#ffffff".to_string(),
                    accent: "#ffffff".to_string(),
                    black: "#000000".to_string(),
                    red: "#ff5555".to_string(),
                    green: "#50fa7b".to_string(),
                    yellow: "#f1fa8c".to_string(),
                    blue: "#bd93f9".to_string(),
                    magenta: "#ff79c6".to_string(),
                    cyan: "#8be9fd".to_string(),
                    white: "#bfbfbf".to_string(),
                },
                Theme {
                    name: "Nord".to_string(),
                    background: "#2e3440".to_string(),
                    foreground: "#d8dee9".to_string(),
                    cursor: "#d8dee9".to_string(),
                    accent: "#88c0d0".to_string(),
                    black: "#3b4252".to_string(),
                    red: "#bf616a".to_string(),
                    green: "#a3be8c".to_string(),
                    yellow: "#ebcb8b".to_string(),
                    blue: "#81a1c1".to_string(),
                    magenta: "#b48ead".to_string(),
                    cyan: "#88c0d0".to_string(),
                    white: "#e5e9f0".to_string(),
                },
                Theme {
                    name: "Catppuccin Mocha".to_string(),
                    background: "#1e1e2e".to_string(),
                    foreground: "#cdd6f4".to_string(),
                    cursor: "#f5e0dc".to_string(),
                    accent: "#cba6f7".to_string(),
                    black: "#45475a".to_string(),
                    red: "#f38ba8".to_string(),
                    green: "#a6e3a1".to_string(),
                    yellow: "#f9e2af".to_string(),
                    blue: "#89b4fa".to_string(),
                    magenta: "#f5c2e7".to_string(),
                    cyan: "#94e2d5".to_string(),
                    white: "#bac2de".to_string(),
                },
                Theme {
                    name: "Gruvbox Dark".to_string(),
                    background: "#282828".to_string(),
                    foreground: "#ebdbb2".to_string(),
                    cursor: "#ebdbb2".to_string(),
                    accent: "#fabd2f".to_string(),
                    black: "#282828".to_string(),
                    red: "#cc241d".to_string(),
                    green: "#98971a".to_string(),
                    yellow: "#d79921".to_string(),
                    blue: "#458588".to_string(),
                    magenta: "#b16286".to_string(),
                    cyan: "#689d6a".to_string(),
                    white: "#a89984".to_string(),
                },
                Theme {
                    name: "Tokyo Night".to_string(),
                    background: "#1a1b26".to_string(),
                    foreground: "#a9b1d6".to_string(),
                    cursor: "#c0caf5".to_string(),
                    accent: "#7aa2f7".to_string(),
                    black: "#15161e".to_string(),
                    red: "#f7768e".to_string(),
                    green: "#9ece6a".to_string(),
                    yellow: "#e0af68".to_string(),
                    blue: "#7aa2f7".to_string(),
                    magenta: "#bb9af7".to_string(),
                    cyan: "#7dcfff".to_string(),
                    white: "#a9b1d6".to_string(),
                },
            ],
            window: WindowConfig {
                width: 700.0,
                height: 400.0,
                always_on_top: true,
            },
        }
    }
}

pub fn get_config_path(app: &AppHandle) -> PathBuf {
    app.path().app_config_dir().unwrap().join("config.toml")
}

pub fn load_config(app: &AppHandle) -> Config {
    let path = get_config_path(app);
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        let config = Config::default();
        save_config(app, &config);
        config
    }
}

pub fn save_config(app: &AppHandle, config: &Config) {
    let path = get_config_path(app);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let content = toml::to_string_pretty(config).unwrap();
    let _ = fs::write(path, content);
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Config {
    load_config(&app)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, config: Config) {
    save_config(&app, &config);
}
