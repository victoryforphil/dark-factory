use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};

use crate::app::App;

use dark_tui_components::PaneBlockComponent;

pub(crate) struct MoveActorFormPanel;

impl MoveActorFormPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let popup = centered_rect(area, 62, 52);

        frame.render_widget(Clear, popup);

        let block = PaneBlockComponent::build("Move Actor", true, theme);
        let inner = block.inner(popup);
        frame.render_widget(block, popup);

        let actor_title = app.move_actor_form_actor_title().unwrap_or("-");
        let source_variant_id = app.move_actor_form_source_variant_id().unwrap_or("-");
        let source_variant_name = app.move_actor_form_source_variant_name().unwrap_or("-");
        let options = app.move_actor_form_options().unwrap_or_default();
        let selected = app.move_actor_form_selected_option_index().unwrap_or(0);

        let mut lines: Vec<Line<'static>> = vec![
            Line::from(Span::styled(
                format!("Actor: {actor_title}"),
                Style::default().fg(theme.text_primary),
            )),
            Line::from(Span::styled(
                format!("From: {source_variant_name} ({source_variant_id})"),
                Style::default().fg(theme.text_muted),
            )),
            Line::raw(""),
            Line::from(Span::styled(
                "Destination (j/k or arrows):",
                Style::default().fg(theme.text_muted),
            )),
        ];

        for (index, (variant_id, variant_name, product_name)) in options.iter().enumerate() {
            let marker = if index == selected { ">" } else { " " };
            let style = if index == selected {
                Style::default().fg(theme.entity_actor)
            } else {
                Style::default().fg(theme.text_primary)
            };

            lines.push(Line::from(Span::styled(
                format!("  {marker} {variant_name} ({variant_id}) [{product_name}]"),
                style,
            )));
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Enter: move actor   Esc: cancel",
            Style::default().fg(theme.text_muted),
        )));

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
