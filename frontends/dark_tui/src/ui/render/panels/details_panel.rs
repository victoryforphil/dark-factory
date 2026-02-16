use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;

use super::super::components::PaneBlockComponent;

pub(crate) struct DetailsPanel;

impl DetailsPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let lines = app
            .detail_lines()
            .into_iter()
            .map(Line::from)
            .collect::<Vec<_>>();

        let details = Paragraph::new(lines)
            .block(PaneBlockComponent::build("Details", false, theme))
            .wrap(Wrap { trim: false });

        frame.render_widget(details, area);
    }
}
