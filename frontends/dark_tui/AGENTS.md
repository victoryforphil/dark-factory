# `frontends/dark_tui` - Agent Notes

This file defines local guidance for agents working in `frontends/dark_tui`.

## Current-State First Rule

- `README.md` must describe only behavior that exists today.
- Planned work belongs in follow-up docs, not in implementation-status sections of `README.md`.

## Package Context (Current Truth)

- Crate name: `dark_tui`.
- Entrypoint exists at `src/main.rs`.
- TUI rendering uses `ratatui` with a `crossterm` backend.
- Shared dark_core REST + websocket client/types live in `lib/dark_rust` and should be reused.
- Shared reusable UI primitives live in `lib/dark_tui_components` and should be preferred over local one-off widget copies.
- Shared chat panel primitives come from `dark_chat::framework` and should be preferred over local chat UI duplication.

## Modularity Direction

- Keep app state, API orchestration, and rendering in separate modules.
- Keep service transport/wire/conversion concerns split across `service.rs`, `service_wire.rs`, and `service_convert.rs`.
- Keep catalog layout logic separate from card rendering helpers (`unified_catalog_view.rs` + `catalog_cards.rs`).
- Keep key handling explicit and centralized.
- Keep render functions focused per pane/widget.

## Runtime and UX

- Preserve keyboard-first usage and human-readable status messaging.
- Avoid blocking operations outside explicit refresh/action paths.
- Keep pane labels and action hints aligned with real behavior.
