use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

use crate::app::App;

use dark_tui_components::{FooterBar, FooterBarProps, LoadingSpinner, StatusPill};

pub(crate) struct FooterPanel;

impl FooterPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();

        // --- State pills ---
        let focus_pill = StatusPill::accent(app.focus().label(), theme);
        let view_pill =
            StatusPill::info(format!("view:{}", app.results_view_mode().label()), theme);
        let dir_pill = StatusPill::muted(format!("dir:{}", app.directory_display()), theme);

        let filter_pill = if app.filter_variants_to_product() {
            StatusPill::warn("filtered", theme)
        } else {
            StatusPill::muted("all", theme)
        };

        let chat_pill = if app.is_chat_visible() {
            if app.is_chat_composing() {
                StatusPill::accent("chat:compose", theme)
            } else {
                StatusPill::info("chat:on", theme)
            }
        } else {
            StatusPill::muted("chat:off", theme)
        };

        // --- Entity count pills (moved from header overview) ---
        let products_pill = StatusPill::info(format!("{}P", app.products().len()), theme);
        let variants_pill = StatusPill::info(format!("{}V", app.variants().len()), theme);
        let actors_pill = StatusPill::info(format!("{}A", app.actors().len()), theme);

        // --- Runtime pill ---
        let runtime = app.runtime_status();
        let runtime_pill = match runtime {
            "ok" | "healthy" | "connected" => StatusPill::ok(runtime, theme),
            "unknown" => StatusPill::muted(runtime, theme),
            _ => StatusPill::warn(runtime, theme),
        };

        let activity_label = if app.has_background_activity() {
            format!(
                "{} net:{}",
                LoadingSpinner::glyph(),
                app.background_activity_label()
            )
        } else {
            format!("net:{}", app.background_activity_label())
        };
        let activity_pill = if app.has_background_activity() {
            StatusPill::accent(activity_label, theme)
        } else {
            StatusPill::muted(activity_label, theme)
        };

        // --- Status message ---
        let status_text = app.status_message();
        let status_span = if status_text.contains("failed") || status_text.contains("error") {
            Span::styled(
                status_text.to_string(),
                Style::default().fg(theme.text_error),
            )
        } else {
            Span::styled(
                status_text.to_string(),
                Style::default().fg(theme.text_status_normal),
            )
        };

        let footer_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.footer_border))
            .title("Status");
        let inner = footer_block.inner(area);
        frame.render_widget(footer_block, area);

        FooterBar::render(
            frame,
            inner,
            FooterBarProps {
                segments: vec![
                    view_pill.span(),
                    dir_pill.span(),
                    focus_pill.span(),
                    filter_pill.span(),
                    chat_pill.span(),
                    products_pill.span(),
                    variants_pill.span(),
                    actors_pill.span(),
                    runtime_pill.span(),
                    activity_pill.span(),
                    status_span,
                ],
                separator: "  ",
            },
            theme,
        );
    }
}
