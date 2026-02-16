use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{App, FocusPane};

pub enum LoopAction {
    None,
    Quit,
    Refresh,
    SelectNextSession,
    SelectPreviousSession,
    SelectNextAgent,
    CreateSession,
    OpenCompose,
    SendPrompt,
    ScrollChatUp,
    ScrollChatDown,
    ScrollRuntimeUp,
    ScrollRuntimeDown,
    ToggleHelp,
    OpenModelSelector,
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> LoopAction {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return LoopAction::Quit;
    }

    if app.is_model_selector_open() {
        return handle_model_selector_key(app, key);
    }

    if app.is_agent_selector_open() {
        return handle_agent_selector_key(app, key);
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
        KeyCode::Char('m') => LoopAction::OpenModelSelector,
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

fn handle_model_selector_key(app: &mut App, key: KeyEvent) -> LoopAction {
    match key.code {
        KeyCode::Esc => {
            app.close_model_selector();
            app.set_status_message("Model selector closed.");
            LoopAction::None
        }
        KeyCode::Tab => {
            app.model_selector_toggle_mode();
            if app.model_selector_raw_mode() {
                app.set_status_message("Model selector: raw input mode.");
            } else {
                app.set_status_message("Model selector: filter mode.");
            }
            LoopAction::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.model_selector_move_up();
            LoopAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.model_selector_move_down();
            LoopAction::None
        }
        KeyCode::Enter => {
            if let Some(model) = app.confirm_model_selector() {
                app.set_status_message(format!("Model selected: {model}"));
            } else {
                app.set_status_message("No model selected.");
            }
            LoopAction::None
        }
        KeyCode::Backspace => {
            app.model_selector_backspace();
            LoopAction::None
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.model_selector_clear();
            LoopAction::None
        }
        KeyCode::Char(value)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            app.model_selector_insert_char(value);
            LoopAction::None
        }
        _ => LoopAction::None,
    }
}

fn handle_agent_selector_key(app: &mut App, key: KeyEvent) -> LoopAction {
    match key.code {
        KeyCode::Esc => {
            app.close_agent_selector();
            app.set_status_message("Agent selector closed.");
            LoopAction::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.agent_selector_move_up();
            LoopAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.agent_selector_move_down();
            LoopAction::None
        }
        KeyCode::Enter => {
            if let Some(agent) = app.confirm_agent_selector() {
                app.set_status_message(format!("Agent selected: {agent}"));
            } else {
                app.set_status_message("No agent selected.");
            }
            LoopAction::None
        }
        KeyCode::Backspace => {
            app.agent_selector_backspace();
            LoopAction::None
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.agent_selector_clear();
            LoopAction::None
        }
        KeyCode::Char(value)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            app.agent_selector_insert_char(value);
            LoopAction::None
        }
        _ => LoopAction::None,
    }
}

fn handle_compose_key(app: &mut App, key: KeyEvent) -> LoopAction {
    if app.composer_autocomplete_open() {
        match key.code {
            KeyCode::Esc => {
                app.close_composer_autocomplete();
                return LoopAction::None;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.composer_autocomplete_move_up();
                return LoopAction::None;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.composer_autocomplete_move_down();
                return LoopAction::None;
            }
            KeyCode::Tab | KeyCode::Enter => {
                let _ = app.apply_composer_autocomplete_selection();
                return LoopAction::None;
            }
            _ => {}
        }
    }

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
