use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::theme::Theme;

use super::PaneBlockComponent;

pub(crate) struct CardGridComponent;

impl CardGridComponent {
    pub(crate) fn render(
        frame: &mut Frame,
        area: Rect,
        title: &str,
        focused: bool,
        selected_index: usize,
        cards: &[(String, Vec<String>)],
        selected_color: Color,
        theme: &Theme,
    ) {
        let panel = PaneBlockComponent::build(title, focused, theme);
        let inner = panel.inner(area);
        frame.render_widget(panel, area);

        if inner.width < 12 || inner.height < 4 {
            return;
        }

        if cards.is_empty() {
            let empty = Paragraph::new("No rows")
                .style(Style::default().fg(theme.text_muted))
                .wrap(Wrap { trim: true });
            frame.render_widget(empty, inner);
            return;
        }

        let max_columns = Self::card_columns(inner.width);
        let card_height = Self::card_height(inner.height);
        let max_rows = (inner.height / card_height).max(1);
        let capacity = ((max_rows as usize) * max_columns).max(1);
        let page_start = selected_index.saturating_div(capacity) * capacity;
        let render_start = page_start.min(cards.len().saturating_sub(1));
        let render_count = (cards.len() - render_start).min(capacity);
        let used_rows = ((render_count + max_columns - 1) / max_columns).max(1);

        let row_constraints = vec![Constraint::Length(card_height); used_rows];
        let row_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(inner);

        for (row_index, row_area) in row_areas.iter().enumerate() {
            let row_start = row_index * max_columns;
            if row_start >= render_count {
                break;
            }

            let row_end = (row_start + max_columns).min(render_count);
            let row_len = row_end - row_start;
            let columns: Vec<Rect> = if row_len == 1 {
                let centered = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(25),
                        Constraint::Percentage(50),
                        Constraint::Percentage(25),
                    ])
                    .split(*row_area);
                vec![centered[1]]
            } else {
                let constraints = vec![Constraint::Ratio(1, row_len as u32); row_len];
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(constraints)
                    .split(*row_area)
                    .to_vec()
            };

            for card_offset in 0..row_len {
                let card_index = render_start + row_start + card_offset;
                let (heading, lines) = &cards[card_index];
                let is_selected = card_index == selected_index;

                let border_style = if is_selected {
                    Style::default()
                        .fg(selected_color)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.pane_unfocused_border)
                };

                let text_style = if is_selected {
                    Style::default().add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let mut content = Vec::with_capacity(lines.len() + 1);
                content.push(Line::from(heading.clone()).style(text_style));
                content.extend(lines.iter().map(|line| Line::from(line.as_str())));

                let card = Paragraph::new(content)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(border_style)
                            .title(if is_selected { "selected" } else { "card" }),
                    )
                    .wrap(Wrap { trim: true });

                frame.render_widget(card, columns[card_offset]);
            }
        }

        if cards.len() > render_count {
            let range_start = render_start + 1;
            let range_end = render_start + render_count;
            let hint = Paragraph::new(format!(
                "showing {}-{} of {}",
                range_start,
                range_end,
                cards.len()
            ))
            .style(Style::default().fg(theme.text_muted))
            .wrap(Wrap { trim: true });

            let hint_area = Rect {
                x: inner.x,
                y: inner.y.saturating_add(inner.height.saturating_sub(1)),
                width: inner.width,
                height: 1,
            };
            frame.render_widget(hint, hint_area);
        }
    }

    fn card_columns(width: u16) -> usize {
        if width >= 120 {
            3
        } else if width >= 72 {
            2
        } else {
            1
        }
    }

    fn card_height(height: u16) -> u16 {
        if height >= 14 {
            7
        } else {
            6
        }
    }
}
