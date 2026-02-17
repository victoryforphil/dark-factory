use ratatui::style::Style;
use ratatui::text::{Line, Span};

use crate::models::SubAgentRow;
use crate::theme::Theme;

const TOKEN_CELL_WIDTH: usize = 14;

pub(crate) fn sub_agent_token_rows(
    agents: &[SubAgentRow],
    content_width: u16,
    max_rows: usize,
    theme: &Theme,
) -> Vec<Line<'static>> {
    if agents.is_empty() || content_width < 8 {
        return Vec::new();
    }

    let cols = ((content_width as usize).max(TOKEN_CELL_WIDTH) / TOKEN_CELL_WIDTH).max(1);
    let max_rows = max_rows.max(1);
    let visible_capacity = cols * max_rows;
    let visible_count = agents.len().min(visible_capacity);

    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut index = 0usize;
    while index < visible_count {
        let end = (index + cols).min(visible_count);
        let mut spans: Vec<Span<'static>> = Vec::new();

        for (cell_idx, agent) in agents[index..end].iter().enumerate() {
            if cell_idx > 0 {
                spans.push(Span::raw(" "));
            }

            let title_budget = TOKEN_CELL_WIDTH.saturating_sub(3);
            let title = compact_token_title(&agent.title, title_budget);
            let prefix = if agent.depth > 0 { ">" } else { "" };
            let label = format!("⚙ {prefix}{title}");
            let padded = format!("{label:<width$}", width = TOKEN_CELL_WIDTH);
            spans.push(Span::styled(padded, token_style(&agent.status, theme)));
        }

        lines.push(Line::from(spans));
        index = end;
    }

    let hidden = agents.len().saturating_sub(visible_count);
    if hidden > 0 {
        lines.push(Line::from(vec![Span::styled(
            format!("+{hidden} more"),
            Style::default().fg(theme.text_muted),
        )]));
    }

    lines
}

fn token_style(status: &str, theme: &Theme) -> Style {
    match status {
        "active" | "running" => Style::default().fg(theme.pill_ok_fg),
        "error" | "failed" | "dead" => Style::default().fg(theme.pill_err_fg),
        "idle" | "waiting" => Style::default().fg(theme.pill_warn_fg),
        _ => Style::default().fg(theme.text_muted),
    }
}

fn compact_token_title(value: &str, max_len: usize) -> String {
    if max_len == 0 {
        return String::new();
    }

    let mut out = String::new();
    for ch in value.chars() {
        if out.chars().count() >= max_len {
            break;
        }
        out.push(ch);
    }

    if value.chars().count() > max_len {
        if max_len == 1 {
            return "~".to_string();
        }
        let keep = max_len.saturating_sub(1);
        let mut trimmed = String::new();
        for ch in out.chars().take(keep) {
            trimmed.push(ch);
        }
        format!("{trimmed}~")
    } else {
        out
    }
}
