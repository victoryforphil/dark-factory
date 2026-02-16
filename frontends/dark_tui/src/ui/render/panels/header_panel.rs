use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;

pub(crate) struct HeaderPanel;

impl HeaderPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let title = format!(
            "Dark Factory // dark_tui  products={} variants={} actors={}  runtime={}",
            app.products().len(),
            app.variants().len(),
            app.actors().len(),
            app.runtime_status()
        );

        let header = Paragraph::new(title)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default()
                            .fg(theme.header_border)
                            .add_modifier(Modifier::BOLD),
                    )
                    .title("Overview"),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(header, area);
    }
}
