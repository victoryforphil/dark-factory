# Dark Factory - `dark_cli`

Rust CLI frontend for interacting with `dark_core`.

## Current Status

- This package is scaffolded and not feature-complete.
- Runtime entrypoint currently prints `Hello, world!` from `src/main.rs`.
- Command routing, config loading, and API calls are not implemented yet.

## Scope (Current)

- Provide a Rust-based CLI surface for `dark_core`.
- Track CLI implementation in this package.
- Keep docs accurate to what is currently implemented.

## Implemented Today

- Rust crate setup via `Cargo.toml`.
- Dependency baseline for planned CLI architecture:
  - `clap` (argument parsing)
  - `serde`, `serde_json`, `toml` (serialization/config)
  - `reqwest` (HTTP client)
  - `anyhow`, `thiserror` (error handling)
  - `log`, `pretty_env_logger` (logging)

## Not Implemented Yet

- User-facing commands beyond the placeholder binary run.
- `--format` output selection behavior.
- Config loading from `.env`/`.env.template`/TOML.
- HTTP command execution against `dark_core`.
- WebSocket transport support.

## API Reference Endpoints (Optional)

When `dark_core` is running locally, these endpoints are useful for manual API exploration:

- `http://localhost:4150/llms.txt`
- `http://localhost:4150/openapi/json`

## Development Notes

- Keep `README.md` current-state only.
- Put actionable implementation tasks in `TODO.md`.
- Put staged future planning in `PLANNING.md`.
