use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{App, FocusPane};

pub enum LoopAction {
    None,
    Quit,
    Refresh,
    SelectNextSession,
    SelectPreviousSession,
    SelectNextAgent,
    SelectNextModel,
    CreateSession,
    OpenCompose,
    SendPrompt,
    ScrollChatUp,
    ScrollChatDown,
    ScrollRuntimeUp,
    ScrollRuntimeDown,
    ToggleHelp,
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> LoopAction {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return LoopAction::Quit;
    }

    if app.is_composing() {
        return handle_compose_key(app, key);
    }

    if key.code == KeyCode::Tab {
        cycle_focus_forward(app);
        return LoopAction::None;
    }

    if key.code == KeyCode::BackTab {
        cycle_focus_backward(app);
        return LoopAction::None;
    }

    match key.code {
        KeyCode::Char('q') => LoopAction::Quit,
        KeyCode::Down | KeyCode::Char('j') => match app.focus() {
            FocusPane::Sessions => LoopAction::SelectNextSession,
            FocusPane::Chat | FocusPane::Composer => LoopAction::ScrollChatDown,
            FocusPane::Runtime => LoopAction::ScrollRuntimeDown,
        },
        KeyCode::Up | KeyCode::Char('k') => match app.focus() {
            FocusPane::Sessions => LoopAction::SelectPreviousSession,
            FocusPane::Chat | FocusPane::Composer => LoopAction::ScrollChatUp,
            FocusPane::Runtime => LoopAction::ScrollRuntimeUp,
        },
        KeyCode::Char('r') => LoopAction::Refresh,
        KeyCode::Char('n') => LoopAction::CreateSession,
        KeyCode::Char('a') => LoopAction::SelectNextAgent,
        KeyCode::Char('m') => LoopAction::SelectNextModel,
        KeyCode::Char('c') => LoopAction::OpenCompose,
        KeyCode::Esc => {
            app.set_focus(FocusPane::Chat);
            app.reset_chat_scroll();
            app.reset_runtime_scroll();
            LoopAction::None
        }
        KeyCode::Char('h') => LoopAction::ToggleHelp,
        _ => LoopAction::None,
    }
}

fn handle_compose_key(app: &mut App, key: KeyEvent) -> LoopAction {
    match key.code {
        KeyCode::Esc => {
            app.cancel_composer();
            app.set_status_message("Compose cancelled.");
            LoopAction::None
        }
        KeyCode::Delete => {
            app.delete_draft_char();
            LoopAction::None
        }
        KeyCode::Left => {
            app.move_draft_cursor_left();
            LoopAction::None
        }
        KeyCode::Right => {
            app.move_draft_cursor_right();
            LoopAction::None
        }
        KeyCode::Home => {
            app.move_draft_cursor_home();
            LoopAction::None
        }
        KeyCode::End => {
            app.move_draft_cursor_end();
            LoopAction::None
        }
        KeyCode::Enter if key.modifiers.contains(KeyModifiers::SHIFT) => {
            app.insert_draft_char('\n');
            LoopAction::None
        }
        KeyCode::Enter => LoopAction::SendPrompt,
        KeyCode::Backspace => {
            app.backspace_draft();
            LoopAction::None
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.clear_draft();
            LoopAction::None
        }
        KeyCode::Char(value)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            app.insert_draft_char(value);
            LoopAction::None
        }
        _ => LoopAction::None,
    }
}

fn cycle_focus_forward(app: &mut App) {
    let next = match app.focus() {
        FocusPane::Sessions => FocusPane::Chat,
        FocusPane::Chat => FocusPane::Runtime,
        FocusPane::Runtime => FocusPane::Sessions,
        FocusPane::Composer => FocusPane::Runtime,
    };
    app.set_focus(next);
}

fn cycle_focus_backward(app: &mut App) {
    let next = match app.focus() {
        FocusPane::Sessions => FocusPane::Runtime,
        FocusPane::Chat => FocusPane::Sessions,
        FocusPane::Runtime => FocusPane::Chat,
        FocusPane::Composer => FocusPane::Sessions,
    };
    app.set_focus(next);
}
