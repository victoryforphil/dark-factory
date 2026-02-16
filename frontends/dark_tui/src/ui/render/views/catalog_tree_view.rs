use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState};

use crate::app::{App, VizSelection};
use crate::models::compact_id;
use crate::theme::EntityKind;

use dark_tui_components::{PaneBlockComponent, StatusPill};

pub(crate) struct CatalogTreeView;

fn compact_text(value: &str, max_len: usize) -> String {
    let normalized = value.trim().replace('\n', " ");
    if normalized.len() <= max_len {
        return normalized;
    }

    format!("{}...", &normalized[..max_len.saturating_sub(3)])
}

impl CatalogTreeView {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let nodes = app.catalog_nodes();
        let active_entity = app
            .viz_selection()
            .map(Self::entity_kind)
            .unwrap_or(EntityKind::Product);

        let items = nodes
            .iter()
            .map(|node| Self::item_for_node(app, node))
            .collect::<Vec<_>>();

        let list = List::new(items)
            .block(PaneBlockComponent::build("Catalog Tree", true, theme))
            .highlight_symbol(" >> ")
            .highlight_style(
                Style::default()
                    .bg(theme.table_highlight_bg_for(active_entity))
                    .fg(theme.table_highlight_fg)
                    .add_modifier(Modifier::BOLD),
            );

        let mut state = ListState::default();
        if !nodes.is_empty() {
            let selected = app
                .viz_selection()
                .and_then(|current| nodes.iter().position(|node| node == current))
                .unwrap_or(0);
            state.select(Some(selected));
        }

        frame.render_stateful_widget(list, area, &mut state);
    }

    fn item_for_node(app: &App, node: &VizSelection) -> ListItem<'static> {
        ListItem::new(Line::from(Self::line_for_node(app, node)))
    }

    fn line_for_node(app: &App, node: &VizSelection) -> Vec<Span<'static>> {
        let theme = app.theme();

        match node {
            VizSelection::Product { product_index } => {
                if let Some(product) = app.products().get(*product_index) {
                    let product_name = if product.display_name.trim().is_empty() {
                        "(untitled)".to_string()
                    } else {
                        product.display_name.clone()
                    };
                    let summary = if product.variant_dirty > 0 || product.variant_drift > 0 {
                        StatusPill::warn(
                            format!(
                                "{}v {}dirty {}drift",
                                product.variant_total, product.variant_dirty, product.variant_drift
                            ),
                            theme,
                        )
                    } else {
                        StatusPill::ok(format!("{}v clean", product.variant_total), theme)
                    };

                    vec![
                        Span::styled(
                            "◆ ",
                            Style::default().fg(theme.entity_color(EntityKind::Product)),
                        ),
                        Span::styled(
                            product_name,
                            Style::default()
                                .fg(theme.text_primary)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::raw("  "),
                        Span::styled(
                            compact_id(&product.id),
                            Style::default().fg(theme.text_muted),
                        ),
                        Span::raw("  "),
                        summary.span(),
                    ]
                } else {
                    vec![Span::styled(
                        "◆ <missing product>",
                        Style::default().fg(theme.text_muted),
                    )]
                }
            }
            VizSelection::Variant { variant_id, .. } => {
                if let Some(variant) = app
                    .variants()
                    .iter()
                    .find(|variant| variant.id == *variant_id)
                {
                    let state_pill = match variant.git_state.as_str() {
                        "clean" => StatusPill::ok("clean", theme),
                        "dirty" => StatusPill::warn("dirty", theme),
                        "no-git" => StatusPill::muted("no-git", theme),
                        _ => StatusPill::muted(&variant.git_state, theme),
                    };

                    vec![
                        Span::styled("  ├─ ", Style::default().fg(theme.catalog_connector)),
                        Span::styled(
                            "◈ ",
                            Style::default().fg(theme.entity_color(EntityKind::Variant)),
                        ),
                        Span::styled(
                            variant.name.clone(),
                            Style::default().fg(theme.text_primary),
                        ),
                        Span::raw("  "),
                        state_pill.span(),
                        Span::raw(" "),
                        Span::styled(
                            format!("{}/{}", variant.ahead, variant.behind),
                            Style::default().fg(theme.text_muted),
                        ),
                    ]
                } else {
                    vec![Span::styled(
                        "  ├─ ◈ <missing variant>",
                        Style::default().fg(theme.text_muted),
                    )]
                }
            }
            VizSelection::Actor { actor_id, .. } => {
                if let Some(actor) = app.actors().iter().find(|actor| actor.id == *actor_id) {
                    let status_pill = match actor.status.as_str() {
                        "active" | "running" => StatusPill::ok(&actor.status, theme),
                        "error" | "failed" | "dead" => StatusPill::error(&actor.status, theme),
                        "idle" | "waiting" => StatusPill::warn(&actor.status, theme),
                        _ => StatusPill::muted(&actor.status, theme),
                    };
                    let title = if actor.title.trim().is_empty() {
                        compact_id(&actor.id)
                    } else {
                        actor.title.clone()
                    };
                    let description =
                        if actor.description.trim().is_empty() || actor.description.trim() == "-" {
                            String::new()
                        } else {
                            format!(" -- {}", compact_text(&actor.description, 56))
                        };

                    vec![
                        Span::styled("    └─ ", Style::default().fg(theme.catalog_connector)),
                        Span::styled(
                            "● ",
                            Style::default().fg(theme.entity_color(EntityKind::Actor)),
                        ),
                        Span::styled(title, Style::default().fg(theme.text_secondary)),
                        Span::styled(description, Style::default().fg(theme.text_muted)),
                        Span::raw("  "),
                        StatusPill::info(&actor.provider, theme).span(),
                        Span::raw(" "),
                        status_pill.span(),
                    ]
                } else {
                    vec![Span::styled(
                        "    └─ ● <missing actor>",
                        Style::default().fg(theme.text_muted),
                    )]
                }
            }
        }
    }

    fn entity_kind(node: &VizSelection) -> EntityKind {
        match node {
            VizSelection::Product { .. } => EntityKind::Product,
            VizSelection::Variant { .. } => EntityKind::Variant,
            VizSelection::Actor { .. } => EntityKind::Actor,
        }
    }
}
