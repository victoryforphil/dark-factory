use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

use dark_tui_components::PaneBlockComponent;

use crate::tui::app::{App, FocusPane};

pub struct SessionsPanel;

impl SessionsPanel {
    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build("Sessions", app.is_focus(FocusPane::Sessions), theme);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if app.sessions().is_empty() {
            frame.render_widget(
                Paragraph::new(vec![Line::styled(
                    "No sessions. Press n to create one.",
                    Style::default().fg(theme.text_muted),
                )]),
                inner,
            );
            return;
        }

        let mut lines = Vec::new();
        for (index, session) in app.sessions().iter().enumerate() {
            let active = index == app.selected_session_index();
            let marker = if active { ">" } else { " " };

            let status_color = match session.status.as_str() {
                "idle" | "ready" => theme.pill_ok_fg,
                "busy" | "running" => theme.pill_info_fg,
                "retry" | "retrying" => theme.pill_warn_fg,
                _ => theme.text_secondary,
            };

            lines.push(Line::from(vec![Span::styled(
                format!("{marker} {}", session.title),
                if active {
                    Style::default().add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.text_secondary)
                },
            )]));
            lines.push(Line::from(vec![
                Span::styled("  id:", Style::default().fg(theme.text_muted)),
                Span::styled(
                    compact_id(&session.id),
                    Style::default().fg(theme.text_secondary),
                ),
                Span::raw("  "),
                Span::styled("status:", Style::default().fg(theme.text_muted)),
                Span::styled(session.status.clone(), Style::default().fg(status_color)),
            ]));
            lines.push(Line::raw(""));
        }

        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
    }
}

fn compact_id(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() <= 14 {
        return trimmed.to_string();
    }

    format!("{}...", &trimmed[..14])
}
