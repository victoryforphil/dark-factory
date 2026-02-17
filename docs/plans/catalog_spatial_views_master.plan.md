# Catalog Spatial Views Master Plan

Status: Completed (implemented in `dark_tui` spatial catalog views)

File type note: this is intentionally a single `*.plan.md` source of truth that inlines context, decisions, plan, and kickoff prompt.

---

## 1) Executive Summary

This plan captures the full direction from the iterative design thread for `dark_tui` catalog spatial rendering.

Approved product direction:

- **Station layout** will replace `graphical-tree` mode internals (`ResultsViewMode::Viz`).
- **Orbit layout** will replace `graphical-node` mode internals (`ResultsViewMode::Graph`).
- **Sub-agents must always render as boxy grids under actor cards** (both Station and Orbit).
- Visual language remains strict TUI style:
  - monochrome/neutral grays
  - monospace
  - boxy rectangles + straight/right-angle connectors
  - no glow/gradient/shadow/decorative effects

---

## 2) Decisions From This Thread (Condensed)

### 2.1 View semantics

- Keep existing mode naming/toggles in app behavior unless explicitly changed later.
- Mode internals are being swapped, not mode identities.

### 2.2 Style values (hard constraints)

- Minimal, terminal-native, functional.
- Readability over aesthetics.
- Alignment and spatial rhythm are prioritized over ornament.
- Sub-agent presentation is structural, not decorative.

### 2.3 Layout preferences (locked)

- **Station**:
  - product block should hug left
  - topology spreads right
  - clear rail -> variant -> actor chain
  - sub-agent grids directly under actor cards
- **Orbit**:
  - keep strong centered product geometry
  - mirrored actor placement left/right
  - sub-agent grids directly under actor cards

---

## 3) Source-of-Truth Artifacts

### 3.1 Final design candidate set (Round 10)

- `docs/reports/graph-node-round10-simple.html`
- `docs/reports/graph-node-round10-station-final.svg`
- `docs/reports/graph-node-round10-station-final-12v8a.svg`
- `docs/reports/graph-node-round10-orbit-final.svg`
- `docs/reports/graph-node-round10-orbit-final-12v8a.svg`

### 3.2 Designer guardrails

- `docs/reports/graph-node-designer-guidelines.md`

### 3.3 Iteration history (reference only)

- `docs/reports/graph-node-round6-simple.html`
- `docs/reports/graph-node-round7-simple.html`
- `docs/reports/graph-node-round8-simple.html`
- `docs/reports/graph-node-round9-simple.html`

---

## 4) Code Mapping (Current -> Target)

### 4.1 Core code paths

- View mode enum and labels:
  - `frontends/dark_tui/src/app/state.rs`
- Render routing by mode:
  - `frontends/dark_tui/src/ui/render/mod.rs`
- Toggle/status message path:
  - `frontends/dark_tui/src/ui/mod.rs`

### 4.2 Mode internals to replace

- Viz mode implementation currently in:
  - `frontends/dark_tui/src/ui/render/views/unified_catalog_view.rs`
  - Target: replace internals with Station renderer behavior
- Graph mode implementation currently in:
  - `frontends/dark_tui/src/ui/render/views/graph_catalog_view.rs`
  - Target: replace internals with Orbit renderer behavior

### 4.3 Shared target primitive to enforce

- Introduce/normalize a shared sub-agent grid renderer (boxy under actor cards), used by both views.

---

## 5) Agent Operating Model (Required)

This section is mandatory for execution:

- Use **small, repeated designer calls** for narrow component feedback (not giant one-shot design handoffs).
- Most implementation actions are done by parent agent or `developer_jr` / `developer_senior`.
- Code discovery/search/context should be done by one or more `explore` agents.
- Designers can work independently only on properly separated/composable components.
- Final integration is done by parent agent with optional (recommended) designer feedback cycles.

Recommended rhythm:

1. `explore` (parallel) for code mapping and target signatures
2. parent/developer implements one focused slice
3. `cargo check -p dark_tui`
4. short `designer` sanity pass on that slice
5. repeat

---

## 6) Implementation Plan

Execution status (2026-02-17):

- Phase A - Foundation: completed
- Phase B - Station integration (Viz): completed
- Phase C - Orbit integration (Graph): completed
- Phase D - Scale hardening: completed
- Phase E - Final cleanup: completed

### Phase A - Foundation

- Confirm shared geometry primitives and world/screen helpers are reusable.
- Add or normalize shared `sub_agent_grid` renderer primitive.

Deliverable:

- reusable sub-agent grid used by both Station and Orbit views.

### Phase B - Station integration (Viz)

- Replace internals of `UnifiedCatalogView` with Station topology:
  - left-hugging product
  - right-spreading rail/variants/actors
  - grid sub-agents below actors
- Preserve hit-test mapping to `VizSelection`.
- Preserve pan/click behavior and small-screen fallback behavior.

Deliverable:

- Viz renders Station as default spatial tree replacement.

### Phase C - Orbit integration (Graph)

- Replace internals of `GraphCatalogView` with Orbit topology:
  - centered product
  - surrounding variants
  - mirrored actors
  - sub-agent grids below actors
- Preserve existing selection and panning semantics.

Deliverable:

- Graph renders Orbit as node-map replacement.

### Phase D - Scale hardening

- Validate base and stress layouts (12V/8A style scenarios).
- Tune spacing and wrapping to avoid overlap.
- Verify legends remain readable without visual noise.

Deliverable:

- stable output for both compact and dense datasets.

### Phase E - Final cleanup

- Remove dead code in replaced render branches where safe.
- Keep cleanup low-risk and localized.
- Final designer feedback micro-pass.

Deliverable:

- clean compile + maintainable view modules.

---

## 7) Verification and Exit Criteria

Minimum verification after each phase:

```bash
cargo check -p dark_tui
```

Recommended full verification before merge:

```bash
cargo check -p dark_tui_components && cargo check -p dark_chat && cargo check -p dark_tui
```

Definition of done:

- Viz mode behaves as Station (with left-hugging product/rightward spread)
- Graph mode behaves as Orbit
- Sub-agent grids consistently render under actors in both modes
- No regressions in click selection, hit-test, or panning
- compile checks pass

---

## 8) Practical Snippets

### 8.1 Explore-first prompt snippet

```text
Thoroughness: quick

Find all call paths and structs used by [target view] in dark_tui.
Return only:
- file paths
- key symbols/functions
- where hit-test and click-select are wired
```

### 8.2 Designer micro-feedback prompt snippet

```text
Review only [single component/single concern].

Constraints:
- monochrome neutral only
- monospace
- box/line only
- no effects

Return:
- 3 concrete tweaks
- 1 recommended default
```

### 8.3 Developer implementation slice prompt snippet

```text
Implement only [single slice] in [file].
Do not refactor unrelated code.
Keep behavior parity for hit-test/pan.
Run: cargo check -p dark_tui
Return changed files + verification result.
```

---

## 9) External Docs / Resources (from docs exploration)

### Ratatui web snapshots

- `docs/external/ratatui_web/index.ext.md`
- `docs/external/ratatui_web/docs__concepts__layout.ext.md`
- `docs/external/ratatui_web/docs__concepts__rendering.ext.md`
- `docs/external/ratatui_web/docs__recipes__layout.ext.md`

Use for:

- layout partitioning patterns
- immediate rendering model reminders
- practical spacing and chunking recipes

### Ratatui docs.rs snapshots

- `docs/external/ratatui_docs/index.ext.md`
- `docs/external/ratatui_docs/docs__all-html.ext.md`

Use for:

- API-level details for `Rect`, layout helpers, frame/buffer behavior

### Related plan context

- `docs/plans/tui_unification_index.md`
- `docs/plans/tui_unification_phase6.md`

Use for:

- broader refactor context and prior catalog-view splitting rationale

---

## 10) Inlined Kickoff Prompt (Copy/Paste)

```text
You are implementing the next iteration of dark_tui catalog spatial rendering.

Goal:
- Replace graphical-tree (Viz) internals with Station layout.
- Replace graphical-node (Graph) internals with Orbit layout.
- In both modes, sub-agents must render as boxy grids under actor cards.

Reference docs:
- docs/plans/catalog_spatial_views_master.plan.md
- docs/reports/graph-node-round10-simple.html

Mandatory orchestration rules:
1) Use small/repeated designer calls for component-level visual feedback and ideas.
2) Keep most implementation in parent agent or developer_jr / developer_senior.
3) Use one or more explore agents for all code discovery/search/context gathering.
4) Designer may work independently only for clearly separated, composable components.
5) Parent agent performs final integration (recommended: quick designer feedback cycle before finalizing).

Style guardrails (strict):
- Monochrome/neutral grays only.
- Monospace text.
- Boxy rectangles + straight/right-angle connectors.
- No glow, gradient, shadow, blur, or decorative effects.

Execution sequence:
1) Use parallel explore calls to map current render/hit-test paths.
2) Implement shared sub-agent grid primitive usable by Station + Orbit.
3) Implement Station in Viz (UnifiedCatalogView) with product left-anchored flow.
4) Implement Orbit in Graph (GraphCatalogView) with centered product and mirrored actors.
5) Reconcile hit-tests and panning behavior.
6) Run cargo check -p dark_tui after each major step.
7) Do one short designer review pass for final spacing/readability.

Required output:
- list of files changed
- what was delegated to explore/designer/developer
- verification result from cargo check -p dark_tui
- deferred risks/follow-ups
```

---

## 11) Linked File Index (all relevant files for this effort)

### Plan files

- `docs/plans/catalog_spatial_views_mapping.md`
- `docs/plans/catalog_spatial_views_plan.md`
- `docs/plans/catalog_spatial_views_prompt.md`
- `docs/plans/catalog_spatial_views_master.plan.md`

### Round 10 design files

- `docs/reports/graph-node-round10-simple.html`
- `docs/reports/graph-node-round10-station-final.svg`
- `docs/reports/graph-node-round10-station-final-12v8a.svg`
- `docs/reports/graph-node-round10-orbit-final.svg`
- `docs/reports/graph-node-round10-orbit-final-12v8a.svg`

### Guardrails and historical references

- `docs/reports/graph-node-designer-guidelines.md`
- `docs/reports/graph-node-round9-simple.html`
- `docs/reports/graph-node-round8-simple.html`
- `docs/reports/graph-node-round7-simple.html`
- `docs/reports/graph-node-round6-simple.html`

### Core implementation targets

- `frontends/dark_tui/src/app/state.rs`
- `frontends/dark_tui/src/ui/mod.rs`
- `frontends/dark_tui/src/ui/render/mod.rs`
- `frontends/dark_tui/src/ui/render/views/unified_catalog_view.rs`
- `frontends/dark_tui/src/ui/render/views/graph_catalog_view.rs`
- `frontends/dark_tui/src/ui/render/views/mod.rs`
