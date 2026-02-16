# `frontends/dark_tui` - Agent Notes

This file defines local guidance for agents working in `frontends/dark_tui`.

## Current-State First Rule

- `README.md` must describe only behavior that exists today.
- Planned work belongs in follow-up docs, not in implementation-status sections of `README.md`.

## Package Context (Current Truth)

- Crate name: `dark_tui`.
- Entrypoint exists at `src/main.rs`.
- TUI rendering uses `ratatui` with a `crossterm` backend.
- Shared dark_core HTTP client/types live in `lib/dark_rust` and should be reused.

## Modularity Direction

- Keep app state, API orchestration, and rendering in separate modules.
- Keep key handling explicit and centralized.
- Keep render functions focused per pane/widget.

## Runtime and UX

- Preserve keyboard-first usage and human-readable status messaging.
- Avoid blocking operations outside explicit refresh/action paths.
- Keep pane labels and action hints aligned with real behavior.
