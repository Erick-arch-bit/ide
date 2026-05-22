# Sprints

| # | Name | Goal | Status | Key deliverables |
|---|------|------|--------|-----------------|
| 1 | egui/eframe Foundation | Replace Slint with egui/eframe. Dark window with correct panel layout. | **Done** | `EmberApp`, menu bar, icon bar, file panel, editor area, status bar. No Slint/rfd remaining. |
| 2 | File Operations | Open folder via dialog, open/edit/save files, dirty state in title bar. | **Done** | rfd folder dialog, TextEdit editor, file panel clicks, Save menu, dirty window title. |
| 3 | Tabs & Command Palette | Tab bar with switch/close, command palette with fuzzy filter, keyboard shortcuts. | **Done** | Tab bar with selectable labels, close buttons, command palette `Area`, fuzzy filter, `Ctrl+S`/`Ctrl+P`/`Ctrl+W`. |
| 4 | LSP Integration | rust-analyzer client: diagnostics, autocomplete, hover, go-to-definition. | **Done** | `LspClient` with JSON-RPC stdio transport, rust-analyzer init, didOpen/didChange/didClose, diagnostics in status bar, completion popup. |
| 5 | Integrated Terminal | Embedded terminal panel with PTY. | **Pending** | Terminal panel widget, PTY bridge, shell launch. |
| 6 | Extension Host | WASM extension sandbox with failure isolation. | **Pending** | Extension API definition, WASM runtime, host process. |
| 7 | Search & Navigation | Find in file, find in project, quick-open fuzzy finder. | **Pending** | Search panel, project-wide grep, fuzzy file picker. |
| 8 | Performance | Lazy loading, incremental parsing, minimal redraws. | **Pending** | Profiling, chunked file loading, redraw optimization. |
| 9 | Documentation | Full technical docs, doc comments, `cargo doc` pass. | **In Progress** | VISION, ARCHITECTURE, MODULES, SPRINTS, DECISIONS, CROSSPLATFORM, ASSEMBLY. |

## Sprint notes

### Sprint 1 — egui/eframe Foundation

- Removed `slint` and `rfd` from workspace dependencies.
- Removed `slint` and `rfd` from `crates/ui/Cargo.toml`.
- Added `eframe = "0.31"` and `egui = "0.31"` as workspace dependencies.
- Deleted all Slint source files: `editor_manager.rs`, `event_handlers.rs`, `file_manager.rs`, `tab_manager.rs`, `ui_layout.rs`.
- Wrote new `crates/ui/src/lib.rs` with `EmberApp` implementing `eframe::App`.
- Layout: menu bar (28px), icon sidebar (44px), file panel (200px), editor area, status bar (22px).
- Colors: `#141414` for chrome panels, `#1e1e1e` for content panels.
- `cargo check --workspace` and `cargo clippy -- -D warnings` pass clean.
- `crates/editor`, `crates/workspace`, `crates/ide-core` unchanged.

### Sprint 2 — File Operations

- Added `rfd = "0.17"` to workspace and `crates/ui` dependencies.
- Replaced static labels in menu bar with `egui::menu::bar` dropdown menus.
- "File > Open Folder..." opens native folder dialog (`rfd::FileDialog`), calls `AppState::open_folder`.
- "File > Save" calls `AppState::save_current()`, logs errors.
- File panel now shows clickable file paths that open the file in the editor.
- Editor area renders `egui::TextEdit::multiline` with monospace font and code_editor mode for active buffer.
- Text changes automatically set `dirty` flag on the buffer.
- Window title updates every frame via `send_viewport_cmd(Command::Title(...))` showing dirty state.
- Status bar shows current file name (or "EmberIDE" if no file is open).
- `cargo check --workspace` and `cargo clippy -- -D warnings` pass clean.
- `crates/editor`, `crates/workspace`, `crates/ide-core` unchanged.

### Sprint 3 — Tabs & Command Palette

- Added `palette_query: String` field to `EmberApp` for command palette search.
- Tab bar: `TopBottomPanel::top("tab_bar")` below menu bar, shows tab titles from `AppState::tab_titles()` with dirty markers. Click to switch, "x" button to close.
- Close tab behavior: switches to the clicked tab first, then calls `close_active()`.
- Command palette: `egui::Area` anchored at center-top, dark frame (#252525) with 1px border. Contains search input and scrollable command list.
- Fuzzy filter: commands are filtered by substring match against `palette_query` (case-insensitive).
- Command palette actions wired: "Abrir carpeta..." opens rfd dialog, "Guardar archivo" saves, "Cerrar archivo" closes tab, "Siguiente tab"/"Tab anterior" navigate.
- Keyboard shortcuts: `Ctrl+S` save, `Ctrl+P` toggle command palette, `Ctrl+W` close active tab, `Escape` close palette.
- File menu updated: added "New File" which creates an empty `editor::Buffer::new_empty()`.
- Icon bar: added logo placeholder ("E" in blue #0088ff, 22px).
- File panel: resizable (100px - 500px range).
- `cargo check --workspace` and `cargo clippy -- -D warnings` pass clean.
- `crates/editor`, `crates/workspace`, `crates/ide-core` unchanged.

### Sprint 4 — LSP Integration

- Added `serde`, `serde_json`, `lsp-types` to workspace dependencies.
- `crates/language` rewritten: LspClient with JSON-RPC stdio transport, spawns rust-analyzer.
- `crates/ui` wired: LSP starts on project open, didOpen/didChange/didClose, diagnostics in status bar, completion popup.
- `cargo check --workspace` and `cargo clippy -- -D warnings` pass clean.
- `crates/editor`, `crates/workspace`, `crates/ide-core` unchanged.
