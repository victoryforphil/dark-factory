#![allow(dead_code)]

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use dark_tui_components::{StatusPill, compact_text_normalized};

use crate::models::{
    ActorRow, ProductRow, VariantRow, compact_id, compact_locator, compact_timestamp,
};
use crate::theme::Theme;
use crate::ui::render::components::sub_agent_badge;

pub(crate) struct ProductGroup<'a> {
    pub(crate) product: &'a ProductRow,
    pub(crate) product_index: usize,
    pub(crate) variants: Vec<&'a VariantRow>,
}

/// Owned representation of a click-select hit target.
pub(crate) enum ClickHit {
    Product {
        product_index: usize,
    },
    Variant {
        product_index: usize,
        variant_id: String,
    },
    Actor {
        product_index: usize,
        variant_id: String,
        actor_id: String,
    },
}

pub(crate) fn draw_trunk(
    buf: &mut ratatui::buffer::Buffer,
    x: u16,
    y_start: u16,
    y_end: u16,
    style: &Style,
) {
    if x < buf.area.x || x >= buf.area.x + buf.area.width {
        return;
    }
    for cy in y_start..y_end {
        if cy < buf.area.y || cy >= buf.area.y + buf.area.height {
            continue;
        }
        buf.set_string(x, cy, "\u{2502}", *style);
    }
}

pub(crate) fn draw_junction(
    buf: &mut ratatui::buffer::Buffer,
    connector_x: u16,
    branch_y: u16,
    target_x: u16,
    junction_char: &str,
    style: &Style,
) {
    if branch_y < buf.area.y || branch_y >= buf.area.y + buf.area.height {
        return;
    }
    if connector_x >= buf.area.x && connector_x < buf.area.x + buf.area.width {
        buf.set_string(connector_x, branch_y, junction_char, *style);
    }
    let arm_end = target_x.min(buf.area.x + buf.area.width);
    for cx in (connector_x + 1)..arm_end {
        if cx < buf.area.x {
            continue;
        }
        buf.set_string(cx, branch_y, "\u{2500}", *style);
    }
}

pub(crate) fn render_variant_card(
    frame: &mut Frame,
    area: Rect,
    variant: &VariantRow,
    is_selected: bool,
    is_active: bool,
    theme: &Theme,
) {
    let border_style = if is_selected {
        Style::default()
            .fg(theme.entity_variant)
            .add_modifier(Modifier::BOLD)
    } else if is_active {
        Style::default().fg(theme.entity_variant)
    } else {
        Style::default().fg(theme.pane_unfocused_border)
    };

    let title = if is_selected {
        format!("◆ {}", variant.name)
    } else {
        format!("Variant: {}", variant.name)
    };

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

    // Path line: compact locator for the variant
    let path_line = Line::from(vec![Span::styled(
        compact_locator(&variant.locator, 40),
        Style::default().fg(theme.text_muted),
    )]);

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

    let card = Paragraph::new(vec![pill_line, path_line, detail_line])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(card, area);
}

pub(crate) fn render_actor_card(
    frame: &mut Frame,
    area: Rect,
    actor: &ActorRow,
    is_selected: bool,
    theme: &Theme,
) {
    let border_style = if is_selected {
        Style::default()
            .fg(theme.entity_actor)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.pane_unfocused_border)
    };

    let title = if is_selected {
        format!("◆ {}", compact_id(&actor.id))
    } else {
        format!("Actor: {}", compact_id(&actor.id))
    };

    let title_text = if actor.title.trim().is_empty() {
        compact_id(&actor.id)
    } else if actor.title.len() > 40 {
        format!("{}...", &actor.title[..37])
    } else {
        actor.title.clone()
    };
    let title_line = Line::from(vec![Span::styled(
        title_text,
        Style::default().fg(theme.text_primary),
    )]);

    let description_text = if actor.description.trim().is_empty() || actor.description.trim() == "-"
    {
        "No description".to_string()
    } else {
        compact_text_normalized(&actor.description, 44)
    };
    let description_line = Line::from(vec![Span::styled(
        description_text,
        Style::default().fg(theme.text_muted),
    )]);

    let provider_pill = StatusPill::info(&actor.provider, theme);
    let status_pill = match actor.status.as_str() {
        "active" | "running" => StatusPill::ok(&actor.status, theme),
        "error" | "failed" | "dead" => StatusPill::error(&actor.status, theme),
        "idle" | "waiting" => StatusPill::warn(&actor.status, theme),
        _ => StatusPill::muted(&actor.status, theme),
    };

    let badges_line = {
        let mut spans = vec![provider_pill.span(), Span::raw(" "), status_pill.span()];
        if let Some(badge) = sub_agent_badge(actor.sub_agent_count(), theme) {
            spans.push(Span::raw(" "));
            spans.push(badge);
        }
        Line::from(spans)
    };

    let card = Paragraph::new(vec![title_line, description_line, badges_line])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(card, area);
}
