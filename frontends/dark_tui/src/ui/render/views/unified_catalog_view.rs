use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, FocusPane};
use crate::models::{compact_id, compact_locator, compact_timestamp, ProductRow, VariantRow};
use crate::theme::Theme;

use super::super::components::{PaneBlockComponent, StatusPill};

/// Max width for product card tiles.
const PRODUCT_CARD_WIDTH: u16 = 48;
/// Max width for variant card tiles.
const VARIANT_CARD_WIDTH: u16 = 44;
/// Height of a product card (border + 2 content lines + border).
const PRODUCT_CARD_HEIGHT: u16 = 4;
/// Height of a variant card (border + 2 content lines + border).
const VARIANT_CARD_HEIGHT: u16 = 4;
/// Left margin before the product card.
const PRODUCT_LEFT_MARGIN: u16 = 2;
/// Connector column width (the `│` / `├─` / `└─` gutter).
const CONNECTOR_WIDTH: u16 = 4;

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

        let groups: Vec<ProductGroup> = app
            .products()
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
            .collect();

        let selected_product_index = app.selected_product_index();
        let product_focused = matches!(app.focus(), FocusPane::Products);
        let variant_focused = matches!(app.focus(), FocusPane::Variants);

        // Compute absolute Y positions for each group (world-space, origin=0).
        let mut group_positions: Vec<i32> = Vec::with_capacity(groups.len());
        let mut world_y: i32 = 0;
        for group in &groups {
            group_positions.push(world_y);
            world_y += Self::group_height(group) as i32 + 1; // +1 gap between groups
        }

        // Read viz offset (camera position).
        let (offset_x, offset_y) = app.viz_offset();

        // Render each group translated by the camera offset.
        for (gi, group) in groups.iter().enumerate() {
            let world_group_y = group_positions[gi];
            let group_h = Self::group_height(group) as i32;

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

            // Offset within the group due to top-clipping.
            let clip_top = (inner.y as i32 - abs_y).max(0) as u16;

            Self::render_product_group(
                frame,
                group_area,
                group,
                selected_product_index,
                product_focused,
                variant_focused,
                app,
                clip_top,
                screen_x,
                theme,
            );
        }

        // Scroll position hint in bottom-right corner.
        if offset_x != 0 || offset_y != 0 {
            let hint = format!("pan({},{})", offset_x, offset_y);
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

    fn group_height(group: &ProductGroup) -> u16 {
        let variant_count = group.variants.len() as u16;
        PRODUCT_CARD_HEIGHT + (variant_count * VARIANT_CARD_HEIGHT)
    }

    fn render_product_group(
        frame: &mut Frame,
        area: Rect,
        group: &ProductGroup,
        selected_product_index: usize,
        product_focused: bool,
        variant_focused: bool,
        app: &App,
        _clip_top: u16,
        _screen_x: i32,
        theme: &Theme,
    ) {
        let is_selected_product = group.product_index == selected_product_index;

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

        let product_border = if is_selected_product && product_focused {
            Style::default()
                .fg(theme.entity_product)
                .add_modifier(Modifier::BOLD)
        } else if is_selected_product {
            Style::default().fg(theme.entity_product)
        } else {
            Style::default().fg(theme.pane_unfocused_border)
        };

        let product = group.product;

        // Block title carries the name: "Selected: Name" or "Product: Name"
        let title = if is_selected_product && product_focused {
            format!("Selected: {}", product.display_name)
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
        let branch_pill = StatusPill::info(&product.branch, theme);
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

        // --- Connector + variant sub-tiles ---
        if group.variants.is_empty() || area.height <= PRODUCT_CARD_HEIGHT {
            return;
        }

        let connector_x = card_x + 3; // anchor connector under product card
        let variant_x = connector_x + CONNECTOR_WIDTH;
        let variant_w = VARIANT_CARD_WIDTH.min(area.width.saturating_sub(variant_x - area.x));
        let remaining_h = area.height.saturating_sub(PRODUCT_CARD_HEIGHT);

        let selected_variant = app.selected_variant();
        let selected_variant_id = selected_variant.map(|v| v.id.as_str());
        let variant_count = group.variants.len();
        let buf = frame.buffer_mut();
        let connector_style = Style::default().fg(theme.catalog_connector);

        for vi in 0..variant_count {
            let vy = area.y + PRODUCT_CARD_HEIGHT + (vi as u16) * VARIANT_CARD_HEIGHT;
            if vy + VARIANT_CARD_HEIGHT > area.y + PRODUCT_CARD_HEIGHT + remaining_h {
                break;
            }

            let is_last = vi == variant_count - 1;

            // --- Draw connector lines ---
            let branch_y = vy + 1; // midpoint of the 4-line variant card
            let trunk_start = if vi == 0 {
                area.y + PRODUCT_CARD_HEIGHT
            } else {
                vy
            };

            for cy in trunk_start..branch_y {
                if cy < buf.area.y || cy >= buf.area.y + buf.area.height {
                    continue;
                }
                if connector_x < buf.area.x || connector_x >= buf.area.x + buf.area.width {
                    continue;
                }
                buf.set_string(connector_x, cy, "\u{2502}", connector_style);
            }

            // Branch junction
            let junction = if is_last { "\u{2514}" } else { "\u{251c}" };
            if branch_y < buf.area.y + buf.area.height && connector_x < buf.area.x + buf.area.width
            {
                buf.set_string(connector_x, branch_y, junction, connector_style);
                // Horizontal arm
                let arm_end = variant_x.min(buf.area.x + buf.area.width);
                for cx in (connector_x + 1)..arm_end {
                    buf.set_string(cx, branch_y, "\u{2500}", connector_style);
                }
            }

            // Continue vertical trunk below for non-last variants
            if !is_last {
                let next_branch_y =
                    area.y + PRODUCT_CARD_HEIGHT + ((vi + 1) as u16) * VARIANT_CARD_HEIGHT + 1;
                for cy in (branch_y + 1)..next_branch_y {
                    if cy >= buf.area.y + buf.area.height {
                        break;
                    }
                    if connector_x >= buf.area.x + buf.area.width {
                        break;
                    }
                    buf.set_string(connector_x, cy, "\u{2502}", connector_style);
                }
            }
        }

        // --- Render variant cards (after connectors so cards paint on top) ---
        for (vi, variant) in group.variants.iter().enumerate() {
            let vy = area.y + PRODUCT_CARD_HEIGHT + (vi as u16) * VARIANT_CARD_HEIGHT;
            if vy + VARIANT_CARD_HEIGHT > area.y + PRODUCT_CARD_HEIGHT + remaining_h {
                break;
            }

            let tile_area = Rect {
                x: variant_x,
                y: vy,
                width: variant_w,
                height: VARIANT_CARD_HEIGHT,
            };

            let is_selected_variant =
                is_selected_product && selected_variant_id == Some(variant.id.as_str());

            Self::render_variant_card(
                frame,
                tile_area,
                variant,
                is_selected_variant,
                variant_focused,
                theme,
            );
        }
    }

    fn render_variant_card(
        frame: &mut Frame,
        area: Rect,
        variant: &VariantRow,
        is_selected: bool,
        variant_focused: bool,
        theme: &Theme,
    ) {
        let border_style = if is_selected && variant_focused {
            Style::default()
                .fg(theme.entity_variant)
                .add_modifier(Modifier::BOLD)
        } else if is_selected {
            Style::default().fg(theme.entity_variant)
        } else {
            Style::default().fg(theme.pane_unfocused_border)
        };

        // Block title carries the name: "Selected: Name" or "Variant: Name"
        let title = if is_selected && variant_focused {
            format!("Selected: {}", variant.name)
        } else {
            format!("Variant: {}", variant.name)
        };

        // Row 1: git state pill + branch pill + ahead/behind
        let state_pill = match variant.git_state.as_str() {
            "clean" => StatusPill::ok("clean", theme),
            "dirty" => StatusPill::warn("dirty", theme),
            "no-git" => StatusPill::muted("no-git", theme),
            _ => StatusPill::muted(&variant.git_state, theme),
        };
        let branch_pill = StatusPill::info(&variant.branch, theme);

        let mut pill_spans = vec![state_pill.span(), Span::raw(" "), branch_pill.span()];

        if variant.ahead > 0 || variant.behind > 0 {
            let ab_pill = if variant.behind > 0 {
                StatusPill::warn(
                    format!("+{}/\u{2212}{}", variant.ahead, variant.behind),
                    theme,
                )
            } else {
                StatusPill::ok(format!("+{}", variant.ahead), theme)
            };
            pill_spans.push(Span::raw(" "));
            pill_spans.push(ab_pill.span());
        }

        let pill_line = Line::from(pill_spans);

        // Row 2: polled timestamp + id
        let detail_line = Line::from(vec![
            Span::styled(
                format!("polled {}", compact_timestamp(&variant.last_polled_at)),
                Style::default().fg(theme.text_muted),
            ),
            Span::raw("  "),
            Span::styled(
                compact_id(&variant.id),
                Style::default().fg(theme.text_muted),
            ),
        ]);

        let content = vec![pill_line, detail_line];

        let card = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(title),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(card, area);
    }
}

struct ProductGroup<'a> {
    product: &'a ProductRow,
    product_index: usize,
    variants: Vec<&'a VariantRow>,
}
