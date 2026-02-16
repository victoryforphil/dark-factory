mod components;
mod panels;
mod views;

use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

use crate::app::{App, ResultsViewMode};

use panels::{DetailsPanel, FooterPanel, HeaderPanel, KeyBarPanel, SpawnFormPanel};
use views::{CatalogTreeView, UnifiedCatalogView};

/// Main layout constants — tuned for readability on 80-col and wider terminals.
const SIDEBAR_PERCENT: u16 = 24;
const MAIN_PERCENT: u16 = 76;

pub fn render_dashboard(frame: &mut Frame, app: &App) {
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

    if app.is_spawn_form_open() {
        SpawnFormPanel::render(frame, root, app);
    }
}

fn render_body(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    match app.results_view_mode() {
        ResultsViewMode::Table => render_body_table(frame, area, app),
        ResultsViewMode::Viz => render_body_viz(frame, area, app),
    }
}

/// Table mode: main/sidebar split. Details panel fills the entire sidebar.
fn render_body_table(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(MAIN_PERCENT),
            Constraint::Percentage(SIDEBAR_PERCENT),
        ])
        .split(area);

    CatalogTreeView::render(frame, columns[0], app);
    DetailsPanel::render(frame, columns[1], app);
}

/// Viz mode: main/sidebar split. Details panel fills the entire sidebar.
fn render_body_viz(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(MAIN_PERCENT),
            Constraint::Percentage(SIDEBAR_PERCENT),
        ])
        .split(area);

    UnifiedCatalogView::render(frame, columns[0], app);
    DetailsPanel::render(frame, columns[1], app);
}

pub(crate) fn try_select_viz_node(root: Rect, app: &mut App, col: u16, row: u16) -> bool {
    let body = body_area(root);
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(MAIN_PERCENT),
            Constraint::Percentage(SIDEBAR_PERCENT),
        ])
        .split(body);
    UnifiedCatalogView::click_select(columns[0], app, col, row)
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
