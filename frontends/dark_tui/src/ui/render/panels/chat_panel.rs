use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::App;
use crate::models::compact_timestamp;

use super::super::components::PaneBlockComponent;

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
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(inner);

        let messages = Self::message_lines(app);
        frame.render_widget(
            Paragraph::new(messages).wrap(Wrap { trim: false }),
            chunks[0],
        );

        let input = Self::input_lines(app);
        frame.render_widget(Paragraph::new(input).wrap(Wrap { trim: false }), chunks[1]);
    }

    fn message_lines(app: &App) -> Vec<Line<'static>> {
        let theme = app.theme();

        let Some(actor) = app.chat_actor() else {
            return vec![
                Line::styled(
                    "Select an actor node to open chat.",
                    Style::default().fg(theme.text_muted),
                ),
                Line::styled(
                    "Use j/k to move in the catalog tree.",
                    Style::default().fg(theme.text_muted),
                ),
            ];
        };

        let mut lines = vec![Line::from(vec![
            Span::styled("Actor: ", Style::default().fg(theme.text_muted)),
            Span::styled(
                actor.title.clone(),
                Style::default()
                    .fg(theme.entity_actor)
                    .add_modifier(Modifier::BOLD),
            ),
        ])];

        let messages = app.chat_messages();
        if messages.is_empty() {
            lines.push(Line::raw(""));
            lines.push(Line::styled(
                "No chat messages yet.",
                Style::default().fg(theme.text_muted),
            ));
            return lines;
        }

        lines.push(Line::raw(""));

        let recent_count = usize::max(1, messages.len().min(20));
        let start = messages.len().saturating_sub(recent_count);

        for message in &messages[start..] {
            let role_color = match message.role.as_str() {
                "user" => theme.entity_variant,
                "assistant" => theme.entity_actor,
                _ => theme.text_secondary,
            };

            lines.push(Line::from(vec![
                Span::styled(
                    format!("{} ", message.role),
                    Style::default().fg(role_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    compact_timestamp(&message.created_at),
                    Style::default().fg(theme.text_muted),
                ),
            ]));
            lines.push(Line::from(Span::styled(
                message.text.clone(),
                Style::default().fg(theme.text_primary),
            )));
            lines.push(Line::raw(""));
        }

        lines
    }

    fn input_lines(app: &App) -> Vec<Line<'static>> {
        let theme = app.theme();

        if app.chat_actor().is_none() {
            return vec![Line::styled(
                "Input disabled until an actor is selected.",
                Style::default().fg(theme.text_muted),
            )];
        }

        if !app.is_chat_composing() {
            return vec![Line::styled(
                "Press c to compose, Enter to send, t to hide chat.",
                Style::default().fg(theme.text_muted),
            )];
        }

        let draft = app.chat_draft();
        let prompt = if draft.is_empty() {
            "> _".to_string()
        } else {
            format!("> {draft}_")
        };

        vec![Line::styled(
            prompt,
            Style::default().fg(theme.text_primary),
        )]
    }
}
