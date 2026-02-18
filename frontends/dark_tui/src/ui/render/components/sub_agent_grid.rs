use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use throbber_widgets_tui::BLACK_CIRCLE;

use crate::theme::Theme;

const SUB_AGENT_MAX_ITEMS: usize = 6;

fn clamped_count(total: usize) -> usize {
    total.min(SUB_AGENT_MAX_ITEMS)
}

pub(crate) fn sub_agent_grid_container_height(total: usize, _area_width: u16) -> u16 {
    clamped_count(total) as u16
}

pub(crate) fn render_sub_agent_grid(
    frame: &mut Frame,
    area: Rect,
    entries: &[(&str, &str)],
    _total_count: usize,
    theme: &Theme,
) {
    if area.width < 8 || area.height == 0 || entries.is_empty() {
        return;
    }

    let draw_count = clamped_count(entries.len());
    let border_style = Style::default().fg(theme.text_muted);
    let text_style = Style::default().fg(theme.text_secondary);

    let frame_index = throbber_frame_index();

    for (index, (title, status)) in entries.iter().take(draw_count).enumerate() {
        let y = area.y + index as u16;
        if y >= area.y + area.height {
            break;
        }

        let marker = sub_agent_marker(status, frame_index);
        let label_width = area.width.saturating_sub(8) as usize;
        let label = compact_cell_text(title, label_width);
        let padded_label = format!(" {marker} {label:<width$} ", width = label_width);
        let line = Line::from(vec![
            Span::styled("\u{2502}", border_style),
            Span::styled(padded_label, text_style),
            Span::styled("\u{2502}", border_style),
        ]);

        frame.render_widget(
            Paragraph::new(line),
            Rect {
                x: area.x + 1,
                y,
                width: area.width.saturating_sub(2),
                height: 1,
            },
        );
    }
}

fn sub_agent_marker(status: &str, frame_index: usize) -> &'static str {
    match status {
        "active" | "running" | "busy" | "working" | "processing" => {
            BLACK_CIRCLE.symbols[frame_index % BLACK_CIRCLE.symbols.len()]
        }
        "idle" | "waiting" => BLACK_CIRCLE.symbols[0],
        "error" | "failed" | "dead" => BLACK_CIRCLE.symbols[1],
        _ => "o",
    }
}

fn throbber_frame_index() -> usize {
    const FRAME_MS: u128 = 120;
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis();
    (elapsed / FRAME_MS) as usize
}

fn compact_cell_text(value: &str, max_len: usize) -> String {
    if max_len == 0 {
        return String::new();
    }

    let chars: Vec<char> = value.chars().collect();
    if chars.len() <= max_len {
        chars.into_iter().collect()
    } else if max_len == 1 {
        "~".to_string()
    } else {
        let mut out: String = chars[..max_len - 1].iter().collect();
        out.push('~');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{BLACK_CIRCLE, sub_agent_marker};

    #[test]
    fn marker_uses_solid_dot_for_active_rows() {
        assert_eq!(sub_agent_marker("active", 0), BLACK_CIRCLE.symbols[0]);
        assert_eq!(sub_agent_marker("running", 1), BLACK_CIRCLE.symbols[1]);
        assert_eq!(sub_agent_marker("busy", 2), BLACK_CIRCLE.symbols[2]);
    }

    #[test]
    fn marker_falls_back_to_tree_dot_for_unknown_status() {
        assert_eq!(sub_agent_marker("unknown", 0), "o");
        assert_eq!(sub_agent_marker("", 3), "o");
    }
}
