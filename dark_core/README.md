# dark_core

`dark_core` runs on Bun and exposes a small CLI through `src/index.ts`.

## Source layout

- Domain-first modules live under `src/modules/`.
- Each module groups its route/controller/config/client/tests by type (for example `src/modules/products/products.routes.ts`, `src/modules/products/products.controller.ts`, and colocated tests).
- Shared cross-module utilities remain in `src/utils/`, while config loader internals stay in `src/config/lib/`.

## Commands

Start server (default behavior):

```bash
bun run src/index.ts
```

Or via script:

```bash
bun run start
```

Config command suite:

```bash
bun run src/index.ts config export [--path <path>]
bun run src/index.ts config print [--path <path>] [--json|--toml]
```

Script aliases:

```bash
bun run config:export
bun run config:print
```

## Product and Variant API

- Products support CRUD at `/products/` and `/products/:id`.
- Variants support CRUD at `/variants/` and `/variants/:id`.
- Creating a local product (`@local://...`) automatically creates a default variant (`name=default`) with the same locator path.
- Multiple variants can share the same locator path and are distinguished by `name`.

## Config behavior

- `config export` writes a TOML file from schema defaults.
- `config print` loads effective config with normal runtime behavior
  (defaults -> TOML -> env overrides), then prints redacted values.
- `--json` prints compact JSON; `--toml` prints TOML.
- Default config path is `dark_core/config.toml`.

## Testing workflow

- Unit tests use Bun's test runner and avoid database I/O where possible.
- Integration tests use a unique SQLite file per test under `.darkfactory/test/`.
- Test setup runs `prisma db push` dynamically for each test database, so schema comes from `prisma/schema.prisma` instead of hard-coded SQL.
- Integration runs are serialized (`--max-concurrency 1`) to keep per-test env overrides deterministic.

Useful commands:

```bash
bun run test
bun run test:unit
bun run test:int
bun run test:watch
bun run test:int:watch
```
