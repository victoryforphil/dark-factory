use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use dark_tui_components::{Action, Component, ComponentResult, Event};
use ratatui::Frame;
use ratatui::layout::Rect;

struct TestCounter {
    count: usize,
    focused: bool,
}

impl Component for TestCounter {
    fn handle_key_event(&mut self, key: KeyEvent) -> ComponentResult<Option<Action>> {
        match key.code {
            KeyCode::Up => Ok(Some(Action::SelectPrevious)),
            KeyCode::Down => Ok(Some(Action::SelectNext)),
            KeyCode::Char('q') => Ok(Some(Action::Quit)),
            _ => Ok(None),
        }
    }

    fn update(&mut self, action: &Action) -> ComponentResult<Option<Action>> {
        match action {
            Action::SelectNext => {
                self.count += 1;
                Ok(Some(Action::Render))
            }
            Action::SelectPrevious => {
                self.count = self.count.saturating_sub(1);
                Ok(Some(Action::Render))
            }
            _ => Ok(None),
        }
    }

    fn draw(
        &self,
        _frame: &mut Frame,
        _area: Rect,
        _theme: &dyn dark_tui_components::ComponentThemeLike,
    ) -> ComponentResult {
        Ok(())
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
}

#[test]
fn component_update_chain_returns_render() {
    let mut counter = TestCounter {
        count: 0,
        focused: false,
    };

    let result = counter
        .update(&Action::SelectNext)
        .expect("update succeeds");
    assert_eq!(counter.count, 1);
    assert_eq!(result, Some(Action::Render));
}

#[test]
fn component_handle_event_routes_key_events() {
    let mut counter = TestCounter {
        count: 0,
        focused: false,
    };

    let event = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
    let action = counter
        .handle_event(&event)
        .expect("event handling succeeds");

    assert_eq!(action, Some(Action::Quit));
}

#[test]
fn component_focus_state_is_mutable() {
    let mut counter = TestCounter {
        count: 0,
        focused: false,
    };

    assert!(!counter.is_focused());
    counter.set_focused(true);
    assert!(counter.is_focused());
}
