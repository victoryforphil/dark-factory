use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders};

use crate::theme::Theme;

pub(crate) struct PaneBlockComponent;

impl PaneBlockComponent {
    pub(crate) fn build<'a>(title: &'a str, focused: bool, theme: &Theme) -> Block<'a> {
        let border_style = if focused {
            Style::default()
                .fg(theme.pane_focused_border)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.pane_unfocused_border)
        };

        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style)
    }
}
