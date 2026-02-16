use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Cell, Row, Table, TableState};
use ratatui::Frame;

use crate::app::{App, FocusPane, ResultsViewMode};
use crate::models::{compact_id, compact_locator};

use dark_tui_components::{CardGridComponent, PaneBlockComponent};

pub(crate) struct ProductsView;

impl ProductsView {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        match app.results_view_mode() {
            ResultsViewMode::Table => Self::render_table(frame, area, app),
            ResultsViewMode::Viz => Self::render_cards(frame, area, app),
        }
    }

    fn render_table(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let header = Row::new(vec![
            Cell::from("ID"),
            Cell::from("Name"),
            Cell::from("Status"),
            Cell::from("Branch"),
            Cell::from("Locator"),
        ])
        .style(
            Style::default()
                .fg(theme.table_header_fg)
                .add_modifier(Modifier::BOLD),
        );

        let rows = app.products().iter().map(|product| {
            Row::new(vec![
                Cell::from(compact_id(&product.id)),
                Cell::from(product.display_name.clone()),
                Cell::from(product.status.clone()),
                Cell::from(product.branch.clone()),
                Cell::from(compact_locator(&product.locator, 38)),
            ])
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(14),
                Constraint::Length(22),
                Constraint::Length(8),
                Constraint::Length(14),
                Constraint::Min(20),
            ],
        )
        .header(header)
        .block(PaneBlockComponent::build(
            "Products",
            matches!(app.focus(), FocusPane::Products),
            theme,
        ))
        .row_highlight_style(
            Style::default()
                .fg(theme.table_highlight_fg)
                .bg(theme.table_highlight_bg_product)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" >> ");

        let mut state = TableState::default();
        if !app.products().is_empty() {
            state.select(Some(app.selected_product_index()));
        }

        frame.render_stateful_widget(table, area, &mut state);
    }

    fn render_cards(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let cards = app
            .products()
            .iter()
            .map(|product| {
                (
                    format!("{}  {}", compact_id(&product.id), product.display_name),
                    vec![
                        format!("Status: {}", product.status),
                        format!("Branch: {}", product.branch),
                        format!(
                            "Variants: total={} dirty={} drift={}",
                            product.variant_total, product.variant_dirty, product.variant_drift
                        ),
                        format!("Locator: {}", compact_locator(&product.locator, 42)),
                    ],
                )
            })
            .collect::<Vec<_>>();

        CardGridComponent::render(
            frame,
            area,
            "Products",
            matches!(app.focus(), FocusPane::Products),
            app.selected_product_index(),
            &cards,
            theme.entity_product,
            theme,
        );
    }
}
