# Spatial Layout Iteration Skill

Use this skill when iterating on `dark_tui` spatial layouts (Station/Orbit) against design references.

## Goal

Converge quickly on visual parity while preserving selection/hit-test/pan behavior.

## Loop

1. Run parallel context discovery with `@explore`:
   - render path wiring
   - hit-test call paths
   - pan offset read/write points
2. Run two focused `@designer` reviews:
   - Station-only deltas + fixes
   - Orbit-only deltas + fixes
3. Implement only high-impact fixes in one small slice.
4. Verify compile:
   - `cargo check -p dark_tui`
5. Verify visual snapshots:
   - `scripts/tui_spatial_snapshots.sh.ts`
   - `scripts/tui_spatial_snapshots.sh.ts --update` when accepting intended changes
6. Run one final `@designer` parity audit and apply last tactical fixes.

## Invariants

- Keep signatures stable:
  - `UnifiedCatalogView::render/click_select/hit_test`
  - `GraphCatalogView::render/click_select/hit_test`
- Keep App integration stable:
  - `VizSelection` mapping
  - `viz_offset` pan semantics
  - mode fallback behavior for small areas

## Output checklist

- Changed files list
- Snapshot names updated
- Compile/test commands run and results
- Remaining deltas (if any) as concrete next edits
