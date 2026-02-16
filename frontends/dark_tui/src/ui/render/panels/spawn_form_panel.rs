use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;

use super::super::components::PaneBlockComponent;

pub(crate) struct SpawnFormPanel;

impl SpawnFormPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let popup = centered_rect(area, 72, 58);

        frame.render_widget(Clear, popup);

        let block = PaneBlockComponent::build("Spawn in TUI", true, theme);
        let inner = block.inner(popup);
        frame.render_widget(block, popup);

        let providers = app.spawn_form_providers().unwrap_or(&[]);
        let selected_provider = app.spawn_form_selected_provider_index().unwrap_or(0);
        let prompt = app.spawn_form_prompt().unwrap_or_default();

        let mut lines = vec![Line::from(Span::styled(
            "Provider (j/k or arrows):",
            Style::default().fg(theme.text_muted),
        ))];

        for (index, provider) in providers.iter().enumerate() {
            let marker = if index == selected_provider { ">" } else { " " };
            let style = if index == selected_provider {
                Style::default().fg(theme.entity_actor)
            } else {
                Style::default().fg(theme.text_primary)
            };
            lines.push(Line::from(Span::styled(
                format!("  {marker} {provider}"),
                style,
            )));
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Initial prompt:",
            Style::default().fg(theme.text_muted),
        )));

        let prompt_line = if prompt.is_empty() {
            Span::styled("> _", Style::default().fg(theme.text_muted))
        } else {
            Span::styled(
                format!("> {prompt}_"),
                Style::default().fg(theme.text_primary),
            )
        };
        lines.push(Line::from(prompt_line));
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Enter: spawn   Esc: cancel   Backspace: edit",
            Style::default().fg(theme.text_muted),
        )));

        let content = Paragraph::new(lines).wrap(Wrap { trim: false });
        frame.render_widget(content, inner);
    }
}

fn centered_rect(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height_percent) / 2),
            Constraint::Percentage(height_percent),
            Constraint::Percentage((100 - height_percent) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_percent) / 2),
            Constraint::Percentage(width_percent),
            Constraint::Percentage((100 - width_percent) / 2),
        ])
        .split(vertical[1])[1]
}
