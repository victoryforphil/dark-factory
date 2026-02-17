use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, VizSelection};
use crate::models::{
    compact_id, compact_locator, compact_timestamp, ActorRow, ProductRow, VariantRow,
};
use crate::theme::{EntityKind, Theme};

use dark_tui_components::{compact_text_normalized, LabeledField, SectionHeader, StatusPill};

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
        lines.push(LabeledField::new("Name", &product.display_name).line(theme));
        lines.push(LabeledField::new("ID", compact_id(&product.id)).line(theme));
        lines.push(LabeledField::new("Type", &product.product_type).line(theme));
        lines.push(
            LabeledField::new(
                "Locator",
                compact_locator(&product.locator, width.saturating_sub(16) as usize),
            )
            .line(theme),
        );
        lines.push(Line::raw(""));

        // --- Repository section ---
        lines.push(SectionHeader::new("Repository", theme.entity_product).line(width, theme));
        lines.push(LabeledField::new("Repo", &product.repo_name).line(theme));
        lines.push(LabeledField::new("Branch", &product.branch).line(theme));
        lines.push(
            LabeledField::new(
                "Branches",
                compact_text_normalized(&product.branches, width.saturating_sub(16) as usize),
            )
            .line(theme),
        );
        lines.push(Self::variant_summary_line(product, theme));
        lines.push(Line::raw(""));

        // --- Timestamps ---
        lines.push(SectionHeader::new("Timestamps", theme.text_muted).line(width, theme));
        lines
            .push(LabeledField::new("Updated", compact_timestamp(&product.updated_at)).line(theme));

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
        lines.push(LabeledField::new("Name", &variant.name).line(theme));
        lines.push(LabeledField::new("ID", compact_id(&variant.id)).line(theme));
        lines.push(LabeledField::new("Product", compact_id(&variant.product_id)).line(theme));
        lines.push(
            LabeledField::new(
                "Locator",
                compact_locator(&variant.locator, width.saturating_sub(16) as usize),
            )
            .line(theme),
        );
        lines.push(Line::raw(""));

        // --- Git section ---
        lines.push(SectionHeader::new("Git", theme.entity_variant).line(width, theme));
        lines.push(LabeledField::new("Branch", &variant.branch).line(theme));
        lines.push(
            LabeledField::new(
                "Worktree",
                compact_locator(&variant.worktree, width.saturating_sub(16) as usize),
            )
            .line(theme),
        );
        lines.push(
            LabeledField::new(
                "Ahead/Behind",
                format!("{}/{}", variant.ahead, variant.behind),
            )
            .line(theme),
        );
        lines.push(Line::raw(""));

        // --- Timestamps ---
        lines.push(SectionHeader::new("Timestamps", theme.text_muted).line(width, theme));
        lines.push(
            LabeledField::new("Polled", compact_timestamp(&variant.last_polled_at)).line(theme),
        );
        lines
            .push(LabeledField::new("Updated", compact_timestamp(&variant.updated_at)).line(theme));

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
        lines.push(LabeledField::new("Title", &actor.title).line(theme));
        lines.push(
            LabeledField::new(
                "Description",
                compact_text_normalized(&actor.description, width.saturating_sub(16) as usize),
            )
            .line(theme),
        );
        lines.push(LabeledField::new("ID", compact_id(&actor.id)).line(theme));
        lines.push(LabeledField::new("Variant", compact_id(&actor.variant_id)).line(theme));
        lines.push(Line::raw(""));

        // --- Runtime section ---
        lines.push(SectionHeader::new("Runtime", theme.entity_actor).line(width, theme));
        lines.push(LabeledField::new("Provider", &actor.provider).line(theme));
        lines.push(
            LabeledField::new(
                "Directory",
                compact_locator(&actor.directory, width.saturating_sub(16) as usize),
            )
            .line(theme),
        );
        lines.push(Line::raw(""));

        // --- Timestamps ---
        lines.push(SectionHeader::new("Timestamps", theme.text_muted).line(width, theme));
        lines.push(LabeledField::new("Created", compact_timestamp(&actor.created_at)).line(theme));
        lines.push(LabeledField::new("Updated", compact_timestamp(&actor.updated_at)).line(theme));

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

        Line::from(vec![
            Span::raw(" "),
            provider_pill.span(),
            Span::raw(" "),
            status_pill.span(),
        ])
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
}
