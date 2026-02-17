# Phase 4: Unify Shared Panels

**Risk**: Low | **Impact**: Medium | **Dependencies**: Phase 1 (compact utils), Phase 2 (Component trait)

## Goal

Extract shared panel patterns from both frontends into `dark_tui_components` as reusable Component implementations. These panels have nearly identical structure but app-specific data.

---

## Step 1: Create `PopupOverlay` component in dark_tui_components

### Why

Both frontends implement the exact same popup rendering pattern:
- Calculate a bounded Rect (clamped to parent area)
- Clear the area
- Render a bordered Block with title
- Render a scrollable list of items with highlight
- Render a query/filter input
- Hit-test for item clicks

### Files affected

Currently duplicated in:
- `frontends/dark_chat/src/tui/panels/chat_panel.rs` lines 530-766 (model/agent/autocomplete popups, ~240 lines)
- `frontends/dark_tui/src/ui/render/panels/chat_panel.rs` lines 225-386 (picker/autocomplete popups, ~160 lines)

### What to create

New file: `lib/dark_tui_components/src/components/popup_overlay.rs`

```rust
use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, Clear, Paragraph, List, ListItem}};
use ratatui::style::{Style, Modifier};
use ratatui::text::{Line, Span};
use crate::theme::ComponentThemeLike;
use crate::utils::rect::{rect_contains, inner_rect};

/// Props for rendering a popup list overlay.
pub struct PopupOverlayProps<'a> {
    pub title: &'a str,
    pub items: &'a [PopupItem<'a>],
    pub selected: usize,
    pub query: Option<&'a str>,
    pub query_label: Option<&'a str>,
    pub hints: Option<Vec<Span<'a>>>,
    pub anchor: PopupAnchor,
    pub max_visible: usize,
    pub min_width: u16,
    pub max_width: u16,
}

pub struct PopupItem<'a> {
    pub label: &'a str,
    pub tag: Option<&'a str>,
    pub active: bool,
}

pub enum PopupAnchor {
    /// Anchor above a given rect (popup grows upward from bottom edge)
    AboveRect(Rect),
    /// Anchor at absolute position
    At { x: u16, y: u16 },
    /// Center in parent
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupHit {
    Outside,
    Popup,
    Query,
    ListItem(usize),
}

pub struct PopupOverlay;

impl PopupOverlay {
    /// Calculate the popup area given constraints and anchor.
    pub fn area(parent: Rect, props: &PopupOverlayProps) -> Option<Rect> {
        // Width: max of item widths, clamped to min_width..max_width
        // Height: min(items.len() + header/footer, max_visible + extras)
        // Position: based on anchor, clamped to parent bounds
        // Returns None if no items
        todo!()
    }

    /// Render the popup (Clear + Block + list + query/hints).
    pub fn render(
        frame: &mut Frame,
        parent: Rect,
        props: &PopupOverlayProps,
        theme: &impl ComponentThemeLike,
    ) {
        let Some(area) = Self::area(parent, props) else { return };

        frame.render_widget(Clear, area);
        let block = Block::default()
            .title(format!(" {} ", props.title))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.pane_border_focused()));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Split inner into: [optional query row] [list rows] [optional hints row]
        // Render list items with viewport scrolling and highlight
        // Render query input if present
        // Render hints if present
        todo!()
    }

    /// Hit-test for popup clicks.
    pub fn hit_test(parent: Rect, props: &PopupOverlayProps, col: u16, row: u16) -> PopupHit {
        let Some(area) = Self::area(parent, props) else {
            return PopupHit::Outside;
        };
        if !rect_contains(area, col, row) {
            return PopupHit::Outside;
        }
        // Check query row, list items, or generic popup
        todo!()
    }
}
```

### Migration

After creating:
1. Refactor `dark_chat`'s `render_model_selector_popup`, `render_agent_selector_popup`, `render_composer_autocomplete_popup` to use `PopupOverlay::render(frame, area, props, theme)` where `props` is constructed from app state.
2. Refactor `dark_tui`'s `render_picker_popup`, `render_autocomplete_popup` similarly.
3. Replace all `*_popup_area` functions with `PopupOverlay::area(parent, props)`.
4. Replace all popup hit-test functions with `PopupOverlay::hit_test(parent, props, col, row)`.

**Estimated reduction**: ~300 lines removed from both chat_panel.rs files combined.

---

## Step 2: Create `ListViewport` utility

### Why

Both frontends implement the same viewport scrolling logic for popup lists:
```rust
let viewport_start = selected.saturating_sub(visible - 1);
let viewport_items = items.iter().skip(viewport_start).take(visible);
// Check if index == selected for highlight
```

This is repeated in every popup render function and in the sessions panel.

### What to create

Add to `lib/dark_tui_components/src/utils/viewport.rs`:

```rust
/// Calculate viewport window for a scrollable list.
pub struct ListViewport {
    pub start: usize,
    pub end: usize,
    pub visible: usize,
}

impl ListViewport {
    /// Calculate viewport centered on selected item.
    pub fn new(total: usize, visible: usize, selected: usize) -> Self {
        let start = if selected >= visible {
            selected - visible + 1
        } else {
            0
        };
        let end = (start + visible).min(total);
        Self { start, end, visible }
    }

    /// Check if an absolute index is the selected one.
    pub fn is_selected(&self, abs_index: usize, selected: usize) -> bool {
        abs_index == selected
    }

    /// Convert absolute index to viewport-relative position.
    pub fn relative_index(&self, abs_index: usize) -> Option<usize> {
        if abs_index >= self.start && abs_index < self.end {
            Some(abs_index - self.start)
        } else {
            None
        }
    }
}
```

---

## Step 3: Extract `FooterBar` component pattern

### Why

Both frontends render footers with the same pattern: a line of StatusPill spans separated by spaces, inside a block. The content differs but the structure is identical.

### What to create

Add to `lib/dark_tui_components/src/components/footer_bar.rs`:

```rust
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::theme::ComponentThemeLike;

pub struct FooterBarProps<'a> {
    pub segments: Vec<Span<'a>>,
    pub separator: &'a str,
}

pub struct FooterBar;

impl FooterBar {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        props: &FooterBarProps,
        theme: &impl ComponentThemeLike,
    ) {
        let mut spans = Vec::new();
        for (i, segment) in props.segments.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(
                    props.separator,
                    Style::default().fg(theme.text_muted()),
                ));
            }
            spans.push(segment.clone());
        }
        let line = Line::from(spans);
        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, area);
    }
}
```

### Migration

Both frontends' `footer_panel.rs` files construct a `Line::from(vec![...spans...])`. Replace with:
```rust
FooterBar::render(frame, area, &FooterBarProps {
    segments: vec![status_pill, model_pill, activity_pill, sync_pill],
    separator: "  ",
}, theme);
```

This is a minor savings (~10-20 lines each) but establishes the pattern.

---

## Step 4: Unify `KeyBarPanel`

### Why

Both frontends define static key bind arrays and render them identically via `KeyHintBar`. The rendering is already shared; this step shares the common key binds.

### What to create

Add to `lib/dark_tui_components/src/components/key_hint_bar.rs` (extend existing):

```rust
/// Common key binds shared across frontends.
pub mod common_keys {
    use super::KeyBind;

    pub const QUIT: KeyBind = KeyBind::new("q", "Quit");
    pub const REFRESH: KeyBind = KeyBind::new("r", "Refresh");
    pub const TAB_FOCUS: KeyBind = KeyBind::new("Tab", "Focus");
    pub const NAV_UP: KeyBind = KeyBind::new("j/k", "Navigate");
    pub const COMPOSE: KeyBind = KeyBind::new("i", "Compose");
    pub const SEND: KeyBind = KeyBind::new("Enter", "Send");
    pub const ESCAPE: KeyBind = KeyBind::new("Esc", "Cancel");
    pub const MODEL: KeyBind = KeyBind::new("m", "Model");
    pub const AGENT: KeyBind = KeyBind::new("a", "Agent");
}
```

### Migration

Both frontends' key_bar_panel files import `common_keys::*` instead of defining their own duplicates. App-specific keys remain local.

---

## Step 5: Wire up new components

### File: `lib/dark_tui_components/src/components/mod.rs`

Add:
```rust
pub mod popup_overlay;
pub mod footer_bar;
```

### File: `lib/dark_tui_components/src/lib.rs`

Add re-exports:
```rust
pub use components::popup_overlay::*;
pub use components::footer_bar::*;
```

---

## Verification

```bash
cargo check -p dark_tui_components
cargo check -p dark_chat
cargo check -p dark_tui
cargo test -p dark_tui_components
```

## Estimated Impact

- `PopupOverlay` eliminates ~300 lines of duplicate popup code across both frontends
- `ListViewport` eliminates ~50 lines of viewport math across both frontends
- `FooterBar` eliminates ~30 lines and establishes a composable pattern
- Common key binds reduce maintenance of duplicate constants
- Total: ~400 lines of duplication removed, cleaner component boundaries
