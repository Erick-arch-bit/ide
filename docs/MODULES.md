# Modules

## `app` — Binary entry point

**Purpose:** Initialize all crates and start the UI event loop.

**Key structs:** None (single `main()` function).

**Public API:**
- `main() -> Result<()>` — entry point, not called by other crates.

**Invariants:**
- Must NOT contain UI layout code.
- Must NOT contain business logic beyond initialization.

**Status:** Complete.

---

## `ide-core` — Central state and dispatch

**Purpose:** Define the shared state struct (`AppState`), the action enum (`Action`), and the dispatch logic. This is the hub that all other crates depend on (directly or indirectly).

**Key structs and enums:**

| Name | Kind | Description |
|------|------|-------------|
| `AppState` | struct | Holds `project: Option<Project>`, `buffers: Vec<Buffer>`, `active_index: usize`, `command_palette_open: bool`. |
| `Action` | enum | Variants: `OpenFolder`, `SaveFile`, `OpenFile(String)`, `CloseTab`, `SwitchTab(usize)`, `ToggleCommandPalette`, `ExecuteCommand(String)`. |

**Public API:**
- `AppState::default()` — create empty state.
- `AppState::active_buffer()` / `active_buffer_mut()` — current buffer.
- `AppState::open_folder(PathBuf)` — set project.
- `AppState::open_file_relative(&str)` — open or switch to file.
- `AppState::close_active()` — close current tab.
- `AppState::switch_to(usize)` / `switch_to_next()` / `switch_to_prev()` — tab navigation.
- `AppState::tab_titles() -> Vec<String>` — tab labels with dirty marker.
- `AppState::file_list() -> Vec<String>` — relative file paths from project.
- `AppState::save_current() -> Result<()>` — save active buffer.
- `AppState::window_title() -> String` — formatted title with dirty marker.
- `AppState::dispatch(&Action) -> Result<()>` — route action to handler.
- `COMMANDS: &[&str]` — static list of command palette entries.
- `init()` — called during startup.

**Invariants:**
- Must NOT import `egui`, `eframe`, or any UI library.
- Must NOT do file I/O directly (delegates to `editor::Buffer` and `workspace::Project`).
- Must NOT depend on `ui` or `app`.

**Status:** Complete.

---

## `ui` — egui/eframe window

**Purpose:** Render the IDE window and forward user actions to `AppState`.

**Key structs:**

| Name | Kind | Description |
|------|------|-------------|
| `EmberApp` | struct | Implements `eframe::App`. Holds `state: ide_core::AppState`. |

**Public API:**
- `run(state: AppState) -> Result<()>` — create window and run event loop.

**Panel layout:**

| Panel | Size | Background | Content |
|-------|------|------------|---------|
| Menu bar | 28px | #141414 | File> New File, Open Folder, Save | Edit | View | Terminal | Help |
| Tab bar | 28px | #1e1e1e | Tab titles with dirty marker + close "x" button |
| Icon sidebar | 44px | #141414 | Logo "E" placeholder (#0088ff, 22px) |
| File panel | 100-500px | #1e1e1e | Clickable project file list, resizable |
| Editor area | (rest) | #1e1e1e | `TextEdit::multiline` (code_editor) or centered "EmberIDE" |
| Status bar | 22px | #141414 | Current filename or "EmberIDE" in 11px #666666 |
| Command palette | 420x320px | #252525 | Search input + filtered command list |
| Terminal panel | 80-200px | #1a1a1a | Shell output (monospace) + input line, resizable, toggleable |

**Invariants:**
- Must NOT contain file I/O.
- Must NOT contain project traversal logic.
- Must NOT depend on `slint`.
- `rfd` is allowed only in `crates/ui` for native file dialogs.

**Status:** Complete. File editing via `TextEdit::multiline`, project open via `rfd::FileDialog`, save flow, dirty state in title bar and status bar. Tab bar with switch/close, command palette with fuzzy filter, keyboard shortcuts (`Ctrl+S`, `Ctrl+P`, `Ctrl+W`).

---

## `editor` — Text buffer

**Purpose:** Read, write, and track changes to file buffers.

**Key structs:**

| Name | Kind | Description |
|------|------|-------------|
| `Buffer` | struct | Fields: `path: Option<PathBuf>`, `text: String`, `dirty: bool`. |

**Public API:**
- `Buffer::open(path) -> Result<Self>` — read file into buffer.
- `Buffer::new_empty() -> Self` — create unnamed empty buffer.
- `Buffer::file_name() -> String` — extract filename from path.
- `Buffer::set_text(String)` — replace text, marks dirty.
- `Buffer::save() -> Result<()>` — write to disk, clears dirty.
- `Buffer::save_as(PathBuf) -> Result<()>` — write to new path, clears dirty.

**Invariants:**
- Must NOT depend on `ide-core`, `workspace`, or `ui`.
- Must NOT know about tabs, projects, or the UI.
- Must NOT import any UI library.

**Status:** Complete.

---

## `workspace` — Project folder

**Purpose:** Recursively scan a folder and collect all files, sorted with directories first.

**Key structs:**

| Name | Kind | Description |
|------|------|-------------|
| `Project` | struct | Fields: `root: PathBuf`, `files: Vec<PathBuf>`. |

**Public API:**
- `Project::open(root) -> Result<Self>` — scan folder recursively. Skips `.git` and all dotfiles. Returns files sorted (directories first, then alphabetical).

**Invariants:**
- Must NOT depend on `ide-core`, `editor`, or `ui`.
- Must NOT know about buffers, tabs, or file content.
- Must NOT import any UI library.

**Status:** Complete.

---

## `language` — LSP client and language support

**Purpose:** Manage communication with language servers via LSP protocol. Currently supports rust-analyzer.

**Key structs:**

| Name | Kind | Description |
|------|------|-------------|
| `LspClient` | struct | Manages rust-analyzer process, JSON-RPC over stdio, diagnostics storage, completion/hover/definition requests. |

**Public API:**
- `LspClient::new()` / `start(root_uri)` — spawn and initialize rust-analyzer.
- `open_file(path, text)` — send `textDocument/didOpen`.
- `change_file(path, text, version)` — send `textDocument/didChange`.
- `close_file(path)` — send `textDocument/didClose`.
- `get_diagnostics(path) -> Vec<Diagnostic>` — retrieve diagnostics for a file.
- `request_completion(path, line, char) -> CompletionResponse` — request completions.
- `request_hover(path, line, char) -> Option<Hover>` — request hover info.
- `request_definition(path, line, char) -> GotoDefinitionResponse` — request go-to-def.
- `init()` — called during startup.

**Status:** Complete. Diagnostics rendering could be improved with squiggly underlines in the TextEdit (Sprint 8).

---

## `terminal` — Integrated terminal

**Purpose:** Embedded terminal panel using a PTY (pseudo-terminal). Spawns a shell and renders output via escape sequence parsing.

**Key structs:**

| Name | Kind | Description |
|------|------|-------------|
| `TerminalSession` | struct | Manages shell process, PTY I/O, and vt100 parser. |

**Public API:**
- `TerminalSession::new(Option<&str>) -> Result<Self>` — spawn shell in PTY.
- `write(&[u8])` / `write_str(&str)` — send input to shell.
- `resize(rows, cols)` — update terminal dimensions.
- `contents() -> String` — get current screen text.
- `rows()` / `cols()` — current dimensions.
- `init()` — called during startup.

**Status:** Complete. Basic terminal with keyboard input. Full escape sequence rendering (colors, bold, cursor) is handled by vt100. Terminal toggle from View/Terminal menus.

---

## `extensions` — Extension host (stub)

**Purpose:** Planned — run WASM-based extensions in an isolated sandbox.

**Public API:**
- `init()` — called during startup.

**Status:** Stub. Not yet implemented — planned for Sprint 6.

---

## Undocumented public items

The following public items lack doc comments (`///`). Generated by `cargo doc` analysis:

- `ide_core::Action` — no doc comment on enum or variants
- `ide_core::AppState` — no doc comment on struct or fields
- `ide_core::COMMANDS` — no doc comment
- `ide_core::init()` — no doc comment
- `editor::Buffer` — no doc comment on struct or fields
- `editor::Buffer::open()` — no doc comment
- `editor::Buffer::new_empty()` — no doc comment
- `editor::Buffer::file_name()` — no doc comment
- `editor::Buffer::set_text()` — no doc comment
- `editor::Buffer::save()` — no doc comment
- `editor::Buffer::save_as()` — no doc comment
- `workspace::Project` — no doc comment on struct or fields
- `workspace::Project::open()` — no doc comment
- `ui::EmberApp` — no doc comment
- `ui::run()` — no doc comment
