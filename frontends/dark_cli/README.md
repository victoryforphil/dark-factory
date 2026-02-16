# Dark Factory - `dark_cli`

Rust CLI frontend for interacting with `dark_core`.

## Current Status

- Command routing is active and calls `dark_core` over HTTP.
- `init` creates a product using the current directory (or a provided path).
- `info` summarizes the current directory's product/variant state and polls variants before rendering.
- `products list` defaults to listing all products (paged client-side).
- Pretty output now renders table-style output for all responses by default.

## Scope (Current)

- Provide a Rust-based CLI surface for `dark_core`.
- Track CLI implementation in this package.
- Keep docs accurate to what is currently implemented.

## Implemented Today

- Rust crate setup via `Cargo.toml`.
- Shared Rust API client in `lib/dark_rust`.
- CLI commands are in parity with `lib/dark_rust` route methods.
- Output formats:
  - default `pretty`
  - `json`
  - `toml`
- Dependency baseline:
  - `clap` (argument parsing)
  - `serde`, `serde_json`, `toml` (serialization/config)
  - `dark_rust` (shared API client/types)
  - `anyhow` (error handling)
  - `log`, `pretty_env_logger` (logging)
  - `prettytable-rs` (pretty table output)

## Not Implemented Yet

- WebSocket transport support.
- End-to-end HTTP integration tests for Rust client/CLI command execution.

## Command Reference

Global options:

| Option | Env | Default | Description |
| --- | --- | --- | --- |
| `--base-url <URL>` | `DARK_CORE_BASE_URL` | `http://localhost:4150` | Base URL for `dark_core` |
| `--format <pretty\|json\|toml>` | `DARK_CLI_FORMAT` | `pretty` | Output format |

Commands:

| CLI Command | HTTP Route | Description |
| --- | --- | --- |
| `init [path]` | `POST /products/` | Create a product using current dir or provided path; `displayName` is directory name |
| `info [path]` | `GET /variants/` + `POST /variants/:id/poll` + `GET /products/:id` | Resolve directory locator, poll active variants, and print a world-state summary |
| `service status` | `GET /` | Service root status |
| `system health` | `GET /system/health` | System health payload |
| `system info` | `GET /system/info` | Service info payload |
| `system metrics` | `GET /system/metrics` | Runtime metrics payload |
| `system providers` | `GET /system/providers` | Active provider configuration and availability |
| `system reset-db` | `POST /system/reset-db` | Back up local DB and clear products/variants |
| `products list [--cursor <id>] [--limit <n>]` | `GET /products/` | List products; without `cursor/limit`, CLI fetches all pages |
| `products create --locator <path> [--display-name <name>]` | `POST /products/` | Create a product directly |
| `products get --id <id>` | `GET /products/:id` | Get product by id |
| `products update --id <id> [--locator <path>] [--display-name <name>]` | `PATCH /products/:id` | Update product fields |
| `products delete --id <id>` | `DELETE /products/:id` | Delete product |
| `variants list [--cursor <id>] [--limit <n>] [--product-id <id>] [--locator <path>] [--name <name>]` | `GET /variants/` | List variants with optional filters |
| `variants create --locator <path> --product-id <id> [--name <name>]` | `POST /variants/` | Create variant linked to product |
| `variants get --id <id>` | `GET /variants/:id` | Get variant by id |
| `variants poll --id <id>` | `POST /variants/:id/poll` | Refresh git metadata/status for one variant |
| `variants update --id <id> [--locator <path>] [--name <name>]` | `PATCH /variants/:id` | Update variant fields |
| `variants delete --id <id>` | `DELETE /variants/:id` | Delete variant |
| `actors list [--cursor <id>] [--limit <n>] [--variant-id <id>] [--product-id <id>] [--provider <name>] [--status <label>]` | `GET /actors/` | List actors with optional filters |
| `actors create --variant-id <id> [--provider <name>] [--title <title>] [--description <text>]` | `POST /actors/` | Spawn actor attached to variant (provider defaults from core config) |
| `actors get --id <id>` | `GET /actors/:id` | Get actor state |
| `actors update --id <id> [--title <title>] [--description <text>]` | `PATCH /actors/:id` | Update actor metadata |
| `actors delete --id <id> [--terminate]` | `DELETE /actors/:id` | Delete actor (optionally terminate provider runtime) |
| `actors poll --id <id>` | `POST /actors/:id/poll` | Refresh actor runtime status |
| `actors attach --id <id> [--model <model>] [--agent <agent>]` | `GET /actors/:id/attach` | Build provider attach command |
| `actors messages send --id <id> --prompt <prompt> [--no-reply]` | `POST /actors/:id/messages` | Send provider-backed prompt |
| `actors messages list --id <id> [--n-last-messages <n>]` | `GET /actors/:id/messages` | Read provider-backed messages |
| `actors commands --id <id> --command <command> [--args <args>]` | `POST /actors/:id/commands` | Run provider command |

Pretty rendering defaults:

- `pretty` output is the default and renders all responses in a table-oriented format.
- Product and variant payloads include git metadata fields when the API returns them.
- Use `--format json` to force JSON output.
- Use `--format toml` to force TOML output.

## Quick Examples

Use the repo helper script:

```bash
./scripts/dcli.sh.ts --help
```

Optional local alias:

```bash
alias dcli='./scripts/dcli.sh.ts'
```

Product lifecycle:

```bash
# Create from current directory
dcli init

# Summarize current directory world state
dcli info

# List all products (pretty table)
dcli products list

# Create explicitly
dcli products create --locator /tmp/demo-project --display-name demo-project

# Get one product
dcli products get --id <product-id>

# Update one product
dcli products update --id <product-id> --display-name demo-project-updated

# Delete one product
dcli products delete --id <product-id>
```

Variant lifecycle:

```bash
# Create variant linked to a product
dcli variants create --locator /tmp/demo-project/default --product-id <product-id> --name default

# List variants (pretty table)
dcli variants list

# List variants for one product
dcli variants list --product-id <product-id>

# Get one variant
dcli variants get --id <variant-id>

# Poll and refresh git metadata
dcli variants poll --id <variant-id>

# Update one variant
dcli variants update --id <variant-id> --name updated

# Delete one variant
dcli variants delete --id <variant-id>
```

Actors:

```bash
# List actors for a provider
dcli actors list --provider mock

# Inspect configured providers/default
dcli system providers

# Spawn actor on a variant
dcli actors create --variant-id <variant-id> --title "My Actor"

# Send prompt to actor
dcli actors messages send --id <actor-id> --prompt "status"
```

## API Reference Endpoints (Optional)

When `dark_core` is running locally, these endpoints are useful for manual API exploration:

- `http://localhost:4150/llms.txt`
- `http://localhost:4150/openapi/json`

## Development Notes

- Keep `README.md` current-state only.
- Put actionable implementation tasks in `TODO.md`.
- Put staged future planning in `PLANNING.md`.
