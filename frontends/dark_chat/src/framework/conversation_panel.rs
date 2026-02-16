use dark_tui_components::{
    ChatComposerComponent, ChatComposerProps, ChatConversationHeaderComponent,
    ChatConversationHeaderProps, ChatMessageEntry, ChatMessageListComponent, ChatMessageListProps,
    ChatMessageRole, ChatPalette, ChatStatusTone, ComponentThemeLike, PaneBlockComponent,
};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversationStatusTone {
    Info,
    Ok,
    Warn,
    Error,
    Muted,
    Accent,
}

#[derive(Debug, Clone)]
pub struct ConversationHeader<'a> {
    pub title: Cow<'a, str>,
    pub subtitle: Option<Cow<'a, str>>,
    pub status_label: Option<Cow<'a, str>>,
    pub status_tone: ConversationStatusTone,
}

#[derive(Debug, Clone, Copy)]
pub struct ConversationMessage<'a> {
    pub role: &'a str,
    pub text: &'a str,
    pub created_at: Option<&'a str>,
}

#[derive(Debug, Clone, Copy)]
pub struct ConversationComposer<'a> {
    pub enabled: bool,
    pub composing: bool,
    pub draft: &'a str,
    pub cursor_index: usize,
    pub idle_hint: &'a str,
    pub disabled_hint: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct ConversationPalette {
    pub text_primary: Color,
    pub role_user: Color,
    pub role_assistant: Color,
    pub role_system: Color,
    pub role_tool: Color,
    pub role_other: Color,
}

#[derive(Debug, Clone)]
pub struct ConversationPanelProps<'a> {
    pub title: &'a str,
    pub focused: bool,
    pub header: ConversationHeader<'a>,
    pub messages: &'a [ConversationMessage<'a>],
    pub empty_label: &'a str,
    pub max_messages: usize,
    pub max_body_lines_per_message: usize,
    pub scroll_offset_lines: u16,
    pub composer: ConversationComposer<'a>,
    pub palette: ConversationPalette,
}

pub fn render_conversation_panel(
    frame: &mut Frame,
    area: Rect,
    theme: &impl ComponentThemeLike,
    props: ConversationPanelProps<'_>,
) {
    let block = PaneBlockComponent::build(props.title, props.focused, theme);
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

    ChatConversationHeaderComponent::render(
        frame,
        chunks[0],
        theme,
        ChatConversationHeaderProps {
            title: props.header.title.to_string(),
            subtitle: props.header.subtitle.map(|value| value.to_string()),
            status_label: props.header.status_label.map(|value| value.to_string()),
            status_tone: chat_status_tone(props.header.status_tone),
        },
    );

    let message_entries = props
        .messages
        .iter()
        .map(|message| {
            ChatMessageEntry::new(
                ChatMessageRole::from_role(message.role),
                message.text,
                message.created_at.map(ToString::to_string),
            )
        })
        .collect::<Vec<_>>();

    ChatMessageListComponent::render(
        frame,
        chunks[1],
        theme,
        ChatMessageListProps {
            messages: &message_entries,
            empty_label: props.empty_label,
            max_messages: props.max_messages,
            max_body_lines_per_message: props.max_body_lines_per_message,
            scroll_offset_lines: props.scroll_offset_lines,
            palette: ChatPalette {
                text_primary: props.palette.text_primary,
                role_user: props.palette.role_user,
                role_assistant: props.palette.role_assistant,
                role_system: props.palette.role_system,
                role_tool: props.palette.role_tool,
                role_other: props.palette.role_other,
            },
        },
    );

    ChatComposerComponent::render(
        frame,
        chunks[2],
        theme,
        ChatComposerProps {
            enabled: props.composer.enabled,
            composing: props.composer.composing,
            draft: props.composer.draft,
            cursor_index: props.composer.cursor_index,
            idle_hint: props.composer.idle_hint,
            disabled_hint: props.composer.disabled_hint,
        },
    );
}

pub fn status_tone_for_status(status: &str) -> ConversationStatusTone {
    match status.trim().to_ascii_lowercase().as_str() {
        "ready" | "active" | "idle" => ConversationStatusTone::Ok,
        "busy" | "running" => ConversationStatusTone::Info,
        "retry" | "retrying" | "warning" => ConversationStatusTone::Warn,
        "error" | "failed" => ConversationStatusTone::Error,
        "stopped" | "offline" => ConversationStatusTone::Muted,
        _ => ConversationStatusTone::Accent,
    }
}

fn chat_status_tone(value: ConversationStatusTone) -> ChatStatusTone {
    match value {
        ConversationStatusTone::Info => ChatStatusTone::Info,
        ConversationStatusTone::Ok => ChatStatusTone::Ok,
        ConversationStatusTone::Warn => ChatStatusTone::Warn,
        ConversationStatusTone::Error => ChatStatusTone::Error,
        ConversationStatusTone::Muted => ChatStatusTone::Muted,
        ConversationStatusTone::Accent => ChatStatusTone::Accent,
    }
}
