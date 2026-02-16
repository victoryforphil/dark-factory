# dark_core Config System Design

## Goals

- Keep config definition human-readable and close to owning domains.
- Use one schema source for static typing + runtime validation.
- Layer values predictably: defaults, file, then env overrides.
- Support operator-friendly TOML export/inspection via CLI.
- Keep runtime behavior Bun-native and simple.

## High-level Approach

- Validation + types: `zod`
- File format: TOML via `@iarna/toml`
- Runtime target: Bun
- Root config is declared in `dark_core/src/config/core.config.ts`.
- Root env semantics are declared in `dark_core/src/config/env.config.ts`.
- Loader/runtime helpers are under `dark_core/src/config/lib/`.

## Ownership Model

- Root composition:
  - `dark_core/src/config/core.config.ts`
- Root env semantics:
  - `dark_core/src/config/env.config.ts`
- Section definitions (owned by domain files):
  - Server section: `dark_core/src/controllers/system.config.ts`
  - Prisma section: `dark_core/src/clients/prisma.config.ts`

This keeps defaults and shape rules local to the subsystem that owns them.

## Data Model

- Root config shape:
  - `env: string`
  - `server: { listenHost: string; listenPort: number }`
  - `prisma: { logQueries: boolean }`

- Explicit env bindings are global and direct (no prefix expansion), for example:
  - `DARKFACTORY_ENV` -> `env`
  - `DARKFACTORY_SERVER_LISTEN_HOST` -> `server.listenHost`
  - `DARKFACTORY_SERVER_PORT` -> `server.listenPort`
  - `DARKFACTORY_PRISMA_LOG_QUERIES` -> `prisma.logQueries`

## Resolution Order

Effective config is built in this order:

1. Zod defaults from schema
2. TOML file (`dark_core/config.toml` by default)
3. Explicit env bindings

After merge, the result is validated again by schema.

## Strictness Behavior

- In production, strict mode is on by default.
- Strictness can be overridden via loader options (`strict` or `allowUnknown`).
- Strict mode rejects unknown keys to catch typos.

## CLI Surface

Entry: `dark_core/src/index.ts`

- Default command starts the server:
  - `bun run src/index.ts`
- Config command suite:
  - `bun run src/index.ts config export [--path <path>]`
  - `bun run src/index.ts config print [--path <path>] [--json|--toml]`

### `config export`

- Builds config from defaults only.
- Writes TOML to disk.
- Default output path: `dark_core/config.toml`.

### `config print`

- Loads config using normal runtime behavior (defaults -> file -> env).
- Prints redacted output by default.
- Formats:
  - default pretty JSON
  - `--json` compact JSON
  - `--toml` TOML output

## Security Notes

- Print path uses redaction for likely secret keys (`password`, `token`, `secret`, key-like fields).
- No secret-specific store is implemented yet.
- Future `~/.darkfactory` support should preserve redaction and precedence semantics.

## Future Direction

- Add optional home-level config support (`~/.darkfactory`) as another file layer.
- Keep root composition minimal and continue defining section defaults near owner modules.
- Extend sections (database URL, telemetry, etc.) by adding a local `*.config.ts` and referencing it from root.
