use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::Frame;

use dark_tui_components::{
    compact_text, FooterBar, FooterBarProps, PaneBlockComponent, StatusPill,
};

use crate::tui::app::App;

pub struct FooterPanel;

impl FooterPanel {
    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let block = PaneBlockComponent::build("Status", false, theme);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let activity_pill = if app.activity_label() == "idle" {
            StatusPill::muted("idle", theme)
        } else {
            StatusPill::info(app.activity_label(), theme)
        };

        FooterBar::render(
            frame,
            inner,
            FooterBarProps {
                segments: vec![
                    Span::styled(
                        app.status_message().to_string(),
                        Style::default().fg(theme.text_secondary),
                    ),
                    StatusPill::accent(
                        format!(
                            "model:{}",
                            compact_text(app.active_model().unwrap_or("-"), 28)
                        ),
                        theme,
                    )
                    .span_compact(),
                    activity_pill.span(),
                    Span::styled(
                        format!("last-sync:{}", app.last_synced()),
                        Style::default().fg(theme.text_muted),
                    ),
                ],
                separator: "  ",
            },
            theme,
        );
    }
}
