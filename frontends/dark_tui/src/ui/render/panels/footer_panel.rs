use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

use super::super::components::StatusPill;

pub(crate) struct FooterPanel;

impl FooterPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();

        // --- State pills ---
        let focus_pill = StatusPill::accent(app.focus().label(), theme);

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

        let line = Line::from(vec![
            focus_pill.span(),
            Span::raw(" "),
            filter_pill.span(),
            Span::raw(" "),
            chat_pill.span(),
            Span::raw("  "),
            products_pill.span(),
            Span::raw(" "),
            variants_pill.span(),
            Span::raw(" "),
            actors_pill.span(),
            Span::raw(" "),
            runtime_pill.span(),
            Span::raw("  "),
            status_span,
        ]);

        let footer = Paragraph::new(line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.footer_border))
                .title("Status"),
        );

        frame.render_widget(footer, area);
    }
}
