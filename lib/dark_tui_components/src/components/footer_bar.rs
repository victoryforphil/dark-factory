use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::theme::ComponentThemeLike;

/// Props for a simple segmented footer row.
pub struct FooterBarProps<'a> {
    pub segments: Vec<Span<'a>>,
    pub separator: &'a str,
}

/// Renderer for footer rows built from styled span segments.
pub struct FooterBar;

impl FooterBar {
    /// Renders footer segments with a themed separator.
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        props: FooterBarProps<'_>,
        theme: &impl ComponentThemeLike,
    ) {
        let mut spans: Vec<Span<'_>> = Vec::new();

        for (index, segment) in props.segments.into_iter().enumerate() {
            if index > 0 {
                spans.push(Span::styled(
                    props.separator.to_string(),
                    Style::default().fg(theme.text_muted()),
                ));
            }

            spans.push(segment);
        }

        frame.render_widget(Paragraph::new(Line::from(spans)), area);
    }
}
