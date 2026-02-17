use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, VizSelection};
use crate::models::{compact_id, compact_locator, VariantRow};
use crate::theme::Theme;

use dark_tui_components::{PaneBlockComponent, StatusPill};

use super::catalog_cards::{
    draw_junction, draw_trunk, render_actor_card, render_variant_card, ClickHit, ProductGroup,
};

/// Max width for product card tiles.
const PRODUCT_CARD_WIDTH: u16 = 48;
/// Max width for variant card tiles.
const VARIANT_CARD_WIDTH: u16 = 44;
/// Max width for actor card tiles.
const ACTOR_CARD_WIDTH: u16 = 38;
/// Height of a product card (border + 2 content lines + border).
const PRODUCT_CARD_HEIGHT: u16 = 4;
/// Height of a variant card (border + 2 content lines + border).
const VARIANT_CARD_HEIGHT: u16 = 4;
/// Height of an actor card (border + 3 content lines + border).
const ACTOR_CARD_HEIGHT: u16 = 5;
/// Left margin before the product card.
const PRODUCT_LEFT_MARGIN: u16 = 2;
/// Connector column width (the `│` / `├─` / `└─` gutter).
const CONNECTOR_WIDTH: u16 = 4;
/// Secondary connector width for actor branches off variants.
const ACTOR_CONNECTOR_WIDTH: u16 = 3;

pub(crate) struct UnifiedCatalogView;

impl UnifiedCatalogView {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let panel = PaneBlockComponent::build("Catalog", true, theme);
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

        let groups = Self::product_groups(app);
        let viz_sel = app.viz_selection();

        // Compute absolute Y positions for each group (world-space, origin=0).
        let mut group_positions: Vec<i32> = Vec::with_capacity(groups.len());
        let mut world_y: i32 = 0;
        for group in &groups {
            group_positions.push(world_y);
            world_y += Self::group_height(group, app) as i32 + 1; // +1 gap between groups
        }

        // Read viz offset (camera position).
        let (offset_x, offset_y) = app.viz_offset();

        // Render each group translated by the camera offset.
        for (gi, group) in groups.iter().enumerate() {
            let world_group_y = group_positions[gi];
            let group_h = Self::group_height(group, app) as i32;

            // Translate to screen coordinates (relative to inner area).
            let screen_y = world_group_y + offset_y;
            let screen_x = offset_x;

            // Cull groups entirely outside the visible viewport.
            if screen_y + group_h < 0 || screen_y >= inner.height as i32 {
                continue;
            }
            if screen_x + (inner.width as i32) < 0 {
                continue;
            }

            // Clamp the group area to the visible inner rect.
            let abs_y = inner.y as i32 + screen_y;
            let abs_x = inner.x as i32 + screen_x;

            // Compute the visible slice of this group.
            let vis_top = abs_y.max(inner.y as i32) as u16;
            let vis_bot = (abs_y + group_h).min((inner.y + inner.height) as i32) as u16;
            let vis_left = abs_x.max(inner.x as i32) as u16;
            let vis_right = (abs_x + inner.width as i32).min((inner.x + inner.width) as i32) as u16;

            if vis_top >= vis_bot || vis_left >= vis_right {
                continue;
            }

            // Build the group area in absolute terminal coordinates.
            let group_area = Rect {
                x: abs_x.max(inner.x as i32) as u16,
                y: abs_y.max(inner.y as i32) as u16,
                width: vis_right.saturating_sub(vis_left),
                height: vis_bot.saturating_sub(vis_top),
            };

            Self::render_product_group(frame, group_area, group, viz_sel, app, theme);
        }

        // Scroll position hint in bottom-right corner.
        if offset_x != 0 || offset_y != 0 {
            let hint = format!("pan({},{}) [0]=reset", offset_x, offset_y);
            let hint_len = hint.len() as u16;
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
        let inner = Self::panel_inner(area);

        if inner.width < 20
            || inner.height < 6
            || app.products().is_empty()
            || col < inner.x
            || col >= inner.x + inner.width
            || row < inner.y
            || row >= inner.y + inner.height
        {
            return false;
        }

        // Determine hit target using only immutable borrows, then apply
        // the mutation after releasing all references into `app`.
        let hit = {
            let groups = Self::product_groups(app);
            let (offset_x, offset_y) = app.viz_offset();
            let world_x = (col - inner.x) as i32 - offset_x;
            let world_y = (row - inner.y) as i32 - offset_y;

            let product_w = PRODUCT_CARD_WIDTH.min(inner.width.saturating_sub(PRODUCT_LEFT_MARGIN));
            if product_w == 0 {
                return false;
            }

            let connector_x_pos = PRODUCT_LEFT_MARGIN + 3;
            let variant_x = connector_x_pos + CONNECTOR_WIDTH;
            let variant_w = VARIANT_CARD_WIDTH.min(inner.width.saturating_sub(variant_x));

            let actor_connector_x = variant_x + 3;
            let actor_x = actor_connector_x + ACTOR_CONNECTOR_WIDTH;
            let actor_w = ACTOR_CARD_WIDTH.min(inner.width.saturating_sub(actor_x));

            let mut result: Option<ClickHit> = None;
            let mut group_y: i32 = 0;

            'outer: for group in &groups {
                let product_left = PRODUCT_LEFT_MARGIN as i32;
                let product_right = product_left + product_w as i32;
                let product_top = group_y;
                let product_bottom = product_top + PRODUCT_CARD_HEIGHT as i32;

                if world_x >= product_left
                    && world_x < product_right
                    && world_y >= product_top
                    && world_y < product_bottom
                {
                    result = Some(ClickHit::Product {
                        product_index: group.product_index,
                    });
                    break;
                }

                let mut slot_y = group_y + PRODUCT_CARD_HEIGHT as i32;
                for variant in &group.variants {
                    let v_top = slot_y;
                    let v_bottom = v_top + VARIANT_CARD_HEIGHT as i32;

                    if variant_w > 0
                        && world_x >= variant_x as i32
                        && world_x < (variant_x + variant_w) as i32
                        && world_y >= v_top
                        && world_y < v_bottom
                    {
                        result = Some(ClickHit::Variant {
                            product_index: group.product_index,
                            variant_id: variant.id.clone(),
                        });
                        break 'outer;
                    }
                    slot_y += VARIANT_CARD_HEIGHT as i32;

                    let actors = app.actors_for_variant(&variant.id);
                    for actor in &actors {
                        let a_top = slot_y;
                        let a_bottom = a_top + ACTOR_CARD_HEIGHT as i32;

                        if actor_w > 0
                            && world_x >= actor_x as i32
                            && world_x < (actor_x + actor_w) as i32
                            && world_y >= a_top
                            && world_y < a_bottom
                        {
                            result = Some(ClickHit::Actor {
                                product_index: group.product_index,
                                variant_id: variant.id.clone(),
                                actor_id: actor.id.clone(),
                            });
                            break 'outer;
                        }
                        slot_y += ACTOR_CARD_HEIGHT as i32;
                    }
                }

                group_y += Self::group_height(group, app) as i32 + 1;
            }

            result
        }; // immutable borrows end here

        // Apply the hit with mutable access.
        match hit {
            Some(ClickHit::Product { product_index }) => {
                app.select_product_by_index(product_index);
                true
            }
            Some(ClickHit::Variant {
                product_index,
                variant_id,
            }) => {
                app.select_variant_in_product(product_index, &variant_id);
                true
            }
            Some(ClickHit::Actor {
                product_index,
                variant_id,
                actor_id,
            }) => {
                app.select_actor_in_viz(product_index, &variant_id, &actor_id);
                true
            }
            None => false,
        }
    }

    fn panel_inner(area: Rect) -> Rect {
        if area.width < 2 || area.height < 2 {
            return area;
        }

        Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
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
                    .filter(|v| v.product_id == product.id)
                    .collect();
                ProductGroup {
                    product,
                    product_index,
                    variants,
                }
            })
            .collect()
    }

    /// Total height of a product group including all variant and actor cards.
    fn group_height(group: &ProductGroup, app: &App) -> u16 {
        let mut h = PRODUCT_CARD_HEIGHT;
        for variant in &group.variants {
            h += VARIANT_CARD_HEIGHT;
            let actor_count = app.actors_for_variant(&variant.id).len() as u16;
            h += actor_count * ACTOR_CARD_HEIGHT;
        }
        h
    }

    fn render_product_group(
        frame: &mut Frame,
        area: Rect,
        group: &ProductGroup,
        viz_sel: Option<&VizSelection>,
        app: &App,
        theme: &Theme,
    ) {
        // Determine selection state from VizSelection.
        let is_product_selected = matches!(
            viz_sel,
            Some(VizSelection::Product { product_index }) if *product_index == group.product_index
        );
        let is_product_tree_active = matches!(
            viz_sel,
            Some(VizSelection::Product { product_index }
                | VizSelection::Variant { product_index, .. }
                | VizSelection::Actor { product_index, .. })
            if *product_index == group.product_index
        );

        // --- Product card (fixed width, left margin) ---
        let card_w = PRODUCT_CARD_WIDTH.min(area.width.saturating_sub(PRODUCT_LEFT_MARGIN));
        let card_x = area.x + PRODUCT_LEFT_MARGIN;
        let card_h = PRODUCT_CARD_HEIGHT.min(area.height);

        let product_area = Rect {
            x: card_x,
            y: area.y,
            width: card_w,
            height: card_h,
        };

        let product = group.product;

        let product_color = if product.is_git_repo {
            theme.entity_variant
        } else {
            theme.entity_product
        };

        let product_border = if is_product_selected {
            Style::default()
                .fg(product_color)
                .add_modifier(Modifier::BOLD)
        } else if is_product_tree_active {
            Style::default().fg(product_color)
        } else {
            Style::default().fg(theme.pane_unfocused_border)
        };

        let title = if is_product_selected {
            format!("\u{25c6} {}", product.display_name)
        } else {
            format!("Product: {}", product.display_name)
        };

        // Row 1: status pill + branch pill + variant counts
        let status_pill = match product.status.as_str() {
            "active" | "clean" => StatusPill::ok(&product.status, theme),
            "dirty" => StatusPill::warn("dirty", theme),
            "error" | "failed" => StatusPill::error(&product.status, theme),
            _ => StatusPill::muted(&product.status, theme),
        };
        let branch_pill = StatusPill::info(&product.branches, theme);
        let variant_summary = if product.variant_dirty > 0 || product.variant_drift > 0 {
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

        let pill_line = Line::from(vec![
            status_pill.span(),
            Span::raw(" "),
            branch_pill.span(),
            Span::raw(" "),
            variant_summary.span(),
        ]);

        // Row 2: compact locator + id
        let loc_line = Line::from(vec![
            Span::styled(
                compact_locator(&product.locator, 30),
                Style::default().fg(theme.text_muted),
            ),
            Span::raw("  "),
            Span::styled(
                compact_id(&product.id),
                Style::default().fg(theme.text_muted),
            ),
        ]);

        let content = vec![pill_line, loc_line];

        let card = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(product_border)
                    .title(title),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(card, product_area);

        // --- Connector + variant sub-tiles + actor sub-tiles ---
        if group.variants.is_empty() || area.height <= PRODUCT_CARD_HEIGHT {
            return;
        }

        let connector_x = card_x + 3; // anchor connector under product card
        let variant_x = connector_x + CONNECTOR_WIDTH;
        let variant_w = VARIANT_CARD_WIDTH.min(area.width.saturating_sub(variant_x - area.x));
        let remaining_h = area.height.saturating_sub(PRODUCT_CARD_HEIGHT);

        // Build a layout schedule: for each variant, compute its Y position and
        // the Y positions of its actor children.
        struct SlotInfo {
            variant_y: u16,
            actor_ys: Vec<u16>,
        }

        let mut slots: Vec<SlotInfo> = Vec::with_capacity(group.variants.len());
        let mut cursor_y = area.y + PRODUCT_CARD_HEIGHT;
        for variant in &group.variants {
            let vy = cursor_y;
            cursor_y += VARIANT_CARD_HEIGHT;
            let actors = app.actors_for_variant(&variant.id);
            let mut actor_ys = Vec::with_capacity(actors.len());
            for _ in &actors {
                actor_ys.push(cursor_y);
                cursor_y += ACTOR_CARD_HEIGHT;
            }
            slots.push(SlotInfo {
                variant_y: vy,
                actor_ys,
            });
        }

        let variant_count = group.variants.len();
        let buf = frame.buffer_mut();
        let connector_style = Style::default().fg(theme.catalog_connector);

        // --- Draw variant connectors (product → variant trunk) ---
        for (vi, slot) in slots.iter().enumerate() {
            let vy = slot.variant_y;
            if vy + VARIANT_CARD_HEIGHT > area.y + PRODUCT_CARD_HEIGHT + remaining_h {
                break;
            }

            let is_last_variant = vi == variant_count - 1 && slot.actor_ys.is_empty();
            // "last child" means no more items in the product trunk below this.
            let is_last_in_trunk = vi == variant_count - 1;

            // Vertical trunk from previous node to this branch point.
            let branch_y = vy + 1; // midpoint of the variant card
            let trunk_start = if vi == 0 {
                area.y + PRODUCT_CARD_HEIGHT
            } else {
                // After previous variant (and its actors), the trunk continues.
                let prev = &slots[vi - 1];
                let prev_end = if prev.actor_ys.is_empty() {
                    prev.variant_y + VARIANT_CARD_HEIGHT
                } else {
                    *prev.actor_ys.last().unwrap() + ACTOR_CARD_HEIGHT
                };
                prev_end
            };

            draw_trunk(buf, connector_x, trunk_start, branch_y, &connector_style);

            // Branch junction.
            let junction = if is_last_in_trunk {
                "\u{251c}"
            } else {
                "\u{251c}"
            };
            // Use └ only when truly the last entry under this product (no actors after last variant).
            let junction = if is_last_variant {
                "\u{2514}"
            } else {
                junction
            };
            draw_junction(
                buf,
                connector_x,
                branch_y,
                variant_x,
                junction,
                &connector_style,
            );

            // Continue trunk below this variant for subsequent variants.
            if !is_last_in_trunk {
                let next_start = branch_y + 1;
                let next_slot = &slots[vi + 1];
                let next_branch = next_slot.variant_y + 1;
                // If this variant has actors, the trunk continues through them.
                if !slot.actor_ys.is_empty() {
                    let actors_end = *slot.actor_ys.last().unwrap() + ACTOR_CARD_HEIGHT;
                    draw_trunk(buf, connector_x, next_start, actors_end, &connector_style);
                    draw_trunk(buf, connector_x, actors_end, next_branch, &connector_style);
                } else {
                    draw_trunk(buf, connector_x, next_start, next_branch, &connector_style);
                }
            } else if !slot.actor_ys.is_empty() {
                // Last variant but has actors — continue trunk for actor branches.
                let next_start = branch_y + 1;
                let actors_end = *slot.actor_ys.last().unwrap() + 1; // branch_y of last actor
                draw_trunk(buf, connector_x, next_start, actors_end, &connector_style);
            }
        }

        // --- Draw actor connectors (variant → actor sub-branches) ---
        let actor_connector_x = variant_x + 3;
        let actor_x = actor_connector_x + ACTOR_CONNECTOR_WIDTH;
        let actor_w =
            ACTOR_CARD_WIDTH.min(area.width.saturating_sub(actor_x.saturating_sub(area.x)));

        for (vi, (variant, slot)) in group.variants.iter().zip(slots.iter()).enumerate() {
            let actors = app.actors_for_variant(&variant.id);
            if actors.is_empty() {
                continue;
            }

            let is_last_variant = vi == variant_count - 1;

            for (ai, actor_y) in slot.actor_ys.iter().enumerate() {
                let ay = *actor_y;
                if ay + ACTOR_CARD_HEIGHT > area.y + area.height {
                    break;
                }

                let is_last_actor = ai == actors.len() - 1;

                // Vertical trunk from variant card bottom / previous actor.
                let branch_y = ay + 1; // midpoint of actor card
                let trunk_start = if ai == 0 {
                    slot.variant_y + VARIANT_CARD_HEIGHT
                } else {
                    slot.actor_ys[ai - 1] + ACTOR_CARD_HEIGHT
                };

                draw_trunk(
                    buf,
                    actor_connector_x,
                    trunk_start,
                    branch_y,
                    &connector_style,
                );

                // Also continue the main product trunk through actor rows
                // (only if this is NOT the last variant).
                if !is_last_variant {
                    draw_trunk(
                        buf,
                        connector_x,
                        trunk_start,
                        branch_y + 1,
                        &connector_style,
                    );
                }

                let junction = if is_last_actor {
                    "\u{2514}"
                } else {
                    "\u{251c}"
                };
                draw_junction(
                    buf,
                    actor_connector_x,
                    branch_y,
                    actor_x,
                    junction,
                    &connector_style,
                );

                // Continue vertical trunk for non-last actors.
                if !is_last_actor {
                    draw_trunk(
                        buf,
                        actor_connector_x,
                        branch_y + 1,
                        slot.actor_ys[ai + 1] + 1,
                        &connector_style,
                    );
                }
            }
        }

        // --- Render variant cards (after connectors so cards paint on top) ---
        for (_vi, (variant, slot)) in group.variants.iter().zip(slots.iter()).enumerate() {
            let vy = slot.variant_y;
            if vy + VARIANT_CARD_HEIGHT > area.y + PRODUCT_CARD_HEIGHT + remaining_h {
                break;
            }

            let tile_area = Rect {
                x: variant_x,
                y: vy,
                width: variant_w,
                height: VARIANT_CARD_HEIGHT,
            };

            let is_variant_selected = matches!(
                viz_sel,
                Some(VizSelection::Variant { product_index, variant_id })
                if *product_index == group.product_index && *variant_id == variant.id
            );
            // Variant is "active" if it or one of its actors is selected.
            let is_variant_active = is_variant_selected
                || matches!(
                    viz_sel,
                    Some(VizSelection::Actor { product_index, variant_id, .. })
                    if *product_index == group.product_index && *variant_id == variant.id
                );

            render_variant_card(
                frame,
                tile_area,
                variant,
                is_variant_selected,
                is_variant_active,
                theme,
            );

            // --- Render actor cards for this variant ---
            let actors = app.actors_for_variant(&variant.id);
            for (ai, actor) in actors.iter().enumerate() {
                let ay = slot.actor_ys[ai];
                if ay + ACTOR_CARD_HEIGHT > area.y + area.height {
                    break;
                }

                let actor_area = Rect {
                    x: actor_x,
                    y: ay,
                    width: actor_w,
                    height: ACTOR_CARD_HEIGHT,
                };

                let is_actor_selected = matches!(
                    viz_sel,
                    Some(VizSelection::Actor { actor_id, .. })
                    if *actor_id == actor.id
                );

                render_actor_card(frame, actor_area, actor, is_actor_selected, theme);
            }
        }
    }
}
