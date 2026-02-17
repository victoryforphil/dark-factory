use std::collections::HashMap;

use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use dark_tui_components::{compact_text, PaneBlockComponent, StatusPill};

use crate::core::ChatSession;
use crate::framework::{tree_prefix, walk_session_tree, SessionLike, SessionTreeRow};
use crate::tui::app::{App, FocusPane};

pub struct SessionsPanel;

impl SessionLike for ChatSession {
    fn id(&self) -> &str {
        &self.id
    }

    fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn status(&self) -> &str {
        &self.status
    }

    fn created_at(&self) -> Option<&str> {
        self.updated_at.as_deref()
    }
}

impl SessionsPanel {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
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

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let rows = walk_session_tree(app.sessions(), app.active_session_id());
        if rows.is_empty() {
            return;
        }

        let by_id = index_by_session_id(app.sessions());
        let show_hint = rows.len() > inner.height as usize;
        let body_height = if show_hint {
            inner.height.saturating_sub(1).max(1)
        } else {
            inner.height
        };

        let visible_count = body_height as usize;
        let max_window_start = rows.len().saturating_sub(visible_count);
        let window_start = app.sessions_scroll_index().min(max_window_start);
        let window_end = (window_start + visible_count).min(rows.len());

        let mut lines = Vec::with_capacity(visible_count);
        for row in &rows[window_start..window_end] {
            let updated_unix = by_id
                .get(row.session_id.as_str())
                .and_then(|index| app.sessions().get(*index))
                .and_then(|session| session.updated_unix);
            lines.push(session_tree_line(row, updated_unix, theme));
        }

        while lines.len() < visible_count {
            lines.push(Line::from(""));
        }

        let body_area = ratatui::layout::Rect {
            x: inner.x,
            y: inner.y,
            width: inner.width,
            height: body_height,
        };
        frame.render_widget(Paragraph::new(lines), body_area);

        if show_hint {
            let hint = Paragraph::new(format!(
                "showing {}-{} of {}",
                window_start + 1,
                window_end,
                rows.len()
            ))
            .style(Style::default().fg(theme.text_muted));
            let hint_area = ratatui::layout::Rect {
                x: inner.x,
                y: inner.y.saturating_add(inner.height.saturating_sub(1)),
                width: inner.width,
                height: 1,
            };
            frame.render_widget(hint, hint_area);
        }
    }

    pub fn session_index_at(area: ratatui::layout::Rect, app: &App, row: u16) -> Option<usize> {
        if app.sessions().is_empty() {
            return None;
        }

        let inner = panel_inner(area);
        if inner.width == 0 || inner.height == 0 {
            return None;
        }

        let rows = walk_session_tree(app.sessions(), app.active_session_id());
        if rows.is_empty() {
            return None;
        }

        let show_hint = rows.len() > inner.height as usize;
        let body_height = if show_hint {
            inner.height.saturating_sub(1).max(1)
        } else {
            inner.height
        };
        let body_bottom = inner.y.saturating_add(body_height);

        if row < inner.y || row >= body_bottom {
            return None;
        }

        let visible_count = body_height as usize;
        let max_window_start = rows.len().saturating_sub(visible_count);
        let window_start = app.sessions_scroll_index().min(max_window_start);

        let local_row = row.saturating_sub(inner.y) as usize;
        let row_index = window_start + local_row;
        let selected = rows.get(row_index)?;
        index_by_session_id(app.sessions())
            .get(selected.session_id.as_str())
            .copied()
    }
}

fn session_tree_line(
    row: &SessionTreeRow,
    updated_unix: Option<i64>,
    theme: &dark_tui_components::ComponentTheme,
) -> Line<'static> {
    let mut spans = Vec::new();
    spans.push(Span::styled(
        tree_prefix(row.depth, row.is_last, &row.ancestors_are_last),
        Style::default().fg(theme.text_muted),
    ));

    spans.push(Span::styled(
        compact_text(&row.title, 24),
        if row.is_active {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text_secondary)
        },
    ));

    if row.is_active {
        spans.push(Span::raw(" "));
        spans.push(StatusPill::accent("active", theme).span_compact());
    }

    if row.depth > 0 {
        spans.push(Span::raw(" "));
        spans.push(StatusPill::muted("subagent", theme).span_compact());
    }

    if row.child_count > 0 {
        spans.push(Span::raw(" "));
        spans.push(StatusPill::muted(format!("threads:{}", row.child_count), theme).span_compact());
    }

    spans.push(Span::raw(" "));
    spans.push(session_status_pill(&row.status, theme).span_compact());
    spans.push(Span::raw(" "));
    spans.push(Span::styled(
        relative_time_label(updated_unix),
        Style::default().fg(theme.text_muted),
    ));

    Line::from(spans)
}

fn session_status_pill(
    status: &str,
    theme: &dark_tui_components::ComponentTheme,
) -> dark_tui_components::StatusPill {
    match status.trim().to_ascii_lowercase().as_str() {
        "idle" | "ready" => StatusPill::ok(status, theme),
        "busy" | "running" => StatusPill::info(status, theme),
        "retry" | "retrying" => StatusPill::warn(status, theme),
        "error" | "failed" => StatusPill::error(status, theme),
        _ => StatusPill::muted(status, theme),
    }
}

fn panel_inner(area: ratatui::layout::Rect) -> ratatui::layout::Rect {
    ratatui::layout::Rect {
        x: area.x.saturating_add(1),
        y: area.y.saturating_add(1),
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    }
}

fn index_by_session_id(sessions: &[ChatSession]) -> HashMap<&str, usize> {
    sessions
        .iter()
        .enumerate()
        .map(|(index, session)| (session.id.as_str(), index))
        .collect()
}

fn relative_time_label(updated_unix: Option<i64>) -> String {
    let Some(updated) = updated_unix.filter(|value| *value > 0) else {
        return "-".to_string();
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let delta = now.saturating_sub(updated).max(0);

    if delta < 10 {
        return "just now".to_string();
    }
    if delta < 60 {
        return format!("{}s ago", delta);
    }
    if delta < 3600 {
        return format!("{}m ago", delta / 60);
    }
    if delta < 86_400 {
        return format!("{}h ago", delta / 3600);
    }
    if delta < 86_400 * 30 {
        return format!("{}d ago", delta / 86_400);
    }

    format!("{}mo ago", delta / (86_400 * 30))
}
