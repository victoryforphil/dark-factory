# Docker Layers

This directory contains layered container targets for dark-factory.

- `common`: minimal shared runtime base.
- `build`: installs build-only toolchain and compiles `dark_core`.
- `run`: copies only runtime outputs required to run `dark_core`.
- `agentbox`: CLI/agent workflow image that includes build toolchain.
- `ci`: CI/test image that includes build toolchain and defaults to tests.
- `devcontainer`: developer UX image with shell/editor terminal tools.

Compose stack: `docker/compose.devcontainers.yml`.

Helper scripts:

- `./scripts/docker-build`
- `./scripts/devcontainer up`
- `./scripts/devcontainer attach`
- `./scripts/agentbox run -- moon run dark_core:check`
- `./scripts/ci`
