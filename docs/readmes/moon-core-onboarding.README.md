# Moon + Core Onboarding Notes

Last reviewed: 2026-02-16

## Goal

Bootstrap `@dark-factory/core` with moon so protobuf generation from `schemas/` is guaranteed before core start/dev tasks.

## What was added

- Workspace config:
  - `.moon/workspace.yml`
  - `.moon/toolchains.yml`
- Core moon project config:
  - `core/moon.yml`
- Scripts:
  - `scripts/proto_codegen_core.sh.ts`
  - `scripts/moon_core.sh.ts`

## Working command flow

From repository root:

```bash
bun scripts/install.sh.ts
bun scripts/moon_core.sh.ts start
```

For local watch mode:

```bash
bun scripts/moon_core.sh.ts dev
```

## PTY validation notes

- `moon run core:start` runs successfully and starts Elysia at `127.0.0.1:4150`.
- `moon run core:dev` runs successfully in watch mode.
- Both targets run `core:install` + `core:codegen-proto` before starting the server.

## Proto config compatibility note

- Current environment expects plugin declarations under `[plugins]` in `.prototools`.
- `[plugins.tools]` failed parse under the current moon/proto runtime pairing.
