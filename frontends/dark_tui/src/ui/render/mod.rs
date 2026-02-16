mod components;
mod panels;
mod views;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

use crate::app::{App, ResultsViewMode};

use panels::{ActionsPanel, DetailsPanel, FooterPanel, HeaderPanel};
use views::{ProductsView, SessionsView, UnifiedCatalogView, VariantsView};

pub fn render_dashboard(frame: &mut Frame, app: &App) {
    let root = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(root);

    HeaderPanel::render(frame, vertical[0], app);
    render_body(frame, vertical[1], app);
    FooterPanel::render(frame, vertical[2], app);
}

fn render_body(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    match app.results_view_mode() {
        ResultsViewMode::Table => render_body_table(frame, area, app),
        ResultsViewMode::Viz => render_body_viz(frame, area, app),
    }
}

/// Table mode: 68/32 horizontal split, products+variants on left, details+sessions+actions on right.
fn render_body_table(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
        .split(area);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(56), Constraint::Percentage(44)])
        .split(columns[0]);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(42),
            Constraint::Percentage(28),
            Constraint::Percentage(30),
        ])
        .split(columns[1]);

    ProductsView::render(frame, left[0], app);
    VariantsView::render(frame, left[1], app);
    DetailsPanel::render(frame, right[0], app);
    SessionsView::render(frame, right[1], app);
    ActionsPanel::render(frame, right[2], app);
}

/// Viz mode: 78/22 horizontal split, unified catalog on left, details+sessions+actions on right.
fn render_body_viz(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(78), Constraint::Percentage(22)])
        .split(area);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(42),
            Constraint::Percentage(28),
            Constraint::Percentage(30),
        ])
        .split(columns[1]);

    UnifiedCatalogView::render(frame, columns[0], app);
    DetailsPanel::render(frame, right[0], app);
    SessionsView::render(frame, right[1], app);
    ActionsPanel::render(frame, right[2], app);
}
