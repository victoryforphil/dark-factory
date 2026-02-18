use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::VizSelection;
use crate::theme::EntityKind;
use crate::ui::command_palette::ContextMenuState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContextMenuHit {
    Item(usize),
    Menu,
    Outside,
}

pub(crate) struct ContextMenuPanel;

impl ContextMenuPanel {
    pub(crate) fn render(
        frame: &mut Frame,
        root: Rect,
        app: &crate::app::App,
        menu: &ContextMenuState,
    ) {
        let rect = Self::rect(root, menu);
        if rect.width < 4 || rect.height < 3 {
            return;
        }

        let theme = app.theme();
        let entity_kind = menu_entity_kind(&menu.target);

        let block = Block::default()
            .title("Actions")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.pane_focused_border));
        let inner = block.inner(rect);

        frame.render_widget(Clear, rect);
        frame.render_widget(block, rect);

        let lines: Vec<Line<'static>> = menu
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let selected = index == menu.selected;
                let line_style = if selected {
                    Style::default()
                        .fg(theme.table_highlight_fg)
                        .bg(theme.table_highlight_bg_for(entity_kind))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.text_primary)
                };

                let key_style = if selected {
                    line_style
                } else {
                    Style::default().fg(theme.key_hint_key_fg)
                };

                Line::from(vec![
                    Span::styled(format!(" {} ", entry.key), key_style),
                    Span::styled(format!(" {}", entry.label), line_style),
                ])
            })
            .collect();

        frame.render_widget(Paragraph::new(lines), inner);
    }

    pub(crate) fn hit_test(
        root: Rect,
        menu: &ContextMenuState,
        col: u16,
        row: u16,
    ) -> ContextMenuHit {
        let rect = Self::rect(root, menu);
        if col < rect.x || col >= rect.x + rect.width || row < rect.y || row >= rect.y + rect.height
        {
            return ContextMenuHit::Outside;
        }

        if row == rect.y || row == rect.y + rect.height.saturating_sub(1) {
            return ContextMenuHit::Menu;
        }
        if col == rect.x || col == rect.x + rect.width.saturating_sub(1) {
            return ContextMenuHit::Menu;
        }

        let list_row = row.saturating_sub(rect.y + 1) as usize;
        if list_row < menu.entries.len() {
            ContextMenuHit::Item(list_row)
        } else {
            ContextMenuHit::Menu
        }
    }

    pub(crate) fn rect(root: Rect, menu: &ContextMenuState) -> Rect {
        let width = menu
            .entries
            .iter()
            .map(|entry| entry.key.len() + entry.label.len() + 4)
            .max()
            .unwrap_or(16)
            .saturating_add(2) as u16;
        let height = menu.entries.len().saturating_add(2) as u16;

        let clamped_width = width.min(root.width.max(1));
        let clamped_height = height.min(root.height.max(1));

        let max_x = root.x + root.width.saturating_sub(clamped_width);
        let max_y = root.y + root.height.saturating_sub(clamped_height);
        let x = menu.anchor_col.min(max_x);
        let y = menu.anchor_row.min(max_y);

        Rect {
            x,
            y,
            width: clamped_width,
            height: clamped_height,
        }
    }
}

fn menu_entity_kind(target: &VizSelection) -> EntityKind {
    match target {
        VizSelection::Product { .. } => EntityKind::Product,
        VizSelection::Variant { .. } => EntityKind::Variant,
        VizSelection::Actor { .. } => EntityKind::Actor,
    }
}
