use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::theme::ComponentThemeLike;
use crate::{rect_contains, with_cursor_tail, ListViewport, StatusPill};

/// Display item for popup selection lists.
#[derive(Debug, Clone)]
pub struct PopupItem {
    pub label: String,
    pub tag: Option<String>,
    pub active: bool,
}

/// Placement hint for popup overlays.
#[derive(Debug, Clone)]
pub enum PopupAnchor {
    AboveRect(Rect),
    At { x: u16, y: u16 },
    Center,
}

/// Props used to render a popup overlay and compute hit-testing.
#[derive(Debug, Clone)]
pub struct PopupOverlayProps {
    pub title: String,
    pub items: Vec<PopupItem>,
    pub selected: usize,
    pub query: Option<String>,
    pub query_label: Option<String>,
    pub hint: Option<String>,
    pub anchor: PopupAnchor,
    pub max_visible: usize,
    pub min_width: u16,
    pub max_width: u16,
}

/// Hit-test result for popup interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupHit {
    Outside,
    Popup,
    Query,
    ListItem(usize),
}

/// Generic popup list renderer with optional query and hint rows.
pub struct PopupOverlay;

impl PopupOverlay {
    /// Computes popup area constrained to parent bounds.
    pub fn area(parent: Rect, props: &PopupOverlayProps) -> Option<Rect> {
        if parent.width < 4 || parent.height < 4 {
            return None;
        }

        let item_width = props
            .items
            .iter()
            .map(|item| {
                let tag_len = item
                    .tag
                    .as_ref()
                    .map(|tag| tag.chars().count() + 1)
                    .unwrap_or(0);
                let active_len = if item.active { 7 } else { 0 };
                2 + item.label.chars().count() + tag_len + active_len
            })
            .max()
            .unwrap_or(12);
        let query_width = props
            .query
            .as_ref()
            .map(|query| {
                let label_len = props
                    .query_label
                    .as_ref()
                    .map(|label| label.chars().count() + 3)
                    .unwrap_or(0);
                label_len + query.chars().count() + 1
            })
            .unwrap_or(0);
        let hint_width = props
            .hint
            .as_ref()
            .map(|hint| hint.chars().count())
            .unwrap_or(0);
        let title_width = props.title.chars().count() + 2;

        let content_width = item_width.max(query_width).max(hint_width).max(title_width) as u16;
        let max_allowed = props.max_width.min(parent.width);
        let min_allowed = props.min_width.min(max_allowed.max(1));
        let width = content_width
            .saturating_add(2)
            .clamp(min_allowed, max_allowed);

        let query_rows = u16::from(props.query.is_some());
        let hint_rows = u16::from(props.hint.is_some());
        let list_rows = props.items.len().max(1).min(props.max_visible.max(1)) as u16;
        let desired_height = list_rows
            .saturating_add(query_rows)
            .saturating_add(hint_rows)
            .saturating_add(2);
        let height = desired_height.clamp(4, parent.height);

        let (desired_x, desired_y) = match props.anchor {
            PopupAnchor::AboveRect(anchor) => {
                let cx = anchor.x.saturating_add(anchor.width / 2);
                let x = cx.saturating_sub(width / 2);
                let y = anchor.y.saturating_sub(height.saturating_sub(1));
                (x, y)
            }
            PopupAnchor::At { x, y } => {
                let px = x.saturating_sub(width / 2);
                let py = y.saturating_sub(height.saturating_sub(1));
                (px, py)
            }
            PopupAnchor::Center => {
                let x = parent
                    .x
                    .saturating_add(parent.width.saturating_sub(width) / 2);
                let y = parent
                    .y
                    .saturating_add(parent.height.saturating_sub(height) / 2);
                (x, y)
            }
        };

        let min_x = parent.x;
        let max_x = parent
            .x
            .saturating_add(parent.width.saturating_sub(width).max(0));
        let min_y = parent.y;
        let max_y = parent
            .y
            .saturating_add(parent.height.saturating_sub(height).max(0));

        Some(Rect {
            x: desired_x.clamp(min_x, max_x),
            y: desired_y.clamp(min_y, max_y),
            width,
            height,
        })
    }

    /// Renders popup chrome, list items, query row, and hint row.
    pub fn render(
        frame: &mut Frame,
        parent: Rect,
        props: &PopupOverlayProps,
        theme: &impl ComponentThemeLike,
    ) {
        let Some(area) = Self::area(parent, props) else {
            return;
        };

        frame.render_widget(Clear, area);
        let block = Block::default()
            .title(format!(" {} ", props.title))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.pane_focused_border()));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let sections = section_layout(inner, props.query.is_some(), props.hint.is_some());
        render_list(frame, sections.list, props, theme);

        if let (Some(query_area), Some(query)) = (sections.query, props.query.as_ref()) {
            let label = props
                .query_label
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("FILTER");
            let line = Line::from(vec![
                StatusPill::accent(label, theme).span_compact(),
                Span::raw(" "),
                Span::styled(
                    with_cursor_tail(query),
                    Style::default().fg(theme.text_secondary()),
                ),
            ]);
            frame.render_widget(Paragraph::new(line).wrap(Wrap { trim: true }), query_area);
        }

        if let (Some(hint_area), Some(hint)) = (sections.hint, props.hint.as_ref()) {
            frame.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    hint.clone(),
                    Style::default().fg(theme.text_muted()),
                )))
                .wrap(Wrap { trim: true }),
                hint_area,
            );
        }
    }

    /// Performs hit-testing against popup sections.
    pub fn hit_test(parent: Rect, props: &PopupOverlayProps, col: u16, row: u16) -> PopupHit {
        let Some(area) = Self::area(parent, props) else {
            return PopupHit::Outside;
        };

        if !rect_contains(area, col, row) {
            return PopupHit::Outside;
        }

        let inner = Rect {
            x: area.x.saturating_add(1),
            y: area.y.saturating_add(1),
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        if inner.width == 0 || inner.height == 0 {
            return PopupHit::Popup;
        }

        let sections = section_layout(inner, props.query.is_some(), props.hint.is_some());
        if sections
            .query
            .is_some_and(|query| rect_contains(query, col, row))
        {
            return PopupHit::Query;
        }

        if !rect_contains(sections.list, col, row) {
            return PopupHit::Popup;
        }

        if props.items.is_empty() {
            return PopupHit::Popup;
        }

        let selected = props.selected.min(props.items.len().saturating_sub(1));
        let viewport =
            ListViewport::new(props.items.len(), sections.list.height as usize, selected);
        let local_row = row.saturating_sub(sections.list.y) as usize;
        let index = viewport.start.saturating_add(local_row);

        if index < props.items.len() {
            PopupHit::ListItem(index)
        } else {
            PopupHit::Popup
        }
    }
}

struct PopupSections {
    list: Rect,
    query: Option<Rect>,
    hint: Option<Rect>,
}

fn section_layout(area: Rect, has_query: bool, has_hint: bool) -> PopupSections {
    let mut constraints = vec![Constraint::Min(1)];
    if has_query {
        constraints.push(Constraint::Length(1));
    }
    if has_hint {
        constraints.push(Constraint::Length(1));
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    PopupSections {
        list: chunks[0],
        query: has_query.then(|| chunks[1]),
        hint: has_hint.then(|| chunks[chunks.len() - 1]),
    }
}

fn render_list(
    frame: &mut Frame,
    area: Rect,
    props: &PopupOverlayProps,
    theme: &impl ComponentThemeLike,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    if props.items.is_empty() {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "No matching options.",
                Style::default().fg(theme.text_muted()),
            ))),
            area,
        );
        return;
    }

    let selected = props.selected.min(props.items.len().saturating_sub(1));
    let viewport = ListViewport::new(props.items.len(), area.height as usize, selected);
    let mut lines = Vec::new();

    for index in viewport.start..viewport.end {
        let item = &props.items[index];
        let is_selected = index == selected;
        let mut spans = vec![Span::styled(
            if is_selected { "▸ " } else { "  " },
            if is_selected {
                Style::default().fg(theme.pill_accent_fg())
            } else {
                Style::default().fg(theme.text_muted())
            },
        )];

        spans.push(Span::styled(
            item.label.clone(),
            if is_selected {
                Style::default().fg(theme.pill_accent_fg())
            } else {
                Style::default().fg(theme.text_secondary())
            },
        ));

        if let Some(tag) = item.tag.as_ref() {
            spans.push(Span::raw(" "));
            spans.push(StatusPill::muted(tag.clone(), theme).span_compact());
        }

        if item.active {
            spans.push(Span::raw(" "));
            spans.push(StatusPill::info("active", theme).span_compact());
        }

        lines.push(Line::from(spans));
    }

    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), area);
}
