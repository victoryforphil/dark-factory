use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

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

    for (index, (title, _status)) in entries.iter().take(draw_count).enumerate() {
        let y = area.y + index as u16;
        if y >= area.y + area.height {
            break;
        }

        let label_width = area.width.saturating_sub(6) as usize;
        let label = compact_cell_text(title, label_width);
        let padded_label = format!(" {label:<width$} ", width = label_width);
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
