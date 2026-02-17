# Phase 6: Final Cleanup — Large File Reduction, Documentation, Tests

**Risk**: Low–Medium | **Impact**: Medium | **Dependencies**: Phases 1–5

## Goal

After the structural unification (Phases 1–5), this phase handles remaining cleanup:
1. Split the last oversized files that weren't addressed in earlier phases.
2. Add doc comments to `dark_tui_components` public API (currently zero `///` comments).
3. Expand test coverage (currently zero tests in `dark_tui`, only 4 in `dark_chat`).
4. Update READMEs to reflect the new architecture.

---

## Step 1: Split `opencode_server.rs` (1,385 → ~4 files)

### Context

`frontends/dark_chat/src/providers/opencode_server.rs` is the largest file in the workspace. After Phase 5 extracts message rendering to the framework, this file still contains:
- HTTP/WS transport layer (~170 LOC)
- `ChatProvider` trait implementation (~370 LOC)
- Wire types / deserialization structs (~100 LOC)
- SSE realtime streaming logic (~90 LOC)
- URL/query/response parsing helpers (~200 LOC)
- OpenCode-specific extraction utilities (sessions, models, agents, config) (~350 LOC)

### Target structure

```
frontends/dark_chat/src/providers/
├── mod.rs                    (re-exports, unchanged)
├── provider.rs               (ChatProvider trait, unchanged)
├── opencode_server.rs        (slim: struct + new + ChatProvider impl, ~400 LOC)
├── opencode_transport.rs     (raw_request, request_json_with_fallback, send_prompt_with_options, ~120 LOC)
├── opencode_realtime.rs      (stream_realtime_events, dispatch_sse_event, ~100 LOC)
├── opencode_wire.rs          (all Wire structs + serde, ~100 LOC)
└── opencode_extract.rs       (all extract_*, format_*, parse_* pure fns, ~400 LOC)
```

### Migration steps

1. Create `opencode_wire.rs` with all `*Wire` structs:
   - `SessionWire` (line 541), `SessionTimeWire` (line 625), `MessageWire` (line 632), `MessageInfoWire` (line 641), `MessageTimeWire` (line 653)
   - Mark all `pub(crate)` — they are only used within this `providers/` module.

2. Create `opencode_extract.rs` with all standalone pure functions:
   - `extract_session_id` (line 781)
   - `extract_session_statuses` (line 886)
   - `extract_status_type` (line 899)
   - `extract_string_options` (line 924)
   - `extract_status_list` (line 969)
   - `extract_mcp_status` (line 1004)
   - `extract_config_path` (line 1078)
   - `extract_message_text` (line 1125) — **may move to framework in Phase 5; if so, skip here**
   - `format_message_part` (line 1142) — same caveat
   - `format_tool_call_part` (line 1197)
   - `format_tool_result_part` (line 1220)
   - `extract_tool_name` (line 1238)
   - `extract_text_from_keys` (line 1277)
   - `dedupe_segments` (line 1287)
   - `first_non_empty_value` (line 1301)
   - `parse_model_selector` (line 956)
   - `compact_timestamp` (line 1038) — **should be removed if Phase 1 already provides this from components**
   - `format_unix_timestamp` (line 1052)
   - `normalize_unix_timestamp` (line 1066)
   - `value_to_string` (line 797)
   - Helper fns: `normalize_path` (811), `append_query` (819), `url_encode` (840), `parse_response_body` (855), `ensure_success` (863), `unwrap_data` (876)
   - Move the `#[cfg(test)] mod tests` block here alongside the functions it tests.

3. Create `opencode_realtime.rs`:
   - `stream_realtime_events` (line 658)
   - `dispatch_sse_event` (line 749)
   - These depend on `ChatRealtimeEvent` and sender channels.

4. Create `opencode_transport.rs`:
   - `raw_request` (line 45)
   - `request_json_with_fallback` (line 83)
   - `send_prompt_with_options` (line 118)
   - These are `impl OpenCodeProvider` methods — extract as `impl OpenCodeProvider` in the new file via `pub(crate)` fields or a thin internal API.

5. Slim `opencode_server.rs` to:
   - `OpenCodeProvider` struct + `new()`
   - `impl ChatProvider for OpenCodeProvider` block
   - All method bodies call into transport/extract/realtime helpers.
   - `mod opencode_transport; mod opencode_realtime; mod opencode_wire; mod opencode_extract;` declarations.

### Verification

```bash
cargo check -p dark_chat
cargo test -p dark_chat
```

---

## Step 2: Split `service.rs` in dark_tui (895 → ~3 files)

### Context

`frontends/dark_tui/src/service.rs` mixes:
- `DashboardService` struct + constructor + public API methods (~390 LOC)
- Private fetch helpers (fetch_all_products/variants/actors) (~110 LOC)
- Wire types (`ProductRecord`, `VariantRecord`, `ActorRecord`, etc.) (~170 LOC)
- Row conversion functions (`to_product_row`, `to_variant_row`, `to_actor_row`) (~130 LOC)
- Generic helpers (`ensure_success`, `directory_name`, `locator_tail`, etc.) (~80 LOC)

### Target structure

```
frontends/dark_tui/src/
├── service.rs             (slim: struct + public methods, ~400 LOC)
├── service_wire.rs        (all Record structs + serde, ~170 LOC)
└── service_convert.rs     (to_*_row fns, collect_product_metrics, helpers, ~250 LOC)
```

### Migration steps

1. Create `service_wire.rs`:
   - Move `ProductRecord` (line 819), `ProductGitInfoRecord` (line 832), `VariantRecord` (line 841), `VariantGitInfoRecord` (line 857), `VariantGitStatusRecord` (line 868), `ActorRecord` (line 879), `ApiListEnvelope` (line 813), `ProductMetrics` (line 530).
   - All `pub(crate)`.

2. Create `service_convert.rs`:
   - Move `to_product_row` (line 566), `to_variant_row` (line 632), `to_actor_row` (line 679).
   - Move `collect_product_metrics` (line 538).
   - Move `ActorOpenCodeContext` (line 707), `actor_opencode_context` (line 713), `required_actor_opencode_context` (line 748).
   - Move standalone helpers: `directory_name` (line 774), `locator_tail` (line 782), `summarize_error` (line 792), `now_label` (line 803), `query_slice_or_none` (line 757), `ensure_success` (line 761).

3. Slim `service.rs`:
   - Keep `DashboardService`, `SpawnOptions`, `impl DashboardService`.
   - Add `mod service_wire; mod service_convert; use service_wire::*; use service_convert::*;`.

### Verification

```bash
cargo check -p dark_tui
```

---

## Step 3: Split `unified_catalog_view.rs` (886 → ~2 files)

### Context

This file has a single `impl UnifiedCatalogView` block with deeply nested rendering for products, variants, and actors in a tree layout. The main `render()` method is ~260 lines. Inner methods are self-contained per entity type.

### Target structure

```
frontends/dark_tui/src/ui/render/views/
├── unified_catalog_view.rs   (slim: render + layout, ~350 LOC)
└── catalog_cards.rs           (card rendering: render_variant_card, render_actor_card, draw_trunk, draw_junction, ~450 LOC)
```

### Migration steps

1. Create `catalog_cards.rs`:
   - Move `render_variant_card` (line 712), `render_actor_card` (line 789), `draw_trunk` (line 668), `draw_junction` (line 687).
   - Move `ClickHit` enum (line 873), `ProductGroup` struct (line 865).
   - These are self-contained rendering helpers that only need `Frame`, `Rect`, `Theme`, and row types.

2. Slim `unified_catalog_view.rs`:
   - Keep `UnifiedCatalogView`, `render()`, `panel_inner()`, `product_groups()`, `group_height()`, `render_product_group()`.
   - Import card helpers from `catalog_cards`.

### Verification

```bash
cargo check -p dark_tui
```

---

## Step 4: Add doc comments to `dark_tui_components` public API

### Context

`lib/dark_tui_components/` has zero `///` or `//!` doc comments. As the shared library, it should have at minimum:
- Module-level `//!` comments in `lib.rs` and each submodule root.
- `///` comments on all public structs, traits, and functions.

### Targets

| File | Items to document |
|------|-------------------|
| `src/lib.rs` | Module-level `//!` crate overview |
| `src/theme.rs` | `ComponentThemeLike` trait + all methods |
| `src/components/mod.rs` | Module-level `//!` |
| `src/components/pane_block.rs` | `PaneBlockComponent` |
| `src/components/status_pill.rs` | `StatusPill` + `StatusKind` |
| `src/components/key_hint_bar.rs` | `KeyHintBar`, `KeyBind` |
| `src/components/labeled_field.rs` | `LabeledField` |
| `src/components/loading_spinner.rs` | `LoadingSpinner` |
| `src/components/chat_message_list.rs` | `ChatMessageListComponent`, `ChatMessageItem`, `RenderableChatMessage` |
| `src/components/chat_composer.rs` | `ChatComposerComponent`, `ChatComposerState` |
| `src/components/card_grid_component.rs` | `CardGridComponent`, `CardItem`, `CardGridState` |
| `src/utils/*.rs` | All public functions (added in Phase 1) |

### Documentation style

Follow Rust conventions:
```rust
/// Brief one-line summary.
///
/// Longer description if needed (optional for simple items).
///
/// # Examples (optional, for key public items)
///
/// ```
/// use dark_tui_components::StatusPill;
/// ```
```

### Verification

```bash
# Check doc warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p dark_tui_components --no-deps
```

---

## Step 5: Expand test coverage

### Current state

| Crate | Tests | Coverage areas |
|-------|-------|----------------|
| `dark_tui_components` | 8 tests in 5 files | chat_message_list (3), labeled_field (1), key_hint_bar (1), status_pill (1), loading_spinner (1) |
| `dark_chat` | 4 tests in 1 file | opencode_server.rs: SessionWire parsing (2), extract_message_text (2) |
| `dark_tui` | 0 tests | — |

### Test additions

**Priority 1: `dark_tui_components` — new Phase 1/2/4 additions**

These modules are being added in earlier phases and should get tests as part of this cleanup:

```
src/utils/compact.rs  — test each compact_* variant
  - compact_text: short string passthrough, long string truncation, unicode boundary safety
  - compact_text_normalized: whitespace collapsing, newline replacement
  - compact_timestamp: various datetime formats, fallback behavior

src/utils/rect.rs  — test rect helpers
  - centered_rect: various container sizes, minimum bounds
  - any other rect utilities added in Phase 1

src/utils/index.rs  — test index helpers
  - wrap-around navigation (next/prev with wrapping)

src/action.rs  — test Action serialization if applicable

src/components/popup_overlay.rs  — test layout computation (from Phase 4)
  - overlay sizing relative to container
  - hit-testing (inside/outside overlay)

src/components/footer_bar.rs  — test key hint rendering (from Phase 4)
```

**Priority 2: `dark_chat` — framework module tests**

After Phase 5 creates framework sub-modules, add tests for:

```
framework/message_types.rs
  - AgentMessage construction, text extraction

framework/session_tree.rs
  - Tree building from flat session list
  - Parent-child relationships
  - Sorting/ordering

framework/autocomplete.rs
  - Prefix matching
  - Slash command parsing
  - Completion cycling
```

**Priority 3: `dark_tui` — basic smoke tests**

```
service_convert.rs (after Step 2 split)
  - to_product_row: field mapping correctness
  - to_variant_row: locator parsing
  - to_actor_row: status mapping
  - collect_product_metrics: aggregation math

models.rs
  - compact_id: truncation behavior
  - compact_locator: protocol stripping
```

### Test pattern

Follow existing test style in the codebase:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptive_test_name() {
        // arrange
        let input = ...;
        // act
        let result = function_under_test(input);
        // assert
        assert_eq!(result, expected);
    }
}
```

### Verification

```bash
cargo test -p dark_tui_components
cargo test -p dark_chat
cargo test -p dark_tui
```

---

## Step 6: Update READMEs

### `lib/dark_tui_components/README.md`

Current: 37 lines. Lists basic components.

Update to reflect:
- New `utils/` module (compact, rect, index helpers) from Phase 1
- `Component` trait and `Action`/`Event` types from Phase 2
- New shared components (`PopupOverlay`, `FooterBar`, `ListViewport`) from Phase 4
- `ComponentThemeLike` trait (already exists but deserves a section)
- Updated component list with brief descriptions
- Testing instructions (`cargo test -p dark_tui_components`)

### `frontends/dark_chat/README.md`

Current: 60 lines. Lists features and keybindings.

Update to reflect:
- Framework library role (not just a binary)
- `dark_chat::framework` module and what it exports
- How `dark_tui` depends on dark_chat for chat panels
- Updated module structure after Phase 3/5 splits

### `frontends/dark_tui/README.md`

Current: 84 lines. Lists dashboard features.

Update to reflect:
- Dependencies on both `dark_tui_components` and `dark_chat::framework`
- Updated module structure after Phase 3 splits
- Any new keybindings or UI changes from unified panels

### Verification

- READMEs should only describe behavior that exists (per `frontends/dark_tui/AGENTS.md`).
- No aspirational content.

---

## Step 7: Update `AGENTS.md` and `STYLE.md` references

### `AGENTS.md` (root)

After all phases complete, update:
- Section 1 (Repository Context): mention `dark_chat` dual role (framework lib + binary), shared component count, new utils module.
- Section 8 (Dark Core API Coverage): no changes needed unless API surface changed.
- Keep all other sections accurate.

### `frontends/dark_tui/AGENTS.md`

Update:
- Package Context: mention dependency on `dark_chat::framework`.
- Modularity Direction: reflect actual module split (app/ sub-modules, service_wire, service_convert).

### `STYLE.md`

If the Component trait establishes new conventions (Action enum, Event handling), add a brief Rust/TUI section noting the pattern.

### Verification

- All references to file paths should match actual files post-refactor.
- No stale guidance.

---

## Execution checklist

After each step, run:
```bash
cargo check -p dark_tui_components && cargo check -p dark_chat && cargo check -p dark_tui
cargo test -p dark_tui_components && cargo test -p dark_chat && cargo test -p dark_tui
```

Steps can be executed in this order:
1. Step 1 (split opencode_server.rs) — independent
2. Step 2 (split service.rs) — independent
3. Step 3 (split unified_catalog_view.rs) — independent
4. Step 4 (doc comments) — after Phases 1,2,4 are done
5. Step 5 (tests) — after Phases 1–5 + Steps 1–3 done
6. Step 6 (READMEs) — after all code changes done
7. Step 7 (AGENTS.md/STYLE.md) — last
