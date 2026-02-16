# `frontends/dark_cli` - Agent Notes

This file defines local guidance for agents working in `frontends/dark_cli`.

## Current-State First Rule

- `README.md` must describe only behavior that exists today.
- Planned work belongs in `TODO.md` and `PLANNING.md`, not in implementation-status sections of `README.md`.
- If implementation changes, update docs in the same task when practical.

## Package Context (Current Truth)

- Crate name: `dark_cli`.
- Entrypoint exists at `src/main.rs` and is currently a placeholder binary.
- `Cargo.toml` already includes baseline dependencies for future CLI expansion.

## Modularity Direction (Without Over-claiming)

- Keep command parsing concerns separate from transport concerns.
- Keep transport logic separate from output rendering logic.
- Keep configuration loading isolated from command handlers.
- Add modules only when used; avoid speculative scaffolding.

## Coding Conventions

- Prefer explicit, readable Rust over clever abstractions.
- Keep functions small and side effects obvious.
- Use typed structs/enums for CLI args, config, and responses once implemented.
- Return contextual errors; do not silently swallow failures.

## Logging and Errors

- Use structured, contextual log messages aligned with repository style:
  - `System // Optional Sub system // Message (Metadata)`
- Do not log secrets or tokens.
- Favor a consistent human-readable error path, with machine-oriented output formats added intentionally when implemented.
