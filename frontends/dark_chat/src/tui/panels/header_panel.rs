use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use dark_tui_components::{PaneBlockComponent, StatusPill, compact_label};

use crate::tui::app::App;

pub struct HeaderPanel;

impl HeaderPanel {
    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build("Dark Chat", true, theme);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let lines = vec![Line::from(vec![
            Span::styled(
                format!("provider:{}", app.provider_name()),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            StatusPill::accent(
                format!("model:{}", compact_label(app.active_model(), 24)),
                theme,
            )
            .span(),
            Span::raw("  "),
            StatusPill::info(
                format!("agent:{}", compact_label(app.active_agent(), 20)),
                theme,
            )
            .span(),
            Span::raw("  "),
            Span::styled(
                format!("server:{}", app.base_url()),
                Style::default().fg(theme.text_muted),
            ),
        ])];

        frame.render_widget(Paragraph::new(lines), inner);
    }
}
