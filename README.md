# EmberIDE

A code editor built with Rust and Slint UI framework.

## About

EmberIDE is a lightweight, cross-platform code editor designed for developers who value speed and simplicity. Built from the ground up in Rust, it leverages the Slint UI framework for a native look and feel across all major platforms.

## Architecture

The project is organized as a Cargo workspace with the following crates:

| Crate | Description |
|---|---|
| `app` | Binary entry point |
| `ide-core` | Core state, actions, and business logic |
| `ui` | Slint-based UI components and event handling |
| `editor` | Text buffer management |
| `workspace` | Project/file tree management |
| `language` | Language support |
| `terminal` | Terminal integration |
| `extensions` | Extension system |

### Data Flow

```
User Input → UI (Slint) → Event Handlers → ide-core (AppState) → UI Update
```

## Build

### Prerequisites

- Rust (edition 2024)
- Cargo

### Commands

```bash
# Build the entire workspace
cargo build

# Build and run
cargo run -p app

# Run with logs
RUST_LOG=info cargo run -p app
```

## Branches

- **`collab`** — Development and collaboration
- **`test`** — Pre-release testing
- **`dev`** — Stable main branch

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full workflow.

## License

MIT License — see [LICENSE](LICENSE) for details.
