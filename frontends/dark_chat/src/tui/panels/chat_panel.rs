use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

use dark_tui_components::{
    ChatComposerComponent, ChatComposerProps, ChatConversationHeaderComponent,
    ChatConversationHeaderProps, ChatMessageListComponent, ChatMessageListProps, ChatPalette,
    ChatStatusTone, PaneBlockComponent,
};

use crate::tui::app::{App, FocusPane};
use crate::tui::components::to_component_messages;

pub struct ChatPanel;

impl ChatPanel {
    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build(
            "Conversation",
            app.is_focus(FocusPane::Chat) || app.is_focus(FocusPane::Composer),
            theme,
        );
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width < 18 || inner.height < 6 {
            return;
        }

        let show_preview = inner.height >= 16;
        let mut constraints = vec![Constraint::Length(3), Constraint::Min(4)];
        if show_preview {
            constraints.push(Constraint::Length(6));
        }
        constraints.push(Constraint::Length(3));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner);

        let composer_index = chunks.len().saturating_sub(1);

        let header = match app.active_session() {
            Some(session) => ChatConversationHeaderProps {
                title: session.title.clone(),
                subtitle: Some(format!("session:{}", compact_id(&session.id))),
                status_label: Some(session.status.clone()),
                status_tone: status_tone(&session.status),
            },
            None => ChatConversationHeaderProps {
                title: "No session".to_string(),
                subtitle: Some("Press n to create a session".to_string()),
                status_label: Some("idle".to_string()),
                status_tone: ChatStatusTone::Muted,
            },
        };

        ChatConversationHeaderComponent::render(frame, chunks[0], theme, header);

        let component_messages = to_component_messages(app.messages());
        let message_list = ChatMessageListProps {
            messages: &component_messages,
            empty_label: "No messages yet. Send a prompt to begin.",
            max_messages: 80,
            max_body_lines_per_message: 18,
            scroll_offset_lines: app.chat_scroll_lines(),
            palette: ChatPalette {
                text_primary: Color::White,
                role_user: theme.pill_info_fg,
                role_assistant: theme.pill_accent_fg,
                role_system: theme.pill_warn_fg,
                role_tool: theme.pill_ok_fg,
                role_other: theme.text_secondary,
            },
        };
        ChatMessageListComponent::render(frame, chunks[1], theme, message_list);

        if show_preview {
            render_code_preview(frame, chunks[2], app);
        }

        if app.is_composing() && app.active_session().is_some() {
            let composer = app.composer().clone();
            frame.render_widget(&composer, chunks[composer_index]);
            return;
        }

        let composer = ChatComposerProps {
            enabled: app.active_session().is_some(),
            composing: app.is_composing(),
            draft: app.draft(),
            cursor_index: app.draft_cursor(),
            idle_hint: "Press c to compose. Enter sends. Shift+Enter is newline.",
            disabled_hint: "Session required before sending prompts.",
        };
        ChatComposerComponent::render(frame, chunks[composer_index], theme, composer);
    }
}

fn render_code_preview(frame: &mut Frame, area: Rect, app: &App) {
    if area.width < 18 || area.height < 2 {
        return;
    }

    let block = Block::default()
        .title(format!("Code Preview ({})", app.code_preview_source()))
        .borders(Borders::TOP);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 8 || inner.height == 0 {
        return;
    }

    frame.render_widget(app.code_preview_editor(), inner);
}

fn compact_id(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() <= 16 {
        return trimmed.to_string();
    }

    format!("{}...", &trimmed[..16])
}

fn status_tone(status: &str) -> ChatStatusTone {
    match status.trim().to_ascii_lowercase().as_str() {
        "idle" | "ready" => ChatStatusTone::Ok,
        "busy" | "running" => ChatStatusTone::Info,
        "retry" | "retrying" => ChatStatusTone::Warn,
        "error" | "failed" => ChatStatusTone::Error,
        _ => ChatStatusTone::Muted,
    }
}
