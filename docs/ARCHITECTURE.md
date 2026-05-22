# Architecture

## Crate dependency diagram

```
  +-----------+     +--------+     +----------+
  |  app      |---->| ui     |---->| ide-core |
  | (binary)  |     | (egui) |     | (state)  |
  +-----------+     +--------+     +----+-----+
       |                                |
       |  +----------+  +-----------+   |
       +->| editor   |  | workspace |<--+
       |  | (buffer) |  | (project) |
       |  +----------+  +-----------+
       |
       |  +----------+  +----------+  +------------+
       +->| language |  | terminal |  | extensions |
          | (stub)   |  | (stub)   |  | (stub)     |
          +----------+  +----------+  +------------+
```

Data flow:

```
User input
    |
    v
  ui::EmberApp.update()       <-- eframe event loop (60 fps)
    |
    | calls self.state.dispatch(&action)
    v
  ide_core::AppState           <-- single source of truth
    |
    | reads/writes
    v
  editor::Buffer               <-- file text, dirty flag
  workspace::Project           <-- file tree
```

## Crate responsibilities

### `app` (binary)
- Initializes `env_logger`, calls `init()` on every crate, creates `AppState`, calls `ui::run()`.
- Must NOT contain UI code or business logic.

### `ide-core`
- Defines `AppState` (the central state struct) and `Action` (the message enum).
- Provides dispatch logic: `AppState::dispatch(&Action)`.
- Owns the `COMMANDS` static list for the command palette.
- Must NOT import `egui`, `eframe`, or any UI library.
- Must NOT contain file I/O or project traversal logic (delegates to `editor` and `workspace`).

### `ui`
- Implements `eframe::App` via `EmberApp`.
- Renders the window layout: menu bar, icon bar, file panel, editor area, status bar.
- Calls `self.state.dispatch()` on user actions.
- Must NOT contain file I/O or project traversal logic.
- Must NOT import `slint`, `rfd`, or any non-egui UI framework.

### `editor`
- Defines `Buffer`: open, save, save_as, dirty state.
- Pure file I/O — no UI, no state machine, no project awareness.
- Must NOT depend on `ide-core`, `workspace`, or `ui`.

### `workspace`
- Defines `Project`: open folder, recursive file collection, sorted output.
- Pure filesystem traversal — no UI, no buffer management.
- Must NOT depend on `ide-core`, `editor`, or `ui`.

### `language`, `terminal`, `extensions`
- Stubs. Each exposes `pub fn init()`.
- Planned functionality: syntax dispatch, embedded terminal, WASM extension host.

## UI-to-core communication

The UI holds `AppState` directly as a field of `EmberApp`. When the user interacts with the UI:

1. An `Action` variant is created (e.g. `Action::SaveFile`).
2. `self.state.dispatch(&action)` is called.
3. The state mutates in place.
4. On the next frame, `EmberApp::update()` reads the updated state and re-renders.

There is no message bus, no channel, no event queue. This is intentional: for a single-threaded editor, direct mutation is simpler and faster. If threading is needed later (e.g. LSP), a channel can be added behind `AppState::dispatch()`.

## Threading model

- **UI thread** (main thread): `eframe` event loop, `EmberApp::update()`, all rendering.
- **All state mutation** happens on the UI thread via `dispatch()`.
- **LSP client** (planned, Sprint 4): runs on a separate thread, sends results through a channel.
- **Terminal** (planned, Sprint 5): runs a child process with its own thread for PTY I/O.
- **Extension host** (planned, Sprint 6): runs on a separate thread, isolated by process boundary.

Current state: everything runs on the UI thread. No threading primitives are used yet.

## Cross-platform portability

- All rendering goes through `eframe`, which abstracts `winit` + OpenGL/Vulkan/Metal.
- File I/O uses `std::fs` (no platform-specific APIs).
- Path handling uses `std::path::PathBuf` (no string concatenation).
- Platform-specific code is isolated behind `cfg(target_os = "...")` gates.
- No assembly. No raw FFI to platform APIs.

## Extension isolation (planned, Sprint 6)

Extensions run in a separate process or WASM sandbox. The host communicates with the core through a serialized message protocol. If an extension crashes, the host restarts it without affecting the editor. This is not yet implemented.
