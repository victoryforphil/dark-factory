# Phase 1: Extract Shared Utilities to dark_tui_components

**Risk**: Low | **Impact**: High | **Dependencies**: None

## Goal

Extract duplicated utility code from both frontends into `lib/dark_tui_components/` to eliminate ~40% of cross-crate duplication. This phase focuses on pure functions and simple types that have zero coupling to app state.

---

## Step 1: Create `utils/compact.rs` module

### What to create

New file: `lib/dark_tui_components/src/utils/compact.rs`

This module consolidates 9+ duplicated compact_* functions from both frontends into a single shared source.

### Functions to include

```rust
/// Truncate to max_len with "..." suffix (head-preserve).
/// Used in: chat status_panel:185, sessions_panel:333, footer:53, chat_panel:784,
///          conversation_panel:204, tui chat_panel:525
pub fn compact_text(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        return value.to_string();
    }
    let head = value.chars().take(max_len.saturating_sub(3)).collect::<String>();
    format!("{head}...")
}

/// Truncate with "..." prefix (tail-preserve). Trims whitespace and newlines.
/// Used in: tui unified_catalog_view:36, details_panel:17, catalog_tree_view:15
pub fn compact_text_normalized(value: &str, max_len: usize) -> String {
    let normalized = value.trim().replace('\n', " ");
    if normalized.len() <= max_len {
        return normalized;
    }
    format!("{}...", &normalized[..max_len.saturating_sub(3)])
}

/// Truncate with "..." prefix (tail-preserve, reverse).
/// Used in: chat status_panel:195
pub fn compact_tail(value: &str, max_len: usize) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= max_len {
        return trimmed.to_string();
    }
    if max_len <= 3 {
        return ".".repeat(max_len);
    }
    let keep = max_len - 3;
    let tail = trimmed.chars().rev().take(keep).collect::<String>()
        .chars().rev().collect::<String>();
    format!("...{tail}")
}

/// Compact an optional label with "..." suffix. Returns "-" for None.
/// Used in: chat header_panel:43
pub fn compact_label(value: Option<&str>, max_len: usize) -> String {
    let Some(value) = value else { return "-".to_string() };
    if value.len() <= max_len { return value.to_string() }
    if max_len <= 3 { return ".".repeat(max_len) }
    format!("{}...", &value[..max_len - 3])
}

/// Compact ID to fixed length (default 12).
/// Used in: tui models:66
pub fn compact_id(value: &str) -> String {
    compact_id_len(value, 12)
}

pub fn compact_id_len(value: &str, max_len: usize) -> String {
    let trimmed = value.trim();
    if trimmed.len() <= max_len { return trimmed.to_string() }
    format!("{}...", &trimmed[..max_len])
}

/// Compact locator (tail-preserve, suffix-keep).
/// Used in: tui models:75
pub fn compact_locator(value: &str, max_len: usize) -> String {
    let trimmed = value.trim();
    if trimmed.len() <= max_len { return trimmed.to_string() }
    if max_len <= 3 { return ".".repeat(max_len) }
    let suffix_len = max_len.saturating_sub(3);
    format!("...{}", &trimmed[trimmed.len() - suffix_len..])
}

/// Compact ISO timestamp to "YYYY-MM-DD HH:MM:SS".
/// Used in: chat opencode_server:1038, tui models:89
pub fn compact_timestamp(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() { return "-".to_string() }
    if let Some((date, rest)) = trimmed.split_once('T') {
        let time = rest.trim_end_matches('Z').split('.').next().unwrap_or(rest);
        return format!("{date} {time}");
    }
    trimmed.to_string()
}

/// Compact session ID to fixed prefix.
/// Used in: tui chat_panel:537
pub fn compact_session_id(value: &str) -> &str {
    if value.len() <= 14 { value } else { &value[..14] }
}
```

### Module wiring

Create `lib/dark_tui_components/src/utils/mod.rs`:
```rust
pub mod compact;
```

Update `lib/dark_tui_components/src/lib.rs` to add:
```rust
pub mod utils;
pub use utils::compact::*;
```

### Files to update (replace local definitions with imports)

#### In `frontends/dark_chat/src/`:
| File | Line | Remove | Replace with |
|------|------|--------|-------------|
| `tui/panels/status_panel.rs` | 185-215 | `fn compact_text` + `fn compact_tail` | `use dark_tui_components::{compact_text, compact_tail};` |
| `tui/panels/sessions_panel.rs` | 333-341 | `fn compact_text` | `use dark_tui_components::compact_text;` |
| `tui/panels/header_panel.rs` | 43-57 | `fn compact_label` | `use dark_tui_components::compact_label;` |
| `tui/panels/footer_panel.rs` | 53-63 | `fn compact_text` | `use dark_tui_components::compact_text;` |
| `tui/panels/chat_panel.rs` | 784-794 | `fn compact_text` | `use dark_tui_components::compact_text;` |
| `providers/opencode_server.rs` | 1038-1050 | `fn compact_timestamp` | `use dark_tui_components::compact_timestamp;` |
| `framework/conversation_panel.rs` | 204-214 | `fn compact_text` | `use dark_tui_components::compact_text;` |

#### In `frontends/dark_tui/src/`:
| File | Line | Remove | Replace with |
|------|------|--------|-------------|
| `ui/render/views/unified_catalog_view.rs` | 36-43 | `fn compact_text` | `use dark_tui_components::compact_text_normalized;` (note: this variant normalizes \n) |
| `ui/render/panels/details_panel.rs` | 17-24 | `fn compact_text` | `use dark_tui_components::compact_text_normalized;` |
| `ui/render/views/catalog_tree_view.rs` | 15-22 | `fn compact_text` | `use dark_tui_components::compact_text_normalized;` |
| `ui/render/panels/chat_panel.rs` | 525-543 | `fn compact_text` + `fn compact_session_id` | `use dark_tui_components::{compact_text, compact_session_id};` |
| `models.rs` | 66-101 | `compact_id`, `compact_locator`, `compact_timestamp` | `pub use dark_tui_components::{compact_id, compact_locator, compact_timestamp};` (re-export to preserve public API) |

**Important**: The `tui` variants in `unified_catalog_view`, `details_panel`, `catalog_tree_view` use `compact_text_normalized` (trims whitespace + replaces `\n` with space). Call sites that previously called `compact_text(...)` in those files should now call `compact_text_normalized(...)`.

---

## Step 2: Create `utils/rect.rs` module (hit-test helpers)

### What to create

New file: `lib/dark_tui_components/src/utils/rect.rs`

```rust
use ratatui::layout::Rect;

/// Check if (col, row) is inside a Rect (inclusive).
/// Duplicated as rect_contains (chat chat_panel:996) and contains (tui chat_panel:501).
pub fn rect_contains(area: Rect, col: u16, row: u16) -> bool {
    col >= area.x
        && col < area.x + area.width
        && row >= area.y
        && row < area.y + area.height
}

/// Shrink a Rect by 1 on each side (inner area for bordered blocks).
/// Duplicated in tui chat_panel:508.
pub fn inner_rect(area: Rect) -> Rect {
    Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    }
}

/// Append a blinking cursor character to text.
/// Duplicated in chat chat_panel:769 and tui chat_panel:517.
pub fn with_cursor_tail(text: &str) -> String {
    let trimmed = text.trim_end();
    if trimmed.is_empty() {
        "\u{2588}".to_string()
    } else {
        format!("{trimmed}\u{2588}")
    }
}
```

### Module wiring

Update `lib/dark_tui_components/src/utils/mod.rs`:
```rust
pub mod compact;
pub mod rect;
```

Update `lib/dark_tui_components/src/lib.rs` to add:
```rust
pub use utils::rect::*;
```

### Files to update

| File | Line | Remove | Replace with |
|------|------|--------|-------------|
| `dark_chat/.../chat_panel.rs` | 769-775 | `fn with_cursor_tail` | `use dark_tui_components::with_cursor_tail;` |
| `dark_chat/.../chat_panel.rs` | 996-1001 | `fn rect_contains` | `use dark_tui_components::rect_contains;` |
| `dark_tui/.../chat_panel.rs` | 501-506 | `fn contains` | `use dark_tui_components::rect_contains;` (rename call sites) |
| `dark_tui/.../chat_panel.rs` | 508-514 | `fn inner_rect` | `use dark_tui_components::inner_rect;` |
| `dark_tui/.../chat_panel.rs` | 517-523 | `fn with_cursor_tail` | `use dark_tui_components::with_cursor_tail;` |

---

## Step 3: Create `utils/index.rs` module (selection helpers)

### What to create

New file: `lib/dark_tui_components/src/utils/index.rs`

```rust
/// Wrap-around previous index.
/// Duplicated in both app.rs files.
pub fn previous_index(current: usize, len: usize) -> usize {
    if len == 0 { return 0; }
    if current == 0 { len - 1 } else { current - 1 }
}

/// Wrap-around next index.
/// Duplicated in both app.rs files.
pub fn next_index(current: usize, len: usize) -> usize {
    if len == 0 { return 0; }
    if current >= len - 1 { 0 } else { current + 1 }
}
```

### Module wiring

Update `lib/dark_tui_components/src/utils/mod.rs`:
```rust
pub mod compact;
pub mod rect;
pub mod index;
```

Update `lib/dark_tui_components/src/lib.rs`:
```rust
pub use utils::index::*;
```

### Files to update

Search both `app.rs` files for `fn previous_index` and `fn next_index` definitions and replace with imports.

---

## Step 4: Add tests

New file: `lib/dark_tui_components/tests/utils_test.rs`

Test all compact_* functions, rect_contains, inner_rect, with_cursor_tail, previous_index, next_index with edge cases (empty strings, zero-length, boundary conditions).

---

## Verification

```bash
cargo check -p dark_tui_components
cargo check -p dark_chat
cargo check -p dark_tui
cargo test -p dark_tui_components
```

All three crates must compile. No behavioral changes expected.

## Estimated Impact

- ~16 local function definitions removed across both frontends
- ~200 lines of duplicated code eliminated
- Single source of truth for text truncation, rect math, index wrapping
