# Dark Factory - Agent Operating Guide

This file is for coding agents working in `dark-factory`.
It reflects only what is currently true in this repository.

## 1) Current Repository Context

- The repository contains active Bun/TypeScript code under `dark_core/` and Prisma schema/config under `prisma/`.
- Prisma models currently include `Product`, `Variant`, and `Actor` with these core assumptions:
  - Creating a local product (`@local://`) should also create a default variant at the same locator path.
  - Product identifiers can canonicalize to git locators (`@git://{remote}#{branch}`) when local product creation detects a valid git remote+branch; default variants still keep local working locators.
  - Variant locator values are not globally unique; multiple variants can share a locator and are differentiated by `(productId, name)`.
  - Product IDs are deterministic `prd_{token}` values derived from normalized locator text (SHA-256 first 64 bits encoded as fixed-width base36); repeated creates for the same canonical locator are idempotent.
  - `Product.workspaceLocator` stores optional local (`@local://...`) workspace home for clone destination defaults.
  - `Product.gitInfo` and `Variant.gitInfo` are optional JSON snapshots populated from local git metadata when available.
  - Variant git polling timestamps are tracked via `gitInfoUpdatedAt` and `gitInfoLastPolledAt`.
  - Actors attach to variants (`Variant 1 -> N Actors`) and store provider/runtime snapshots (`actorLocator`, `workingLocator`, `providerSessionId`, `attachCommand`, `connectionInfo`).
  - Actors now include optional `subAgents` JSON snapshots for provider-defined nested read-only sub-agent/thread metadata.
- Rust workspace code now includes:
  - `frontends/dark_cli/` for the CLI binary.
  - `frontends/dark_chat/` for the OpenCode-focused chat TUI frontend (library + binary).
    - `framework/` is the shared chat UI/state surface consumed by `dark_chat` and `dark_tui`.
    - `providers/opencode_*` modules now split provider surface, transport, realtime, extract helpers, and wire DTOs.
    - Tinyverse-style `insta` snapshots now cover `tui/panels/chat_panel.rs` under `frontends/dark_chat/src/tui/panels/snapshots/` for default + selector-open render states.
  - `frontends/dark_tui/` for the Ratatui dashboard frontend.
    - service internals are split across `service.rs`, `service_wire.rs`, and `service_convert.rs`.
    - unified catalog rendering helpers are split into `ui/render/views/catalog_cards.rs`.
  - `lib/dark_rust/` for shared dark_core API client/types used by Rust frontends.
  - `lib/dark_tui_components/` for reusable Ratatui component primitives shared by Rust TUIs.
    - includes shared `utils/` helpers (`compact`, `rect`, `index`, `viewport`) and component framework primitives (`Action`, `Event`, `Component`).
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
- OpenCode project config now exists at `opencode.jsonc` and allows external-directory access for `~/repos/vfp/**`.
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
- `.opencode/agents/planner.md` is a planning-first subagent for interactive scope gathering before implementation.
- `.opencode/agents/developer_senior.md` is a general senior coding subagent for complex, logic-heavy implementation tasks.
- `.opencode/agents/developer_jr.md` is a lightweight coding subagent for simpler cleanup/refactor/script tasks (model: `opencode/big-pickle`).
- `.opencode/skills/builder/SKILL.md` documents parallel divide-and-conquer delegation between `@developer_senior` and `@developer_jr`.
- `.opencode/skills/gitter-commit/SKILL.md` documents when to route commit tasks to `@gitter`.
- `.opencode/skills/tsc-fix/SKILL.md` documents safe-first TypeScript error remediation.
- `.opencode/skills/proto-install/SKILL.md` documents standardized script-based `proto install` usage.
- `.opencode/skills/script-authoring/SKILL.md` documents how to build reusable Bun scripts from prompt/example/bash/context.
- `.opencode/skills/devcontainer-workflows/SKILL.md` documents layered Docker devcontainer workflows and helper script usage.
- `.opencode/skills/docs-scraping/SKILL.md` documents reusable external docs scraping workflows.
- `.opencode/skills/research/SKILL.md` documents inspection-first, evidence-backed research workflows using `@explore`.
- `.opencode/skills/spatial-layout-iteration/SKILL.md` documents a designer+explore iteration loop for Station/Orbit spatial layout parity with snapshot-based checks.
- `.opencode/skills/reflect/SKILL.md` documents quick end-of-task reflection and lesson capture.
- `.opencode/skills/reflect-constant/SKILL.md` documents periodic background reflection with `@reflector`.
- `.opencode/commands/scrape_docs.md` provides the command entrypoint for docs scraping tasks.
- `.opencode/commands/adhd.md` provides a two-phase plan-then-build entrypoint that pairs `@planner` with parallel `@developer_senior`/`@developer_jr` execution.
- `.opencode/commands/tsc_fix.md` provides the command entrypoint for safe TypeScript fixing loops.
- `.opencode/commands/rust_lint.md` provides the command entrypoint for Rust lint/build diagnostics with parallel `@developer_jr` safe-fix batches.
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
  - For `dark_chat` component debugging/iteration, use the `insta` snapshot tests in `frontends/dark_chat/src/tui/panels/chat_panel.rs` and snapshot fixtures in `frontends/dark_chat/src/tui/panels/snapshots/`.
  - Recommended loop: `cargo test -p dark_chat tui::panels::chat_panel::tests` for verification and `INSTA_UPDATE=always cargo test -p dark_chat tui::panels::chat_panel::tests` when intentionally accepting UI snapshot updates.
- Use `@planner` when work is still being shaped and you need interactive clarification before implementation.
- Use `@explore` when inspecting code or gathering implementation context before editing.
- Launch multiple `@explore` tasks in parallel when inspection tracks are independent.
- Use `@developer_senior` for free-form or tricky implementation where reasoning depth matters.
- Use `@developer_jr` for smaller, bounded coding work such as script cleanup and low-risk refactors.
- Use `builder` skill when a task splits into multiple tracks so parent agents can orchestrate these subagents in parallel.
- Use `research` skill when a task is analysis-heavy and requires file-backed findings before implementation.

## 8) Dark Core API Coverage

- Current REST coverage includes CRUD handlers for `/products`, `/variants`, and `/actors`, plus:
  - product-scoped variant routes: `GET /products/:id/variants`, `POST /products/:id/variants`, and `POST /products/:id/variants/clone`.
  - `POST /variants/:id/poll` for on-demand variant git metadata refresh.
  - `POST /variants/:id/actors/import` for opt-in import of provider-managed active sessions into actor rows.
    - OpenCode provider imports treat root sessions as actors and attach nested child sessions as `subAgents` snapshots on the parent actor.
  - actor lifecycle/interaction endpoints: `POST /actors/:id/poll`, `GET /actors/:id/attach`, `POST /actors/:id/messages`, `GET /actors/:id/messages`, `POST /actors/:id/commands`.
  - `PATCH /actors/:id` accepts optional `variantId` and updates `workingLocator` to the destination variant locator so actors can move between variants.
  - actor create/update payloads may include optional `subAgents` snapshots; actor responses can return `subAgents` alongside existing actor fields.
  - provider config discovery endpoint: `GET /system/providers`.
- Realtime websocket coverage now includes `GET /ws` (upgrade) with JSON RPC-like envelopes:
  - client `rpc_request` payloads (`method`, `path`, optional `query`/`body`) dispatch through the same HTTP route handlers.
  - server `rpc_response` payloads return `status`, `path`, and `body` using the same API success/failure shape as HTTP.
  - server broadcasts `event=routes.mutated` after successful `POST`/`PATCH`/`DELETE` route mutations (source=`http` or `ws`).

## 9) Provider Configuration

- Provider selection defaults are configured in core config TOML under `providers`.
- `providers.defaultProvider` controls spawn fallback when `/actors` create payload omits `provider`.
- `providers.enabledProviders` controls which provider keys are allowed for actor operations.
- OpenCode import fallback now supports loading recent sessions when the status map is empty via `opencode.includeRecentSessionsWhenStatusEmpty`, `opencode.recentSessionWindowHours`, and `opencode.recentSessionLimit`.
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
- `scripts/tui_spatial_snapshots.sh.ts` runs `cargo test -p dark_tui spatial_snapshots` and supports `--update` (`INSTA_UPDATE=always`) for accepted snapshot updates.
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
