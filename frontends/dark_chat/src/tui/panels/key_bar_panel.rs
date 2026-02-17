use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use dark_tui_components::KeyHintBar;

use crate::tui::app::App;
use crate::tui::components::KEY_BINDS;

pub struct KeyBarPanel;

impl KeyBarPanel {
    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let lines = KeyHintBar::new(&KEY_BINDS).lines_wrapped(area.width, theme);
        frame.render_widget(Paragraph::new(lines), area);
    }
}
