use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::{Rect, Size};
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::event::Event;
use crate::theme::ComponentThemeLike;

/// Standard result type used by component lifecycle methods.
pub type ComponentResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Shared component lifecycle trait for Ratatui surfaces.
pub trait Component: Send + Sync {
    /// Registers a channel used to emit follow-up actions.
    fn register_action_handler(&mut self, _tx: UnboundedSender<Action>) -> ComponentResult {
        Ok(())
    }

    /// Initializes component state for a viewport size.
    fn init(&mut self, _area: Size) -> ComponentResult {
        Ok(())
    }

    /// Handles a high-level event and optionally returns an action.
    fn handle_event(&mut self, event: &Event) -> ComponentResult<Option<Action>> {
        match event {
            Event::Key(key) => self.handle_key_event(*key),
            Event::Mouse(mouse) => self.handle_mouse_event(*mouse),
            Event::Tick => Ok(None),
            Event::Resize(width, height) => {
                self.init(Size::new(*width, *height))?;
                Ok(None)
            }
        }
    }

    /// Handles a key event.
    fn handle_key_event(&mut self, _key: KeyEvent) -> ComponentResult<Option<Action>> {
        Ok(None)
    }

    /// Handles a mouse event.
    fn handle_mouse_event(&mut self, _mouse: MouseEvent) -> ComponentResult<Option<Action>> {
        Ok(None)
    }

    /// Applies an incoming action and optionally emits another action.
    fn update(&mut self, _action: &Action) -> ComponentResult<Option<Action>> {
        Ok(None)
    }

    /// Draws the component into the provided frame area.
    fn draw(
        &self,
        frame: &mut Frame,
        area: Rect,
        theme: &dyn ComponentThemeLike,
    ) -> ComponentResult;

    /// Returns whether this component can be focus-targeted.
    fn wants_focus(&self) -> bool {
        false
    }

    /// Returns whether the component is currently focused.
    fn is_focused(&self) -> bool {
        false
    }

    /// Updates focused state.
    fn set_focused(&mut self, _focused: bool) {}
}

/// Trait object alias for storing heterogeneous components.
pub type DynComponent = Box<dyn Component>;
