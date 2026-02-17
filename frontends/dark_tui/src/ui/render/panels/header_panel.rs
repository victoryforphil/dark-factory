use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;

pub(crate) struct HeaderPanel;

impl HeaderPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();

        // Compact single-line title with brand + view mode pill.
        let brand = Span::styled(
            " Dark Factory ",
            Style::default()
                .fg(theme.header_border)
                .add_modifier(Modifier::BOLD),
        );

        let line = Line::from(vec![brand]);

        // Second line: thin horizontal rule for visual separation.
        let rule_len = area.width as usize;
        let rule = Line::styled(
            "\u{2500}".repeat(rule_len),
            Style::default().fg(theme.pane_unfocused_border),
        );

        let widget = Paragraph::new(vec![line, rule]);
        frame.render_widget(widget, area);
    }
}
