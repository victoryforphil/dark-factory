use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::{App, VizSelection};
use crate::models::{ActorRow, ProductRow, SubAgentRow, VariantRow, compact_timestamp};
use crate::theme::{EntityKind, Theme};
use crate::ui::render::components::sub_agent_badge;

use dark_tui_components::{SectionHeader, StatusPill, compact_text_normalized};

pub(crate) struct DetailsPanel;

impl DetailsPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();

        // Determine which entity type is being shown for border accent.
        let entity_kind = Self::active_entity_kind(app);
        let product_border = app
            .selected_product()
            .map(|product| {
                if product.is_git_repo {
                    theme.entity_variant
                } else {
                    theme.entity_product
                }
            })
            .unwrap_or(theme.entity_product);
        let (title, border_color) = match entity_kind {
            EntityKind::Product => ("\u{25a0} Product", product_border),
            EntityKind::Variant => ("\u{25b6} Variant", theme.entity_variant),
            EntityKind::Actor => ("\u{25cf} Actor", theme.entity_actor),
        };

        // Build block with entity-colored border.
        let border_style = Style::default().fg(border_color);
        let block = ratatui::widgets::Block::default()
            .title(title)
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(border_style);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width < 8 || inner.height < 3 {
            return;
        }

        let lines = Self::build_detail_lines(app, entity_kind, inner.width, theme);

        let widget = Paragraph::new(lines);
        frame.render_widget(widget, inner);
    }

    /// Determine the currently-active entity kind based on selection state.
    fn active_entity_kind(app: &App) -> EntityKind {
        if let Some(sel) = app.viz_selection() {
            return match sel {
                VizSelection::Product { .. } => EntityKind::Product,
                VizSelection::Variant { .. } => EntityKind::Variant,
                VizSelection::Actor { .. } => EntityKind::Actor,
            };
        }
        match app.focus() {
            crate::app::FocusPane::Products => EntityKind::Product,
            crate::app::FocusPane::Variants => EntityKind::Variant,
        }
    }

    /// Build structured, visually-grouped detail lines.
    fn build_detail_lines(
        app: &App,
        entity_kind: EntityKind,
        width: u16,
        theme: &Theme,
    ) -> Vec<Line<'static>> {
        match entity_kind {
            EntityKind::Product => Self::product_lines(app, width, theme),
            EntityKind::Variant => Self::variant_lines(app, width, theme),
            EntityKind::Actor => Self::actor_lines(app, width, theme),
        }
    }

    fn product_lines(app: &App, width: u16, theme: &Theme) -> Vec<Line<'static>> {
        let Some(product) = app.selected_product() else {
            return vec![Line::styled(
                "  No product selected",
                Style::default().fg(theme.text_muted),
            )];
        };

        let mut lines: Vec<Line<'static>> = Vec::new();

        // --- Status row: pills ---
        lines.push(Self::pill_row(product, theme));
        lines.push(Line::raw(""));

        // --- Identity section ---
        lines.push(SectionHeader::new("Identity", theme.entity_product).line(width, theme));
        Self::push_stacked_field(&mut lines, "Name", &product.display_name, width, theme);
        Self::push_stacked_field(&mut lines, "ID", &product.id, width, theme);
        Self::push_stacked_field(&mut lines, "Type", &product.product_type, width, theme);
        Self::push_stacked_field(&mut lines, "Locator", &product.locator, width, theme);
        Self::push_stacked_field(
            &mut lines,
            "Workspace",
            &product.workspace_locator,
            width,
            theme,
        );
        lines.push(Line::raw(""));

        // --- Repository section ---
        lines.push(SectionHeader::new("Repository", theme.entity_product).line(width, theme));
        Self::push_stacked_field(&mut lines, "Repo", &product.repo_name, width, theme);
        Self::push_stacked_field(&mut lines, "Branch", &product.branch, width, theme);
        Self::push_stacked_field(
            &mut lines,
            "Branches",
            &compact_text_normalized(&product.branches, width.saturating_sub(8) as usize),
            width,
            theme,
        );
        lines.push(Self::variant_summary_line(product, theme));
        lines.push(Line::raw(""));

        // --- Timestamps ---
        lines.push(SectionHeader::new("Timestamps", theme.text_muted).line(width, theme));
        Self::push_stacked_field(
            &mut lines,
            "Updated",
            compact_timestamp(&product.updated_at),
            width,
            theme,
        );

        lines
    }

    fn variant_lines(app: &App, width: u16, theme: &Theme) -> Vec<Line<'static>> {
        let Some(variant) = app.selected_variant() else {
            return vec![Line::styled(
                "  No variant selected",
                Style::default().fg(theme.text_muted),
            )];
        };

        let mut lines: Vec<Line<'static>> = Vec::new();

        // --- Status row: pills ---
        lines.push(Self::variant_pill_row(variant, theme));
        lines.push(Line::raw(""));

        // --- Identity section ---
        lines.push(SectionHeader::new("Identity", theme.entity_variant).line(width, theme));
        Self::push_stacked_field(&mut lines, "Name", &variant.name, width, theme);
        Self::push_stacked_field(&mut lines, "ID", &variant.id, width, theme);
        Self::push_stacked_field(&mut lines, "Product", &variant.product_id, width, theme);
        Self::push_stacked_field(&mut lines, "Locator", &variant.locator, width, theme);
        lines.push(Line::raw(""));

        // --- Git section ---
        lines.push(SectionHeader::new("Git", theme.entity_variant).line(width, theme));
        Self::push_stacked_field(&mut lines, "Branch", &variant.branch, width, theme);
        Self::push_stacked_field(&mut lines, "Worktree", &variant.worktree, width, theme);
        Self::push_stacked_field(
            &mut lines,
            "Ahead/Behind",
            format!("{}/{}", variant.ahead, variant.behind),
            width,
            theme,
        );
        lines.push(Line::raw(""));

        // --- Timestamps ---
        lines.push(SectionHeader::new("Timestamps", theme.text_muted).line(width, theme));
        Self::push_stacked_field(
            &mut lines,
            "Polled",
            compact_timestamp(&variant.last_polled_at),
            width,
            theme,
        );
        Self::push_stacked_field(
            &mut lines,
            "Updated",
            compact_timestamp(&variant.updated_at),
            width,
            theme,
        );

        lines
    }

    fn actor_lines(app: &App, width: u16, theme: &Theme) -> Vec<Line<'static>> {
        let actor = if let Some(VizSelection::Actor { actor_id, .. }) = app.viz_selection() {
            app.actors().iter().find(|a| a.id == *actor_id)
        } else {
            app.selected_actor()
        };

        let Some(actor) = actor else {
            return vec![Line::styled(
                "  No actor selected",
                Style::default().fg(theme.text_muted),
            )];
        };

        let mut lines: Vec<Line<'static>> = Vec::new();

        // --- Status row: pills ---
        lines.push(Self::actor_pill_row(actor, theme));
        lines.push(Line::raw(""));

        // --- Identity section ---
        lines.push(SectionHeader::new("Identity", theme.entity_actor).line(width, theme));
        Self::push_stacked_field(&mut lines, "Title", &actor.title, width, theme);
        Self::push_stacked_field(
            &mut lines,
            "Description",
            &compact_text_normalized(&actor.description, width.saturating_sub(8) as usize),
            width,
            theme,
        );
        Self::push_stacked_field(&mut lines, "ID", &actor.id, width, theme);
        Self::push_stacked_field(&mut lines, "Variant", &actor.variant_id, width, theme);
        lines.push(Line::raw(""));

        // --- Runtime section ---
        lines.push(SectionHeader::new("Runtime", theme.entity_actor).line(width, theme));
        Self::push_stacked_field(&mut lines, "Provider", &actor.provider, width, theme);
        Self::push_stacked_field(&mut lines, "Directory", &actor.directory, width, theme);
        if actor.sub_agent_count() > 0 {
            Self::push_stacked_field(
                &mut lines,
                "Sub-Agents",
                actor.sub_agent_count().to_string(),
                width,
                theme,
            );
        }
        lines.push(Line::raw(""));

        // --- Sub-Agents section (when entries exist) ---
        if !actor.sub_agents.is_empty() {
            lines.push(SectionHeader::new("Sub-Agents", theme.entity_actor).line(width, theme));
            Self::push_sub_agent_rows(&mut lines, &actor.sub_agents, width, theme);
            lines.push(Line::raw(""));
        }

        // --- Timestamps ---
        lines.push(SectionHeader::new("Timestamps", theme.text_muted).line(width, theme));
        Self::push_stacked_field(
            &mut lines,
            "Created",
            compact_timestamp(&actor.created_at),
            width,
            theme,
        );
        Self::push_stacked_field(
            &mut lines,
            "Updated",
            compact_timestamp(&actor.updated_at),
            width,
            theme,
        );

        lines
    }

    // --- Pill row helpers ---

    fn pill_row(product: &ProductRow, theme: &Theme) -> Line<'static> {
        let status_pill = match product.status.as_str() {
            "active" | "clean" => StatusPill::ok(&product.status, theme),
            "dirty" => StatusPill::warn("dirty", theme),
            "error" | "failed" => StatusPill::error(&product.status, theme),
            _ => StatusPill::muted(&product.status, theme),
        };
        let branch_pill = StatusPill::info(&product.branch, theme);

        Line::from(vec![
            Span::raw(" "),
            status_pill.span(),
            Span::raw(" "),
            branch_pill.span(),
        ])
    }

    fn variant_pill_row(variant: &VariantRow, theme: &Theme) -> Line<'static> {
        let state_pill = match variant.git_state.as_str() {
            "clean" => StatusPill::ok("clean", theme),
            "dirty" => StatusPill::warn("dirty", theme),
            "no-git" => StatusPill::muted("no-git", theme),
            _ => StatusPill::muted(&variant.git_state, theme),
        };
        let branch_pill = StatusPill::info(&variant.branch, theme);

        let mut spans = vec![
            Span::raw(" "),
            state_pill.span(),
            Span::raw(" "),
            branch_pill.span(),
        ];

        if variant.ahead > 0 || variant.behind > 0 {
            let ab_pill = if variant.behind > 0 {
                StatusPill::warn(
                    format!("+{}/\u{2212}{}", variant.ahead, variant.behind),
                    theme,
                )
            } else {
                StatusPill::ok(format!("+{}", variant.ahead), theme)
            };
            spans.push(Span::raw(" "));
            spans.push(ab_pill.span());
        }

        Line::from(spans)
    }

    fn actor_pill_row(actor: &ActorRow, theme: &Theme) -> Line<'static> {
        let provider_pill = StatusPill::info(&actor.provider, theme);
        let status_pill = match actor.status.as_str() {
            "active" | "running" => StatusPill::ok(&actor.status, theme),
            "error" | "failed" | "dead" => StatusPill::error(&actor.status, theme),
            "idle" | "waiting" => StatusPill::warn(&actor.status, theme),
            _ => StatusPill::muted(&actor.status, theme),
        };

        let mut spans = vec![
            Span::raw(" "),
            provider_pill.span(),
            Span::raw(" "),
            status_pill.span(),
        ];

        if let Some(badge) = sub_agent_badge(actor.sub_agent_count(), theme) {
            spans.push(Span::raw(" "));
            spans.push(badge);
        }

        Line::from(spans)
    }

    fn variant_summary_line(product: &ProductRow, theme: &Theme) -> Line<'static> {
        let summary_pill = if product.variant_dirty > 0 || product.variant_drift > 0 {
            StatusPill::warn(
                format!(
                    "{}v {}dirty {}drift",
                    product.variant_total, product.variant_dirty, product.variant_drift
                ),
                theme,
            )
        } else {
            StatusPill::muted(format!("{}v", product.variant_total), theme)
        };

        Line::from(vec![
            Span::styled("  Variants    ", Style::default().fg(theme.text_muted)),
            summary_pill.span(),
        ])
    }

    fn push_stacked_field(
        lines: &mut Vec<Line<'static>>,
        label: &str,
        value: impl AsRef<str>,
        width: u16,
        theme: &Theme,
    ) {
        lines.push(Line::styled(
            format!("  {label}"),
            Style::default().fg(theme.text_muted),
        ));

        let value_str = value.as_ref();
        let content_width = width.saturating_sub(2) as usize;
        let value_width = value_str.chars().count();
        let right_pad = content_width.saturating_sub(value_width);
        lines.push(Line::styled(
            format!("  {}{}", " ".repeat(right_pad), value_str),
            Style::default().fg(theme.text_secondary),
        ));
    }

    /// Render flattened sub-agent rows with depth-aware indentation and status pills.
    fn push_sub_agent_rows(
        lines: &mut Vec<Line<'static>>,
        sub_agents: &[SubAgentRow],
        _width: u16,
        theme: &Theme,
    ) {
        for agent in sub_agents {
            // Depth-aware tree prefix: "  " base + "  " per depth level + connector.
            let indent = "  ".repeat(agent.depth);
            let connector = if agent.depth > 0 {
                "\u{251c}\u{2500} "
            } else {
                "\u{25aa} "
            };

            let title_text = format!("  {indent}{connector}{}", agent.title);

            let status_pill = match agent.status.as_str() {
                "active" | "running" => StatusPill::ok(&agent.status, theme),
                "error" | "failed" | "dead" => StatusPill::error(&agent.status, theme),
                "idle" | "waiting" => StatusPill::warn(&agent.status, theme),
                "-" => StatusPill::muted("--", theme),
                _ => StatusPill::muted(&agent.status, theme),
            };

            let mut spans = vec![
                Span::styled(title_text, Style::default().fg(theme.text_secondary)),
                Span::raw("  "),
                status_pill.span(),
            ];

            // Add summary hint if non-trivial.
            if agent.summary != "-" && !agent.summary.is_empty() {
                let summary_display = if agent.summary.len() > 32 {
                    format!("{}...", &agent.summary[..29])
                } else {
                    agent.summary.clone()
                };
                spans.push(Span::raw("  "));
                spans.push(Span::styled(
                    summary_display,
                    Style::default().fg(theme.text_muted),
                ));
            }

            lines.push(Line::from(spans));
        }
    }
}
