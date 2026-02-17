use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::components::status_pill::StatusPill;
use crate::theme::ComponentThemeLike;

/// Semantic tone for conversation status labels.
#[derive(Debug, Clone, Copy)]
pub enum ChatStatusTone {
    Info,
    Ok,
    Warn,
    Error,
    Muted,
    Accent,
}

/// Props for rendering a conversation header section.
#[derive(Debug, Clone)]
pub struct ChatConversationHeaderProps {
    pub title: String,
    pub subtitle: Option<String>,
    pub status_label: Option<String>,
    pub status_tone: ChatStatusTone,
}

/// Renderer for conversation title/subtitle/status rows.
pub struct ChatConversationHeaderComponent;

impl ChatConversationHeaderComponent {
    /// Renders a conversation header.
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        theme: &impl ComponentThemeLike,
        props: ChatConversationHeaderProps,
    ) {
        let mut lines = vec![Line::from(vec![Span::styled(
            props.title,
            Style::default().add_modifier(Modifier::BOLD),
        )])];

        let mut subline = Vec::new();

        if let Some(subtitle) = props.subtitle.filter(|value| !value.trim().is_empty()) {
            subline.push(Span::styled(
                subtitle,
                Style::default().fg(theme.text_secondary()),
            ));
        }

        if let Some(status_label) = props.status_label.filter(|value| !value.trim().is_empty()) {
            if !subline.is_empty() {
                subline.push(Span::styled(
                    "  ".to_string(),
                    Style::default().fg(theme.text_muted()),
                ));
            }

            let pill = match props.status_tone {
                ChatStatusTone::Info => StatusPill::info(status_label, theme),
                ChatStatusTone::Ok => StatusPill::ok(status_label, theme),
                ChatStatusTone::Warn => StatusPill::warn(status_label, theme),
                ChatStatusTone::Error => StatusPill::error(status_label, theme),
                ChatStatusTone::Muted => StatusPill::muted(status_label, theme),
                ChatStatusTone::Accent => StatusPill::accent(status_label, theme),
            };
            subline.push(pill.span());
        }

        if !subline.is_empty() {
            lines.push(Line::from(subline));
        }

        frame.render_widget(Paragraph::new(lines), area);
    }
}
