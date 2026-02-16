# dark_tui_components

Reusable Ratatui-first building blocks shared across Dark Factory frontends.

This crate extracts composable widgets from `frontends/dark_tui` so multiple
systems can share a consistent component layer while keeping app state and
domain logic in each consumer crate.

## Included Components

- `PaneBlockComponent`
- `StatusPill`
- `KeyHintBar` and `KeyBind`
- `LabeledField`
- `SectionHeader`
- `CardGridComponent`
- `LoadingSpinner`

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
