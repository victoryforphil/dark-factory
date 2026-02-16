use std::collections::{HashMap, HashSet};

use ratatui::Frame;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use dark_tui_components::{PaneBlockComponent, StatusPill};

use crate::core::ChatSession;
use crate::tui::app::{App, FocusPane};

pub struct SessionsPanel;

struct SessionTreeRow {
    session_index: usize,
    depth: usize,
    ancestors_have_next: Vec<bool>,
    has_next_sibling: bool,
    child_count: usize,
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

        let rows = session_tree_rows(app.sessions());
        if rows.is_empty() {
            return;
        }

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
            let session = &app.sessions()[row.session_index];
            let selected = row.session_index == app.selected_session_index();
            lines.push(session_tree_line(session, row, selected, theme));
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
            let visible_end = window_end;
            let hint = Paragraph::new(format!(
                "showing {}-{} of {}",
                window_start + 1,
                visible_end,
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

        let rows = session_tree_rows(app.sessions());
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
        rows.get(row_index).map(|entry| entry.session_index)
    }
}

fn session_tree_rows(sessions: &[ChatSession]) -> Vec<SessionTreeRow> {
    if sessions.is_empty() {
        return Vec::new();
    }

    let mut index_by_id = HashMap::new();
    for (index, session) in sessions.iter().enumerate() {
        index_by_id.insert(session.id.clone(), index);
    }

    let mut children_by_parent: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut roots = Vec::new();
    for (index, session) in sessions.iter().enumerate() {
        let parent_index = session
            .parent_id
            .as_deref()
            .and_then(|id| index_by_id.get(id).copied())
            .filter(|parent| *parent != index);

        if let Some(parent) = parent_index {
            children_by_parent.entry(parent).or_default().push(index);
        } else {
            roots.push(index);
        }
    }

    let mut rows = Vec::with_capacity(sessions.len());
    let mut visited = HashSet::new();

    for (position, root) in roots.iter().copied().enumerate() {
        let has_next_sibling = position + 1 < roots.len();
        walk_session_tree(
            root,
            0,
            &[],
            has_next_sibling,
            &children_by_parent,
            &mut visited,
            &mut rows,
        );
    }

    for index in 0..sessions.len() {
        if visited.contains(&index) {
            continue;
        }

        walk_session_tree(
            index,
            0,
            &[],
            false,
            &children_by_parent,
            &mut visited,
            &mut rows,
        );
    }

    rows
}

fn walk_session_tree(
    index: usize,
    depth: usize,
    ancestors_have_next: &[bool],
    has_next_sibling: bool,
    children_by_parent: &HashMap<usize, Vec<usize>>,
    visited: &mut HashSet<usize>,
    rows: &mut Vec<SessionTreeRow>,
) {
    if !visited.insert(index) {
        return;
    }

    let child_count = children_by_parent.get(&index).map(Vec::len).unwrap_or(0);
    rows.push(SessionTreeRow {
        session_index: index,
        depth,
        ancestors_have_next: ancestors_have_next.to_vec(),
        has_next_sibling,
        child_count,
    });

    let Some(children) = children_by_parent.get(&index) else {
        return;
    };

    for (position, child) in children.iter().copied().enumerate() {
        let child_has_next_sibling = position + 1 < children.len();
        let mut child_ancestors = ancestors_have_next.to_vec();
        child_ancestors.push(has_next_sibling);
        walk_session_tree(
            child,
            depth + 1,
            &child_ancestors,
            child_has_next_sibling,
            children_by_parent,
            visited,
            rows,
        );
    }
}

fn session_tree_line<'a>(
    session: &'a ChatSession,
    row: &SessionTreeRow,
    selected: bool,
    theme: &dark_tui_components::ComponentTheme,
) -> Line<'a> {
    let mut spans = Vec::new();
    spans.push(Span::styled(
        tree_prefix(row),
        Style::default().fg(theme.text_muted),
    ));

    spans.push(Span::styled(
        compact_text(&session.title, 24),
        if selected {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text_secondary)
        },
    ));

    if selected {
        spans.push(Span::raw(" "));
        spans.push(StatusPill::accent("active", theme).span_compact());
    }

    if session.parent_id.is_some() {
        spans.push(Span::raw(" "));
        spans.push(StatusPill::muted("subagent", theme).span_compact());
    }

    if row.child_count > 0 {
        spans.push(Span::raw(" "));
        spans.push(StatusPill::muted(format!("threads:{}", row.child_count), theme).span_compact());
    }

    spans.push(Span::raw(" "));
    spans.push(session_status_pill(&session.status, theme).span_compact());
    spans.push(Span::raw(" "));
    spans.push(Span::styled(
        relative_time_label(session.updated_unix),
        Style::default().fg(theme.text_muted),
    ));

    Line::from(spans)
}

fn tree_prefix(row: &SessionTreeRow) -> String {
    let mut prefix = String::new();
    for has_next in &row.ancestors_have_next {
        if *has_next {
            prefix.push_str("|  ");
        } else {
            prefix.push_str("   ");
        }
    }

    if row.depth == 0 {
        prefix.push_str("o ");
        return prefix;
    }

    if row.has_next_sibling {
        prefix.push_str("|- ");
    } else {
        prefix.push_str("\\- ");
    }

    prefix
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

fn compact_text(value: &str, max_width: usize) -> String {
    if value.chars().count() <= max_width {
        return value.to_string();
    }

    let head_len = max_width.saturating_sub(3);
    let head = value.chars().take(head_len).collect::<String>();
    format!("{head}...")
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
