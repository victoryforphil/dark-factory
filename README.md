# Dark Factory - Autonomous Agentic Development At Scale

# Concept

- A central `dark-factory` "core" is used to track `products`
- A `product` is the canonical definition of a code product we want to work on
  - In Stage 0 this is identified by a local path-based locator key
  - Locators are path/url-like identifiers, currently using `@local://{abs_path}`
  - `product` may optionally include a `display_name` for human-friendly rendering
- A `variant` is a spawned instance of a `product` where work actually runs
  - In Stage 0 a local product immediately gets one default variant on creation
  - The default variant inherits the product locator path directly
  - Additional variants can be created at the same location path (for example `default`, `wt-main`, `wt-exp`) and are distinguished by `name`
- Stage 0 topology is intentionally strict and simple:
  - One product locator
  - One default variant created automatically
  - One actor bound to that variant
- `actor` is a spawned agent that operates on a variant
  - Can be various agent backends (currently scoped in OpenCode, Codex and a custom agent), but first iteration is JUST OpenCode
  - Future work will allow multiple variants per product and multiple actors where operations allow

# Stack + Tools

- Agentic Coding: `opencode`
- Main languages: `rust` and `bun` / `typescript`
- Scripting: shebanged Bun TypeScript scripts under `scripts/` (`*.sh.ts`)
- Optional build system: `moon` / `proto`
- Common Schema Definition: `prisma`

# Scripts

- Project shell-style scripts are Bun TypeScript files with shebangs: `#!/usr/bin/env bun`.
- Shared helpers live in `scripts/helpers/`.
- Toolchain/bootstrap install runs through `bun scripts/install.sh.ts` (`proto install` -> root-driven Bun install -> `moon :install`).
- Dev/test entrypoints are `bun scripts/dev.sh.ts` and `bun scripts/test.sh.ts`.
- External docs snapshots can be generated with source scripts (for example `scripts/scrapes/scrape_opencode_docs.sh.ts`, `scripts/scrapes/scrape_elysia_docs.sh.ts`, and `scripts/scrapes/scrape_prisma_docs.sh.ts`).
- Use `bun scripts/scrapes/scrape_docs.sh.ts <source>` to dispatch a source scraper (`opencode`, `elysia`, `prisma`, `moonrepo`).


# Components

## Core

- Central service that manages tracking of all products/variants/actors/settings, etc.
  - TBD: will either be stateless w/ DB and route all commands just to the agents
    - OR: a background API service (REST even) we can query from frontends
- Currently looks like it will be Bun + Elysia JS
- Moon support is configured via `.moon/workspace.yml`, `.moon/toolchains.yml`, `dark_core/moon.yml`, `prisma/moon.yml`, `generated/moon.yml`, `lib/dark_rust/moon.yml`, and `frontends/dark_cli/moon.yml`.
- `prisma:build` uses `bunx prisma generate --schema schema.prisma` and writes generated outputs to `generated/prisma/` and `generated/prismabox/`.
- `dark_core` tasks (`install`, `build`, `dev`, `check`, `test`) depend on generated artifacts through `generated:build` -> `prisma:build`.
- Verified command: `moon run dark_core:test` (passes and runs Prisma generation dependency chain).
- Product and variant CRUD endpoints are now available (`/products/*` and `/variants/*`).

## Frontends

- Various frontends that invoke / connect to Core
- Main way of iterating with the system
- Uses the Core API to communicate (REST / WS or GRPC in the future)
- Some Frontends:
  - Rust TUI (Ratatui.rs) - First One
  - Web Client (Bun + Vite + Shadcn + React)
  - Pure CLI (Rust)

## Agents

- Abstracted communication with actual agents doing work
- First is OpenCode
