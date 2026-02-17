# Catalog Spatial Views - Execution Plan

## Objective

Implement two production-ready spatial catalog views in `dark_tui`:

- **Station** layout for `graphical-tree` (`ResultsViewMode::Viz`)
- **Orbit** layout for `graphical-node` (`ResultsViewMode::Graph`)

with consistent **boxy sub-agent grids under actors**.

## Agent operating model (required)

- **Parent agent owns orchestration + final integration**.
- **Most implementation actions are done by parent or `developer_jr` / `developer_senior`**.
- **Codebase discovery/context gathering is done by one or more `explore` agents**.
- **`designer` calls must be small and repeated** for component sketches, visual feedback, and isolated design experiments.
- `designer` can work independently only on properly separated, composable components.
- Final merge/integration is done by parent agent, with optional (recommended) quick designer feedback cycles.

## Phase plan

### Phase 0 - Ground truth and scope lock

- Use `explore` to map current spatial render code paths and hit-test flows.
- Confirm no unintended mode label/command regressions.
- Freeze style guardrails (monochrome, monospace, box/line only, no fancy effects).

Deliverables:

- short findings note in commit/PR description (no new doc required)

### Phase 1 - Shared primitives and sub-agent grid

- Implement reusable sub-agent grid renderer component.
- Extract world rect/screen conversion and line/box helpers if duplicated.
- Keep API small and view-agnostic.

Verification:

- `cargo check -p dark_tui`

### Phase 2 - Station view replacement (Viz)

- Replace `UnifiedCatalogView` internals with Station topology:
  - product left-aligned, spread right
  - rail + variant columns + actor cards
  - sub-agent grids under actors
- Maintain `VizSelection` hit-testing parity.

Verification:

- `cargo check -p dark_tui`
- manual run screenshot pass for base + high-density data

### Phase 3 - Orbit view replacement (Graph)

- Replace `GraphCatalogView` internals with Orbit topology:
  - strong centered product
  - variant tiers around center
  - mirrored actors left/right
  - sub-agent grids under actors
- Keep pan behavior and click-select behavior stable.

Verification:

- `cargo check -p dark_tui`
- manual run screenshot pass for base + high-density data

### Phase 4 - Scale hardening (12V/8A baseline)

- Tune spacing heuristics and wrapping for dense datasets.
- Ensure no overlap on small/medium/large terminal widths.
- Validate legends remain readable.

Verification:

- `cargo check -p dark_tui`
- optional focused tests if view logic extracted cleanly

### Phase 5 - Final polish and lock

- Minimal cleanup of dead code from superseded render branches.
- Keep only low-risk cleanup within touched view paths.
- Final visual review pass with `designer` in short iterations.

Verification:

- `cargo check -p dark_tui`

## Suggested delegation loops

- **Explore loop** (parallel):
  - locate symbols
  - locate callsites
  - summarize file-level responsibilities
- **Designer loop** (serial short turns):
  - one component or one concern per prompt
  - return only actionable diffs or concise change notes
- **Developer loop**:
  - apply code edits
  - run checks
  - hand back compile status and touched files

## Success criteria

- Viz mode reads clearly as Station layout.
- Graph mode reads clearly as Orbit layout.
- Sub-agents consistently render as boxy grids under actors in both modes.
- No regressions to selection, panning, or mode toggles.
- `cargo check -p dark_tui` passes.
