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
| `opencode state --directory <path>` | `GET /opencode/state` | OpenCode directory runtime state |
| `opencode sessions list --directory <path>` | `GET /opencode/sessions` | List OpenCode sessions |
| `opencode sessions create --directory <path> [--title <title>]` | `POST /opencode/sessions` | Create OpenCode session |
| `opencode sessions get --id <id> --directory <path> [--include-messages]` | `GET /opencode/sessions/:id` | Get session state |
| `opencode sessions attach --id <id> --directory <path> [--model <model>] [--agent <agent>]` | `GET /opencode/sessions/:id/attach` | Build attach command payload |
| `opencode sessions command --id <id> --directory <path> --command <command>` | `POST /opencode/sessions/:id/command` | Send command to session |
| `opencode sessions prompt --id <id> --directory <path> --prompt <prompt> [--no-reply]` | `POST /opencode/sessions/:id/prompt` | Send prompt to session |
| `opencode sessions abort --id <id> --directory <path>` | `POST /opencode/sessions/:id/abort` | Abort running session operation |
| `opencode sessions delete --id <id> --directory <path>` | `DELETE /opencode/sessions/:id` | Delete session |

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

OpenCode sessions:

```bash
# List sessions for a directory (pretty table)
dcli opencode sessions list --directory /Users/alex/repos/vfp/dark-factory

# Create a session
dcli opencode sessions create --directory /Users/alex/repos/vfp/dark-factory --title "My Session"

# Send prompt to a session
dcli opencode sessions prompt --id <session-id> --directory /Users/alex/repos/vfp/dark-factory --prompt "status"
```

## API Reference Endpoints (Optional)

When `dark_core` is running locally, these endpoints are useful for manual API exploration:

- `http://localhost:4150/llms.txt`
- `http://localhost:4150/openapi/json`

## Development Notes

- Keep `README.md` current-state only.
- Put actionable implementation tasks in `TODO.md`.
- Put staged future planning in `PLANNING.md`.
