# Dark Factory - Agent Operating Guide

This file is for coding agents working in `dark-factory`.
It reflects only what is currently true in this repository.

## 1) Current Repository Context

- The repository contains active Bun/TypeScript code under `dark_core/` and Prisma schema/config under `prisma/`.
- Prisma models currently include `Product` and `Variant` with these core assumptions:
  - Creating a local product (`@local://`) should also create a default variant at the same locator path.
  - Variant locator values are not globally unique; multiple variants can share a locator and are differentiated by `(productId, name)`.
- Rust workspace code now includes:
  - `frontends/dark_cli/` for the CLI binary.
  - `lib/dark_rust/` for shared dark_core API client/types used by Rust frontends.
- Moon workspace/project config is present at:
  - `.moon/workspace.yml`
  - `.moon/toolchains.yml`
  - `dark_core/moon.yml`
  - `prisma/moon.yml`
  - `generated/moon.yml`
  - `lib/dark_rust/moon.yml`
  - `frontends/dark_cli/moon.yml`
- Prisma generated outputs are written under `generated/` (for example `generated/prisma/` and `generated/prismabox/`).

## 2) Reflection Lessons (Read Early)

- Read `docs/lessons/*.lessons.md` near the start of each task when files exist.
- Reflection artifacts should be stored as timestamped files in `docs/reflections/`.
- Keep lessons short and practical: plain bullet points, no heavy structure, ideally 10-50 lines and always below 100 lines.
- Capture durable, generic learnings from normal work (troubleshooting wins, successful command sequences, recurring pitfalls, and workflow tips).
- Include agent behavior lessons from rule updates in `.opencode/commands/rule.md`.
- Reflection work may revise existing lessons and refine agent context files (`AGENTS.md`, `.opencode/skills/*`, `.opencode/agents/*`) for maintenance and optimization.
- Avoid design-direction changes and one-off human preference/style edits during reflection unless explicitly requested.
- Do not store secrets, credentials, or sensitive one-off details in lessons.

## 3) Build/Lint/Test Status (Current Truth)

- Verified via PTY in this repo:
  - `bun scripts/install.sh.ts`
  - `moon run dark_core:test`
- `moon run dark_core:test` runs the dependency chain `generated:build` -> `prisma:build` and then executes `dark_core:test`.
- Keep documenting only commands that are actually executed and validated.

## 4) Cursor and Copilot Rules

- Checked and currently absent:
  - `.cursor/rules/`
  - `.cursorrules`
  - `.github/copilot-instructions.md`
- If any of these files appear later, treat them as authoritative and update this guide.

## 5) Coding Style (Minimal Baseline)

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

## 6) Git Workflow

- Commit meaningful units of work; avoid giant mixed commits.
- Do not push unless the user explicitly requests it.
- Commit message format:
  - `{Component/Meta} // {Optional Addition} // {Short Description} (Optional,Tags)`
- Examples:
  - `Docs // Added converted PDF docs`
  - `Pipelines // ODM // Added initial ODM pipeline (WIP)`
- In commit body, add a short rationale and signature when possible:
  - `// Chappie/Model (your model if known)`

## 7) OpenCode Integration

- `.opencode/agents/gitter.md` is the git-focused subagent.
- `.opencode/agents/tsc-fixer.md` is the TypeScript safe-fix iteration subagent.
- `.opencode/agents/reflector.md` is the reflection/review subagent for post-task lessons.
- `.opencode/skills/gitter-commit/SKILL.md` documents when to route commit tasks to `@gitter`.
- `.opencode/skills/tsc-fix/SKILL.md` documents safe-first TypeScript error remediation.
- `.opencode/skills/proto-install/SKILL.md` documents standardized script-based `proto install` usage.
- `.opencode/skills/script-authoring/SKILL.md` documents how to build reusable Bun scripts from prompt/example/bash/context.
- `.opencode/skills/docs-scraping/SKILL.md` documents reusable external docs scraping workflows.
- `.opencode/skills/reflect/SKILL.md` documents quick end-of-task reflection and lesson capture.
- `.opencode/skills/reflect-constant/SKILL.md` documents periodic background reflection with `@reflector`.
- `.opencode/commands/scrape_docs.md` provides the command entrypoint for docs scraping tasks.
- `.opencode/commands/tsc_fix.md` provides the command entrypoint for safe TypeScript fixing loops.
- `.opencode/commands/crtique.md` provides the command entrypoint for critique-driven fix-then-rule updates.
- `.opencode/commands/critique.md` provides an alias entrypoint for the same critique-driven workflow.
- `.opencode/commands/reflect.md` provides the command entrypoint for reflection/lesson capture.
- `.opencode/commands/reflect_constant.md` provides the command entrypoint for periodic background reflection.
- Use `@gitter` when the user asks for commit support or cleanup.
- Use `@tsc-fixer` when the user asks for iterative TypeScript compiler error cleanup.
- Use `@reflector` when you want transcript/session-aware reflection (including one-agent-reviewing-another-agent workflows).

## 8) Dark Core MVC Skeleton Style

- `dark_core/src` follows a lightweight MVC REST skeleton:
  - `clients/` for external system clients and their config types.
  - `controllers/` for async, library-style business functions (routes call these).
  - `routes/` for minimal Elysia route modules (`{type}.routes.ts`).
  - `utils/` for shared generic helpers.
- Keep route handlers thin: parse input, call controllers, map errors to API JSON shape.
- Keep controllers modular and promise-based so routes and other controllers can reuse them.
- Prefer minimal, early-stage pragmatism over heavy abstractions while keeping code DRY and readable.
- Current REST coverage includes CRUD handlers for both `/products` and `/variants`.

## 9) Script Conventions

- Project scripts use shebanged Bun TypeScript files (`#!/usr/bin/env bun`) under `scripts/`.
- Script naming uses `*.sh.ts` to indicate executable shell-style Bun scripts.
- Shared script helpers live under `scripts/helpers/`.
- `scripts/install.sh.ts` runs ordered bootstrap/install steps: `proto install`, Bun install (root-invoked), then `moon :install`.
- `scripts/dev.sh.ts` runs `moon run dark_core:dev`.
- `scripts/test.sh.ts` runs `moon run dark_core:test`.
- `scripts/dcli.sh.ts` runs `dark_cli` via Cargo from repo root and forwards all CLI args.
- `scripts/reflect_constant.sh.ts` manages periodic reflector loops (`start`, `status`, `stop`, or foreground `run`).
- `scripts/reflect_constant.sh.ts` writes timestamped reflector outputs to `docs/reflections/`.
- `dark_core:check` now includes `dark_core:typecheck` plus `format:check` for full checks.
- `scripts/` should remain shell-style pragmatic automation; prefer readability over strict TypeScript purity.
- TypeScript safe-fix loops should ignore `scripts/` diagnostics unless user explicitly opts in.
- External docs snapshots can be generated with source scripts like `scripts/scrapes/scrape_opencode_docs.sh.ts`, `scripts/scrapes/scrape_elysia_docs.sh.ts`, and `scripts/scrapes/scrape_prisma_docs.sh.ts`.
- `scripts/scrapes/scrape_docs.sh.ts` dispatches supported source scrapers (`opencode`, `elysia`, `prisma`, `moonrepo`).

## 10) Bun Runtime Conventions

- `dark_core` runs on Bun; prefer Bun-native APIs when available instead of Node-compatible APIs.
- For environment variables, use `Bun.env` instead of `process.env`.
- Apply this convention in both app code and scripts unless a dependency explicitly requires Node-specific behavior.

## 11) Path Minimalism Rule

- Prefer one maintained "hot path" per workflow and build automation around it.
- Remove dead or duplicate approaches once a supported path is verified.
- Avoid adding backup implementations "just in case" when one clean path is sufficient.
- Optimize for easier human navigation: fewer entrypoints, clearer ownership, less branching process logic.

## 12) Keep This File Updated

- Update this file when real code, tooling, or CI is added.
- Keep instructions tied to verified repository behavior.
- Prefer short, accurate guidance over aspirational process docs.
- When `prisma/schema.prisma` or `dark_core/src/routes/*.ts` changes, update this file with any durable impacts on clients/contracts and agent workflow expectations.
