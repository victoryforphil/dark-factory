use ratatui::layout::{Position, Rect, Size};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use tui_scrollview::{ScrollView, ScrollViewState};

use dark_tui_components::{PaneBlockComponent, StatusPill};

use crate::core::ChatSession;
use crate::tui::app::{App, FocusPane};

pub struct SessionsPanel;

const SESSION_CARD_HEIGHT: u16 = 4;
const SESSION_CARD_GAP: u16 = 0;

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

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let visible_count = visible_session_count(inner.height);
        let max_window_start = app.sessions().len().saturating_sub(visible_count);
        let window_start = app.sessions_scroll_index().min(max_window_start);

        let slot_height = SESSION_CARD_HEIGHT + SESSION_CARD_GAP;
        let viewport_height = inner.height.max(1);
        let mut scroll_view = ScrollView::new(Size::new(inner.width.max(1), viewport_height));

        for offset in 0..visible_count {
            let index = window_start + offset;
            let Some(session) = app.sessions().get(index) else {
                break;
            };

            let y = ((offset as u32)
                .saturating_mul(slot_height as u32)
                .min(u16::MAX as u32)) as u16;

            let widget = session_card_widget(session, index == app.selected_session_index(), theme);
            scroll_view.render_widget(
                widget,
                Rect {
                    x: 0,
                    y,
                    width: inner.width,
                    height: SESSION_CARD_HEIGHT,
                },
            );
        }

        let mut state = ScrollViewState::new();
        state.set_offset(Position { x: 0, y: 0 });

        frame.render_stateful_widget(scroll_view, inner, &mut state);

        if app.sessions().len() > visible_count {
            let visible_end = (window_start + visible_count).min(app.sessions().len());
            let hint = Paragraph::new(format!(
                "showing {}-{} of {}",
                window_start + 1,
                visible_end,
                app.sessions().len()
            ))
            .style(Style::default().fg(theme.text_muted));
            let hint_area = Rect {
                x: inner.x,
                y: inner.y.saturating_add(inner.height.saturating_sub(1)),
                width: inner.width,
                height: 1,
            };
            frame.render_widget(hint, hint_area);
        }
    }

    pub fn session_index_at(area: Rect, app: &App, row: u16) -> Option<usize> {
        if app.sessions().is_empty() {
            return None;
        }

        let inner = panel_inner(area);
        if inner.width == 0 || inner.height == 0 {
            return None;
        }

        if row < inner.y || row >= inner.y.saturating_add(inner.height) {
            return None;
        }

        let visible_count = visible_session_count(inner.height);
        let max_window_start = app.sessions().len().saturating_sub(visible_count);
        let window_start = app.sessions_scroll_index().min(max_window_start);

        let local_row = row.saturating_sub(inner.y);
        let slot_height = SESSION_CARD_HEIGHT + SESSION_CARD_GAP;
        let slot_index = (local_row / slot_height) as usize;
        let slot_row = local_row % slot_height;

        if slot_index >= visible_count || slot_row >= SESSION_CARD_HEIGHT {
            return None;
        }

        let index = window_start + slot_index;
        (index < app.sessions().len()).then_some(index)
    }
}

fn session_card_widget(
    session: &ChatSession,
    selected: bool,
    theme: &dark_tui_components::ComponentTheme,
) -> Paragraph<'static> {
    let border_style = if selected {
        Style::default()
            .fg(theme.pane_focused_border)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.pane_unfocused_border)
    };

    let mut heading = vec![Span::styled(
        compact_text(&session.title, 28),
        if selected {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text_secondary)
        },
    )];

    if selected {
        heading.push(Span::raw(" "));
        heading.push(StatusPill::accent("active", theme).span_compact());
    }

    let status_pill = session_status_pill(&session.status, theme);
    let relative = relative_time_label(session.updated_unix);
    let lines = vec![
        Line::from(heading),
        Line::from(vec![
            StatusPill::muted(format!("id:{}", compact_id(&session.id)), theme).span_compact(),
            Span::raw(" "),
            status_pill.span_compact(),
            Span::raw(" "),
            Span::styled(relative, Style::default().fg(theme.text_muted)),
        ]),
    ];

    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .wrap(Wrap { trim: true })
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

fn visible_session_count(height: u16) -> usize {
    if height < SESSION_CARD_HEIGHT {
        return 1;
    }

    let slot_height = SESSION_CARD_HEIGHT + SESSION_CARD_GAP;
    (height.saturating_add(SESSION_CARD_GAP) / slot_height).max(1) as usize
}

fn panel_inner(area: Rect) -> Rect {
    Rect {
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

fn compact_id(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() <= 14 {
        return trimmed.to_string();
    }

    format!("{}...", &trimmed[..14])
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
