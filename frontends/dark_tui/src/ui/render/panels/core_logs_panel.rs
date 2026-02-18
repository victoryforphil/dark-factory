use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub(crate) struct CoreLogsPanel;

impl CoreLogsPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let title = format!(
            "Core Logs // {} // {}",
            app.core_logs_session(),
            app.core_logs_status()
        );
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.footer_border));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let mut lines: Vec<Line<'static>> = app
            .core_logs_lines()
            .iter()
            .rev()
            .take(inner.height as usize)
            .cloned()
            .map(Line::from)
            .collect();
        lines.reverse();

        if lines.is_empty() {
            lines.push(Line::styled(
                "(no log lines yet)",
                Style::default().fg(theme.text_muted),
            ));
        }

        frame.render_widget(
            Paragraph::new(lines).style(Style::default().fg(theme.text_secondary)),
            inner,
        );
    }
}
