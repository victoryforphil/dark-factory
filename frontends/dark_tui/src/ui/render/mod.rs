mod components;
mod panels;
mod views;

use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use dark_tui_components::HorizontalSplit;

use crate::app::{App, ResizeTarget, ResultsViewMode};
use crate::ui::command_palette::ContextMenuState;

use panels::{
    ChatPanel, CloneFormPanel, ContextMenuPanel, DeleteVariantFormPanel, DetailsPanel, FooterPanel,
    HeaderPanel, KeyBarPanel, MoveActorFormPanel, SpawnFormPanel,
};
use views::{CatalogTreeView, UnifiedCatalogView};

pub(crate) use panels::ChatPanelHit;
pub(crate) use panels::ContextMenuHit;
pub(crate) use panels::KeyHintAction;

#[derive(Debug, Clone)]
pub(crate) struct DragPreview {
    pub(crate) col: u16,
    pub(crate) row: u16,
    pub(crate) actor_label: String,
    pub(crate) can_drop: bool,
}

pub fn render_dashboard(
    frame: &mut Frame,
    app: &App,
    context_menu: Option<&ContextMenuState>,
    drag_preview: Option<&DragPreview>,
) {
    let root = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // header (compact title)
            Constraint::Length(2), // key-hint bar
            Constraint::Min(10),   // body (catalog + sidebar)
            Constraint::Length(3), // footer/status
        ])
        .split(root);

    HeaderPanel::render(frame, vertical[0], app);
    KeyBarPanel::render(frame, vertical[1], app);
    render_body(frame, vertical[2], app);
    FooterPanel::render(frame, vertical[3], app);

    if let Some(menu) = context_menu {
        ContextMenuPanel::render(frame, root, app, menu);
    }

    if app.is_spawn_form_open() {
        SpawnFormPanel::render(frame, root, app);
    }

    if app.is_clone_form_open() {
        CloneFormPanel::render(frame, root, app);
    }

    if app.is_delete_variant_form_open() {
        DeleteVariantFormPanel::render(frame, root, app);
    }

    if app.is_move_actor_form_open() {
        MoveActorFormPanel::render(frame, root, app);
    }

    if let Some(preview) = drag_preview {
        render_drag_preview(frame, root, app, preview);
    }
}

fn render_body(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    match app.results_view_mode() {
        ResultsViewMode::Table => render_body_table(frame, area, app),
        ResultsViewMode::Viz => render_body_viz(frame, area, app),
    }
}

fn active_split(app: &App) -> &HorizontalSplit {
    if app.is_chat_visible() {
        app.body_split_with_chat()
    } else {
        app.body_split_without_chat()
    }
}

fn resolve_columns(area: Rect, app: &App) -> Vec<Rect> {
    active_split(app).resolve(area)
}

/// Table mode: main/sidebar split. Details panel fills the entire sidebar.
fn render_body_table(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let columns = resolve_columns(area, app);
    if app.is_chat_visible() && columns.len() >= 3 {
        CatalogTreeView::render(frame, columns[0], app);
        ChatPanel::render(frame, columns[1], app);
        DetailsPanel::render(frame, columns[2], app);
    } else if columns.len() >= 2 {
        CatalogTreeView::render(frame, columns[0], app);
        DetailsPanel::render(frame, columns[1], app);
    }
}

/// Viz mode: main/sidebar split. Details panel fills the entire sidebar.
fn render_body_viz(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let columns = resolve_columns(area, app);
    if app.is_chat_visible() && columns.len() >= 3 {
        UnifiedCatalogView::render(frame, columns[0], app);
        ChatPanel::render(frame, columns[1], app);
        DetailsPanel::render(frame, columns[2], app);
    } else if columns.len() >= 2 {
        UnifiedCatalogView::render(frame, columns[0], app);
        DetailsPanel::render(frame, columns[1], app);
    }
}

pub(crate) fn try_select_viz_node(root: Rect, app: &mut App, col: u16, row: u16) -> bool {
    let body = body_area(root);
    let columns = resolve_columns(body, app);
    let Some(main_area) = columns.first().copied() else {
        return false;
    };

    UnifiedCatalogView::click_select(main_area, app, col, row)
}

pub(crate) fn viz_hit_test(
    root: Rect,
    app: &App,
    col: u16,
    row: u16,
) -> Option<crate::app::VizSelection> {
    let body = body_area(root);
    let columns = resolve_columns(body, app);
    let main_area = columns.first().copied()?;

    UnifiedCatalogView::hit_test(main_area, app, col, row)
}

pub(crate) fn chat_area(root: Rect, app: &App) -> Option<Rect> {
    if !app.is_chat_visible() {
        return None;
    }

    let body = body_area(root);
    resolve_columns(body, app).get(1).copied()
}

pub(crate) fn divider_hit(root: Rect, app: &App, col: u16) -> Option<ResizeTarget> {
    let body = body_area(root);
    if body.width < 20 {
        return None;
    }

    if app.is_chat_visible() {
        let divider = active_split(app).divider_hit(body, col, 1)?;
        return Some(ResizeTarget::BodyWithChat(divider));
    }

    let divider = active_split(app).divider_hit(body, col, 1)?;
    Some(ResizeTarget::BodyWithoutChat(divider))
}

pub(crate) fn resize_divider(root: Rect, app: &mut App, target: ResizeTarget, col: u16) -> bool {
    let body = body_area(root);
    if body.width < 20 {
        return false;
    }

    match target {
        ResizeTarget::BodyWithChat(divider) => app
            .body_split_with_chat_mut()
            .resize_from_pointer(body, divider, col),
        ResizeTarget::BodyWithoutChat(divider) => app
            .body_split_without_chat_mut()
            .resize_from_pointer(body, divider, col),
    }
}

pub(crate) fn chat_hit_test(root: Rect, app: &App, col: u16, row: u16) -> ChatPanelHit {
    let Some(chat) = chat_area(root, app) else {
        return ChatPanelHit::Outside;
    };

    ChatPanel::hit_test(chat, app, col, row)
}

fn body_area(root: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(root)[2]
}

/// Compute the key bar area for hit testing.
pub(crate) fn key_bar_area(root: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // header
            Constraint::Length(2), // key bar
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(root)[1]
}

/// Hit test: returns the key hint action at the given position, or None if not on a key hint.
pub(crate) fn key_bar_hit_test(root: Rect, app: &App, row: u16, col: u16) -> Option<KeyHintAction> {
    let area = key_bar_area(root);
    KeyBarPanel::hit_test(area, app, row, col)
}

pub(crate) fn context_menu_hit_test(
    root: Rect,
    menu: &ContextMenuState,
    col: u16,
    row: u16,
) -> ContextMenuHit {
    ContextMenuPanel::hit_test(root, menu, col, row)
}

fn render_drag_preview(frame: &mut Frame, root: Rect, app: &App, preview: &DragPreview) {
    if root.width < 12 || root.height < 4 {
        return;
    }

    let theme = app.theme();
    let text = format!("Move actor: {}", preview.actor_label);
    let width = (text.len() as u16 + 2).clamp(14, root.width);
    let x = preview
        .col
        .saturating_add(1)
        .min(root.x + root.width.saturating_sub(width));
    let y = preview
        .row
        .saturating_add(1)
        .min(root.y + root.height.saturating_sub(3));
    let area = Rect {
        x,
        y,
        width,
        height: 3,
    };

    let border_color = if preview.can_drop {
        theme.entity_variant
    } else {
        theme.entity_actor
    };
    let title = if preview.can_drop {
        "Drop to move"
    } else {
        "Dragging"
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
    let inner = block.inner(area);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(
        Paragraph::new(vec![Line::from(text)]).style(Style::default().fg(theme.text_primary)),
        inner,
    );
}
