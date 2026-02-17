use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use dark_tui_components::{PaneBlockComponent, StatusPill};

use crate::app::{App, VizSelection};
use crate::models::{compact_id, compact_locator, VariantRow};
use crate::theme::Theme;
use crate::ui::render::components::{sub_agent_badge, sub_agent_token_rows};

use super::catalog_cards::ProductGroup;
use super::unified_catalog_view::UnifiedCatalogView;

const GRAPH_MIN_WIDTH: u16 = 80;
const GRAPH_MIN_HEIGHT: u16 = 10;
const NODE_PANEL_TITLE: &str = "Catalog Graphical Node";

const PRODUCT_W: u16 = 30;
const PRODUCT_H: u16 = 4;
const PRODUCT_X: i32 = 2;

const PRODUCT_TO_VARIANT_GAP: i32 = 7;
const VARIANT_W: u16 = 24;
const VARIANT_H: u16 = 4;
const VARIANT_COL_GAP: i32 = 4;
const VARIANT_ROW_GAP: i32 = 2;

const VARIANT_TO_ACTOR_OFFSET_X: i32 = 3;
const ACTOR_W: u16 = 28;
const ACTOR_H: u16 = 4;
const ACTOR_ROW_GAP: i32 = 1;

const GROUP_GAP: i32 = 2;

pub(crate) struct GraphCatalogView;

#[derive(Clone, Copy)]
struct WorldRect {
    x: i32,
    y: i32,
    width: u16,
    height: u16,
}

impl WorldRect {
    fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x
            && x < self.x + self.width as i32
            && y >= self.y
            && y < self.y + self.height as i32
    }
}

struct GraphLayout {
    groups: Vec<ProductLayout>,
    world_width: i32,
}

struct ProductLayout {
    product_index: usize,
    rect: WorldRect,
    trunk_x: i32,
    bottom_y: i32,
    variants: Vec<VariantLayout>,
}

struct VariantLayout {
    product_index: usize,
    variant_id: String,
    rect: WorldRect,
    actor_trunk_x: i32,
    actors: Vec<ActorLayout>,
}

struct ActorLayout {
    product_index: usize,
    variant_id: String,
    actor_id: String,
    rect: WorldRect,
    sub_agent_rows: usize,
}

impl GraphCatalogView {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        if area.width < GRAPH_MIN_WIDTH || area.height < GRAPH_MIN_HEIGHT {
            UnifiedCatalogView::render(frame, area, app);
            return;
        }

        let theme = app.theme();
        let panel = PaneBlockComponent::build(NODE_PANEL_TITLE, true, theme);
        let inner = panel.inner(area);
        frame.render_widget(panel, area);

        if inner.width < 20 || inner.height < 6 {
            return;
        }

        if app.products().is_empty() {
            let empty = Paragraph::new("No products")
                .style(Style::default().fg(theme.text_muted))
                .wrap(Wrap { trim: true });
            frame.render_widget(empty, inner);
            return;
        }

        let (offset_x, offset_y) = app.viz_offset();
        let layout = Self::build_layout(app, inner.width);

        Self::render_edges(
            frame.buffer_mut(),
            inner,
            offset_x,
            offset_y,
            app,
            &layout,
            theme,
        );
        Self::render_group_separators(
            frame.buffer_mut(),
            inner,
            offset_x,
            offset_y,
            &layout,
            theme,
        );

        for group in &layout.groups {
            Self::render_product_node(frame, inner, offset_x, offset_y, app, group, theme);

            for variant in &group.variants {
                Self::render_variant_node(frame, inner, offset_x, offset_y, app, variant, theme);

                for actor in &variant.actors {
                    Self::render_actor_node(frame, inner, offset_x, offset_y, app, actor, theme);
                    Self::render_sub_agents(frame, inner, offset_x, offset_y, app, actor, theme);
                }
            }
        }

        Self::render_legend(frame, inner, theme);

        if offset_x != 0 || offset_y != 0 {
            let hint = format!("pan({offset_x},{offset_y}) [0]=reset");
            let hint_len = hint.chars().count() as u16;
            if hint_len < inner.width {
                let hint_area = Rect {
                    x: inner.x + inner.width - hint_len,
                    y: inner.y + inner.height - 1,
                    width: hint_len,
                    height: 1,
                };
                let hint_widget = Paragraph::new(hint).style(Style::default().fg(theme.text_muted));
                frame.render_widget(hint_widget, hint_area);
            }
        }
    }

    pub(crate) fn click_select(area: Rect, app: &mut App, col: u16, row: u16) -> bool {
        let Some(selection) = Self::hit_test(area, app, col, row) else {
            return false;
        };

        app.set_viz_selection(selection);
        true
    }

    pub(crate) fn hit_test(area: Rect, app: &App, col: u16, row: u16) -> Option<VizSelection> {
        if area.width < GRAPH_MIN_WIDTH || area.height < GRAPH_MIN_HEIGHT {
            return UnifiedCatalogView::hit_test(area, app, col, row);
        }

        let panel = PaneBlockComponent::build(NODE_PANEL_TITLE, true, app.theme());
        let inner = panel.inner(area);

        if inner.width < 20
            || inner.height < 6
            || app.products().is_empty()
            || col < inner.x
            || col >= inner.x + inner.width
            || row < inner.y
            || row >= inner.y + inner.height
        {
            return None;
        }

        let (offset_x, offset_y) = app.viz_offset();
        let world_x = (col - inner.x) as i32 - offset_x;
        let world_y = (row - inner.y) as i32 - offset_y;

        let layout = Self::build_layout(app, inner.width);
        for group in &layout.groups {
            if group.rect.contains(world_x, world_y) {
                return Some(VizSelection::Product {
                    product_index: group.product_index,
                });
            }

            for variant in &group.variants {
                if variant.rect.contains(world_x, world_y) {
                    return Some(VizSelection::Variant {
                        product_index: variant.product_index,
                        variant_id: variant.variant_id.clone(),
                    });
                }

                for actor in &variant.actors {
                    if actor.rect.contains(world_x, world_y) {
                        return Some(VizSelection::Actor {
                            product_index: actor.product_index,
                            variant_id: actor.variant_id.clone(),
                            actor_id: actor.actor_id.clone(),
                        });
                    }
                }
            }
        }

        None
    }

    fn build_layout(app: &App, inner_width: u16) -> GraphLayout {
        let groups = Self::product_groups(app);
        let variant_cols = Self::variant_columns(inner_width).max(1);
        let variant_col_span =
            (VARIANT_W as i32).max(VARIANT_TO_ACTOR_OFFSET_X + ACTOR_W as i32) + VARIANT_COL_GAP;

        let mut layouts: Vec<ProductLayout> = Vec::with_capacity(groups.len());
        let mut cursor_y: i32 = 0;
        let mut max_world_x: i32 = PRODUCT_X + PRODUCT_W as i32;

        for group in groups {
            let product_rect = WorldRect {
                x: PRODUCT_X,
                y: cursor_y,
                width: PRODUCT_W,
                height: PRODUCT_H,
            };
            let variant_base_x =
                product_rect.x + product_rect.width as i32 + PRODUCT_TO_VARIANT_GAP;

            struct VariantBlueprint {
                variant_id: String,
                actors: Vec<(String, usize)>,
                cluster_height: i32,
            }

            let mut blueprints: Vec<VariantBlueprint> = Vec::with_capacity(group.variants.len());
            for variant in &group.variants {
                let actors = app.actors_for_variant(&variant.id);
                let mut actor_specs: Vec<(String, usize)> = Vec::with_capacity(actors.len());
                let mut actor_height_total: i32 = 0;
                for actor in actors {
                    let sub_rows = sub_agent_token_rows(
                        &actor.sub_agents,
                        ACTOR_W.saturating_sub(2),
                        2,
                        app.theme(),
                    )
                    .len();
                    actor_specs.push((actor.id.clone(), sub_rows));
                    actor_height_total += ACTOR_H as i32 + sub_rows as i32 + ACTOR_ROW_GAP;
                }
                if actor_height_total > 0 {
                    actor_height_total -= ACTOR_ROW_GAP;
                }

                let cluster_height = VARIANT_H as i32
                    + if actor_specs.is_empty() {
                        0
                    } else {
                        1 + actor_height_total
                    };

                blueprints.push(VariantBlueprint {
                    variant_id: variant.id.clone(),
                    actors: actor_specs,
                    cluster_height,
                });
            }

            let mut variants: Vec<VariantLayout> = Vec::with_capacity(blueprints.len());
            let mut row_start_y = cursor_y;
            for row in blueprints.chunks(variant_cols) {
                let row_height = row
                    .iter()
                    .map(|variant| variant.cluster_height)
                    .max()
                    .unwrap_or(VARIANT_H as i32);

                for (index_in_row, blueprint) in row.iter().enumerate() {
                    let variant_x = variant_base_x + index_in_row as i32 * variant_col_span;
                    let variant_rect = WorldRect {
                        x: variant_x,
                        y: row_start_y,
                        width: VARIANT_W,
                        height: VARIANT_H,
                    };

                    let mut actors: Vec<ActorLayout> = Vec::with_capacity(blueprint.actors.len());
                    let mut actor_y = row_start_y + VARIANT_H as i32 + 1;
                    for (actor_id, sub_agent_rows) in &blueprint.actors {
                        let rect = WorldRect {
                            x: variant_x + VARIANT_TO_ACTOR_OFFSET_X,
                            y: actor_y,
                            width: ACTOR_W,
                            height: ACTOR_H,
                        };

                        actors.push(ActorLayout {
                            product_index: group.product_index,
                            variant_id: blueprint.variant_id.clone(),
                            actor_id: actor_id.clone(),
                            rect,
                            sub_agent_rows: *sub_agent_rows,
                        });

                        actor_y += ACTOR_H as i32 + *sub_agent_rows as i32 + ACTOR_ROW_GAP;
                    }

                    variants.push(VariantLayout {
                        product_index: group.product_index,
                        variant_id: blueprint.variant_id.clone(),
                        rect: variant_rect,
                        actor_trunk_x: variant_rect.x + variant_rect.width as i32 + 1,
                        actors,
                    });

                    max_world_x = max_world_x.max(variant_rect.x + variant_rect.width as i32 + 2);
                    if let Some(last_actor) =
                        variants.last().and_then(|variant| variant.actors.last())
                    {
                        max_world_x =
                            max_world_x.max(last_actor.rect.x + last_actor.rect.width as i32 + 2);
                    }
                }

                row_start_y += row_height + VARIANT_ROW_GAP;
            }

            let variants_bottom = if variants.is_empty() {
                cursor_y
            } else {
                row_start_y - VARIANT_ROW_GAP
            };

            let product_bottom = cursor_y + PRODUCT_H as i32;
            let group_bottom = product_bottom.max(variants_bottom);

            layouts.push(ProductLayout {
                product_index: group.product_index,
                rect: product_rect,
                trunk_x: product_rect.x + product_rect.width as i32 + 2,
                bottom_y: group_bottom,
                variants,
            });

            cursor_y = group_bottom + GROUP_GAP;
        }

        GraphLayout {
            groups: layouts,
            world_width: max_world_x,
        }
    }

    fn product_groups<'a>(app: &'a App) -> Vec<ProductGroup<'a>> {
        app.products()
            .iter()
            .enumerate()
            .map(|(product_index, product)| {
                let variants: Vec<&VariantRow> = app
                    .variants()
                    .iter()
                    .filter(|variant| variant.product_id == product.id)
                    .collect();

                ProductGroup {
                    product,
                    product_index,
                    variants,
                }
            })
            .collect()
    }

    fn variant_columns(inner_width: u16) -> usize {
        match inner_width {
            0..=109 => 1,
            110..=159 => 2,
            _ => 3,
        }
    }

    fn render_edges(
        buf: &mut Buffer,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        layout: &GraphLayout,
        theme: &Theme,
    ) {
        let product_edge = Style::default().fg(Self::dim_color(theme.entity_product));
        let variant_edge = Style::default().fg(Self::dim_color(theme.entity_variant));

        for group in &layout.groups {
            if group.variants.is_empty() {
                continue;
            }

            let product_mid_y = group.rect.y + 1;
            let product_right_x = group.rect.x + group.rect.width as i32;
            let variant_centers: Vec<i32> = group
                .variants
                .iter()
                .map(|variant| variant.rect.y + 1)
                .collect();

            let min_vy = variant_centers
                .iter()
                .copied()
                .min()
                .unwrap_or(product_mid_y)
                .min(product_mid_y);
            let max_vy = variant_centers
                .iter()
                .copied()
                .max()
                .unwrap_or(product_mid_y)
                .max(product_mid_y);

            Self::draw_world_hline(
                buf,
                inner,
                offset_x,
                offset_y,
                product_mid_y,
                product_right_x,
                group.trunk_x,
                product_edge,
                "─",
            );
            Self::draw_world_vline(
                buf,
                inner,
                offset_x,
                offset_y,
                group.trunk_x,
                min_vy,
                max_vy,
                product_edge,
                "│",
            );

            let mut variant_refs: Vec<&VariantLayout> = group.variants.iter().collect();
            variant_refs.sort_by_key(|variant| variant.rect.y);

            for (index, variant) in variant_refs.iter().enumerate() {
                let variant_mid_y = variant.rect.y + 1;
                let junction = if index == variant_refs.len().saturating_sub(1) {
                    "└"
                } else {
                    "├"
                };
                Self::draw_world_char(
                    buf,
                    inner,
                    offset_x,
                    offset_y,
                    group.trunk_x,
                    variant_mid_y,
                    junction,
                    product_edge,
                );
                Self::draw_world_hline(
                    buf,
                    inner,
                    offset_x,
                    offset_y,
                    variant_mid_y,
                    group.trunk_x + 1,
                    variant.rect.x - 1,
                    product_edge,
                    "─",
                );

                if variant.actors.is_empty() {
                    continue;
                }

                let variant_anchor_y = variant.rect.y + 1;
                let variant_right = variant.rect.x + variant.rect.width as i32;
                let actor_centers: Vec<i32> = variant
                    .actors
                    .iter()
                    .map(|actor| actor.rect.y + 1)
                    .collect();

                let min_ay = actor_centers
                    .iter()
                    .copied()
                    .min()
                    .unwrap_or(variant_anchor_y)
                    .min(variant_anchor_y);
                let max_ay = actor_centers
                    .iter()
                    .copied()
                    .max()
                    .unwrap_or(variant_anchor_y)
                    .max(variant_anchor_y);

                Self::draw_world_hline(
                    buf,
                    inner,
                    offset_x,
                    offset_y,
                    variant_anchor_y,
                    variant_right,
                    variant.actor_trunk_x,
                    variant_edge,
                    "─",
                );
                Self::draw_world_vline(
                    buf,
                    inner,
                    offset_x,
                    offset_y,
                    variant.actor_trunk_x,
                    min_ay,
                    max_ay,
                    variant_edge,
                    "│",
                );

                for (actor_index, actor) in variant.actors.iter().enumerate() {
                    let actor_mid_y = actor.rect.y + 1;
                    let actor_junction = if actor_index == variant.actors.len().saturating_sub(1) {
                        "└"
                    } else {
                        "├"
                    };

                    Self::draw_world_char(
                        buf,
                        inner,
                        offset_x,
                        offset_y,
                        variant.actor_trunk_x,
                        actor_mid_y,
                        actor_junction,
                        variant_edge,
                    );
                    Self::draw_world_hline(
                        buf,
                        inner,
                        offset_x,
                        offset_y,
                        actor_mid_y,
                        variant.actor_trunk_x + 1,
                        actor.rect.x - 1,
                        variant_edge,
                        "─",
                    );
                }
            }
        }
    }

    fn render_group_separators(
        buf: &mut Buffer,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        layout: &GraphLayout,
        theme: &Theme,
    ) {
        if layout.groups.len() < 2 {
            return;
        }

        let style = Style::default().fg(Self::dim_color(theme.catalog_connector));
        for group in layout
            .groups
            .iter()
            .take(layout.groups.len().saturating_sub(1))
        {
            Self::draw_world_hline(
                buf,
                inner,
                offset_x,
                offset_y,
                group.bottom_y + 1,
                0,
                layout.world_width,
                style,
                "─",
            );
        }
    }

    fn render_product_node(
        frame: &mut Frame,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        app: &App,
        group: &ProductLayout,
        theme: &Theme,
    ) {
        let Some(product) = app.products().get(group.product_index) else {
            return;
        };
        let Some(area) = Self::world_rect_to_screen(inner, offset_x, offset_y, group.rect) else {
            return;
        };

        let viz_sel = app.viz_selection();
        let is_selected = matches!(
            viz_sel,
            Some(VizSelection::Product { product_index }) if *product_index == group.product_index
        );
        let is_active = matches!(
            viz_sel,
            Some(VizSelection::Product { product_index }
                | VizSelection::Variant { product_index, .. }
                | VizSelection::Actor { product_index, .. })
            if *product_index == group.product_index
        );

        let product_color = if product.is_git_repo {
            theme.entity_variant
        } else {
            theme.entity_product
        };
        let border = if is_selected {
            Style::default()
                .fg(product_color)
                .add_modifier(Modifier::BOLD)
        } else if is_active {
            Style::default().fg(product_color)
        } else {
            Style::default().fg(Self::dim_color(product_color))
        };

        let title = if is_selected {
            format!("◆ {}", product.display_name)
        } else {
            format!("Product {}", product.display_name)
        };

        let status_pill = match product.status.as_str() {
            "active" | "clean" => StatusPill::ok(&product.status, theme),
            "dirty" => StatusPill::warn("dirty", theme),
            "error" | "failed" => StatusPill::error(&product.status, theme),
            _ => StatusPill::muted(&product.status, theme),
        };

        let variants_pill = if product.variant_dirty > 0 || product.variant_drift > 0 {
            StatusPill::warn(
                format!(
                    "{}v {}d {}dr",
                    product.variant_total, product.variant_dirty, product.variant_drift
                ),
                theme,
            )
        } else {
            StatusPill::muted(format!("{}v", product.variant_total), theme)
        };

        let content = vec![
            Line::from(vec![
                status_pill.span(),
                Span::raw(" "),
                variants_pill.span(),
            ]),
            Line::from(vec![
                Span::styled(
                    compact_locator(&product.locator, 22),
                    Style::default().fg(theme.text_muted),
                ),
                Span::raw(" "),
                Span::styled(
                    compact_id(&product.id),
                    Style::default().fg(theme.text_muted),
                ),
            ]),
        ];

        let widget = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border)
                    .title(title),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, area);
    }

    fn render_variant_node(
        frame: &mut Frame,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        app: &App,
        variant_layout: &VariantLayout,
        theme: &Theme,
    ) {
        let Some(variant) = app
            .variants()
            .iter()
            .find(|variant| variant.id == variant_layout.variant_id)
        else {
            return;
        };

        let Some(area) = Self::world_rect_to_screen(inner, offset_x, offset_y, variant_layout.rect)
        else {
            return;
        };

        let viz_sel = app.viz_selection();
        let is_selected = matches!(
            viz_sel,
            Some(VizSelection::Variant {
                product_index,
                variant_id
            }) if *product_index == variant_layout.product_index && *variant_id == variant_layout.variant_id
        );
        let is_active = is_selected
            || matches!(
                viz_sel,
                Some(VizSelection::Actor {
                    product_index,
                    variant_id,
                    ..
                }) if *product_index == variant_layout.product_index && *variant_id == variant_layout.variant_id
            );

        let border = if is_selected {
            Style::default()
                .fg(theme.entity_variant)
                .add_modifier(Modifier::BOLD)
        } else if is_active {
            Style::default().fg(theme.entity_variant)
        } else {
            Style::default().fg(Self::dim_color(theme.entity_variant))
        };

        let state_pill = match variant.git_state.as_str() {
            "clean" => StatusPill::ok("clean", theme),
            "dirty" => StatusPill::warn("dirty", theme),
            "no-git" => StatusPill::muted("no-git", theme),
            _ => StatusPill::muted(&variant.git_state, theme),
        };
        let drift_text = format!("+{} -{}", variant.ahead, variant.behind);

        let title = if is_selected {
            format!("◈ {}", variant.name)
        } else {
            variant.name.clone()
        };

        let content = vec![
            Line::from(vec![
                state_pill.span(),
                Span::raw(" "),
                Span::styled(
                    variant.branch.clone(),
                    Style::default().fg(theme.text_secondary),
                ),
            ]),
            Line::from(vec![Span::styled(
                drift_text,
                Style::default().fg(theme.text_muted),
            )]),
        ];

        let widget = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border)
                    .title(title),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, area);
    }

    fn render_actor_node(
        frame: &mut Frame,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        app: &App,
        actor_layout: &ActorLayout,
        theme: &Theme,
    ) {
        let Some(actor) = app
            .actors()
            .iter()
            .find(|actor| actor.id == actor_layout.actor_id)
        else {
            return;
        };

        let Some(area) = Self::world_rect_to_screen(inner, offset_x, offset_y, actor_layout.rect)
        else {
            return;
        };

        let is_selected = matches!(
            app.viz_selection(),
            Some(VizSelection::Actor { actor_id, .. }) if *actor_id == actor_layout.actor_id
        );
        let border = if is_selected {
            Style::default()
                .fg(theme.entity_actor)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Self::dim_color(theme.entity_actor))
        };

        let title = if actor.title.trim().is_empty() {
            compact_id(&actor.id)
        } else {
            actor.title.clone()
        };

        let status_pill = match actor.status.as_str() {
            "active" | "running" => StatusPill::ok(&actor.status, theme),
            "error" | "failed" | "dead" => StatusPill::error(&actor.status, theme),
            "idle" | "waiting" => StatusPill::warn(&actor.status, theme),
            _ => StatusPill::muted(&actor.status, theme),
        };

        let mut spans = vec![
            StatusPill::info(&actor.provider, theme).span(),
            Span::raw(" "),
            status_pill.span(),
        ];
        if let Some(badge) = sub_agent_badge(actor.sub_agent_count(), theme) {
            spans.push(Span::raw(" "));
            spans.push(badge);
        }

        let content = vec![
            Line::from(vec![Span::styled(
                title,
                Style::default().fg(theme.text_primary),
            )]),
            Line::from(spans),
        ];

        let widget = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border)
                    .title(if is_selected {
                        format!("● {}", compact_id(&actor.id))
                    } else {
                        compact_id(&actor.id)
                    }),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, area);
    }

    fn render_sub_agents(
        frame: &mut Frame,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        app: &App,
        actor_layout: &ActorLayout,
        theme: &Theme,
    ) {
        if actor_layout.sub_agent_rows == 0 {
            return;
        }

        let Some(actor) = app
            .actors()
            .iter()
            .find(|actor| actor.id == actor_layout.actor_id)
        else {
            return;
        };

        let rows = sub_agent_token_rows(&actor.sub_agents, actor_layout.rect.width, 2, theme);
        let base_y = actor_layout.rect.y + actor_layout.rect.height as i32;

        for (row_index, line) in rows.into_iter().enumerate() {
            let world_y = base_y + row_index as i32;
            let screen_y = inner.y as i32 + world_y + offset_y;
            if screen_y < inner.y as i32 || screen_y >= (inner.y + inner.height) as i32 {
                continue;
            }

            let screen_x = inner.x as i32 + actor_layout.rect.x + offset_x;
            if screen_x < inner.x as i32
                || screen_x + actor_layout.rect.width as i32 > (inner.x + inner.width) as i32
            {
                continue;
            }

            let area = Rect {
                x: screen_x as u16,
                y: screen_y as u16,
                width: actor_layout.rect.width,
                height: 1,
            };
            frame.render_widget(Paragraph::new(line), area);
        }
    }

    fn render_legend(frame: &mut Frame, inner: Rect, theme: &Theme) {
        if inner.width < 42 || inner.height < 2 {
            return;
        }

        let legend = Line::from(vec![
            Span::styled("◆", Style::default().fg(theme.entity_product)),
            Span::styled(" product  ", Style::default().fg(theme.text_muted)),
            Span::styled("◈", Style::default().fg(theme.entity_variant)),
            Span::styled(" variant  ", Style::default().fg(theme.text_muted)),
            Span::styled("●", Style::default().fg(theme.entity_actor)),
            Span::styled(" actor  ", Style::default().fg(theme.text_muted)),
            Span::styled("⚙", Style::default().fg(theme.pill_accent_fg)),
            Span::styled(" sub-agent", Style::default().fg(theme.text_muted)),
        ]);

        let area = Rect {
            x: inner.x,
            y: inner.y + inner.height - 1,
            width: inner.width,
            height: 1,
        };
        frame.render_widget(Paragraph::new(legend), area);
    }

    fn world_rect_to_screen(
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        rect: WorldRect,
    ) -> Option<Rect> {
        let x = inner.x as i32 + rect.x + offset_x;
        let y = inner.y as i32 + rect.y + offset_y;

        if x < inner.x as i32
            || y < inner.y as i32
            || x + rect.width as i32 > (inner.x + inner.width) as i32
            || y + rect.height as i32 > (inner.y + inner.height) as i32
        {
            return None;
        }

        Some(Rect {
            x: x as u16,
            y: y as u16,
            width: rect.width,
            height: rect.height,
        })
    }

    fn dim_color(color: Color) -> Color {
        match color {
            Color::Rgb(r, g, b) => Color::Rgb(r / 3, g / 3, b / 3),
            Color::Indexed(index) => Color::Indexed(index.saturating_sub(8)),
            Color::Black => Color::Black,
            _ => color,
        }
    }

    fn draw_world_char(
        buf: &mut Buffer,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        world_x: i32,
        world_y: i32,
        ch: &str,
        style: Style,
    ) {
        let screen_x = inner.x as i32 + world_x + offset_x;
        let screen_y = inner.y as i32 + world_y + offset_y;

        if screen_x < inner.x as i32
            || screen_y < inner.y as i32
            || screen_x >= (inner.x + inner.width) as i32
            || screen_y >= (inner.y + inner.height) as i32
        {
            return;
        }

        buf.set_string(screen_x as u16, screen_y as u16, ch, style);
    }

    fn draw_world_hline(
        buf: &mut Buffer,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        world_y: i32,
        x_start: i32,
        x_end: i32,
        style: Style,
        ch: &str,
    ) {
        let from = x_start.min(x_end);
        let to = x_start.max(x_end);
        for x in from..=to {
            Self::draw_world_char(buf, inner, offset_x, offset_y, x, world_y, ch, style);
        }
    }

    fn draw_world_vline(
        buf: &mut Buffer,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        world_x: i32,
        y_start: i32,
        y_end: i32,
        style: Style,
        ch: &str,
    ) {
        let from = y_start.min(y_end);
        let to = y_start.max(y_end);
        for y in from..=to {
            Self::draw_world_char(buf, inner, offset_x, offset_y, world_x, y, ch, style);
        }
    }
}
