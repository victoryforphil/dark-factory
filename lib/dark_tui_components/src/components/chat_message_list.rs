use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

use crate::components::chat_types::{ChatMessageEntry, ChatMessageRole};
use crate::theme::ComponentThemeLike;

#[derive(Debug, Clone, Copy)]
pub struct ChatPalette {
    pub text_primary: Color,
    pub role_user: Color,
    pub role_assistant: Color,
    pub role_system: Color,
    pub role_tool: Color,
    pub role_other: Color,
}

impl ChatPalette {
    pub fn from_theme(theme: &impl ComponentThemeLike) -> Self {
        Self {
            text_primary: Color::White,
            role_user: theme.pill_info_fg(),
            role_assistant: theme.pill_accent_fg(),
            role_system: theme.pill_warn_fg(),
            role_tool: theme.pill_ok_fg(),
            role_other: theme.text_secondary(),
        }
    }

    fn role_color(&self, role: &ChatMessageRole) -> Color {
        match role {
            ChatMessageRole::User => self.role_user,
            ChatMessageRole::Assistant => self.role_assistant,
            ChatMessageRole::System => self.role_system,
            ChatMessageRole::Tool => self.role_tool,
            ChatMessageRole::Other(_) => self.role_other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessageListProps<'a> {
    pub messages: &'a [ChatMessageEntry],
    pub empty_label: &'a str,
    pub max_messages: usize,
    pub max_body_lines_per_message: usize,
    pub scroll_offset_lines: u16,
    pub palette: ChatPalette,
}

impl<'a> ChatMessageListProps<'a> {
    pub fn from_messages(
        messages: &'a [ChatMessageEntry],
        theme: &impl ComponentThemeLike,
    ) -> Self {
        Self {
            messages,
            empty_label: "No chat messages yet.",
            max_messages: 60,
            max_body_lines_per_message: 14,
            scroll_offset_lines: 0,
            palette: ChatPalette::from_theme(theme),
        }
    }
}

pub struct ChatMessageListComponent;

impl ChatMessageListComponent {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        theme: &impl ComponentThemeLike,
        props: ChatMessageListProps<'_>,
    ) {
        let lines = Self::lines(theme, &props);
        let viewport_height = area.height as usize;
        let total_lines = lines.len();
        let base_scroll = total_lines.saturating_sub(viewport_height);
        let scroll = base_scroll.saturating_sub(props.scroll_offset_lines as usize) as u16;

        frame.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: false })
                .scroll((scroll, 0)),
            area,
        );
    }

    fn lines(
        theme: &impl ComponentThemeLike,
        props: &ChatMessageListProps<'_>,
    ) -> Vec<Line<'static>> {
        if props.messages.is_empty() {
            return vec![Line::styled(
                props.empty_label.to_string(),
                Style::default().fg(theme.text_muted()),
            )];
        }

        let mut lines = Vec::new();
        let cap = props.max_messages.max(1);
        let start = props.messages.len().saturating_sub(cap);

        for message in &props.messages[start..] {
            let role_style = Style::default()
                .fg(props.palette.role_color(&message.role))
                .add_modifier(Modifier::BOLD);

            let mut header_spans = vec![Span::styled(
                format!("[{}]", role_label(&message.role)),
                role_style,
            )];

            if let Some(created_at) = message.created_at.as_ref() {
                header_spans.push(Span::raw(" "));
                header_spans.push(Span::styled(
                    created_at.clone(),
                    Style::default().fg(theme.text_muted()),
                ));
            }

            lines.push(Line::from(header_spans));

            let mut rendered_body = render_message_body(
                message,
                props.palette,
                props.max_body_lines_per_message,
                theme,
            );

            if rendered_body.is_empty() {
                rendered_body.push(Line::from(Span::styled(
                    "  (no content)",
                    Style::default().fg(theme.text_muted()),
                )));
            }

            lines.extend(rendered_body);
            lines.push(Line::from(Span::styled(
                "------------------------",
                Style::default().fg(theme.text_muted()),
            )));
            lines.push(Line::raw(""));
        }

        lines
    }
}

fn render_message_body(
    message: &ChatMessageEntry,
    palette: ChatPalette,
    max_body_lines: usize,
    theme: &impl ComponentThemeLike,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut in_code_block = false;
    let mut blank_streak = 0usize;
    let mut rendered = 0usize;

    let cap = max_body_lines.max(1);

    for raw_line in message.text.replace("\r\n", "\n").lines() {
        if rendered >= cap {
            lines.push(Line::from(Span::styled(
                "  ...",
                Style::default().fg(theme.text_muted()),
            )));
            break;
        }

        let trimmed = raw_line.trim_end();

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            lines.push(Line::from(Span::styled(
                if in_code_block {
                    "  [code:start]"
                } else {
                    "  [code:end]"
                },
                Style::default().fg(theme.text_secondary()),
            )));
            rendered += 1;
            blank_streak = 0;
            continue;
        }

        if trimmed.is_empty() {
            blank_streak += 1;
            if blank_streak > 1 {
                continue;
            }

            lines.push(Line::raw(""));
            rendered += 1;
            continue;
        }

        blank_streak = 0;

        let body_style = if in_code_block {
            Style::default().fg(theme.text_secondary())
        } else {
            Style::default().fg(palette.text_primary)
        };

        let prefix = if in_code_block { "  | " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!("{prefix}{trimmed}"),
            body_style,
        )));
        rendered += 1;
    }

    lines
}

fn role_label(role: &ChatMessageRole) -> &str {
    match role {
        ChatMessageRole::User => "YOU",
        ChatMessageRole::Assistant => "AI",
        ChatMessageRole::System => "SYS",
        ChatMessageRole::Tool => "TOOL",
        ChatMessageRole::Other(value) => value.as_str(),
    }
}
