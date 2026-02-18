use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};

use crate::app::App;

use dark_tui_components::PaneBlockComponent;

pub(crate) struct InitProductFormPanel;

impl InitProductFormPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let popup = centered_rect(area, 76, 38);

        frame.render_widget(Clear, popup);

        let block = PaneBlockComponent::build("Init Product", true, theme);
        let inner = block.inner(popup);
        frame.render_widget(block, popup);

        let directory = app.init_product_form_directory().unwrap_or_default();
        let display = if directory.is_empty() { "_" } else { directory };

        let lines = vec![
            Line::from(Span::styled(
                "Directory (editable):",
                Style::default().fg(theme.text_muted),
            )),
            Line::from(Span::styled(
                format!("> {display}"),
                Style::default().fg(theme.entity_product),
            )),
            Line::raw(""),
            Line::from(Span::styled(
                "Defaults to current working directory.",
                Style::default().fg(theme.text_muted),
            )),
            Line::from(Span::styled(
                "Enter: initialize   Esc: cancel   Backspace: edit",
                Style::default().fg(theme.text_muted),
            )),
        ];

        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
    }
}

fn centered_rect(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height_percent) / 2),
            Constraint::Percentage(height_percent),
            Constraint::Percentage((100 - height_percent) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_percent) / 2),
            Constraint::Percentage(width_percent),
            Constraint::Percentage((100 - width_percent) / 2),
        ])
        .split(vertical[1])[1]
}
