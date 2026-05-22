# Contributing to EmberIDE

Thank you for your interest in contributing to EmberIDE! This document outlines the workflow and guidelines for collaboration.

## Branch Strategy

EmberIDE uses a three-tier branch workflow:

```
collab → test → dev
```

- **`collab`** — Active development branch. Submit your pull requests here.
- **`test`** — Pre-release testing branch. Changes from `collab` are merged here for validation.
- **`dev`** — Stable main branch. Only merged from `test` after passing review and testing.

## Workflow

1. Fork the repository or clone the `collab` branch.
2. Create a feature branch from `collab`:

   ```bash
   git checkout collab
   git checkout -b feature/my-feature
   ```

3. Make your changes and commit them.
4. Push your branch and open a pull request against `collab`.
5. After review, changes are merged into `collab`, then promoted to `test` for validation.
6. Once tested and approved, changes are merged into `dev`.

## Code Style

- Follow Rust standard formatting (`cargo fmt`).
- Ensure no compiler warnings (`cargo build` without warnings).
- Write clear, descriptive commit messages.

## Pull Request Guidelines

- Keep PRs focused on a single feature or fix.
- Include a clear description of what the PR does and why.
- Ensure the project builds successfully before submitting.

## Questions?

Open an issue on GitHub for any questions or suggestions.
