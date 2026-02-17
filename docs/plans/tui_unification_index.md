# TUI Unification — Plan Index

## Overview

This plan unifies three Ratatui-based TUI crates in the dark-factory workspace:

- **`lib/dark_tui_components/`** — shared widget library (~1,700 LOC)
- **`frontends/dark_chat/`** — OpenCode chat TUI, maturing into a reusable framework lib + binary (~6,400 LOC)
- **`frontends/dark_tui/`** — dashboard TUI for products/variants/actors (~7,000 LOC)

The work eliminates ~40% code duplication between the two frontends, introduces a formal Component trait, splits monolithic files into focused modules, and matures `dark_chat` as a framework library that `dark_tui` imports.

## Architecture (Target State)

```
┌───────────────────────────────────────────────────┐
│                   dark_tui (binary)               │
│   Dashboard app: catalog viz, spawn, chat panels  │
│   Depends on: dark_chat::framework, dark_rust,    │
│               dark_tui_components                  │
└───────────┬──────────────────┬────────────────────┘
            │                  │
            ▼                  ▼
┌──────────────────┐  ┌────────────────────────────┐
│   dark_chat      │  │   dark_rust                │
│   (lib + binary) │  │   (API client lib)         │
│                  │  │   REST + WS client, types  │
│   framework/     │  └────────────────────────────┘
│     conversation │
│     autocomplete │
│     selectors    │
│     session_tree │
│     message_types│
│   providers/     │
│   tui/ (binary)  │
└────────┬─────────┘
         │
         ▼
┌────────────────────────────────────────────┐
│          dark_tui_components (lib)          │
│                                            │
│  components/   utils/    trait Component    │
│    PaneBlock     compact    Action enum     │
│    StatusPill    rect       Event enum      │
│    KeyHintBar    index      ComponentResult │
│    FooterBar                               │
│    PopupOverlay  theme.rs                  │
│    ChatMessage   ComponentThemeLike        │
│    ChatComposer                            │
│    CardGrid                                │
│    LoadingSpinner                          │
│    LabeledField                            │
└────────────────────────────────────────────┘
```

## Phases

| Phase | Title | Risk | LOC Impact | Depends On |
|-------|-------|------|------------|------------|
| **1** | Extract Shared Utilities | Low | -200 dup | None |
| **2** | Component Trait | Low | +250 new | None |
| **3** | Split app.rs | Medium | ~0 net (restructure) | None |
| **4** | Unify Shared Panels | Medium | -400 dup | Phase 1, 2 |
| **5** | Mature dark_chat Framework | Medium | -1000 dup | Phase 1, 2, 4 |
| **6** | Final Cleanup | Low | +500 tests/docs | Phase 1–5 |

## Dependency Graph

```
Phase 1 (utils) ──────────┐
                           ├──► Phase 4 (panels) ──► Phase 5 (framework) ──► Phase 6 (cleanup)
Phase 2 (trait) ──────────┘                                                      ▲
                                                                                 │
Phase 3 (split app.rs) ─────────────────────────────────────────────────────────┘
```

Phases 1, 2, and 3 are independent and can run in parallel.
Phase 4 requires 1+2. Phase 5 requires 1+2+4. Phase 6 requires all others.

## Execution Strategy

Each phase is a self-contained plan document with:
- Context explaining what and why
- Numbered steps with exact file paths and code snippets
- Verification commands (`cargo check`, `cargo test`)
- Risk assessment

Phases are designed to be executed by smaller agents following the plan step-by-step. Each step should result in a compilable workspace (`cargo check` passes for all 3 crates).

### Recommended execution order

1. **Phase 1** → commit
2. **Phase 2** → commit
3. **Phase 3** (can overlap with 1/2 if no file conflicts) → commit
4. **Phase 4** → commit
5. **Phase 5** → commit
6. **Phase 6** → commit (or split into sub-commits per step)

## Plan Files

| File | Description |
|------|-------------|
| [`tui_unification_phase1.md`](tui_unification_phase1.md) | Extract `compact_*`, rect, and index helpers to `dark_tui_components/src/utils/` |
| [`tui_unification_phase2.md`](tui_unification_phase2.md) | Define `Component` trait, `Action` enum, `Event` enum in components lib |
| [`tui_unification_phase3.md`](tui_unification_phase3.md) | Split `app.rs` in both frontends into 5–7 sub-modules each |
| [`tui_unification_phase4.md`](tui_unification_phase4.md) | Unify `PopupOverlay`, `ListViewport`, `FooterBar` as shared components |
| [`tui_unification_phase5.md`](tui_unification_phase5.md) | Mature `dark_chat::framework` with autocomplete, selectors, session tree, message types |
| [`tui_unification_phase6.md`](tui_unification_phase6.md) | Split remaining large files, add doc comments, expand tests, update READMEs |

## Key Decisions (Captured)

1. **Component trait pattern**: Full Ratatui template (init/handle_events/update/render with Action enum and channel registration).
2. **Chat framework location**: `dark_chat` owns all chat-specific logic. `dark_tui_components` has generic primitives only.
3. **dark_tui_components scope**: Stateless widgets, Component trait, theme contract, pure utility functions. No app state, no network, no provider logic.
4. **Variant locator uniqueness**: Not enforced — multiple variants can share a locator (differentiated by `productId + name`).

## Metrics (Expected)

| Metric | Before | After |
|--------|--------|-------|
| Cross-crate duplication | ~40% | <10% |
| Largest file (LOC) | 1,711 (dark_tui app.rs) | ~400 |
| Test count (dark_tui_components) | 8 | ~25 |
| Test count (dark_chat) | 4 | ~15 |
| Test count (dark_tui) | 0 | ~10 |
| Doc comments (dark_tui_components) | 0 | ~50+ |
| Shared components | 6 | 10+ |
| Shared utility functions | 0 | 10+ |
