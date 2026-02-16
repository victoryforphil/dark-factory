use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Cell, Row, Table, TableState};
use ratatui::Frame;

use crate::app::{App, FocusPane, ResultsViewMode};
use crate::models::{compact_id, compact_locator, compact_timestamp};

use dark_tui_components::{CardGridComponent, PaneBlockComponent};

pub(crate) struct SessionsView;

impl SessionsView {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        match app.results_view_mode() {
            ResultsViewMode::Table => Self::render_table(frame, area, app),
            ResultsViewMode::Viz => Self::render_cards(frame, area, app),
        }
    }

    fn render_table(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let header = Row::new(vec![
            Cell::from("Actor"),
            Cell::from("Title"),
            Cell::from("Status"),
            Cell::from("Updated"),
        ])
        .style(
            Style::default()
                .fg(theme.table_header_fg)
                .add_modifier(Modifier::BOLD),
        );

        let rows = app.actors().iter().map(|actor| {
            Row::new(vec![
                Cell::from(compact_id(&actor.id)),
                Cell::from(actor.title.clone()),
                Cell::from(actor.status.clone()),
                Cell::from(actor.updated_at.clone()),
            ])
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(14),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ],
        )
        .header(header)
        .block(PaneBlockComponent::build(
            "Actors",
            matches!(app.focus(), FocusPane::Sessions),
            theme,
        ))
        .row_highlight_style(
            Style::default()
                .fg(theme.table_highlight_fg)
                .bg(theme.table_highlight_bg_actor)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" >> ");

        let mut state = TableState::default();
        if !app.actors().is_empty() {
            state.select(Some(app.selected_actor_index()));
        }

        frame.render_stateful_widget(table, area, &mut state);
    }

    fn render_cards(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let cards = app
            .actors()
            .iter()
            .map(|actor| {
                (
                    format!("{}  {}", compact_id(&actor.id), actor.title),
                    vec![
                        format!("Updated: {}", compact_timestamp(&actor.updated_at)),
                        format!("Created: {}", compact_timestamp(&actor.created_at)),
                        format!("Provider: {}", actor.provider),
                        format!("Status: {}", actor.status),
                        format!("Dir: {}", compact_locator(&actor.directory, 42)),
                    ],
                )
            })
            .collect::<Vec<_>>();

        CardGridComponent::render(
            frame,
            area,
            "Actors",
            matches!(app.focus(), FocusPane::Sessions),
            app.selected_actor_index(),
            &cards,
            theme.entity_actor,
            theme,
        );
    }
}
