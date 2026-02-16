use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use dark_tui_components::PaneBlockComponent;

use crate::tui::app::{App, FocusPane};

pub struct StatusPanel;

impl StatusPanel {
    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build("Runtime", app.is_focus(FocusPane::Runtime), theme);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let health = if app.health().healthy {
            "healthy"
        } else {
            "unhealthy"
        };

        let mut lines = vec![
            Line::from(format!("health: {health}")),
            Line::from(format!(
                "version: {}",
                app.health().version.as_deref().unwrap_or("-")
            )),
            Line::from(format!(
                "realtime: {} ({})",
                if app.realtime_supported() {
                    if app.realtime_connected() {
                        "connected"
                    } else {
                        "disconnected"
                    }
                } else {
                    "disabled"
                },
                app.realtime_event_count()
            )),
            Line::from(format!(
                "last-event: {}",
                app.realtime_last_event().unwrap_or("-")
            )),
            Line::from(format!(
                "lsp: {}",
                join_statuses(app.runtime_status().lsp.as_slice(), 2)
            )),
            Line::from(format!(
                "formatter: {}",
                join_statuses(app.runtime_status().formatter.as_slice(), 2)
            )),
            Line::from(format!(
                "mcp: {}",
                join_statuses(app.runtime_status().mcp.as_slice(), 2)
            )),
            Line::from(format!(
                "directory: {}",
                compact_locator(app.directory(), 34)
            )),
            Line::from(""),
        ];

        if app.show_help() {
            lines.extend(vec![
                Line::styled("help", Style::default().fg(theme.text_secondary)),
                Line::styled(
                    "- j/k: sessions or scroll focus",
                    Style::default().fg(theme.text_muted),
                ),
                Line::styled("- n: new session", Style::default().fg(theme.text_muted)),
                Line::styled(
                    "- a/m: cycle agent/model",
                    Style::default().fg(theme.text_muted),
                ),
                Line::styled("- c: compose mode", Style::default().fg(theme.text_muted)),
                Line::styled(
                    "- Enter send, Shift+Enter newline",
                    Style::default().fg(theme.text_muted),
                ),
                Line::styled(
                    "- /help /refresh /agent /model",
                    Style::default().fg(theme.text_muted),
                ),
                Line::styled(
                    "- /grep <pattern> and @file refs",
                    Style::default().fg(theme.text_muted),
                ),
                Line::styled(
                    "- h: toggle this panel",
                    Style::default().fg(theme.text_muted),
                ),
            ]);
        } else {
            lines.push(Line::styled(
                "Press h to show help.",
                Style::default().fg(theme.text_muted),
            ));
        }

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((app.runtime_scroll_lines(), 0));
        frame.render_widget(paragraph, inner);
    }
}

fn compact_locator(value: &str, max_len: usize) -> String {
    let trimmed = value.trim();
    if trimmed.len() <= max_len {
        return trimmed.to_string();
    }

    if max_len <= 3 {
        return ".".repeat(max_len);
    }

    let suffix_len = max_len.saturating_sub(3);
    format!("...{}", &trimmed[trimmed.len() - suffix_len..])
}

fn join_statuses(entries: &[String], max_count: usize) -> String {
    if entries.is_empty() {
        return "-".to_string();
    }

    let shown = entries.iter().take(max_count).cloned().collect::<Vec<_>>();
    let mut joined = shown.join(", ");

    if entries.len() > max_count {
        joined.push_str(&format!(" +{}", entries.len() - max_count));
    }

    joined
}
