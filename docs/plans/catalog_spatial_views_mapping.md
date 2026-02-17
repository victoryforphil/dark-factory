# Catalog Spatial Views - Implementation Mapping

This mapping translates the approved visual direction into `dark_tui` implementation targets.

## Approved direction

- `ResultsViewMode::Viz` (`graphical-tree`) -> replace with **Station** layout family.
- `ResultsViewMode::Graph` (`graphical-node`) -> replace with **Orbit** layout family.
- Sub-agents should always render as **boxy grid blocks under actor cards**.

Design references:

- Station base: `docs/reports/graph-node-round10-station-final.svg`
- Station scale: `docs/reports/graph-node-round10-station-final-12v8a.svg`
- Orbit base: `docs/reports/graph-node-round10-orbit-final.svg`
- Orbit scale: `docs/reports/graph-node-round10-orbit-final-12v8a.svg`

## Current code surface

- View modes: `frontends/dark_tui/src/app/state.rs`
  - `ResultsViewMode::Viz` label `graphical-tree`
  - `ResultsViewMode::Graph` label `graphical-node`
- Render routing: `frontends/dark_tui/src/ui/render/mod.rs`
  - Viz route currently uses `UnifiedCatalogView`
  - Graph route uses `GraphCatalogView`
- Tree-like view: `frontends/dark_tui/src/ui/render/views/unified_catalog_view.rs`
- Node-like view: `frontends/dark_tui/src/ui/render/views/graph_catalog_view.rs`
- Shared view exports: `frontends/dark_tui/src/ui/render/views/mod.rs`
- View toggle status text: `frontends/dark_tui/src/ui/mod.rs`

## Target architecture

- Keep mode names and toggles unchanged (no UX rename required now).
- Replace internals only:
  - `UnifiedCatalogView` becomes Station renderer implementation.
  - `GraphCatalogView` becomes Orbit renderer implementation.
- Factor composable primitives to keep both views maintainable.

Suggested component split (same crate path, small focused modules):

- `ui/render/views/components/spatial_types.rs`
  - `WorldRect`, hit-test helpers, world->screen conversion
- `ui/render/views/components/spatial_primitives.rs`
  - box draw, hline/vline, elbow connector draw, text cell helpers
- `ui/render/views/components/station_layout.rs`
  - station world layout computation (product-left, rail, variant columns)
- `ui/render/views/components/orbit_layout.rs`
  - orbit world layout computation (center, tiers, mirrored actors)
- `ui/render/views/components/sub_agent_grid.rs`
  - shared boxy grid renderer used by BOTH Station and Orbit

## Behavioral parity checklist

- Preserve click selection behavior:
  - product/variant/actor hit-test must still resolve into `VizSelection`
- Preserve panning and viewport clipping behavior
- Preserve connector state/activity coloring logic where present
- Preserve fallback behavior for small terminal sizes
- Preserve panel titles and legend semantics per mode

## Integration order

1. Extract shared spatial primitives + sub-agent grid component.
2. Implement Station layout internals in `UnifiedCatalogView`.
3. Implement Orbit layout internals in `GraphCatalogView`.
4. Rewire hit-testing to new layout structs.
5. Validate in both base and scaled datasets.
