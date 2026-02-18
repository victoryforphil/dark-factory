use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};

use crate::app::App;

use dark_tui_components::PaneBlockComponent;

pub(crate) struct BranchFormPanel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BranchFormHit {
    Input,
    Suggestion(usize),
    Popup,
    Outside,
}

impl BranchFormPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let popup = centered_rect(area, 64, 48);

        frame.render_widget(Clear, popup);

        let block = PaneBlockComponent::build("Switch Variant Branch", true, theme);
        let inner = block.inner(popup);
        frame.render_widget(block, popup);

        let branch_name = app.branch_form_branch_name().unwrap_or_default();
        let suggestions = app.branch_form_suggestions().unwrap_or_default();
        let selected = app.branch_form_selected_suggestion_index().unwrap_or(0);

        let mut lines = Vec::new();
        lines.push(Line::from(Span::styled(
            "Branch:",
            Style::default().fg(theme.text_muted),
        )));
        lines.push(Line::from(Span::styled(
            format!(
                "  > {}",
                if branch_name.is_empty() {
                    "_"
                } else {
                    branch_name
                }
            ),
            Style::default().fg(theme.entity_variant),
        )));
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Suggestions:",
            Style::default().fg(theme.text_muted),
        )));

        if suggestions.is_empty() {
            lines.push(Line::from(Span::styled(
                "  (none)",
                Style::default().fg(theme.text_muted),
            )));
        } else {
            for (index, suggestion) in suggestions.iter().enumerate() {
                let marker = if index == selected { ">" } else { " " };
                let style = if index == selected {
                    Style::default().fg(theme.entity_variant)
                } else {
                    Style::default().fg(theme.text_secondary)
                };
                lines.push(Line::from(Span::styled(
                    format!("  {marker} {suggestion}"),
                    style,
                )));
            }
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Enter: switch   Tab: autocomplete   arrows: pick suggestion",
            Style::default().fg(theme.text_muted),
        )));
        lines.push(Line::from(Span::styled(
            "Esc: cancel   Backspace: edit",
            Style::default().fg(theme.text_muted),
        )));

        let content = Paragraph::new(lines).wrap(Wrap { trim: false });
        frame.render_widget(content, inner);
    }

    pub(crate) fn hit_test(area: Rect, app: &App, col: u16, row: u16) -> BranchFormHit {
        let popup = centered_rect(area, 64, 48);

        if col < popup.x
            || col >= popup.x + popup.width
            || row < popup.y
            || row >= popup.y + popup.height
        {
            return BranchFormHit::Outside;
        }

        let inner = Rect {
            x: popup.x.saturating_add(1),
            y: popup.y.saturating_add(1),
            width: popup.width.saturating_sub(2),
            height: popup.height.saturating_sub(2),
        };

        if col < inner.x
            || col >= inner.x + inner.width
            || row < inner.y
            || row >= inner.y + inner.height
        {
            return BranchFormHit::Popup;
        }

        let local_row = row.saturating_sub(inner.y);
        if local_row == 1 {
            return BranchFormHit::Input;
        }

        let suggestions = app.branch_form_suggestions().unwrap_or_default();
        let suggestions_start = 4u16;
        let suggestions_end = suggestions_start.saturating_add(suggestions.len() as u16);
        if local_row >= suggestions_start && local_row < suggestions_end {
            return BranchFormHit::Suggestion((local_row - suggestions_start) as usize);
        }

        BranchFormHit::Popup
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
