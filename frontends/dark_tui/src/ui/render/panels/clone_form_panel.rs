use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;

use dark_tui_components::PaneBlockComponent;

pub(crate) struct CloneFormPanel;

impl CloneFormPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let popup = centered_rect(area, 76, 62);

        frame.render_widget(Clear, popup);

        let block = PaneBlockComponent::build("Clone Variant", true, theme);
        let inner = block.inner(popup);
        frame.render_widget(block, popup);

        let selected = app.clone_form_selected_field().unwrap_or(0);
        let name = app.clone_form_name().unwrap_or_default();
        let target = app.clone_form_target_path().unwrap_or_default();
        let branch = app.clone_form_branch_name().unwrap_or_default();
        let clone_type = app.clone_form_clone_type().unwrap_or_default();
        let source_variant_id = app.clone_form_source_variant_id().unwrap_or_default();

        let mut lines = Vec::new();
        lines.push(Line::from(Span::styled(
            "Field (arrows, Tab/Shift+Tab):",
            Style::default().fg(theme.text_muted),
        )));
        lines.push(field_line("Name", name, selected == 0, theme));
        lines.push(field_line("Target path", target, selected == 1, theme));
        lines.push(field_line("Branch name", branch, selected == 2, theme));
        lines.push(field_line("Clone type", clone_type, selected == 3, theme));
        lines.push(field_line(
            "Source variant",
            source_variant_id,
            selected == 4,
            theme,
        ));
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Leave blank to use automatic defaults.",
            Style::default().fg(theme.text_muted),
        )));
        lines.push(Line::from(Span::styled(
            "Enter: clone   Esc: cancel   Backspace: edit",
            Style::default().fg(theme.text_muted),
        )));

        let content = Paragraph::new(lines).wrap(Wrap { trim: false });
        frame.render_widget(content, inner);
    }
}

fn field_line(
    label: &str,
    value: &str,
    selected: bool,
    theme: &crate::theme::Theme,
) -> Line<'static> {
    let marker = if selected { ">" } else { " " };
    let display = if value.is_empty() {
        "_".to_string()
    } else {
        value.to_string()
    };
    let style = if selected {
        Style::default().fg(theme.entity_variant)
    } else {
        Style::default().fg(theme.text_primary)
    };

    Line::from(Span::styled(
        format!("  {marker} {label}: {display}"),
        style,
    ))
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
