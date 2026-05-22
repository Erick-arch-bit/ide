# Contribuir a EmberIDE

Gracias por tu interés en contribuir a EmberIDE. Este documento describe el flujo de trabajo y las pautas para la colaboración.

## Estrategia de ramas

EmberIDE utiliza un flujo de trabajo de tres niveles:

```
collab → test → dev
```

- **`collab`** — Rama de desarrollo activo. Envía tus pull requests aquí.
- **`test`** — Rama de pruebas previas al lanzamiento. Los cambios de `collab` se fusionan aquí para validación.
- **`dev`** — Rama principal estable. Solo se fusiona desde `test` después de pasar las pruebas y revisión.

## Flujo de trabajo

1. Haz un fork del repositorio o clona la rama `collab`.
2. Crea una rama de características desde `collab`:

   ```bash
   git checkout collab
   git checkout -b feature/mi-caracteristica
   ```

3. Realiza tus cambios y haz commit.
4. Sube tu rama y abre un pull request contra `collab`.
5. Después de la revisión, los cambios se fusionan en `collab`, luego pasan a `test` para validación.
6. Una vez probados y aprobados, se fusionan en `dev`.

## Estilo de código

- Sigue el formato estándar de Rust (`cargo fmt`).
- Asegura que no haya advertencias del compilador.
- Escribe mensajes de commit claros y descriptivos.

## Guía para pull requests

- Mantén los PR enfocados en una sola característica o corrección.
- Incluye una descripción clara de qué hace el PR y por qué.
- Asegúrate de que el proyecto compile correctamente antes de enviar.

## ¿Preguntas?

Abre un issue en GitHub para cualquier pregunta o sugerencia.
