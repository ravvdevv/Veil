# Veil

High-performance, glassmorphic terminal overlay for Windows. Built with Tauri v2, Svelte 5, and Rust.

## Features

- **Native Glassmorphism**: Mica/Acrylic transparency effects.
- **Ghost Mode**: Auto-hide on focus loss for seamless workflows.
- **Synchronized Zoom**: Deterministic window and font scaling.
- **Nudge Movement**: Precision window positioning via hotkeys.
- **Zero Latency**: Rust-powered PTY backend.

## Keybindings

| Shortcut | Action |
| :--- | :--- |
| `Alt + Space` | Toggle Veil |
| `Ctrl + M` | Movement Mode |
| `Ctrl + Arrows` | Nudge (20px) |
| `Ctrl + Shift + Arrows`| Large Nudge (100px) |
| `Ctrl + + / -` | Synchronized Zoom |
| `Ctrl + 0` | Reset Zoom |
| `Esc` | Close HUD / Settings |

## Quick Start

```bash
bun install
bun tauri dev
```

## Configuration

Settings are stored in `src-tauri/config.toml`. Keybindings, themes, and transparency can be adjusted via the in-app Settings menu (`Ctrl + ,` or Header Icon).

## License

MIT
