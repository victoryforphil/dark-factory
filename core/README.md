# Dark Factory Core

This package is the early `core` service for the `dark-factory` project.

It currently provides a small Bun + Elysia HTTP service that acts as a starting point for Core APIs around:

- `product` tracking
- `variant` management
- `actor` orchestration

## Current Status

- Stage 0 product CRUD API is wired via protobuf endpoints
- Persistent local store uses `~/.darkfactory`
  - `~/.darkfactory/core.duckdb`
  - `~/.darkfactory/store.toml`
  - `~/.darkfactory/configs/*.toml`
- No validated test/lint/typecheck commands yet

## Endpoints (Current)

- Protobuf endpoints expect `Content-Type: application/x-protobuf`.
- `GET /` returns service metadata and project concepts
- `GET /health` returns basic health status
- `POST /v1/products/create`
- `POST /v1/products/get`
- `POST /v1/products/list`
- `POST /v1/products/update`
- `POST /v1/products/delete`

## Run Locally

From this directory (`core/`):

```bash
bun install
bun run dev
```

Server default:

- `http://localhost:3000`
- Override port with `PORT` env var

## Scripts

- `bun run dev` - watch mode for local development
- `bun run start` - run once without watch mode

## OpenCode Client

- `src/clients/opencode-client.ts` wraps the OpenCode SDK for Core usage.
- `src/helpers/opencode-config.ts` persists settings in the Core store as `configs/opencode.toml`.
- Supported config fields: `connection_mode` (`attach` or `managed`), `hostname`, `port`, `timeout_ms`, optional `base_url`, optional `directory`.
- Environment defaults are available via:
  - `DARKFACTORY_OPENCODE_CONNECTION_MODE`
  - `DARKFACTORY_OPENCODE_HOSTNAME`
  - `DARKFACTORY_OPENCODE_PORT`
  - `DARKFACTORY_OPENCODE_TIMEOUT_MS`
  - `DARKFACTORY_OPENCODE_BASE_URL`
  - `DARKFACTORY_OPENCODE_DIRECTORY`

## Notes

- This package replaces the default Elysia template copy with repository-specific context.
- As Core capabilities are added, keep this README focused on verified behavior in this repo.
