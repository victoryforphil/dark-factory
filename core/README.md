# Dark Factory Core

This package is the early `core` service for the `dark-factory` project.

It currently provides a small Bun + Elysia HTTP service that acts as a starting point for Core APIs around:

- `world` tracking
- `env` management
- `actor` orchestration

## Current Status

- Early bootstrap service
- No database or persistence wired yet
- No validated test/lint/typecheck commands yet

## Endpoints (Current)

- `GET /` returns service metadata and project concepts
- `GET /health` returns basic health status

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

## Notes

- This package replaces the default Elysia template copy with repository-specific context.
- As Core capabilities are added, keep this README focused on verified behavior in this repo.
