# Dark Factory - Agent Operating Guide

This file is for coding agents working in `dark-factory`.
It reflects only what is currently true in this repository.

## 1) Current Repository Context

- The repository contains active Bun/TypeScript code under `dark_core/` and Prisma schema/config under `prisma/`.
- Moon workspace/project config is present at:
  - `.moon/workspace.yml`
  - `.moon/toolchains.yml`
  - `dark_core/moon.yml`
  - `prisma/moon.yml`
  - `generated/moon.yml`
- Prisma generated outputs are written under `generated/` (for example `generated/prisma/` and `generated/prismabox/`).

## 2) Build/Lint/Test Status (Current Truth)

- Verified via PTY in this repo:
  - `bun scripts/install.sh.ts`
  - `moon run dark_core:test`
- `moon run dark_core:test` runs the dependency chain `generated:build` -> `prisma:build` and then executes `dark_core:test`.
- Keep documenting only commands that are actually executed and validated.

## 3) Cursor and Copilot Rules

- Checked and currently absent:
  - `.cursor/rules/`
  - `.cursorrules`
  - `.github/copilot-instructions.md`
- If any of these files appear later, treat them as authoritative and update this guide.

## 4) Coding Style (Minimal Baseline)

Until language/tool-specific configs exist, follow pragmatic defaults:

- Favor readable, explicit code over clever shortcuts.
- Keep functions focused and avoid hidden side effects.
- Avoid dead code, unused imports, and speculative abstractions.
- Keep naming consistent (`PascalCase` types, `camelCase` values/functions, `UPPER_SNAKE_CASE` constants).
- Handle errors with context; do not swallow exceptions silently.
- Never log secrets or credentials.
- Log messages should follow: `System // Optional Sub system // Message (Metadata)`.
  - Example: `Core // HTTP // Listening (env=development,host=127.0.0.1,port=4150)`.

## 5) Git Workflow

- Commit meaningful units of work; avoid giant mixed commits.
- Do not push unless the user explicitly requests it.
- Commit message format:
  - `{Component/Meta} // {Optional Addition} // {Short Description} (Optional,Tags)`
- Examples:
  - `Docs // Added converted PDF docs`
  - `Pipelines // ODM // Added initial ODM pipeline (WIP)`
- In commit body, add a short rationale and signature when possible:
  - `// Chappie/Model (your model if known)`

## 6) OpenCode Integration

- `.opencode/agents/gitter.md` is the git-focused subagent.
- `.opencode/skills/gitter-commit/SKILL.md` documents when to route commit tasks to `@gitter`.
- `.opencode/skills/proto-install/SKILL.md` documents standardized script-based `proto install` usage.
- `.opencode/skills/script-authoring/SKILL.md` documents how to build reusable Bun scripts from prompt/example/bash/context.
- `.opencode/skills/docs-scraping/SKILL.md` documents reusable external docs scraping workflows.
- `.opencode/commands/scrape_docs.md` provides the command entrypoint for docs scraping tasks.
- Use `@gitter` when the user asks for commit support or cleanup.

## 8) Script Conventions

- Project scripts use shebanged Bun TypeScript files (`#!/usr/bin/env bun`) under `scripts/`.
- Script naming uses `*.sh.ts` to indicate executable shell-style Bun scripts.
- Shared script helpers live under `scripts/helpers/`.
- `scripts/install.sh.ts` runs ordered bootstrap/install steps: `proto install`, Bun install (root-invoked), then `moon :install`.
- `scripts/dev.sh.ts` runs `moon run dark_core:dev`.
- `scripts/test.sh.ts` runs `moon run dark_core:test`.
- External docs snapshots can be generated with source scripts like `scripts/scrapes/scrape_opencode_docs.sh.ts`, `scripts/scrapes/scrape_elysia_docs.sh.ts`, and `scripts/scrapes/scrape_prisma_docs.sh.ts`.
- `scripts/scrapes/scrape_docs.sh.ts` dispatches supported source scrapers (`opencode`, `elysia`, `prisma`, `moonrepo`).

## 9) Keep This File Updated

- Update this file when real code, tooling, or CI is added.
- Keep instructions tied to verified repository behavior.
- Prefer short, accurate guidance over aspirational process docs.
