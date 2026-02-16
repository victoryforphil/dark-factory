# Dark Factory - Agent Operating Guide

This file is for coding agents working in `dark-factory`.
It reflects only what is currently true in this repository.

## 1) Current Repository Context

- The repo is early-stage and mostly docs/config right now.
- Tracked project files currently include:
  - `README.md`
  - `TODO.md`
  - `AGENTS.md`
  - `opencode.jsonc`
  - `.opencode/agents/gitter.md`
  - `.opencode/skills/gitter-commit/SKILL.md`
- No application code tree is committed yet (`src/`, `apps/`, `crates/`, etc.).

## 2) Build/Lint/Test Status (Current Truth)

- There are no validated build, lint, typecheck, or test commands yet.
- Do not invent or assume project commands in commits or PR notes.
- If new tooling is added, document only commands that are verified in this repo.

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
- `.opencode/skills/protobuf-schema-authoring/SKILL.md` documents how to author proto2/proto3 schemas and provide Rust/TypeScript codegen workflows.
- `.opencode/skills/script-authoring/SKILL.md` documents how to build reusable Bun scripts from prompt/example/bash/context.
- Use `@gitter` when the user asks for commit support or cleanup.

## 7) Local Fork Workflow (`elysia-protobuf`)

- `elysia-protobuf` is also cloned locally at `~/repos/vfp/elysia-protobuf`.
- If Core hits a bug in that dependency, patch it in the local fork repo, push the fork, then update the pinned git ref in `core/package.json` and refresh `core/bun.lock`.
- Core currently tracks `elysia-protobuf` from the fork `master` branch in `core/package.json`; after pushing fork changes, refresh `core/bun.lock` in this repo.

## 8) Script Conventions

- Project scripts use shebanged Bun TypeScript files (`#!/usr/bin/env bun`) under `scripts/`.
- Script naming uses `*.sh.ts` to indicate executable shell-style Bun scripts.
- Shared script helpers live under `scripts/helpers/`.
- `scripts/install.sh.ts` runs a reusable ordered list of install steps.
- `scripts/proto_install.sh.ts` runs `proto install` from repository root.
- `scripts/scrape_moon_docs.sh.ts` scrapes moonrepo docs into split per-page `docs/external/moonrepo/*.ext.md` files and writes `docs/external/moonrepo/index.ext.md`.
- After a new command sequence succeeds in a conversation, suggest capturing it into a reusable script via `.opencode/commands/capture_script_from_context.md`.

## 9) Keep This File Updated

- Update this file when real code, tooling, or CI is added.
- Keep instructions tied to verified repository behavior.
- Prefer short, accurate guidance over aspirational process docs.
