use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

use super::super::components::StatusPill;

pub(crate) struct FooterPanel;

impl FooterPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();

        let focus_pill = StatusPill::accent(app.focus().label(), theme);
        let view_pill = StatusPill::info(app.results_view_mode().label(), theme);

        let filter_pill = if app.filter_variants_to_product() {
            StatusPill::warn("filtered", theme)
        } else {
            StatusPill::muted("all", theme)
        };

        let status_text = app.status_message();
        let status_span = if status_text.contains("failed") || status_text.contains("error") {
            Span::styled(
                status_text.to_string(),
                Style::default().fg(theme.text_error),
            )
        } else {
            Span::styled(
                status_text.to_string(),
                Style::default().fg(theme.text_status_normal),
            )
        };

        let line = Line::from(vec![
            focus_pill.span(),
            Span::raw(" "),
            view_pill.span(),
            Span::raw(" "),
            filter_pill.span(),
            Span::raw("  "),
            status_span,
        ]);

        let footer = Paragraph::new(line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.footer_border))
                .title("Status"),
        );

        frame.render_widget(footer, area);
    }
}
