mod components;
mod panels;
mod views;

use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

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
pub(crate) use panels::KeyHoverToken;

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
    key_hover_token: Option<&KeyHoverToken>,
    key_hover_hint: Option<&str>,
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

    if let Some(token) = key_hover_token {
        render_key_hover_token(frame, app, token);
    }

    if let Some(hint) = key_hover_hint {
        render_key_hover_hint(frame, root, app, hint);
    }
}

fn render_body(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    match app.results_view_mode() {
        ResultsViewMode::Table => render_body_table(frame, area, app),
        ResultsViewMode::Viz => render_body_viz(frame, area, app),
    }
}

fn resolve_columns(area: Rect, app: &App) -> Vec<Rect> {
    if app.is_inspector_visible() {
        if app.is_chat_visible() {
            return app.body_split_with_chat().resolve(area);
        }
        return app.body_split_without_chat().resolve(area);
    }

    if app.is_chat_visible() {
        return app.body_split_without_chat().resolve(area);
    }

    vec![area]
}

/// Table mode: main/sidebar split. Details panel fills the entire sidebar.
fn render_body_table(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let columns = resolve_columns(area, app);
    if app.is_inspector_visible() && app.is_chat_visible() && columns.len() >= 3 {
        CatalogTreeView::render(frame, columns[0], app);
        ChatPanel::render(frame, columns[1], app);
        DetailsPanel::render(frame, columns[2], app);
    } else if app.is_inspector_visible() && columns.len() >= 2 {
        CatalogTreeView::render(frame, columns[0], app);
        DetailsPanel::render(frame, columns[1], app);
    } else if app.is_chat_visible() && columns.len() >= 2 {
        CatalogTreeView::render(frame, columns[0], app);
        ChatPanel::render(frame, columns[1], app);
    } else if let Some(main) = columns.first() {
        CatalogTreeView::render(frame, *main, app);
    }
}

/// Viz mode: main/sidebar split. Details panel fills the entire sidebar.
fn render_body_viz(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let columns = resolve_columns(area, app);
    if app.is_inspector_visible() && app.is_chat_visible() && columns.len() >= 3 {
        UnifiedCatalogView::render(frame, columns[0], app);
        ChatPanel::render(frame, columns[1], app);
        DetailsPanel::render(frame, columns[2], app);
    } else if app.is_inspector_visible() && columns.len() >= 2 {
        UnifiedCatalogView::render(frame, columns[0], app);
        DetailsPanel::render(frame, columns[1], app);
    } else if app.is_chat_visible() && columns.len() >= 2 {
        UnifiedCatalogView::render(frame, columns[0], app);
        ChatPanel::render(frame, columns[1], app);
    } else if let Some(main) = columns.first() {
        UnifiedCatalogView::render(frame, *main, app);
    }
}

pub(crate) fn try_select_viz_node(root: Rect, app: &mut App, col: u16, row: u16) -> bool {
    if !app.results_view_mode().is_spatial() {
        return false;
    }

    let body = body_area(root);
    let columns = resolve_columns(body, app);
    let Some(main_area) = columns.first().copied() else {
        return false;
    };

    match app.results_view_mode() {
        ResultsViewMode::Viz => UnifiedCatalogView::click_select(main_area, app, col, row),
        ResultsViewMode::Table => false,
    }
}

pub(crate) fn tree_hit_test(
    root: Rect,
    app: &App,
    col: u16,
    row: u16,
) -> Option<crate::app::VizSelection> {
    let body = body_area(root);
    let columns = resolve_columns(body, app);
    let main_area = columns.first().copied()?;

    CatalogTreeView::hit_test(main_area, app, col, row)
}

pub(crate) fn tree_contains(root: Rect, app: &App, col: u16, row: u16) -> bool {
    let body = body_area(root);
    let columns = resolve_columns(body, app);
    let Some(main_area) = columns.first().copied() else {
        return false;
    };

    col >= main_area.x
        && col < main_area.x + main_area.width
        && row >= main_area.y
        && row < main_area.y + main_area.height
}

pub(crate) fn viz_hit_test(
    root: Rect,
    app: &App,
    col: u16,
    row: u16,
) -> Option<crate::app::VizSelection> {
    if !app.results_view_mode().is_spatial() {
        return None;
    }

    let body = body_area(root);
    let columns = resolve_columns(body, app);
    let main_area = columns.first().copied()?;

    match app.results_view_mode() {
        ResultsViewMode::Viz => UnifiedCatalogView::hit_test(main_area, app, col, row),
        ResultsViewMode::Table => None,
    }
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

    if !app.is_inspector_visible() {
        if app.is_chat_visible() {
            let divider = app.body_split_without_chat().divider_hit(body, col, 1)?;
            return Some(ResizeTarget::BodyWithoutChat(divider));
        }

        return None;
    }

    if app.is_chat_visible() {
        let divider = app.body_split_with_chat().divider_hit(body, col, 1)?;
        return Some(ResizeTarget::BodyWithChat(divider));
    }

    let divider = app.body_split_without_chat().divider_hit(body, col, 1)?;
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

pub(crate) fn chat_message_index_at_point(
    root: Rect,
    app: &App,
    col: u16,
    row: u16,
) -> Option<usize> {
    let chat = chat_area(root, app)?;
    ChatPanel::message_index_at_point(chat, app, col, row)
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

pub(crate) fn key_bar_hover_hint(root: Rect, app: &App, row: u16, col: u16) -> Option<String> {
    let area = key_bar_area(root);
    KeyBarPanel::hover_hint(area, app, row, col)
}

pub(crate) fn key_bar_hover_token(
    root: Rect,
    app: &App,
    row: u16,
    col: u16,
) -> Option<KeyHoverToken> {
    let area = key_bar_area(root);
    KeyBarPanel::hover_token(area, app, row, col)
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

fn render_key_hover_hint(frame: &mut Frame, root: Rect, app: &App, hint: &str) {
    if hint.is_empty() || root.width < 20 || root.height < 4 {
        return;
    }

    let theme = app.theme();
    let width = (hint.chars().count() as u16 + 2).clamp(18, root.width);
    let x = root.x + root.width.saturating_sub(width);
    let area = Rect {
        x,
        y: root.y + 2,
        width,
        height: 1,
    };

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(Line::from(hint.to_string())).style(
            Style::default()
                .fg(theme.key_hint_key_fg)
                .bg(theme.key_hint_key_bg),
        ),
        area,
    );
}

fn render_key_hover_token(frame: &mut Frame, app: &App, token: &KeyHoverToken) {
    if token.width == 0 {
        return;
    }

    let theme = app.theme();
    let area = Rect {
        x: token.col,
        y: token.row,
        width: token.width,
        height: 1,
    };

    frame.render_widget(
        Paragraph::new(Line::from(token.text.clone())).style(
            Style::default()
                .fg(theme.key_hint_key_bg)
                .bg(theme.key_hint_key_fg)
                .add_modifier(Modifier::BOLD),
        ),
        area,
    );
}
