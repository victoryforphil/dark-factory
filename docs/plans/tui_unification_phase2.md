# Phase 2: Introduce Component Trait in dark_tui_components

**Risk**: Medium | **Impact**: High | **Dependencies**: None (can run parallel with Phase 1)

## Goal

Introduce a formal `Component` trait following the full Ratatui template pattern (init/handle_events/update/render with Action enum and channel registration). This provides the structural foundation for all subsequent phases.

---

## Step 1: Add `crossterm` dependency to dark_tui_components

### File: `lib/dark_tui_components/Cargo.toml`

Add to `[dependencies]`:
```toml
crossterm = "0.29.0"
tokio = { version = "1.48.0", features = ["sync"] }
```

These are needed for `KeyEvent`/`MouseEvent` types in the trait and `mpsc::UnboundedSender` for action channels.

---

## Step 2: Create `action.rs` - Core Action enum

### File: `lib/dark_tui_components/src/action.rs`

```rust
/// Core actions shared across all components.
/// Frontends extend this with app-specific actions via the `Custom` variant.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    // Lifecycle
    Tick,
    Render,
    Resize(u16, u16),
    Quit,

    // Focus
    FocusNext,
    FocusPrevious,

    // Scroll
    ScrollUp,
    ScrollDown,
    ScrollPageUp,
    ScrollPageDown,
    ScrollToTop,
    ScrollToBottom,

    // Selection
    SelectNext,
    SelectPrevious,
    Select(usize),
    Confirm,
    Cancel,

    // Input
    InsertChar(char),
    Backspace,
    Delete,
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,

    // Status
    Error(String),
    StatusMessage(String),

    // App-specific extension point.
    // Frontends define their own enum and wrap it here.
    Custom(Box<dyn std::any::Any + Send + Sync>),

    /// No-op (swallow event).
    Noop,
}
```

**Design notes**:
- `Custom(Box<dyn Any>)` allows frontends to pass app-specific actions through the shared trait without modifying the library. Frontends downcast:
  ```rust
  if let Action::Custom(payload) = action {
      if let Some(app_action) = payload.downcast_ref::<MyAppAction>() { ... }
  }
  ```
- `PartialEq` impl will need manual implementation for `Custom` (always false) and `Error`/`StatusMessage` (string compare).

---

## Step 3: Create `event.rs` - Event wrapper

### File: `lib/dark_tui_components/src/event.rs`

```rust
use crossterm::event::{KeyEvent, MouseEvent};

/// Unified event type wrapping crossterm events.
#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
    Resize(u16, u16),
}
```

---

## Step 4: Create `component.rs` - Core Component trait

### File: `lib/dark_tui_components/src/component.rs`

```rust
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::{Rect, Size};
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::event::Event;
use crate::theme::ComponentThemeLike;

/// Result type for component operations.
pub type ComponentResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Core component trait following Ratatui's component template pattern.
///
/// Components are self-contained units that:
/// - Own their state
/// - Handle events and produce actions
/// - Update state in response to actions
/// - Render themselves given theme + area
///
/// All methods have default no-op implementations to allow passive components
/// (pure render, no interactivity) to implement only `draw`.
pub trait Component: Send + Sync {
    /// Register an action sender for propagating actions to the app loop.
    /// Called once during component setup.
    fn register_action_handler(&mut self, _tx: UnboundedSender<Action>) -> ComponentResult {
        Ok(())
    }

    /// Initialize the component with available area size.
    /// Called on first render and on resize.
    fn init(&mut self, _area: Size) -> ComponentResult {
        Ok(())
    }

    /// Handle a raw event, delegating to key/mouse handlers.
    /// Returns an optional action to propagate.
    fn handle_event(&mut self, event: &Event) -> ComponentResult<Option<Action>> {
        match event {
            Event::Key(key) => self.handle_key_event(*key),
            Event::Mouse(mouse) => self.handle_mouse_event(*mouse),
            Event::Tick => Ok(None),
            Event::Resize(w, h) => {
                self.init(Size::new(*w, *h))?;
                Ok(None)
            }
        }
    }

    /// Handle a key event. Override for interactive components.
    fn handle_key_event(&mut self, _key: KeyEvent) -> ComponentResult<Option<Action>> {
        Ok(None)
    }

    /// Handle a mouse event. Override for clickable/scrollable components.
    fn handle_mouse_event(&mut self, _mouse: MouseEvent) -> ComponentResult<Option<Action>> {
        Ok(None)
    }

    /// Update internal state based on an action.
    /// May return a follow-up action for chaining.
    fn update(&mut self, _action: &Action) -> ComponentResult<Option<Action>> {
        Ok(None)
    }

    /// Render the component into the frame at the given area.
    /// `theme` provides consistent styling.
    fn draw(&self, frame: &mut Frame, area: Rect, theme: &dyn ComponentThemeLike) -> ComponentResult;

    /// Whether this component currently wants focus (for focus management).
    fn wants_focus(&self) -> bool {
        false
    }

    /// Whether this component is currently focused.
    fn is_focused(&self) -> bool {
        false
    }

    /// Set focus state.
    fn set_focused(&mut self, _focused: bool) {}
}

/// Type alias for boxed dynamic components.
pub type DynComponent = Box<dyn Component>;
```

**Design decisions**:
- `&dyn ComponentThemeLike` in `draw()` instead of generic `T` to allow object-safe `Box<dyn Component>`.
- `&Event` and `&Action` by reference to avoid unnecessary cloning.
- `Send + Sync` bound for compatibility with tokio tasks.
- Default implementations for everything except `draw` — but even `draw` has no required body (passive components exist).
- Focus management via `wants_focus`/`is_focused`/`set_focused` built into the trait.

---

## Step 5: Update `theme.rs` for object safety

### File: `lib/dark_tui_components/src/theme.rs`

The `ComponentThemeLike` trait must be object-safe (no `Self: Sized` bounds, no generics). Review current definition. It's already object-safe (all methods return `Color` by value, `&self` receiver). No changes expected unless there are generic methods.

Verify with: `let _: &dyn ComponentThemeLike;` compiles.

---

## Step 6: Wire up module exports

### File: `lib/dark_tui_components/src/lib.rs`

Add:
```rust
pub mod action;
pub mod component;
pub mod event;

pub use action::Action;
pub use component::{Component, ComponentResult, DynComponent};
pub use event::Event;
```

---

## Step 7: Implement Component for existing widgets (optional, incremental)

This is optional in Phase 2 — it demonstrates the pattern but doesn't need to be exhaustive. Implement for one interactive widget as proof-of-concept.

### Example: `KeyHintBar` as a Component

This is a stateless component that only renders. It demonstrates the minimal impl:

```rust
impl Component for KeyHintBarComponent {
    fn draw(&self, frame: &mut Frame, area: Rect, theme: &dyn ComponentThemeLike) -> ComponentResult {
        let lines = self.bar.lines_wrapped(area.width as usize, theme);
        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
        Ok(())
    }
}
```

Do NOT force-convert all existing widgets in this phase. The trait is additive.

---

## Step 8: Add tests

### File: `lib/dark_tui_components/tests/component_trait_test.rs`

```rust
use dark_tui_components::{Component, Action, Event, ComponentResult, ComponentTheme};
use ratatui::{Frame, layout::Rect};

// Minimal test component
struct TestCounter {
    count: usize,
    focused: bool,
}

impl Component for TestCounter {
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> ComponentResult<Option<Action>> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key.code {
            KeyCode::Up => Ok(Some(Action::SelectPrevious)),
            KeyCode::Down => Ok(Some(Action::SelectNext)),
            KeyCode::Char('q') => Ok(Some(Action::Quit)),
            _ => Ok(None),
        }
    }

    fn update(&mut self, action: &Action) -> ComponentResult<Option<Action>> {
        match action {
            Action::SelectNext => { self.count += 1; Ok(Some(Action::Render)) }
            Action::SelectPrevious => { self.count = self.count.saturating_sub(1); Ok(Some(Action::Render)) }
            _ => Ok(None)
        }
    }

    fn draw(&self, frame: &mut Frame, area: Rect, theme: &dyn dark_tui_components::ComponentThemeLike) -> ComponentResult {
        let text = format!("Count: {}", self.count);
        let paragraph = ratatui::widgets::Paragraph::new(text);
        frame.render_widget(paragraph, area);
        Ok(())
    }

    fn is_focused(&self) -> bool { self.focused }
    fn set_focused(&mut self, focused: bool) { self.focused = focused; }
}

#[test]
fn test_component_update_chain() {
    let mut counter = TestCounter { count: 0, focused: false };
    let result = counter.update(&Action::SelectNext).unwrap();
    assert_eq!(counter.count, 1);
    assert_eq!(result, Some(Action::Render));
}

#[test]
fn test_component_focus() {
    let mut counter = TestCounter { count: 0, focused: false };
    assert!(!counter.is_focused());
    counter.set_focused(true);
    assert!(counter.is_focused());
}
```

---

## Verification

```bash
cargo check -p dark_tui_components
cargo test -p dark_tui_components
cargo check -p dark_chat     # Should still compile (no breaking changes)
cargo check -p dark_tui      # Should still compile (no breaking changes)
```

## Notes

- This phase is purely additive. No existing code is modified except `lib.rs` exports and `Cargo.toml` deps.
- The `Action::Custom(Box<dyn Any>)` pattern is intentionally simple. If it proves awkward, Phase 5 can replace it with a generic `Action<A>` or trait-based dispatch.
- The `ComponentThemeLike` trait becoming `dyn`-compatible is critical. If it has any non-object-safe methods, they need `where Self: Sized` bounds added.

## Estimated Impact

- ~150 lines of new shared infrastructure
- Foundation for all subsequent phases (Component-based panel extraction, app.rs splits)
- No breaking changes to existing code
