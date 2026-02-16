# dark_core

`dark_core` runs on Bun and exposes a small CLI through `src/index.ts`.

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

## Config behavior

- `config export` writes a TOML file from schema defaults.
- `config print` loads effective config with normal runtime behavior
  (defaults -> TOML -> env overrides), then prints redacted values.
- `--json` prints compact JSON; `--toml` prints TOML.
- Default config path is `dark_core/config.toml`.
