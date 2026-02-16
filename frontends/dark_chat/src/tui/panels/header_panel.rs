use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use dark_tui_components::PaneBlockComponent;

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
            Span::styled(
                format!("  server:{}", app.base_url()),
                Style::default().fg(theme.text_secondary),
            ),
            Span::styled(
                format!("  model:{}", compact_label(app.active_model(), 24)),
                Style::default().fg(theme.text_secondary),
            ),
            Span::styled(
                format!("  agent:{}", compact_label(app.active_agent(), 20)),
                Style::default().fg(theme.text_secondary),
            ),
        ])];

        frame.render_widget(Paragraph::new(lines), inner);
    }
}

fn compact_label(value: Option<&str>, max_len: usize) -> String {
    let Some(value) = value else {
        return "-".to_string();
    };

    if value.len() <= max_len {
        return value.to_string();
    }

    if max_len <= 3 {
        return ".".repeat(max_len);
    }

    format!("{}...", &value[..max_len - 3])
}
