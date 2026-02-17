use crossterm::event::{KeyEvent, MouseEvent};

/// Normalized input events consumed by component implementations.
#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
    Resize(u16, u16),
}
