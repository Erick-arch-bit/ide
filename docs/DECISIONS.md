# Technical Decisions

## 1. egui/eframe instead of Slint or Dioxus

**Sprint:** 1
**Context:** The original prototype used Slint for UI. Slint's DSL creates a disconnect between Rust logic and UI layout, and its widget set is limited. Dioxus requires a WebView. Tauri also requires a WebView.
**Decision:** Replace Slint with egui/eframe 0.31. egui is an immediate-mode GUI that runs natively (OpenGL/Vulkan/Metal), requires no WebView, and keeps all UI code in Rust.
**Alternatives considered:**
- Slint: DSL-based, limited widget set, harder to extend.
- Dioxus: requires WebView, heavier runtime.
- Tauri: web-based, violates the "no WebView" policy.
- Iced: Elm-like architecture, but less mature ecosystem.
**Consequences:**
- All UI code is pure Rust in `crates/ui/src/lib.rs`.
- Immediate-mode rendering simplifies state management (no widget tree to keep in sync).
- egui's panel system maps directly to the desired layout.
- No WebView dependency, smaller binary.

---

## 2. Cargo workspace with separated crates instead of a monolith

**Sprint:** 1
**Context:** Need to enforce clear boundaries between UI, editor logic, project management, and language support.
**Decision:** Use a Cargo workspace with 8 crates: `app`, `ide-core`, `ui`, `editor`, `workspace`, `language`, `terminal`, `extensions`.
**Alternatives considered:**
- Single crate: no enforced boundaries, risk of circular dependencies, harder to test in isolation.
- Workspace with 3 crates (core, ui, editor): not enough separation for the extension host and terminal.
**Consequences:**
- Clear dependency direction: `ui` -> `ide-core` -> `editor` + `workspace`.
- No crate can accidentally import a UI library from a logic crate (compiler enforces this).
- Faster compilation for individual crates.
- Slightly more boilerplate for cross-crate `pub use`.

---

## 3. Extensions isolated from core

**Sprint:** 6 (planned)
**Context:** Running untrusted extension code in-process risks crashes and security issues.
**Decision:** Extensions run in a separate process or WASM sandbox. The host communicates with the core through a serialized message protocol. If an extension crashes, the host restarts it without affecting the editor.
**Alternatives considered:**
- In-process dynamic loading (`libloading`): no memory isolation, crash takes down the editor.
- Lua scripting (`mlua`): interpreted, safe, but not WASM.
- WASM with WASI: memory-safe, cross-platform, sandboxed by default.
**Consequences:**
- Extensions cannot corrupt editor state.
- Extension crashes do not affect the editor.
- Communication overhead for RPC between core and host.
- Not yet implemented — this decision will be validated in Sprint 6.

---

## 4. Assembly policy

**Sprint:** 1
**Context:** Need a clear policy for when assembly is acceptable to prevent premature optimization and platform lock-in.
**Decision:** Assembly is forbidden unless ALL of these conditions are met:
1. Real profiler output (perf, flamegraph) proves the hot path is a bottleneck.
2. A pure-Rust fallback exists and is the default.
3. The assembly is isolated in an `impl` block behind `cfg(any(target_arch = "x86_64", ...))`.
4. The assembly module is in a separate file with `unsafe` at the call site.
**Alternatives considered:**
- Always allow assembly: risk of unmaintainable, non-portable code.
- Never allow assembly: may miss legitimate performance gains.
**Consequences:**
- No assembly in Sprints 1-3.
- Assembly may be added later if profiling justifies it.
- Every assembly addition must include benchmarks proving the benefit.
- No assembly has been added to the project as of Sprint 1.

---

## 5. Cross-platform strategy

**Sprint:** 1
**Context:** Need to ensure the editor works on Linux, Windows, and macOS without platform-specific rewrites.
**Decision:**
- All rendering goes through `eframe` (abstracts `winit` + OpenGL/Vulkan/Metal).
- File I/O uses `std::fs` exclusively.
- Path handling uses `std::path::PathBuf`.
- Platform-specific behavior is isolated behind `cfg(target_os = "...")` gates.
- No raw FFI to platform APIs.
**Alternatives considered:**
- Native widgets per platform (GTK on Linux, Win32 on Windows, AppKit on macOS): too much maintenance.
- Custom OpenGL from scratch: re-inventing eframe.
**Consequences:**
- Consistent look across all platforms (egui's own rendering, not native widgets).
- A single code path for most features.
- Platform-specific issues (e.g. file dialogs) need `cfg` gates.
- No platform has been tested beyond Linux as of Sprint 1.

---

## 6. Direct state mutation instead of a message bus

**Sprint:** 1
**Context:** Initial design of UI-to-core communication.
**Decision:** The UI holds `AppState` directly and mutates it through `dispatch()`. No channel, no event queue.
**Alternatives considered:**
- Cross-thread message bus: needed when LSP and terminal run on separate threads, but over-engineered for Sprint 1.
- Redux-style reducer: adds boilerplate without benefit for single-threaded usage.
**Consequences:**
- Simple, direct code for Sprint 1.
- Can add a channel-based dispatch later by wrapping `AppState` in an `Arc<Mutex<>>`.
- No threading issues until Sprint 4.
