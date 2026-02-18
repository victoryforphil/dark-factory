use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};

use crate::app::App;

use dark_tui_components::PaneBlockComponent;

pub(crate) struct DeleteVariantFormPanel;

impl DeleteVariantFormPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let popup = centered_rect(area, 64, 36);

        frame.render_widget(Clear, popup);

        let block = PaneBlockComponent::build("Delete Variant", true, theme);
        let inner = block.inner(popup);
        frame.render_widget(block, popup);

        let variant_id = app.delete_variant_form_variant_id().unwrap_or("-");
        let remove_clone_dir = app.delete_variant_form_remove_clone_directory();
        let (mode_label, action_label) = if remove_clone_dir {
            ("DESTRUCTIVE", "Delete row + remove clone directory")
        } else {
            ("SAFE", "Delete row only, keep clone directory")
        };

        let lines = vec![
            Line::from(Span::styled(
                "Confirm delete for variant:",
                Style::default().fg(theme.text_muted),
            )),
            Line::from(Span::styled(
                format!("  {variant_id}"),
                Style::default().fg(theme.text_primary),
            )),
            Line::raw(""),
            Line::from(vec![
                Span::styled(
                    format!("[{}] ", mode_label),
                    Style::default().fg(if remove_clone_dir {
                        theme.text_error
                    } else {
                        theme.pill_ok_fg
                    }),
                ),
                Span::styled(action_label, Style::default().fg(theme.text_primary)),
            ]),
            Line::raw(""),
            Line::from(Span::styled(
                "Space: toggle safe/destructive mode",
                Style::default().fg(theme.text_muted),
            )),
            Line::from(Span::styled(
                "Enter: confirm delete   Esc: cancel",
                Style::default().fg(theme.text_muted),
            )),
        ];

        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
    }
}

fn centered_rect(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height_percent) / 2),
            Constraint::Percentage(height_percent),
            Constraint::Percentage((100 - height_percent) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_percent) / 2),
            Constraint::Percentage(width_percent),
            Constraint::Percentage((100 - width_percent) / 2),
        ])
        .split(vertical[1])[1]
}
