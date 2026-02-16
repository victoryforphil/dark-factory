use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use dark_tui_components::{PaneBlockComponent, StatusPill};

use crate::tui::app::App;

pub struct FooterPanel;

impl FooterPanel {
    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build("Status", false, theme);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let activity_pill = if app.activity_label() == "idle" {
            StatusPill::muted("idle", theme)
        } else {
            StatusPill::info(app.activity_label(), theme)
        };

        let line = Line::from(vec![
            Span::styled(
                app.status_message().to_string(),
                Style::default().fg(theme.text_secondary),
            ),
            Span::raw("  "),
            activity_pill.span(),
            Span::raw("  "),
            Span::styled(
                format!("last-sync:{}", app.last_synced()),
                Style::default().fg(theme.text_muted),
            ),
        ]);

        frame.render_widget(Paragraph::new(vec![line]), inner);
    }
}
