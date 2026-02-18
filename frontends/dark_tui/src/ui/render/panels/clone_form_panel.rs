use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph};

use crate::app::App;

use dark_tui_components::{
    PaneBlockComponent, PopupAnchor, PopupHit, PopupItem, PopupOverlay, PopupOverlayProps,
    inner_rect,
};

pub(crate) struct CloneFormPanel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CloneFormHit {
    Field(usize),
    PickerItem(usize),
    PickerPopup,
    Popup,
    Outside,
}

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
        let remote_host = app.clone_form_remote_host().unwrap_or_default();
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
        lines.push(field_line("Remote host", remote_host, selected == 2, theme));
        lines.push(field_line("Branch name", branch, selected == 3, theme));
        lines.push(field_line("Clone type", clone_type, selected == 4, theme));
        lines.push(field_line(
            "Source variant",
            source_variant_id,
            selected == 5,
            theme,
        ));
        if selected == 2 {
            lines.push(Line::from(Span::styled(
                "      Use Left/Right to cycle discovered SSH hosts.",
                Style::default().fg(theme.text_muted),
            )));
        }
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Leave blank to use automatic defaults.",
            Style::default().fg(theme.text_muted),
        )));
        lines.push(Line::from(Span::styled(
            "Remote target example: @ssh://devbox/srv/workspace/clone-01",
            Style::default().fg(theme.text_muted),
        )));
        lines.push(Line::from(Span::styled(
            "Enter: clone (or open host picker on Remote host)",
            Style::default().fg(theme.text_muted),
        )));
        lines.push(Line::from(Span::styled(
            "Esc: cancel   Backspace: edit   Left/Right: host",
            Style::default().fg(theme.text_muted),
        )));

        let content = Paragraph::new(lines);
        frame.render_widget(content, inner);

        render_remote_host_picker(frame, popup, app);
    }

    pub(crate) fn hit_test(area: Rect, app: &App, col: u16, row: u16) -> CloneFormHit {
        let popup = centered_rect(area, 76, 62);

        if col < popup.x
            || col >= popup.x + popup.width
            || row < popup.y
            || row >= popup.y + popup.height
        {
            return CloneFormHit::Outside;
        }

        if let Some(props) = clone_host_picker_props(popup, app) {
            match PopupOverlay::hit_test(popup, &props, col, row) {
                PopupHit::Outside => {}
                PopupHit::ListItem(index) => return CloneFormHit::PickerItem(index),
                PopupHit::Popup | PopupHit::Query => return CloneFormHit::PickerPopup,
            }
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
            return CloneFormHit::Popup;
        }

        let local_row = row.saturating_sub(inner.y);
        if (1..=6).contains(&local_row) {
            return CloneFormHit::Field((local_row - 1) as usize);
        }

        CloneFormHit::Popup
    }
}

fn render_remote_host_picker(frame: &mut Frame, area: Rect, app: &App) {
    let Some(props) = clone_host_picker_props(area, app) else {
        return;
    };

    PopupOverlay::render(frame, area, &props, app.theme());
}

fn clone_host_picker_props(area: Rect, app: &App) -> Option<PopupOverlayProps> {
    if !app.clone_host_picker_open() {
        return None;
    }

    let items = app.clone_host_picker_items();
    if items.is_empty() {
        return None;
    }

    let inner = inner_rect(area);
    Some(PopupOverlayProps {
        title: "Select Remote Host".to_string(),
        items: items
            .iter()
            .map(|item| PopupItem {
                label: item.clone(),
                tag: None,
                active: false,
            })
            .collect(),
        selected: app.clone_host_picker_selected(),
        query: Some(app.clone_host_picker_query().to_string()),
        query_label: Some("FILTER".to_string()),
        hint: Some("enter select  up/down scroll  bksp delete  esc close".to_string()),
        anchor: PopupAnchor::At {
            x: inner.x.saturating_add(2),
            y: inner.y.saturating_add(5),
        },
        max_visible: 8,
        min_width: 34,
        max_width: inner.width.min(56),
    })
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
