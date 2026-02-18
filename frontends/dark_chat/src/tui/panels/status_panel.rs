use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

use dark_tui_components::{KeyBind, PaneBlockComponent, StatusPill, compact_tail, compact_text};

use crate::tui::app::{App, FocusPane};

pub struct StatusPanel;

impl StatusPanel {
    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build("Runtime", app.is_focus(FocusPane::Runtime), theme);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let mut lines = vec![
            health_line(app, theme),
            selection_line(app, theme),
            detail_mode_line(app, theme),
            realtime_line(app, theme),
            service_line("lsp", app.runtime_status().lsp.as_slice(), theme),
            service_line("fmt", app.runtime_status().formatter.as_slice(), theme),
            service_line("mcp", app.runtime_status().mcp.as_slice(), theme),
            config_line(app, theme),
            directory_line(app, theme),
            Line::raw(""),
        ];

        if app.show_help() {
            lines.push(Line::from(vec![
                StatusPill::muted("keys", theme).span_compact(),
                Span::raw(" "),
                Span::styled("navigation", Style::default().fg(theme.text_secondary)),
            ]));
            lines.push(help_line_styled("j/k", "sessions or scroll focus", theme));
            lines.push(help_line_styled("n", "new session", theme));
            lines.push(help_line_styled("a/m", "cycle agent/model", theme));
            lines.push(help_line_styled("c", "open composer", theme));
            lines.push(help_line_styled("v", "open/close detail popup", theme));
            lines.push(help_line_styled("Enter", "send prompt", theme));
            lines.push(help_line_styled("z", "toggle detail expansion", theme));
            lines.push(help_line_styled("h", "toggle help", theme));
        } else {
            lines.push(Line::from(vec![
                StatusPill::muted("help", theme).span_compact(),
                Span::raw(" "),
                Span::styled("press h", Style::default().fg(theme.text_muted)),
            ]));
        }

        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((app.runtime_scroll_lines(), 0));
        frame.render_widget(paragraph, inner);
    }
}

fn health_line(app: &App, theme: &dark_tui_components::ComponentTheme) -> Line<'static> {
    let health = if app.health().healthy {
        StatusPill::ok("healthy", theme)
    } else {
        StatusPill::error("unhealthy", theme)
    };

    let version = app.health().version.as_deref().unwrap_or("-");
    Line::from(vec![
        health.span_compact(),
        Span::raw(" "),
        StatusPill::muted(format!("v:{version}"), theme).span_compact(),
    ])
}

fn realtime_line(app: &App, theme: &dark_tui_components::ComponentTheme) -> Line<'static> {
    let realtime = if !app.realtime_supported() {
        StatusPill::muted("realtime:off", theme)
    } else if app.realtime_connected() {
        StatusPill::ok("realtime:on", theme)
    } else {
        StatusPill::warn("realtime:down", theme)
    };

    let mut spans = vec![
        realtime.span_compact(),
        Span::raw(" "),
        StatusPill::muted(format!("events:{}", app.realtime_event_count()), theme).span_compact(),
    ];

    if let Some(last_event) = app.realtime_last_event() {
        spans.push(Span::raw(" "));
        spans.push(StatusPill::info(compact_text(last_event, 22), theme).span_compact());
    }

    Line::from(spans)
}

fn detail_mode_line(app: &App, theme: &dark_tui_components::ComponentTheme) -> Line<'static> {
    let mode = if app.message_detail_expanded() {
        StatusPill::info("details:expanded", theme)
    } else {
        StatusPill::muted("details:compact", theme)
    };

    Line::from(vec![
        mode.span_compact(),
        Span::raw(" "),
        Span::styled(
            "tool and shell blocks",
            Style::default().fg(theme.text_muted),
        ),
    ])
}

fn selection_line(app: &App, theme: &dark_tui_components::ComponentTheme) -> Line<'static> {
    Line::from(vec![
        StatusPill::accent("model", theme).span_compact(),
        Span::raw(" "),
        StatusPill::info(compact_text(app.active_model().unwrap_or("-"), 24), theme).span_compact(),
        Span::raw(" "),
        StatusPill::accent("agent", theme).span_compact(),
        Span::raw(" "),
        StatusPill::muted(compact_text(app.active_agent().unwrap_or("-"), 14), theme)
            .span_compact(),
    ])
}

fn service_line(
    label: &str,
    entries: &[String],
    theme: &dark_tui_components::ComponentTheme,
) -> Line<'static> {
    let mut spans = vec![
        StatusPill::accent(label, theme).span_compact(),
        Span::raw(" "),
    ];

    if entries.is_empty() {
        spans.push(StatusPill::muted("none", theme).span_compact());
        return Line::from(spans);
    }

    for (index, entry) in entries.iter().take(2).enumerate() {
        if index > 0 {
            spans.push(Span::raw(" "));
        }
        spans.push(StatusPill::info(compact_text(entry, 18), theme).span_compact());
    }

    if entries.len() > 2 {
        spans.push(Span::raw(" "));
        spans.push(StatusPill::muted(format!("+{}", entries.len() - 2), theme).span_compact());
    }

    Line::from(spans)
}

fn directory_line(app: &App, theme: &dark_tui_components::ComponentTheme) -> Line<'static> {
    Line::from(vec![
        StatusPill::muted("dir", theme).span_compact(),
        Span::raw(" "),
        Span::styled(
            compact_tail(app.directory(), 34),
            Style::default().fg(theme.text_secondary),
        ),
    ])
}

fn config_line(app: &App, theme: &dark_tui_components::ComponentTheme) -> Line<'static> {
    let config_path = app
        .runtime_status()
        .config_path
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("-");

    Line::from(vec![
        StatusPill::muted("config", theme).span_compact(),
        Span::raw(" "),
        Span::styled(
            compact_tail(config_path, 34),
            Style::default().fg(theme.text_secondary),
        ),
    ])
}

fn help_line_styled(
    key: &'static str,
    action: &'static str,
    theme: &dark_tui_components::ComponentTheme,
) -> Line<'static> {
    Line::from(KeyBind::new(key, action).spans(theme))
}
