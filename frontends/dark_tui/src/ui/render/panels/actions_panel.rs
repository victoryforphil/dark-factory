use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;

use super::super::components::{KeyBind, KeyHintBar, PaneBlockComponent};

const KEY_BINDS: &[KeyBind] = &[
    KeyBind::new("q", "Quit"),
    KeyBind::new("Tab", "Focus"),
    KeyBind::new("j/k", "Select"),
    KeyBind::new("r", "Refresh"),
    KeyBind::new("f", "Filter"),
    KeyBind::new("v", "View"),
    KeyBind::new("p", "Poll"),
    KeyBind::new("i", "Init"),
    KeyBind::new("n", "Session"),
    KeyBind::new("a", "Attach"),
];

pub(crate) struct ActionsPanel;

impl ActionsPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build("Keys", false, theme);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let hint_bar = KeyHintBar::new(KEY_BINDS);
        let mut lines = hint_bar.lines_wrapped(inner.width, theme);

        // Append CLI parity info below the key hints.
        let cli_lines = app.command_examples();
        if !cli_lines.is_empty() {
            lines.push(Line::raw(""));
            for cmd in cli_lines {
                lines.push(Line::raw(cmd));
            }
        }

        let widget = Paragraph::new(lines);
        frame.render_widget(widget, inner);
    }
}
