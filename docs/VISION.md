# Vision

## What is EmberIDE?

EmberIDE is a modular, native IDE written entirely in Rust. It is designed for developers who want a fast, minimal editor that stays out of the way. Every feature is opt-in through a crate-based architecture.

The editor core (file buffers, project tree, syntax dispatch) is separated from the UI, LSP, terminal, and extension systems. Each lives in its own crate with a defined public API.

## Who it is for

- Rust developers who want an IDE written in their language of choice.
- Developers who want a lightweight alternative to VS Code, Zed, or IntelliJ.
- Developers who want to understand exactly what their editor does.

## Core design philosophy

- **Fast.** Startup time under 200ms. No background processes, no telemetry, no hidden indexing.
- **Minimal.** The window shows only what is needed. No decorative widgets, no gradients, no rounded corners unless they aid UX.
- **Modular.** Each crate has a single responsibility. Crates communicate through `ide_core::AppState` and `Action` messages. You can replace the UI, the terminal, or the LSP client without touching the rest.
- **Cross-platform.** Linux, Windows, and macOS are first-class targets. Platform-specific code is isolated behind `cfg` gates.

## What it is NOT

- EmberIDE is **not** Electron, Tauri, WebView, or any web-rendered editor.
- EmberIDE is **not** a VS Code replacement. It does not run VS Code extensions.
- EmberIDE is **not** an IDE for non-programmers. There is no GUI for settings; configuration is through code.

## Long-term direction

| Area | Plan |
|------|------|
| LSP | rust-analyzer integration: diagnostics, completions, hover, go-to-def (Sprint 4) |
| Terminal | Embedded terminal panel (Sprint 5) |
| Extensions | WASM-based extension host with failure isolation (Sprint 6) |
| Search | Find in file, find in project, quick-open fuzzy finder (Sprint 7) |
| Performance | Lazy loading, incremental parsing, minimal redraws (Sprint 8) |
| Language support | Highlighting via tree-sitter (post-Sprint 9) |
| WASM plugins | Extension host using WASI (post-Sprint 9) |
