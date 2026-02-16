use dark_chat::framework::{
    ConversationComposer, ConversationHeader, ConversationMessage, ConversationPalette,
    ConversationPanelProps, ConversationStatusTone, render_conversation_panel,
    status_tone_for_status,
};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Color;
use std::borrow::Cow;

use crate::app::App;

pub(crate) struct ChatPanel;

impl ChatPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let header = Self::header(app);
        let messages = app
            .chat_messages()
            .iter()
            .map(|message| ConversationMessage {
                role: &message.role,
                text: &message.text,
                created_at: Some(message.created_at.as_str()),
            })
            .collect::<Vec<_>>();

        let empty_label = if app.chat_actor().is_some() {
            "No chat messages yet."
        } else {
            "Select an actor node to open chat."
        };

        render_conversation_panel(
            frame,
            area,
            theme,
            ConversationPanelProps {
                title: "Actor Chat",
                focused: true,
                header,
                messages: &messages,
                empty_label,
                max_messages: 20,
                max_body_lines_per_message: 12,
                scroll_offset_lines: 0,
                composer: Self::composer(app),
                palette: Self::palette(app),
            },
        );
    }

    fn header(app: &App) -> ConversationHeader<'_> {
        let Some(actor) = app.chat_actor() else {
            return ConversationHeader {
                title: Cow::Borrowed("No actor selected"),
                subtitle: Some(Cow::Borrowed("Select an actor node in the catalog")),
                status_label: Some(Cow::Borrowed("idle")),
                status_tone: ConversationStatusTone::Muted,
            };
        };

        let subtitle = if actor.description.trim().is_empty() || actor.description.trim() == "-" {
            Some(Cow::Owned(format!("provider:{}", actor.provider)))
        } else {
            Some(Cow::Borrowed(actor.description.as_str()))
        };

        ConversationHeader {
            title: Cow::Borrowed(actor.title.as_str()),
            subtitle,
            status_label: Some(Cow::Borrowed(actor.status.as_str())),
            status_tone: status_tone_for_status(&actor.status),
        }
    }

    fn palette(app: &App) -> ConversationPalette {
        let theme = app.theme();
        ConversationPalette {
            text_primary: theme.text_primary,
            role_user: theme.entity_variant,
            role_assistant: theme.entity_actor,
            role_system: Color::Yellow,
            role_tool: Color::Cyan,
            role_other: theme.text_secondary,
        }
    }

    fn composer(app: &App) -> ConversationComposer<'_> {
        ConversationComposer {
            enabled: app.chat_actor().is_some(),
            composing: app.is_chat_composing(),
            draft: app.chat_draft(),
            cursor_index: app.chat_draft().chars().count(),
            idle_hint: "Press c to compose, Enter to send, t to hide chat.",
            disabled_hint: "Input disabled until an actor is selected.",
        }
    }
}
