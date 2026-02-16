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
- Use `@gitter` when the user asks for commit support or cleanup.

## 7) Keep This File Updated

- Update this file when real code, tooling, or CI is added.
- Keep instructions tied to verified repository behavior.
- Prefer short, accurate guidance over aspirational process docs.
