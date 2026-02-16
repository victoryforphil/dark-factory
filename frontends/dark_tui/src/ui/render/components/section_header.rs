use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::theme::Theme;

/// A thin section divider line within a panel.
///
/// Renders as: `  LABEL ─────` with the label in the given color and the
/// trailing rule in muted. Useful for breaking panels into logical groups.
///
/// Usage:
/// ```ignore
/// let header = SectionHeader::new("Git Info", theme.entity_variant);
/// lines.push(header.line(inner_width, theme));
/// ```
pub(crate) struct SectionHeader {
    label: String,
    accent: ratatui::style::Color,
}

impl SectionHeader {
    pub(crate) fn new(label: impl Into<String>, accent: ratatui::style::Color) -> Self {
        Self {
            label: label.into(),
            accent,
        }
    }

    /// Produce a styled `Line` with the label and a trailing horizontal rule.
    ///
    /// Layout: `  LABEL ────────`
    /// - Label is bold + accented
    /// - Rule fills remaining width in muted color
    pub(crate) fn line(&self, available_width: u16, theme: &Theme) -> Line<'static> {
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
            Span::styled(rule, Style::default().fg(theme.text_muted)),
        ])
    }
}
