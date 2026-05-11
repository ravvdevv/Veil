# Veil

High-performance, glassmorphic terminal overlay for Windows. Engineered for low-latency command-line workflows with a "Scientific UI" philosophy.

## Technical Foundation

- **Backend**: Rust-powered PTY engine via `portable-pty`.
- **Frontend**: Svelte 5 (Runes) with GPU-accelerated rendering.
- **Aesthetics**: Native Windows 11 Mica/Acrylic translucency.
- **Shell**: Optimized for `cmd.exe` by default.

## Scientific UI & Physics

Veil uses a deterministic mathematical layout to ensure stability across all resolutions:
- **78/8 Ratio**: 78% viewport output area, 8% command interaction zone.
- **Inertia Damping**: Physics-based scrolling interpolation (`damping = 0.15`).
- **Adaptive Clamp**: Font scaling bound by `clamp(14px, 1.2vw, 18px)`.
- **Hardware Sync**: Forced GPU composition via `translateZ(0)`.

## Features

- **Ghost Mode**: Auto-hide on focus loss for seamless task switching.
- **Nudge Movement**: Precision window positioning (20px/100px steps).
- **Draggable Settings**: Fully movable glassmorphic configuration modal.
- **Auto-Copy**: Instant clipboard sync on terminal selection.
- **Zero-Bake Layout**: Seamless edge-to-edge terminal rendering.

## Keybindings

| Shortcut | Action |
| :--- | :--- |
| `Alt + Space` | Toggle Veil Visibility |
| `Ctrl + M` | Enter Movement Mode |
| `Ctrl + Arrows` | Precision Nudge (20px) |
| `Ctrl + Shift + Arrows`| Large Nudge (100px) |
| `Ctrl + ,` | Toggle Draggable Settings |
| `Esc` | Exit HUD / Close Modal |

## Development

```bash
bun install
bun tauri dev
```

## License

MIT © 2026 The Veil Authors
