# Dark Factory - Agent Operating Guide

This file is for coding agents working in `dark-factory`.
It reflects only what is currently true in this repository.

## 1) Current Repository Context

- The repository contains active Bun/TypeScript code under `dark_core/` and Prisma schema/config under `prisma/`.
- Prisma models currently include `Product`, `Variant`, and `Actor` with these core assumptions:
  - Creating a local product (`@local://`) should also create a default variant at the same locator path.
  - Variant locator values are not globally unique; multiple variants can share a locator and are differentiated by `(productId, name)`.
  - Product IDs are deterministic `prd_{token}` values derived from normalized locator text (SHA-256 first 64 bits encoded as fixed-width base36); repeated creates for the same canonical locator are idempotent.
  - `Product.gitInfo` and `Variant.gitInfo` are optional JSON snapshots populated from local git metadata when available.
  - Variant git polling timestamps are tracked via `gitInfoUpdatedAt` and `gitInfoLastPolledAt`.
  - Actors attach to variants (`Variant 1 -> N Actors`) and store provider/runtime snapshots (`actorLocator`, `workingLocator`, `providerSessionId`, `attachCommand`, `connectionInfo`).
- Rust workspace code now includes:
  - `frontends/dark_cli/` for the CLI binary.
  - `frontends/dark_chat/` for the OpenCode-focused chat TUI frontend (library + binary).
  - `frontends/dark_tui/` for the Ratatui dashboard frontend.
  - `lib/dark_rust/` for shared dark_core API client/types used by Rust frontends.
  - `lib/dark_tui_components/` for reusable Ratatui component primitives shared by Rust TUIs.
- TUI work in this repo uses Ratatui; local scraped references live at `docs/external/ratatui_web/index.ext.md` and `docs/external/ratatui_docs/index.ext.md`.
- Moon workspace/project config is present at:
  - `.moon/workspace.yml`
  - `.moon/toolchains.yml`
  - `dark_core/moon.yml`
  - `prisma/moon.yml`
  - `generated/moon.yml`
  - `lib/dark_rust/moon.yml`
  - `lib/dark_tui_components/moon.yml`
  - `frontends/dark_cli/moon.yml`
  - `frontends/dark_chat/moon.yml`
  - `frontends/dark_tui/moon.yml`
- Prisma generated outputs are written under `generated/` (for example `generated/prisma/` and `generated/prismabox/`).
- Do not implement ad-hoc/manual Prisma schema compatibility SQL in app code; evolve `prisma/schema.prisma` and apply changes via Prisma tooling (`prisma db push` / migrations).
- Layered Docker/devcontainer setup now exists under `docker/` with a multi-stage `docker/Dockerfile` and compose stack at `docker/compose.devcontainers.yml`.

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

## 5) Style Guide

- Style, naming, architecture, and runtime conventions now live in `STYLE.md`.
- Read and apply `STYLE.md` before editing code.

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
- `.opencode/agents/sweeper.md` is the project hygiene subagent for cleanup audits and low-risk fixes.
- `.opencode/agents/designer.md` is the UI/UX-focused Ratatui design subagent for component-first TUI polish.
- `.opencode/skills/gitter-commit/SKILL.md` documents when to route commit tasks to `@gitter`.
- `.opencode/skills/tsc-fix/SKILL.md` documents safe-first TypeScript error remediation.
- `.opencode/skills/proto-install/SKILL.md` documents standardized script-based `proto install` usage.
- `.opencode/skills/script-authoring/SKILL.md` documents how to build reusable Bun scripts from prompt/example/bash/context.
- `.opencode/skills/devcontainer-workflows/SKILL.md` documents layered Docker devcontainer workflows and helper script usage.
- `.opencode/skills/docs-scraping/SKILL.md` documents reusable external docs scraping workflows.
- `.opencode/skills/reflect/SKILL.md` documents quick end-of-task reflection and lesson capture.
- `.opencode/skills/reflect-constant/SKILL.md` documents periodic background reflection with `@reflector`.
- `.opencode/commands/scrape_docs.md` provides the command entrypoint for docs scraping tasks.
- `.opencode/commands/tsc_fix.md` provides the command entrypoint for safe TypeScript fixing loops.
- `.opencode/commands/crtique.md` provides the command entrypoint for critique-driven fix-then-rule updates.
- `.opencode/commands/critique.md` provides an alias entrypoint for the same critique-driven workflow.
- `.opencode/commands/reflect.md` provides the command entrypoint for reflection/lesson capture.
- `.opencode/commands/reflect_constant.md` provides the command entrypoint for periodic background reflection.
- `.opencode/commands/sweeper.md` provides the command entrypoint for project-wide cleanup sweeps.
- `.opencode/commands/sweep.md` provides an alias entrypoint for the same sweeper workflow.
- Use `@gitter` when the user asks for commit support or cleanup.
- Use `@tsc-fixer` when the user asks for iterative TypeScript compiler error cleanup.
- Use `@reflector` when you want transcript/session-aware reflection (including one-agent-reviewing-another-agent workflows).
- Use `@sweeper` when you need repo-wide hygiene audits (style conformance, dead code, and stale docs checks).
- Use `@designer` when you need visual/UI direction, reusable TUI component design, or Ratatui-focused UX polish.

## 8) Dark Core API Coverage

- Current REST coverage includes CRUD handlers for `/products`, `/variants`, and `/actors`, plus:
  - `POST /variants/:id/poll` for on-demand variant git metadata refresh.
  - `POST /variants/:id/actors/import` for opt-in import of provider-managed active sessions into actor rows.
  - actor lifecycle/interaction endpoints: `POST /actors/:id/poll`, `GET /actors/:id/attach`, `POST /actors/:id/messages`, `GET /actors/:id/messages`, `POST /actors/:id/commands`.
  - provider config discovery endpoint: `GET /system/providers`.
- Realtime websocket coverage now includes `GET /ws` (upgrade) with JSON RPC-like envelopes:
  - client `rpc_request` payloads (`method`, `path`, optional `query`/`body`) dispatch through the same HTTP route handlers.
  - server `rpc_response` payloads return `status`, `path`, and `body` using the same API success/failure shape as HTTP.
  - server broadcasts `event=routes.mutated` after successful `POST`/`PATCH`/`DELETE` route mutations (source=`http` or `ws`).

## 9) Provider Configuration

- Provider selection defaults are configured in core config TOML under `providers`.
- `providers.defaultProvider` controls spawn fallback when `/actors` create payload omits `provider`.
- `providers.enabledProviders` controls which provider keys are allowed for actor operations.
- Client/frontends should query `GET /system/providers` when they need current runtime provider configuration.

## 10) Script Conventions

- Project scripts use shebanged Bun TypeScript files (`#!/usr/bin/env bun`) under `scripts/`.
- Script naming uses `*.sh.ts` to indicate executable shell-style Bun scripts.
- Shared script helpers live under `scripts/helpers/`.
- `scripts/install.sh.ts` runs ordered bootstrap/install steps: `proto install`, Bun install (root-invoked), then `moon :install`.
- `scripts/dev.sh.ts` runs `moon run dark_core:dev`.
- `scripts/test.sh.ts` runs `moon run dark_core:test`.
- `scripts/dcli.sh.ts` runs `dark_cli` via Cargo from repo root and forwards all CLI args.
- `scripts/dchat.sh.ts` runs `dark_chat` via Cargo while preserving the launch CWD so OpenCode config resolution matches the caller terminal.
- `scripts/dtui.sh.ts` runs `dark_tui` via Cargo from repo root and forwards all CLI args.
- `scripts/sys_install.sh.ts` manages shell aliases and `DARKFACTORY_SRC_PATH` in user shell rc files.
- `scripts/reflect_constant.sh.ts` manages periodic reflector loops (`start`, `status`, `stop`, or foreground `run`).
- `scripts/reflect_constant.sh.ts` writes timestamped reflector outputs to `docs/reflections/`.
- `scripts/docker_build.sh.ts` builds layered container targets (`run`, `agentbox`, `ci`, `devcontainer`).
- `scripts/docker_devcontainer.sh.ts` manages devcontainer lifecycle (`up`, `attach`, `exec -- <cmd>`).
- `scripts/docker_agentbox.sh.ts` runs one-shot commands in the `agentbox` service and supports `--no-tty`.
- `scripts/docker_ci.sh.ts` runs CI commands in the `ci` service (default `moon run dark_core:test`).
- Short wrappers are available as executable paths: `./scripts/docker-build`, `./scripts/devcontainer`, `./scripts/agentbox`, and `./scripts/ci`.
- `dark_core:check` now includes `dark_core:typecheck` plus `format:check` for full checks.
- `scripts/` should remain shell-style pragmatic automation; prefer readability over strict TypeScript purity.
- TypeScript safe-fix loops should ignore `scripts/` diagnostics unless user explicitly opts in.
- External docs snapshots can be generated with source scripts like `scripts/scrapes/scrape_opencode_docs.sh.ts`, `scripts/scrapes/scrape_elysia_docs.sh.ts`, `scripts/scrapes/scrape_prisma_docs.sh.ts`, `scripts/scrapes/scrape_ratatui_web_docs.sh.ts`, and `scripts/scrapes/scrape_ratatui_docs_docs.sh.ts`.
- `scripts/helpers/docsrs_scrape.sh.ts` provides reusable docs.rs sitemap-shard discovery helpers for crate/version/module scoped scrapers.
- `scripts/scrapes/scrape_docs.sh.ts` dispatches supported source scrapers (`opencode`, `elysia`, `prisma`, `moonrepo`, `ratatui_web`, `ratatui_docs`).

## 11) Keep This File Updated

- Update this file when real code, tooling, or CI is added.
- Keep instructions tied to verified repository behavior.
- Prefer short, accurate guidance over aspirational process docs.
- When `prisma/schema.prisma` or `dark_core/src/modules/*/*.routes.ts` changes, update this file with any durable impacts on clients/contracts and agent workflow expectations.
