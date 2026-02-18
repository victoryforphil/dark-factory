use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use dark_tui_components::{compact_text_normalized, PaneBlockComponent, StatusPill};

use crate::app::{App, VizDensity, VizSelection};
use crate::models::compact_locator;
use crate::theme::Theme;
use crate::ui::render::components::{render_sub_agent_grid, sub_agent_grid_container_height};

use super::catalog_cards::ProductGroup;
use super::catalog_tree_view::CatalogTreeView;

const TREE_PANEL_TITLE: &str = "Catalog Graphical Tree";

const STATION_MIN_WIDTH: u16 = 72;
const STATION_MIN_HEIGHT: u16 = 16;

const PRODUCT_LEFT_MARGIN: i32 = 8;
const PRODUCT_W: u16 = 62;
const PRODUCT_H: u16 = 10;

const RAIL_DROP_ROWS: i32 = 1;

const VARIANT_H: u16 = 5;

const ACTOR_H: u16 = 7;
const ACTOR_STACK_GAP: i32 = 2;

/// Max actors rendered per variant column in station mode.
/// Beyond this count a "+N more" overflow indicator is shown.
const STATION_MAX_ACTORS_PER_VARIANT: usize = 4;
const MAX_SUB_AGENT_ROWS: usize = 12;

const GROUP_GAP_Y: i32 = 2;
const MIN_VISIBLE_PRODUCT_W: u16 = 40;
const MIN_VISIBLE_VARIANT_W: u16 = 26;
const MIN_VISIBLE_ACTOR_W: u16 = 28;

pub(crate) struct UnifiedCatalogView;

#[derive(Debug, Clone, Copy)]
struct WorldRect {
    x: i32,
    y: i32,
    width: u16,
    height: u16,
}

fn variant_state_pill(state: &str, theme: &Theme) -> StatusPill {
    match state {
        "clean" => StatusPill::ok("clean", theme),
        "dirty" => StatusPill::warn("dirty", theme),
        "no-git" => StatusPill::muted("no-git", theme),
        _ => StatusPill::muted(state, theme),
    }
}

fn ahead_behind_pill(ahead: u64, behind: u64, theme: &Theme) -> StatusPill {
    if behind > 0 {
        StatusPill::warn(format!("+{ahead}/-{behind}"), theme)
    } else if ahead > 0 {
        StatusPill::ok(format!("+{ahead}/-0"), theme)
    } else {
        StatusPill::muted("+0/-0", theme)
    }
}

fn actor_status_pill(status: &str, theme: &Theme) -> StatusPill {
    match status {
        "active" | "running" => StatusPill::ok(status, theme),
        "error" | "failed" | "dead" => StatusPill::error(status, theme),
        "idle" | "waiting" | "stopped" | "stop" => StatusPill::warn(status, theme),
        _ => StatusPill::muted(status, theme),
    }
}

impl WorldRect {
    fn contains(self, x: i32, y: i32) -> bool {
        x >= self.x
            && y >= self.y
            && x < self.x + self.width as i32
            && y < self.y + self.height as i32
    }

    fn right(self) -> i32 {
        self.x + self.width as i32
    }

    fn bottom(self) -> i32 {
        self.y + self.height as i32
    }

    fn mid_x(self) -> i32 {
        self.x + (self.width as i32) / 2
    }
}

struct StationLayout {
    groups: Vec<ProductLayout>,
}

struct ProductLayout {
    product_index: usize,
    product_rect: WorldRect,
    rail_y: i32,
    rail_end_x: i32,
    variants: Vec<VariantLayout>,
}

struct VariantLayout {
    product_index: usize,
    variant_id: String,
    tick_x: i32,
    variant_rect: WorldRect,
    actors: Vec<ActorLayout>,
    /// Number of actors beyond the visible cap (for overflow indicator).
    overflow_count: usize,
}

struct ActorLayout {
    product_index: usize,
    variant_id: String,
    actor_id: String,
    actor_rect: WorldRect,
    sub_grid_rect: Option<WorldRect>,
}

impl UnifiedCatalogView {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        if area.width < STATION_MIN_WIDTH || area.height < STATION_MIN_HEIGHT {
            CatalogTreeView::render(frame, area, app);
            return;
        }

        let theme = app.theme();
        let panel = PaneBlockComponent::build(TREE_PANEL_TITLE, true, theme);
        let inner = panel.inner(area);
        frame.render_widget(panel, area);

        if inner.width < STATION_MIN_WIDTH || inner.height < 8 {
            return;
        }

        if app.products().is_empty() {
            frame.render_widget(
                Paragraph::new("No products")
                    .style(Style::default().fg(theme.text_muted))
                    .wrap(Wrap { trim: true }),
                inner,
            );
            return;
        }

        let (offset_x, offset_y) = app.viz_offset();
        let layout = Self::build_layout(app, inner.width);

        Self::render_connectors(
            frame.buffer_mut(),
            inner,
            offset_x,
            offset_y,
            &layout,
            theme,
        );

        // Depth guides (D0/D1/D2/D3) in the left margin per SVG reference
        let depth_style = Style::default().fg(Color::Rgb(0x38, 0x38, 0x38));
        for group in &layout.groups {
            // D0 = rail row
            Self::draw_world_char(
                frame.buffer_mut(),
                inner,
                offset_x,
                offset_y,
                0,
                group.rail_y,
                "D0",
                depth_style,
            );
            // D1 = first variant row
            if let Some(first_v) = group.variants.first() {
                Self::draw_world_char(
                    frame.buffer_mut(),
                    inner,
                    offset_x,
                    offset_y,
                    0,
                    first_v.variant_rect.y + 1,
                    "D1",
                    depth_style,
                );
                // D2 = first actor row
                if let Some(first_a) = first_v.actors.first() {
                    Self::draw_world_char(
                        frame.buffer_mut(),
                        inner,
                        offset_x,
                        offset_y,
                        0,
                        first_a.actor_rect.y + 1,
                        "D2",
                        depth_style,
                    );
                    // D3 = sub-agent grid row
                    if let Some(sub) = first_a.sub_grid_rect {
                        Self::draw_world_char(
                            frame.buffer_mut(),
                            inner,
                            offset_x,
                            offset_y,
                            0,
                            sub.y + 1,
                            "D3",
                            depth_style,
                        );
                    }
                }
            }
        }

        for group in &layout.groups {
            Self::render_product(frame, inner, offset_x, offset_y, app, group, theme);
            for variant in &group.variants {
                Self::render_variant(frame, inner, offset_x, offset_y, app, variant, theme);
                for actor in &variant.actors {
                    Self::render_actor(frame, inner, offset_x, offset_y, app, actor, theme);
                }
                // Overflow indicator when actors are capped
                if variant.overflow_count > 0 {
                    let overflow_y = variant
                        .actors
                        .last()
                        .map(|a| {
                            a.sub_grid_rect
                                .map(|s| s.bottom())
                                .unwrap_or(a.actor_rect.bottom())
                                + 1
                        })
                        .unwrap_or(variant.variant_rect.bottom() + 3);
                    let label = format!("+{} more", variant.overflow_count);
                    let label_w = label.len() as u16;
                    let overflow_rect = WorldRect {
                        x: variant.variant_rect.mid_x() - label_w as i32 / 2,
                        y: overflow_y,
                        width: label_w,
                        height: 1,
                    };
                    if let Some(ov_area) = Self::to_screen(inner, offset_x, offset_y, overflow_rect)
                    {
                        frame.render_widget(
                            Paragraph::new(label).style(Style::default().fg(theme.text_muted)),
                            ov_area,
                        );
                    }
                }
            }
        }

        if offset_x != 0 || offset_y != 0 {
            let hint = format!("pan({offset_x},{offset_y}) [0]=reset");
            let hint_len = hint.chars().count() as u16;
            if hint_len < inner.width {
                frame.render_widget(
                    Paragraph::new(hint).style(Style::default().fg(theme.text_muted)),
                    Rect {
                        x: inner.x + inner.width - hint_len,
                        y: inner.y + inner.height - 1,
                        width: hint_len,
                        height: 1,
                    },
                );
            }
        }
    }

    pub(crate) fn click_select(area: Rect, app: &mut App, col: u16, row: u16) -> bool {
        let Some(hit) = Self::hit_test(area, app, col, row) else {
            return false;
        };
        app.set_viz_selection(hit);
        true
    }

    pub(crate) fn hit_test(area: Rect, app: &App, col: u16, row: u16) -> Option<VizSelection> {
        if area.width < STATION_MIN_WIDTH || area.height < STATION_MIN_HEIGHT {
            return CatalogTreeView::hit_test(area, app, col, row);
        }

        let panel = PaneBlockComponent::build(TREE_PANEL_TITLE, true, app.theme());
        let inner = panel.inner(area);

        if col < inner.x
            || row < inner.y
            || col >= inner.x + inner.width
            || row >= inner.y + inner.height
        {
            return None;
        }

        let (offset_x, offset_y) = app.viz_offset();
        let world_x = (col - inner.x) as i32 - offset_x;
        let world_y = (row - inner.y) as i32 - offset_y;

        let layout = Self::build_layout(app, inner.width);
        for group in &layout.groups {
            for variant in &group.variants {
                for actor in &variant.actors {
                    if actor.actor_rect.contains(world_x, world_y) {
                        return Some(VizSelection::Actor {
                            product_index: actor.product_index,
                            variant_id: actor.variant_id.clone(),
                            actor_id: actor.actor_id.clone(),
                        });
                    }
                }
                if variant.variant_rect.contains(world_x, world_y) {
                    return Some(VizSelection::Variant {
                        product_index: variant.product_index,
                        variant_id: variant.variant_id.clone(),
                    });
                }
            }

            if group.product_rect.contains(world_x, world_y) {
                return Some(VizSelection::Product {
                    product_index: group.product_index,
                });
            }
        }

        None
    }

    fn build_layout(app: &App, inner_width: u16) -> StationLayout {
        let groups = Self::product_groups(app);
        let mut cursor_y = 0i32;
        let mut layouts = Vec::with_capacity(groups.len());

        for group in groups {
            let density = app.viz_density();
            let product_w = match density {
                VizDensity::Compact => 44,
                VizDensity::Normal => PRODUCT_W,
                VizDensity::Wide => 72,
                VizDensity::XWide => 82,
            };

            let product_rect = WorldRect {
                x: PRODUCT_LEFT_MARGIN,
                y: cursor_y,
                width: product_w.min(inner_width.saturating_sub(4)),
                height: PRODUCT_H,
            };
            let rail_y = product_rect.y + PRODUCT_H as i32 + RAIL_DROP_ROWS;
            let variant_count = group.variants.len();
            let compact_many = variant_count >= 4;
            let column_gap = match density {
                VizDensity::Compact => {
                    if compact_many {
                        2
                    } else {
                        3
                    }
                }
                VizDensity::Normal => {
                    if compact_many {
                        4
                    } else {
                        6
                    }
                }
                VizDensity::Wide => {
                    if compact_many {
                        6
                    } else {
                        8
                    }
                }
                VizDensity::XWide => {
                    if compact_many {
                        8
                    } else {
                        10
                    }
                }
            };
            let col_start_x = product_rect.right() + column_gap;

            let mut variants = Vec::with_capacity(group.variants.len());
            let mut group_bottom = product_rect.bottom();
            let mut rail_end_x = product_rect.mid_x() + 10;

            let wide_columns = group.variants.len() <= 2;
            let base_variant_w: i32 = if wide_columns {
                46
            } else if compact_many {
                30
            } else {
                38
            };
            let base_actor_w: i32 = if wide_columns {
                46
            } else if compact_many {
                28
            } else {
                36
            };
            let width_delta: i32 = match density {
                VizDensity::Compact => -12,
                VizDensity::Normal => 0,
                VizDensity::Wide => 10,
                VizDensity::XWide => 18,
            };
            let variant_w = (base_variant_w + width_delta).max(22) as u16;
            let actor_w = (base_actor_w + width_delta).max(22) as u16;
            // Widen pitch when any variant has 2+ actors (wider cards need breathing room)
            let max_actors_in_group: usize = group
                .variants
                .iter()
                .map(|v| app.actors_for_variant(&v.id).len())
                .max()
                .unwrap_or(0);
            let mut base_variant_pitch: i32 = if wide_columns {
                if max_actors_in_group >= 2 {
                    62
                } else {
                    58
                }
            } else if compact_many {
                if max_actors_in_group >= 2 {
                    40
                } else {
                    36
                }
            } else {
                if max_actors_in_group >= 2 {
                    54
                } else {
                    50
                }
            };
            base_variant_pitch += match density {
                VizDensity::Compact => -18,
                VizDensity::Normal => 0,
                VizDensity::Wide => 12,
                VizDensity::XWide => 22,
            };
            base_variant_pitch = base_variant_pitch.max(24);

            // Keep a fixed spatial pitch so toggling sidebar width does not
            // reflow/overlap columns in-place; off-screen content is handled by pan.
            let variant_pitch = base_variant_pitch;

            for (index, variant) in group.variants.iter().enumerate() {
                let tick_x = col_start_x + (index as i32) * variant_pitch;
                rail_end_x = rail_end_x.max(tick_x);
                let variant_rect = WorldRect {
                    x: tick_x - (variant_w as i32 / 2),
                    y: rail_y + 2,
                    width: variant_w,
                    height: VARIANT_H,
                };

                let actors = app.actors_for_variant(&variant.id);
                let visible_count = actors.len().min(STATION_MAX_ACTORS_PER_VARIANT);
                let overflow_count = actors.len().saturating_sub(STATION_MAX_ACTORS_PER_VARIANT);
                let mut actor_layouts = Vec::with_capacity(visible_count);
                let mut actor_y = variant_rect.bottom() + 2; // extra gap before first actor
                for actor in actors.iter().take(visible_count) {
                    let actor_rect = WorldRect {
                        x: variant_rect.x + (variant_w as i32 - actor_w as i32) / 2,
                        y: actor_y,
                        width: actor_w,
                        height: ACTOR_H,
                    };

                    let grid_h = sub_agent_grid_container_height(actor.sub_agents.len(), actor_w);
                    let sub_grid_rect = if grid_h > 0 {
                        Some(WorldRect {
                            x: actor_rect.x,
                            y: actor_rect.bottom(),
                            width: actor_rect.width,
                            height: grid_h,
                        })
                    } else {
                        None
                    };

                    actor_y = actor_rect.bottom()
                        + sub_grid_rect.map(|r| r.height as i32).unwrap_or(0)
                        + ACTOR_STACK_GAP;

                    group_bottom = group_bottom.max(actor_y);

                    actor_layouts.push(ActorLayout {
                        product_index: group.product_index,
                        variant_id: variant.id.clone(),
                        actor_id: actor.id.clone(),
                        actor_rect,
                        sub_grid_rect,
                    });
                }
                // Reserve space for overflow indicator
                if overflow_count > 0 {
                    group_bottom = group_bottom.max(actor_y + 1);
                }

                if actor_layouts.is_empty() {
                    group_bottom = group_bottom.max(variant_rect.bottom() + 3);
                }

                variants.push(VariantLayout {
                    product_index: group.product_index,
                    variant_id: variant.id.clone(),
                    tick_x,
                    variant_rect,
                    actors: actor_layouts,
                    overflow_count,
                });
            }

            let group_end_y = group_bottom + 1;

            layouts.push(ProductLayout {
                product_index: group.product_index,
                product_rect,
                rail_y,
                rail_end_x,
                variants,
            });

            cursor_y = group_end_y + GROUP_GAP_Y;
        }

        StationLayout { groups: layouts }
    }

    fn render_connectors(
        buf: &mut Buffer,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        layout: &StationLayout,
        theme: &Theme,
    ) {
        let strong = Style::default().fg(theme.catalog_connector);
        let muted = Style::default().fg(theme.text_muted);

        for group in &layout.groups {
            let stem_x = group.product_rect.mid_x();
            let stem_start = group.product_rect.bottom();
            let stem_end = group.rail_y - 1;
            for y in stem_start..=stem_end {
                let glyph = if (y - stem_start) % 2 == 0 {
                    "\u{2502}"
                } else {
                    " "
                };
                Self::draw_world_char(buf, inner, offset_x, offset_y, stem_x, y, glyph, muted);
            }
            Self::draw_world_char(
                buf,
                inner,
                offset_x,
                offset_y,
                stem_x,
                group.rail_y - 1,
                "\u{25c7}",
                muted,
            );

            Self::draw_world_hline(
                buf,
                inner,
                offset_x,
                offset_y,
                group.rail_y,
                stem_x,
                group.rail_end_x,
                strong,
                "\u{2501}",
            );
            // Rail end-cap block (matches SVG terminal marker)
            Self::draw_world_char(
                buf,
                inner,
                offset_x,
                offset_y,
                group.rail_end_x + 1,
                group.rail_y,
                "\u{2588}",
                strong,
            );

            for variant in &group.variants {
                Self::draw_world_vline(
                    buf,
                    inner,
                    offset_x,
                    offset_y,
                    variant.tick_x,
                    group.rail_y,
                    variant.variant_rect.y - 1,
                    strong,
                    "\u{2502}",
                );

                if variant.actors.is_empty() {
                    Self::draw_world_vline(
                        buf,
                        inner,
                        offset_x,
                        offset_y,
                        variant.tick_x,
                        variant.variant_rect.bottom(),
                        variant.variant_rect.bottom() + 2,
                        muted,
                        "\u{2502}",
                    );
                    let box_top = variant.variant_rect.bottom() + 2;
                    let box_left = variant.variant_rect.x;
                    let box_right = variant.variant_rect.right() - 1;
                    let box_bottom = box_top + 2;
                    Self::draw_dashed_hline(
                        buf, inner, offset_x, offset_y, box_top, box_left, box_right, muted,
                    );
                    Self::draw_dashed_hline(
                        buf, inner, offset_x, offset_y, box_bottom, box_left, box_right, muted,
                    );
                    Self::draw_world_vline(
                        buf, inner, offset_x, offset_y, box_left, box_top, box_bottom, muted,
                        "\u{2506}",
                    );
                    Self::draw_world_vline(
                        buf, inner, offset_x, offset_y, box_right, box_top, box_bottom, muted,
                        "\u{2506}",
                    );
                    // "no actors" centered text inside placeholder
                    let label = "no actors";
                    let label_x = box_left + (box_right - box_left - label.len() as i32) / 2 + 1;
                    let label_y = box_top + 1;
                    for (i, ch) in label.chars().enumerate() {
                        Self::draw_world_char(
                            buf,
                            inner,
                            offset_x,
                            offset_y,
                            label_x + i as i32,
                            label_y,
                            &ch.to_string(),
                            muted,
                        );
                    }
                    continue;
                }

                for (ai, actor) in variant.actors.iter().enumerate() {
                    let actor_x = actor.actor_rect.mid_x();
                    // Chain connectors: first actor from variant bottom,
                    // subsequent actors from previous actor's bottom (or sub-grid bottom).
                    let connector_start = if ai == 0 {
                        variant.variant_rect.bottom()
                    } else {
                        let prev = &variant.actors[ai - 1];
                        prev.sub_grid_rect
                            .map(|s| s.bottom())
                            .unwrap_or(prev.actor_rect.bottom())
                    };
                    Self::draw_world_vline(
                        buf,
                        inner,
                        offset_x,
                        offset_y,
                        actor_x,
                        connector_start,
                        actor.actor_rect.y,
                        muted,
                        "\u{2502}",
                    );

                    if let Some(sub_rect) = actor.sub_grid_rect {
                        Self::draw_world_vline(
                            buf,
                            inner,
                            offset_x,
                            offset_y,
                            actor_x,
                            actor.actor_rect.bottom(),
                            sub_rect.y,
                            muted,
                            "\u{2502}",
                        );
                    }
                }
            }
        }
    }

    fn render_product(
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
        let Some(area) = Self::to_screen(inner, offset_x, offset_y, group.product_rect) else {
            return;
        };
        if area.width < MIN_VISIBLE_PRODUCT_W || area.height < PRODUCT_H.saturating_sub(2) {
            return;
        }

        let selected = matches!(
            app.viz_selection(),
            Some(VizSelection::Product { product_index }) if *product_index == group.product_index
        );

        let border = if selected {
            Style::default()
                .fg(theme.entity_product)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.entity_product)
        };

        let inner_w = area.width.saturating_sub(2) as usize;

        let display_name = {
            let name = product.display_name.trim();
            if name.is_empty() || name == "-" {
                let repo_name = product.repo_name.trim();
                if repo_name.is_empty() || repo_name == "-" {
                    "-".to_string()
                } else {
                    repo_name.to_string()
                }
            } else {
                name.to_string()
            }
        };

        let type_label = {
            let value = product.product_type.trim();
            if value.is_empty() {
                "-"
            } else {
                value
            }
        };

        let status_pill = match product.status.as_str() {
            "ready" | "active" | "running" | "clean" => StatusPill::ok(&product.status, theme),
            "dirty" | "warning" => StatusPill::warn(&product.status, theme),
            "error" | "failed" => StatusPill::error(&product.status, theme),
            _ => StatusPill::muted(&product.status, theme),
        };
        let type_pill = StatusPill::info(format!("󰏖 {type_label}"), theme);
        let locator_mode_pill = if product.is_git_repo {
            StatusPill::ok(" git", theme)
        } else {
            StatusPill::muted("󰉋 local", theme)
        };

        let row0 = Line::from(vec![Span::styled(
            compact_text_normalized(&display_name, inner_w.max(8)),
            Style::default()
                .fg(theme.text_primary)
                .add_modifier(Modifier::BOLD),
        )]);

        let row1 = Line::from(vec![
            status_pill.span(),
            Span::raw(" "),
            type_pill.span(),
            Span::raw(" "),
            locator_mode_pill.span(),
        ]);

        // Topology rows embedded inside product card (rows 2-7)
        let variants: Vec<_> = app
            .variants()
            .iter()
            .filter(|v| v.product_id == product.id)
            .collect();
        let variant_ids: Vec<&str> = variants.iter().map(|v| v.id.as_str()).collect();
        let actors: Vec<_> = app
            .actors()
            .iter()
            .filter(|a| variant_ids.contains(&a.variant_id.as_str()))
            .collect();
        let sub_agent_count: usize = actors.iter().map(|a| a.sub_agents.len()).sum();
        let running = actors
            .iter()
            .filter(|a| a.status == "running" || a.status == "active")
            .count();
        let stopped = actors
            .iter()
            .filter(|a| a.status == "stopped" || a.status == "stop")
            .count();
        let empty_v = variants.len()
            - variants
                .iter()
                .filter(|v| actors.iter().any(|a| a.variant_id == v.id))
                .count();

        let topo_metrics = Line::from(vec![Span::styled(
            format!(
                "󰍹 {} variants  󰘧 {} actors  󱐌 {} sub-agents",
                product.variant_total,
                actors.len(),
                sub_agent_count
            ),
            Style::default().fg(theme.text_muted),
        )]);
        let locator_line = Line::from(vec![Span::styled(
            compact_text_normalized(
                &format!("󰉋 {}", compact_locator(&product.locator, inner_w.max(8))),
                inner_w.max(8),
            ),
            Style::default().fg(theme.text_secondary),
        )]);
        let topo_status = Line::from(vec![Span::styled(
            format!("󰐊 {running} running   󰓛 {stopped} stopped   󰜌 {empty_v} empty"),
            Style::default().fg(theme.text_muted),
        )]);

        frame.render_widget(
            Paragraph::new(vec![
                row0,
                row1,
                topo_metrics,
                locator_line,
                Line::from(""),
                topo_status,
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border)
                    .title(Line::from(vec![Span::styled(
                        " 󰏖 PRODUCT ",
                        Style::default()
                            .fg(theme.text_primary)
                            .add_modifier(Modifier::BOLD),
                    )])),
            ),
            area,
        );

        // Left accent bar
        Self::draw_accent_bar_left(frame.buffer_mut(), area, theme.entity_product);
    }

    fn render_variant(
        frame: &mut Frame,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        app: &App,
        layout: &VariantLayout,
        theme: &Theme,
    ) {
        let Some(variant) = app.variants().iter().find(|v| v.id == layout.variant_id) else {
            return;
        };
        let Some(area) = Self::to_screen(inner, offset_x, offset_y, layout.variant_rect) else {
            return;
        };
        if area.width < MIN_VISIBLE_VARIANT_W || area.height < VARIANT_H.saturating_sub(1) {
            return;
        }

        let selected = matches!(
            app.viz_selection(),
            Some(VizSelection::Variant {
                product_index,
                variant_id,
            }) if *product_index == layout.product_index && *variant_id == layout.variant_id
        );

        let border = if selected {
            Style::default()
                .fg(theme.entity_variant)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.entity_variant)
        };

        // Row 0: variant name
        let inner_w = area.width.saturating_sub(2) as usize;
        let row0 = Line::from(vec![Span::styled(
            variant.name.clone(),
            Style::default()
                .fg(theme.text_primary)
                .add_modifier(Modifier::BOLD),
        )]);

        // Row 1: reusable status pills
        let branch = if variant.branch.trim().is_empty() {
            "-"
        } else {
            variant.branch.as_str()
        };
        let worktree = if variant.worktree.trim().is_empty() {
            "-"
        } else {
            variant.worktree.as_str()
        };
        let clone_suffix = match variant.clone_status.as_str() {
            "cloning" => "   cloning",
            "failed" => "   clone-failed",
            _ => "",
        };

        let mut row1_spans = vec![
            variant_state_pill(&variant.git_state, theme).span(),
            Span::raw(" "),
            StatusPill::info(format!(" {branch}"), theme).span(),
            Span::raw(" "),
            StatusPill::muted(format!("󰉋 {worktree}"), theme).span(),
            Span::raw(" "),
            ahead_behind_pill(variant.ahead, variant.behind, theme).span(),
        ];
        if !clone_suffix.is_empty() {
            row1_spans.push(Span::raw(" "));
            row1_spans.push(StatusPill::warn(clone_suffix.trim(), theme).span());
        }

        let progress_line = if variant.clone_status == "cloning" && variant.clone_last_line != "-" {
            let compact = compact_text_normalized(&variant.clone_last_line, inner_w.max(8));
            Some(Line::from(vec![Span::styled(
                compact,
                Style::default().fg(theme.text_muted),
            )]))
        } else {
            None
        };
        let row1 = Line::from(row1_spans);

        frame.render_widget(
            Paragraph::new(if let Some(progress) = progress_line {
                vec![row0, row1, progress]
            } else {
                vec![row0, row1]
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border)
                    .title(Line::from(vec![Span::styled(
                        "  VARIANT ",
                        Style::default()
                            .fg(theme.text_primary)
                            .add_modifier(Modifier::BOLD),
                    )])),
            ),
            area,
        );

        Self::draw_accent_bar_left(frame.buffer_mut(), area, theme.entity_variant);
    }

    fn render_actor(
        frame: &mut Frame,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        app: &App,
        layout: &ActorLayout,
        theme: &Theme,
    ) {
        let Some(actor) = app.actors().iter().find(|a| a.id == layout.actor_id) else {
            return;
        };
        let Some(area) = Self::to_screen(inner, offset_x, offset_y, layout.actor_rect) else {
            return;
        };
        if area.width < MIN_VISIBLE_ACTOR_W || area.height < ACTOR_H.saturating_sub(1) {
            return;
        }

        let selected = matches!(
            app.viz_selection(),
            Some(VizSelection::Actor { actor_id, .. }) if *actor_id == layout.actor_id
        );
        let border = if selected {
            Style::default()
                .fg(theme.entity_actor)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.entity_actor)
        };

        // Row 0: actor title
        let inner_w = area.width.saturating_sub(2) as usize;
        let title_label = compact_text_normalized(&actor.title, inner_w.saturating_sub(6).max(8));
        let row0 = Line::from(vec![Span::styled(
            title_label,
            Style::default()
                .fg(theme.text_primary)
                .add_modifier(Modifier::BOLD),
        )]);

        let actor_type_pill = StatusPill::muted(Self::actor_type_label(actor), theme);
        let status_pill = actor_status_pill(&actor.status, theme);
        let provider_pill = StatusPill::info(format!("󰘧 {}", actor.provider), theme);
        let row1 = Line::from(vec![
            status_pill.span(),
            Span::raw(" "),
            provider_pill.span(),
            Span::raw(" "),
            actor_type_pill.span(),
        ]);

        // Row 2: last message preview when available
        let last_message = Self::actor_last_message_preview(app, &actor.id)
            .or_else(|| {
                let text = actor.description.trim();
                if text.is_empty() || text == "-" {
                    None
                } else {
                    Some(text.to_string())
                }
            })
            .unwrap_or_else(|| "no messages yet".to_string());
        let row2 = Line::from(vec![
            Span::styled("󰍩 ", Style::default().fg(theme.text_muted)),
            Span::styled(last_message, Style::default().fg(theme.text_secondary)),
        ]);

        frame.render_widget(
            Paragraph::new(vec![row0, row1, row2])
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(border)
                        .title(Line::from(vec![Span::styled(
                            " 󰘧 ACTOR ",
                            Style::default()
                                .fg(theme.text_primary)
                                .add_modifier(Modifier::BOLD),
                        )])),
                )
                .wrap(Wrap { trim: true }),
            area,
        );

        // Left accent bar
        Self::draw_accent_bar_left(frame.buffer_mut(), area, theme.entity_actor);

        if let Some(sub_rect) = layout.sub_grid_rect {
            let Some(sub_area) = Self::to_screen(inner, offset_x, offset_y, sub_rect) else {
                return;
            };
            let entries: Vec<(&str, &str)> = actor
                .sub_agents
                .iter()
                .skip(actor.sub_agents.len().saturating_sub(MAX_SUB_AGENT_ROWS))
                .map(|sub| (sub.title.as_str(), sub.status.as_str()))
                .collect();
            render_sub_agent_grid(frame, sub_area, &entries, actor.sub_agents.len(), theme);
        }
    }

    fn product_groups<'a>(app: &'a App) -> Vec<ProductGroup<'a>> {
        app.products()
            .iter()
            .enumerate()
            .map(|(product_index, product)| ProductGroup {
                product,
                product_index,
                variants: app
                    .variants()
                    .iter()
                    .filter(|v| v.product_id == product.id)
                    .collect(),
            })
            .collect()
    }

    fn to_screen(inner: Rect, offset_x: i32, offset_y: i32, rect: WorldRect) -> Option<Rect> {
        let sx = inner.x as i32 + rect.x + offset_x;
        let sy = inner.y as i32 + rect.y + offset_y;
        let inner_right = (inner.x + inner.width) as i32;
        let inner_bottom = (inner.y + inner.height) as i32;

        if sx >= inner_right || sy >= inner_bottom {
            return None;
        }

        // Keep cards visible when only their right/bottom edge crosses viewport bounds.
        // We intentionally avoid left/top partial clipping for widget surfaces, because
        // ratatui widgets render from the top-left origin and cannot offset content origin.
        if sx < inner.x as i32 || sy < inner.y as i32 {
            return None;
        }

        let available_w = (inner_right - sx).max(0) as u16;
        let available_h = (inner_bottom - sy).max(0) as u16;
        let clipped_w = rect.width.min(available_w);
        let clipped_h = rect.height.min(available_h);

        if clipped_w == 0 || clipped_h == 0 {
            return None;
        }

        Some(Rect {
            x: sx as u16,
            y: sy as u16,
            width: clipped_w,
            height: clipped_h,
        })
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
        let sx = inner.x as i32 + world_x + offset_x;
        let sy = inner.y as i32 + world_y + offset_y;

        if sx < inner.x as i32
            || sy < inner.y as i32
            || sx >= (inner.x + inner.width) as i32
            || sy >= (inner.y + inner.height) as i32
        {
            return;
        }

        buf.set_string(sx as u16, sy as u16, ch, style);
    }

    fn actor_last_message_preview(app: &App, actor_id: &str) -> Option<String> {
        if let Some(value) = app.actor_last_message_preview(actor_id) {
            let text = value.trim();
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }

        if !app.chat_actor().is_some_and(|actor| actor.id == actor_id) {
            return None;
        }

        app.chat_messages().iter().rev().find_map(|message| {
            let text = message.text.trim();
            if text.is_empty() {
                None
            } else {
                Some(text.to_string())
            }
        })
    }

    fn actor_type_label(actor: &crate::models::ActorRow) -> String {
        let provider = actor.provider.trim();
        let kind = if provider.is_empty() {
            "actor".to_string()
        } else {
            provider.to_string()
        };

        format!("󰘧 {kind}")
    }

    fn draw_world_hline(
        buf: &mut Buffer,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        y: i32,
        x1: i32,
        x2: i32,
        style: Style,
        ch: &str,
    ) {
        let (start, end) = (x1.min(x2), x1.max(x2));
        for x in start..=end {
            Self::draw_world_char(buf, inner, offset_x, offset_y, x, y, ch, style);
        }
    }

    fn draw_world_vline(
        buf: &mut Buffer,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        x: i32,
        y1: i32,
        y2: i32,
        style: Style,
        ch: &str,
    ) {
        let (start, end) = (y1.min(y2), y1.max(y2));
        for y in start..=end {
            Self::draw_world_char(buf, inner, offset_x, offset_y, x, y, ch, style);
        }
    }

    fn draw_dashed_hline(
        buf: &mut Buffer,
        inner: Rect,
        offset_x: i32,
        offset_y: i32,
        y: i32,
        x1: i32,
        x2: i32,
        style: Style,
    ) {
        let (start, end) = (x1.min(x2), x1.max(x2));
        for x in start..=end {
            let ch = if (x - start) % 2 == 0 {
                "\u{2500}"
            } else {
                " "
            };
            Self::draw_world_char(buf, inner, offset_x, offset_y, x, y, ch, style);
        }
    }

    /// Draw a left accent bar (▌ glyph) on the left edge of a screen-space Rect.
    fn draw_accent_bar_left(buf: &mut Buffer, area: Rect, color: Color) {
        let style = Style::default().fg(color);
        if area.height < 3 {
            return;
        }

        for row in (area.y + 1)..(area.y + area.height - 1) {
            if row < buf.area.y || row >= buf.area.y + buf.area.height {
                continue;
            }
            if area.x < buf.area.x || area.x >= buf.area.x + buf.area.width {
                continue;
            }
            buf.set_string(area.x, row, "\u{258c}", style);
        }
    }
}
