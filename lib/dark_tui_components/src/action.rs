use std::any::Any;
use std::sync::Arc;

/// Generic actions that components can emit and consume.
#[derive(Debug, Clone)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Quit,
    FocusNext,
    FocusPrevious,
    ScrollUp,
    ScrollDown,
    ScrollPageUp,
    ScrollPageDown,
    ScrollToTop,
    ScrollToBottom,
    SelectNext,
    SelectPrevious,
    Select(usize),
    Confirm,
    Cancel,
    InsertChar(char),
    Backspace,
    Delete,
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,
    Error(String),
    StatusMessage(String),
    Custom(Arc<dyn Any + Send + Sync>),
    Noop,
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tick, Self::Tick)
            | (Self::Render, Self::Render)
            | (Self::Quit, Self::Quit)
            | (Self::FocusNext, Self::FocusNext)
            | (Self::FocusPrevious, Self::FocusPrevious)
            | (Self::ScrollUp, Self::ScrollUp)
            | (Self::ScrollDown, Self::ScrollDown)
            | (Self::ScrollPageUp, Self::ScrollPageUp)
            | (Self::ScrollPageDown, Self::ScrollPageDown)
            | (Self::ScrollToTop, Self::ScrollToTop)
            | (Self::ScrollToBottom, Self::ScrollToBottom)
            | (Self::SelectNext, Self::SelectNext)
            | (Self::SelectPrevious, Self::SelectPrevious)
            | (Self::Confirm, Self::Confirm)
            | (Self::Cancel, Self::Cancel)
            | (Self::Backspace, Self::Backspace)
            | (Self::Delete, Self::Delete)
            | (Self::CursorLeft, Self::CursorLeft)
            | (Self::CursorRight, Self::CursorRight)
            | (Self::CursorHome, Self::CursorHome)
            | (Self::CursorEnd, Self::CursorEnd)
            | (Self::Noop, Self::Noop) => true,
            (Self::Resize(w1, h1), Self::Resize(w2, h2)) => w1 == w2 && h1 == h2,
            (Self::Select(v1), Self::Select(v2)) => v1 == v2,
            (Self::InsertChar(c1), Self::InsertChar(c2)) => c1 == c2,
            (Self::Error(m1), Self::Error(m2)) => m1 == m2,
            (Self::StatusMessage(m1), Self::StatusMessage(m2)) => m1 == m2,
            (Self::Custom(_), Self::Custom(_)) => false,
            _ => false,
        }
    }
}
