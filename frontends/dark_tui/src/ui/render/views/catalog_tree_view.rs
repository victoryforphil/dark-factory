use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState};

use crate::app::{App, VizSelection};
use crate::models::compact_id;
use crate::theme::EntityKind;
use crate::ui::render::components::{sub_agent_badge, sub_agent_tree_line};

use dark_tui_components::{PaneBlockComponent, StatusPill, compact_text_normalized};

pub(crate) struct CatalogTreeView;
const MAX_SUB_AGENT_ROWS: usize = 12;

/// A display row in the tree list — either a selectable node or a
/// decorative (non-selectable) sub-agent line.
enum TreeRow {
    /// Maps to a `VizSelection` node at `node_index` in the selectable list.
    Selectable { node_index: usize },
    /// Decorative sub-agent entry — skipped by selection logic.
    SubAgent,
}

impl CatalogTreeView {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let nodes = app.catalog_nodes();
        let active_entity = app
            .viz_selection()
            .map(Self::entity_kind)
            .unwrap_or(EntityKind::Product);

        // Build display rows: selectable nodes interleaved with decorative
        // sub-agent lines that sit beneath their parent actor row.
        let mut items: Vec<ListItem<'static>> = Vec::new();
        let mut row_map: Vec<TreeRow> = Vec::new();

        for (ni, node) in nodes.iter().enumerate() {
            items.push(Self::item_for_node(app, node));
            row_map.push(TreeRow::Selectable { node_index: ni });

            // After an actor node, append its sub-agent lines (if any).
            if let VizSelection::Actor { actor_id, .. } = node {
                if let Some(actor) = app.actors().iter().find(|a| a.id == *actor_id) {
                    let sub_agents = &actor.sub_agents;
                    let visible_sub_agents = recent_sub_agents(sub_agents);
                    for (si, agent) in visible_sub_agents.iter().enumerate() {
                        let is_last = si == visible_sub_agents.len() - 1;
                        let line = sub_agent_tree_line(agent, "       ", is_last, theme);
                        items.push(ListItem::new(line));
                        row_map.push(TreeRow::SubAgent);
                    }
                }
            }
        }

        let list = List::new(items)
            .block(PaneBlockComponent::build("Catalog Tree", true, theme))
            .highlight_symbol(" >> ")
            .highlight_style(
                Style::default()
                    .bg(theme.table_highlight_bg_for(active_entity))
                    .fg(theme.table_highlight_fg)
                    .add_modifier(Modifier::BOLD),
            );

        // Map the currently selected selectable node to its display-row index.
        let mut state = ListState::default();
        if !nodes.is_empty() {
            let selected_node = app
                .viz_selection()
                .and_then(|current| nodes.iter().position(|node| node == current))
                .unwrap_or(0);
            // Find the display-row index that corresponds to this selectable node.
            let display_idx = row_map
                .iter()
                .position(|r| matches!(r, TreeRow::Selectable { node_index } if *node_index == selected_node))
                .unwrap_or(0);
            state.select(Some(display_idx));
        }

        frame.render_stateful_widget(list, area, &mut state);
    }

    pub(crate) fn hit_test(area: Rect, app: &App, col: u16, row: u16) -> Option<VizSelection> {
        let block = PaneBlockComponent::build("Catalog Tree", true, app.theme());
        let inner = block.inner(area);

        if col < inner.x
            || col >= inner.x + inner.width
            || row < inner.y
            || row >= inner.y + inner.height
        {
            return None;
        }

        let nodes = app.catalog_nodes();
        if nodes.is_empty() {
            return None;
        }

        let row_map = Self::display_row_map(app, &nodes);
        let row_index = row.saturating_sub(inner.y) as usize;
        let Some(row_kind) = row_map.get(row_index) else {
            return None;
        };

        match row_kind {
            TreeRow::Selectable { node_index } => nodes.get(*node_index).cloned(),
            TreeRow::SubAgent => None,
        }
    }

    fn display_row_map(app: &App, nodes: &[VizSelection]) -> Vec<TreeRow> {
        let mut row_map: Vec<TreeRow> = Vec::new();

        for (node_index, node) in nodes.iter().enumerate() {
            row_map.push(TreeRow::Selectable { node_index });

            if let VizSelection::Actor { actor_id, .. } = node {
                if let Some(actor) = app.actors().iter().find(|actor| actor.id == *actor_id) {
                    for _ in recent_sub_agents(&actor.sub_agents) {
                        row_map.push(TreeRow::SubAgent);
                    }
                }
            }
        }

        row_map
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
                    let state_pill = variant_state_pill(&variant.git_state, theme);
                    let branch = if variant.branch.trim().is_empty() {
                        "-"
                    } else {
                        variant.branch.as_str()
                    };
                    let ahead_behind = ahead_behind_pill(variant.ahead, variant.behind, theme);

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
                        StatusPill::info(format!(" {branch}"), theme).span(),
                        Span::raw(" "),
                        ahead_behind.span(),
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
                    let description = app
                        .actor_last_message_preview(&actor.id)
                        .map(|value| value.trim())
                        .filter(|value| !value.is_empty())
                        .map(|value| format!(" -- {}", compact_text_normalized(value, 56)))
                        .or_else(|| {
                            let fallback = actor.description.trim();
                            if fallback.is_empty() || fallback == "-" {
                                None
                            } else {
                                Some(format!(" -- {}", compact_text_normalized(fallback, 56)))
                            }
                        })
                        .unwrap_or_default();

                    let mut spans = vec![
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
                    ];
                    if let Some(badge) = sub_agent_badge(actor.sub_agent_count(), theme) {
                        spans.push(Span::raw(" "));
                        spans.push(badge);
                    }
                    spans
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

fn recent_sub_agents(sub_agents: &[crate::models::SubAgentRow]) -> &[crate::models::SubAgentRow] {
    let start = sub_agents.len().saturating_sub(MAX_SUB_AGENT_ROWS);
    &sub_agents[start..]
}

fn variant_state_pill(state: &str, theme: &crate::theme::Theme) -> StatusPill {
    match state {
        "clean" => StatusPill::ok("clean", theme),
        "dirty" => StatusPill::warn("dirty", theme),
        "no-git" => StatusPill::muted("no-git", theme),
        _ => StatusPill::muted(state, theme),
    }
}

fn ahead_behind_pill(ahead: u64, behind: u64, theme: &crate::theme::Theme) -> StatusPill {
    if behind > 0 {
        StatusPill::warn(format!("+{ahead}/-{behind}"), theme)
    } else if ahead > 0 {
        StatusPill::ok(format!("+{ahead}/-0"), theme)
    } else {
        StatusPill::muted("+0/-0", theme)
    }
}
