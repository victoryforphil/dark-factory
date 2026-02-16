---
name: devcontainer-workflows
description: Build layered Docker dev containers, attach quickly, and run agent commands in containerized workspaces.
---

## What I do

- Standardize Docker-based developer environments for this repository.
- Use layered images so runtime, CI, agentbox, and devcontainer targets stay clear and composable.
- Reuse helper scripts in `scripts/` for common compose workflows.

## Layer contract

- `common`: minimal shared runtime base.
- `build`: all build tooling (`proto`, `moon`, Bun, Rust toolchain support).
- `run`: runtime-only `dark_core` image from build outputs.
- `agentbox`: sibling of `run`, child of `build`, optimized for CLI/agent commands.
- `ci`: sibling of `run`, child of `build`, optimized for deterministic checks and tests.
- `devcontainer`: full DX image with shell and terminal tooling.

## Compose + script entrypoints

- Compose file: `docker/compose.devcontainers.yml`.
- Build all layers: `./scripts/docker-build`.
- Devcontainer lifecycle: `./scripts/devcontainer up|attach|run -- <cmd>`.
- Agentbox command run: `./scripts/agentbox run -- <cmd>`.
- CI command run: `./scripts/ci [run] [custom command...]`.

## Workflow rules

1. Keep runtime images minimal and avoid shipping build-only tools in `run`.
2. Keep `agentbox` and `ci` based on `build` to inherit toolchains consistently.
3. Mount repo as `/workspace` for interactive services (`agentbox`, `devcontainer`, `ci`).
4. Prefer helper scripts for repeatable local + agent workflows instead of ad-hoc docker commands.
