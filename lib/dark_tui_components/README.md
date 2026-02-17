# dark_tui_components

Reusable Ratatui-first building blocks shared across Dark Factory frontends.

This crate centralizes composable widgets, utility helpers, and lightweight
component framework primitives so frontend crates can share UI behavior without
sharing app state.

## Included Modules

- `components/`
  - pane and status primitives: `PaneBlockComponent`, `StatusPill`, `SectionHeader`
  - chat primitives: `ChatConversationHeaderComponent`, `ChatMessageListComponent`, `ChatComposerComponent`
  - overlay + footer primitives: `PopupOverlay`, `FooterBar`
  - generic UI helpers: `CardGridComponent`, `KeyHintBar`, `LabeledField`, `LoadingSpinner`
- `utils/`
  - compacting helpers: `compact_*` text/id/locator/timestamp/session helpers
  - geometry helpers: `rect_contains`, `inner_rect`, `with_cursor_tail`
  - index helpers: `next_index`, `previous_index`
  - list viewport helper: `ListViewport`
- component framework primitives
  - `Action`, `Event`, and `Component` trait for app-level composition

## Theme Contract

Components read colors through `ComponentThemeLike`.

- Use `ComponentTheme::default()` for a ready-to-use baseline.
- Or implement `ComponentThemeLike` for your app-specific theme type.

This keeps components reusable while avoiding a hard dependency on one app's
theme struct.

## Example

See `examples/components_preview.rs` for a minimal usage sample.

## Testing

- Unit tests live with modules in `src/components/*`.
- Integration tests live under `tests/`.
- Run crate tests with:

```bash
cargo test -p dark_tui_components
```
