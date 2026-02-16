use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Cell, Row, Table, TableState};
use ratatui::Frame;

use crate::app::{App, FocusPane, ResultsViewMode};
use crate::models::{compact_id, compact_timestamp};

use dark_tui_components::{CardGridComponent, PaneBlockComponent};

pub(crate) struct VariantsView;

impl VariantsView {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        match app.results_view_mode() {
            ResultsViewMode::Table => Self::render_table(frame, area, app),
            ResultsViewMode::Viz => Self::render_cards(frame, area, app),
        }
    }

    fn render_table(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let visible_variants = app.visible_variants();
        let header = Row::new(vec![
            Cell::from("Variant"),
            Cell::from("Name"),
            Cell::from("State"),
            Cell::from("A/B"),
            Cell::from("Branch"),
            Cell::from("Worktree"),
            Cell::from("Last Polled"),
        ])
        .style(
            Style::default()
                .fg(theme.table_header_fg)
                .add_modifier(Modifier::BOLD),
        );

        let rows = visible_variants.iter().map(|variant| {
            Row::new(vec![
                Cell::from(compact_id(&variant.id)),
                Cell::from(variant.name.clone()),
                Cell::from(variant.git_state.clone()),
                Cell::from(format!("{}/{}", variant.ahead, variant.behind)),
                Cell::from(variant.branch.clone()),
                Cell::from(variant.worktree.clone()),
                Cell::from(variant.last_polled_at.clone()),
            ])
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(14),
                Constraint::Length(14),
                Constraint::Length(8),
                Constraint::Length(9),
                Constraint::Length(12),
                Constraint::Length(10),
                Constraint::Min(20),
            ],
        )
        .header(header)
        .block(PaneBlockComponent::build(
            "Variants",
            matches!(app.focus(), FocusPane::Variants),
            theme,
        ))
        .row_highlight_style(
            Style::default()
                .fg(theme.table_highlight_fg)
                .bg(theme.table_highlight_bg_variant)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" >> ");

        let mut state = TableState::default();
        if !visible_variants.is_empty() {
            state.select(Some(app.selected_variant_index()));
        }

        frame.render_stateful_widget(table, area, &mut state);
    }

    fn render_cards(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let visible_variants = app.visible_variants();
        let cards = visible_variants
            .iter()
            .map(|variant| {
                (
                    format!("{}  {}", compact_id(&variant.id), variant.name),
                    vec![
                        format!("State: {}", variant.git_state),
                        format!("A/B: {}/{}", variant.ahead, variant.behind),
                        format!("Branch: {}", variant.branch),
                        format!("Worktree: {}", variant.worktree),
                        format!("Polled: {}", compact_timestamp(&variant.last_polled_at)),
                    ],
                )
            })
            .collect::<Vec<_>>();

        CardGridComponent::render(
            frame,
            area,
            "Variants",
            matches!(app.focus(), FocusPane::Variants),
            app.selected_variant_index(),
            &cards,
            theme.entity_variant,
            theme,
        );
    }
}
