# EmberIDE

Un editor de código construido con Rust y Slint UI.

## Acerca de

EmberIDE es un editor de código ligero y multiplataforma, diseñado para desarrolladores que valoran la velocidad y la simplicidad. Construido desde cero en Rust, utiliza Slint UI para una apariencia nativa en todas las plataformas.

## Arquitectura

El proyecto está organizado como un workspace de Cargo con los siguientes crates:

| Crate | Descripción |
|---|---|
| `app` | Punto de entrada binario |
| `ide-core` | Estado global, acciones y lógica de negocio |
| `ui` | Componentes de interfaz (Slint) y manejadores de eventos |
| `editor` | Gestión de buffers de texto |
| `workspace` | Gestión de proyectos y árbol de archivos |
| `language` | Soporte de lenguajes |
| `terminal` | Integración de terminal |
| `extensions` | Sistema de extensiones |

### Flujo de datos

```
Entrada de usuario → UI (Slint) → Manejadores de eventos → ide-core (AppState) → Actualización de UI
```

## Compilar

### Requisitos

- Rust (edición 2024)
- Cargo

### Comandos

```bash
# Compilar todo el workspace
cargo build

# Compilar y ejecutar
cargo run -p app

# Ejecutar con logs
RUST_LOG=info cargo run -p app
```

## Ramas

- **`collab`** — Desarrollo y colaboración
- **`test`** — Pruebas previas al lanzamiento
- **`dev`** — Rama principal estable

Consulta [CONTRIBUTING.md](CONTRIBUTING.md) para el flujo de trabajo completo.

## Licencia

Licencia MIT — ver [LICENSE](LICENSE) para más detalles.
