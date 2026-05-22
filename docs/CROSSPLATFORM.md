# Cross-platform

## Target platforms

| Platform | Status | Notes |
|----------|--------|-------|
| Linux (x86_64) | **Primary development target** | Tested with X11 and Wayland via winit. |
| Windows (x86_64) | Not yet tested | Should work via eframe/winit. File dialog may need rfd. |
| macOS (aarch64, x86_64) | Not yet tested | Should work via eframe/winit. |

## Allowed dependencies

All dependencies must support Linux, Windows, and macOS. The following are explicitly allowed:

- `eframe` / `egui` — cross-platform rendering via winit + OpenGL/Vulkan/Metal.
- `winit` — window creation (pulled by eframe).
- `anyhow`, `log`, `env_logger` — pure Rust, no platform-specific code.

## Prohibited dependencies

- `slint` — removed in Sprint 1.
- `rfd` — may be added later for native file dialogs, but only as an optional dep in `crates/ui`.
- Any crate with platform-specific native widgets (e.g. `gtk-rs`, `winapi` directly).

## Platform-specific behavior

Platform-specific code is handled through `cfg` gates:

```rust
// Example pattern for file dialog (planned, Sprint 2)
#[cfg(target_os = "linux")]
fn open_file_dialog() -> Option<PathBuf> {
    // native dialog or rfd
}

#[cfg(target_os = "windows")]
fn open_file_dialog() -> Option<PathBuf> {
    // native dialog or rfd
}

#[cfg(target_os = "macos")]
fn open_file_dialog() -> Option<PathBuf> {
    // native dialog or rfd
}
```

Known differences to handle:

| Feature | Linux | Windows | macOS |
|---------|-------|---------|-------|
| Path separator | `/` | `\` | `/` |
| Home directory | `$HOME` | `%USERPROFILE%` | `$HOME` |
| File dialog | `zenity`/`kdialog` or rfd | COM dialog or rfd | Native panel or rfd |
| Terminal | `$TERMINAL` or built-in PTY | `cmd.exe` or PowerShell | `Terminal.app` |

All path handling uses `std::path::PathBuf` and `std::path::Path`, which handle platform-specific separators correctly.

## Build and run

### Prerequisites

- Rust 1.85+ (edition 2024)
- Linux: `libxkbcommon-dev`, `libwayland-dev`, `libx11-dev`, `libgl1-mesa-dev`

### Build

```sh
cargo build --workspace
```

### Run

```sh
cargo run -p app
```

### Check

```sh
cargo check --workspace
cargo clippy --workspace -- -D warnings
```

### Platform-specific build notes

- **Linux:** Ensure the X11/Wayland development packages are installed. eframe will prefer Wayland if available.
- **Windows:** Build with MSVC toolchain. eframe uses ANGLE (OpenGL over Direct3D) if OpenGL is not available.
- **macOS:** Build with `x86_64-apple-darwin` or `aarch64-apple-darwin` target. eframe uses Metal.

## Current limitations

- Only tested on Linux (X11). No testing has been done on Windows or macOS.
- File dialogs (`rfd`) have not been integrated — Sprint 2 will add them.
- No CI matrix for cross-platform testing (planned for Sprint 8+).
