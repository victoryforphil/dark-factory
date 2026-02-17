use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Wrap};

use crate::theme::ComponentThemeLike;

/// Props for rendering the chat composer body.
#[derive(Debug, Clone)]
pub struct ChatComposerProps<'a> {
    pub enabled: bool,
    pub composing: bool,
    pub draft: &'a str,
    pub cursor_index: usize,
    pub idle_hint: &'a str,
    pub disabled_hint: &'a str,
}

impl<'a> ChatComposerProps<'a> {
    /// Creates default composer props for a draft buffer.
    pub fn new(draft: &'a str) -> Self {
        Self {
            enabled: true,
            composing: false,
            draft,
            cursor_index: draft.chars().count(),
            idle_hint: "Press c to compose, Enter to send.",
            disabled_hint: "Input disabled.",
        }
    }
}

/// Renderer for chat composer hints and draft text.
pub struct ChatComposerComponent;

impl ChatComposerComponent {
    /// Renders composer content into the target area.
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        theme: &impl ComponentThemeLike,
        props: ChatComposerProps<'_>,
    ) {
        let lines = Self::lines(theme, props);
        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
    }

    fn lines(theme: &impl ComponentThemeLike, props: ChatComposerProps<'_>) -> Vec<Line<'static>> {
        if !props.enabled {
            return vec![
                Line::styled(
                    props.disabled_hint.to_string(),
                    Style::default().fg(theme.text_muted()),
                ),
                Line::styled(
                    "Tip: select a session/actor first.",
                    Style::default().fg(theme.text_muted()),
                ),
            ];
        }

        if !props.composing {
            return vec![
                Line::styled(
                    props.idle_hint.to_string(),
                    Style::default().fg(theme.text_muted()),
                ),
                Line::styled(
                    "Supports /commands, /grep and @file/path context refs.",
                    Style::default().fg(theme.text_secondary()),
                ),
            ];
        }

        let line = if props.draft.trim().is_empty() {
            "> _".to_string()
        } else {
            format!("> {}", with_cursor(props.draft, props.cursor_index))
        };

        let char_count = props.draft.chars().count();

        vec![
            Line::styled(line, Style::default()),
            Line::styled(
                format!("Enter=send  Esc=cancel  chars={char_count}"),
                Style::default().fg(theme.text_muted()),
            ),
        ]
    }
}

fn with_cursor(value: &str, cursor_index: usize) -> String {
    let chars = value.chars().collect::<Vec<_>>();
    let index = cursor_index.min(chars.len());

    let mut output = String::with_capacity(value.len() + 1);
    for (position, ch) in chars.iter().enumerate() {
        if position == index {
            output.push('|');
        }
        output.push(*ch);
    }

    if index >= chars.len() {
        output.push('|');
    }

    output
}
