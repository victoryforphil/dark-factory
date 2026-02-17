use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::theme::ComponentThemeLike;

/// Stylized section header with trailing rule fill.
pub struct SectionHeader {
    label: String,
    accent: Color,
}

impl SectionHeader {
    /// Creates a section header with an accent color.
    pub fn new(label: impl Into<String>, accent: Color) -> Self {
        Self {
            label: label.into(),
            accent,
        }
    }

    /// Renders the header line padded to `available_width`.
    pub fn line(&self, available_width: u16, theme: &impl ComponentThemeLike) -> Line<'static> {
        let prefix = "  ";
        let label_text = self.label.to_uppercase();
        let suffix = " ";

        let used = prefix.len() + label_text.len() + suffix.len();
        let rule_len = (available_width as usize).saturating_sub(used);
        let rule: String = "\u{2500}".repeat(rule_len);

        Line::from(vec![
            Span::raw(prefix.to_string()),
            Span::styled(
                label_text,
                Style::default()
                    .fg(self.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(suffix.to_string()),
            Span::styled(rule, Style::default().fg(theme.text_muted())),
        ])
    }
}
