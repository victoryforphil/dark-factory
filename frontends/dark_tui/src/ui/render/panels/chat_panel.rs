use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;

use crate::app::App;

use dark_tui_components::{
    ChatComposerComponent, ChatComposerProps, ChatConversationHeaderComponent,
    ChatConversationHeaderProps, ChatMessageEntry, ChatMessageListComponent, ChatMessageListProps,
    ChatMessageRole, ChatPalette, ChatStatusTone, PaneBlockComponent,
};

pub(crate) struct ChatPanel;

impl ChatPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build("Actor Chat", true, theme);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width < 16 || inner.height < 5 {
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(inner);

        let header = Self::header_props(app);
        ChatConversationHeaderComponent::render(frame, chunks[0], theme, header);

        let message_entries = app
            .chat_messages()
            .iter()
            .map(|message| {
                ChatMessageEntry::new(
                    ChatMessageRole::from_role(&message.role),
                    message.text.clone(),
                    Some(message.created_at.clone()),
                )
            })
            .collect::<Vec<_>>();

        let empty_label = if app.chat_actor().is_some() {
            "No chat messages yet."
        } else {
            "Select an actor node to open chat."
        };
        let messages = ChatMessageListProps {
            messages: &message_entries,
            empty_label,
            max_messages: 20,
            max_body_lines_per_message: 12,
            scroll_offset_lines: 0,
            palette: Self::chat_palette(app),
        };
        ChatMessageListComponent::render(frame, chunks[1], theme, messages);

        let composer = Self::composer_props(app);
        ChatComposerComponent::render(frame, chunks[2], theme, composer);
    }

    fn header_props(app: &App) -> ChatConversationHeaderProps {
        let Some(actor) = app.chat_actor() else {
            return ChatConversationHeaderProps {
                title: "No actor selected".to_string(),
                subtitle: Some("Select an actor node in the catalog".to_string()),
                status_label: Some("idle".to_string()),
                status_tone: ChatStatusTone::Muted,
            };
        };

        let subtitle = if actor.description.trim().is_empty() || actor.description.trim() == "-" {
            Some(format!("provider:{}", actor.provider))
        } else {
            Some(actor.description.clone())
        };

        ChatConversationHeaderProps {
            title: actor.title.clone(),
            subtitle,
            status_label: Some(actor.status.clone()),
            status_tone: status_tone(&actor.status),
        }
    }

    fn chat_palette(app: &App) -> ChatPalette {
        let theme = app.theme();
        ChatPalette {
            text_primary: theme.text_primary,
            role_user: theme.entity_variant,
            role_assistant: theme.entity_actor,
            role_system: Color::Yellow,
            role_tool: Color::Cyan,
            role_other: theme.text_secondary,
        }
    }

    fn composer_props(app: &App) -> ChatComposerProps<'_> {
        ChatComposerProps {
            enabled: app.chat_actor().is_some(),
            composing: app.is_chat_composing(),
            draft: app.chat_draft(),
            cursor_index: app.chat_draft().chars().count(),
            idle_hint: "Press c to compose, Enter to send, t to hide chat.",
            disabled_hint: "Input disabled until an actor is selected.",
        }
    }
}

fn status_tone(status: &str) -> ChatStatusTone {
    match status.trim().to_ascii_lowercase().as_str() {
        "ready" | "active" | "idle" => ChatStatusTone::Ok,
        "busy" | "running" => ChatStatusTone::Info,
        "retrying" | "warning" => ChatStatusTone::Warn,
        "error" | "failed" => ChatStatusTone::Error,
        "stopped" | "offline" => ChatStatusTone::Muted,
        _ => ChatStatusTone::Accent,
    }
}
