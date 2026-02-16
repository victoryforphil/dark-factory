# Dark Factory - Style Guide

This file is the source of truth for style and implementation conventions in `dark-factory`.

## 1) Coding Style (Minimal Baseline)

Until language/tool-specific configs exist, follow pragmatic defaults:

- Favor readable, explicit code over clever shortcuts.
- Keep functions focused and avoid hidden side effects.
- Avoid dead code, unused imports, and speculative abstractions.
- Keep naming consistent (`PascalCase` types, `camelCase` values/functions, `UPPER_SNAKE_CASE` constants).
- Handle errors with context; do not swallow exceptions silently.
- Never log secrets or credentials.
- Log messages should follow: `System // Optional Sub system // Message (meta={...})`.
  - Prefer structured JSON metadata over comma-delimited `key=value` text so long IDs/paths remain readable.
  - In `dark_core`, use `formatLogMetadata` from `dark_core/src/utils/logging.ts` when adding metadata.
  - Example: `Core // HTTP // Listening (meta={"env":"development","host":"127.0.0.1","port":4150})`.

## 2) Dark Core Module-First Style

- `dark_core/src` uses a domain-first module layout under `modules/`.
- Each domain module keeps related files colocated with `{domain}.{kind}.ts` naming, for example:
  - `modules/products/products.routes.ts`
  - `modules/products/products.controller.ts`
  - `modules/system/system.config.ts`
  - `modules/providers/opencode/opencode.controller.ts`
  - colocated tests like `modules/products/products.unit.test.ts` and `modules/products/products.int.test.ts`
- Prefer one unit test file and one integration test file per domain module instead of split route/controller test files.
- Keep route handlers thin: parse input, call controllers, map errors to API JSON shape.
- Keep controllers modular and promise-based so routes and other controllers can reuse them.
- Keep shared cross-domain helpers in `dark_core/src/utils/` and config internals in `dark_core/src/config/lib/`.

## 3) Runtime Conventions

- `dark_core` runs on Bun; prefer Bun-native APIs when available instead of Node-compatible APIs.
- For environment variables, use `Bun.env` instead of `process.env`.
- Apply this convention in both app code and scripts unless a dependency explicitly requires Node-specific behavior.

## 4) Engineering Principles

- **Path Minimalism:** Prefer one maintained "hot path" per workflow and build automation around it.
  - Remove dead or duplicate approaches once a supported path is verified.
  - Avoid adding backup implementations "just in case" when one clean path is sufficient.
  - Optimize for easier human navigation: fewer entrypoints, clearer ownership, less branching process logic.
- **DRY (Don't Repeat Yourself):** Keep a single source of truth for shared behavior, rules, and transformations.
  - When duplication appears across modules, extract reusable helpers or modules instead of repeating logic.
  - Avoid copy-paste updates that can drift; prefer centralizing constants, validation rules, and serialization/parsing logic.
