# Catalog Spatial Views - Agent Kickoff Prompt

Paste this into a fresh implementation session.

---

## Prompt

You are implementing the next iteration of `dark_tui` catalog spatial rendering.

### Goal

- Replace `graphical-tree` (Viz) internals with **Station** layout.
- Replace `graphical-node` (Graph) internals with **Orbit** layout.
- In both modes, sub-agents must render as **boxy grids under actor cards**.

Reference docs:

- `docs/plans/catalog_spatial_views_mapping.md`
- `docs/plans/catalog_spatial_views_plan.md`
- `docs/reports/graph-node-round10-simple.html`

### Mandatory orchestration rules

1. Use **small/repeated `designer` calls** for component-level visual feedback and ideas.
2. Keep **most implementation** in parent agent or `developer_jr` / `developer_senior`.
3. Use one or more **`explore` agents for all code discovery/search/context gathering**.
4. `designer` may work independently only for clearly separated, composable components.
5. Parent agent performs **final integration** (recommended: quick designer feedback cycle before finalizing).

### Style guardrails (strict)

- Monochrome/neutral grays only.
- Monospace text.
- Boxy rectangles + straight/right-angle connectors.
- No glow, gradient, shadow, blur, or decorative effects.

### Execution sequence

1. Use parallel `explore` calls to map current render/hit-test paths.
2. Implement shared sub-agent grid primitive usable by Station + Orbit.
3. Implement Station in Viz (`UnifiedCatalogView`) with product left-anchored flow.
4. Implement Orbit in Graph (`GraphCatalogView`) with centered product and mirrored actors.
5. Reconcile hit-tests and panning behavior.
6. Run `cargo check -p dark_tui` after each major step.
7. Do one short designer review pass for final spacing/readability.

### Required output

- List of files changed.
- What was delegated to explore/designer/developer.
- Verification results from `cargo check -p dark_tui`.
- Any deferred risks/follow-ups.
