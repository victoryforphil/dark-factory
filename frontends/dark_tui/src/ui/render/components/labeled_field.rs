use ratatui::style::Style;
use ratatui::text::{Line, Span};

use crate::theme::Theme;

/// Renders a `label: value` pair with consistent dim label + brighter value.
///
/// Used in detail panels to show structured metadata fields with a clear
/// visual hierarchy between field names and their values.
///
/// Usage:
/// ```ignore
/// let field = LabeledField::new("Branch", "main");
/// lines.push(field.line(theme));
/// ```
pub(crate) struct LabeledField {
    label: String,
    value: String,
}

impl LabeledField {
    pub(crate) fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }

    /// Produce a `Line` with dimmed label and primary-colored value.
    ///
    /// Layout: `  label  value`
    pub(crate) fn line(&self, theme: &Theme) -> Line<'static> {
        let label_width = 12; // fixed column for label alignment
        let padded_label = format!("  {:width$}", self.label, width = label_width);

        Line::from(vec![
            Span::styled(padded_label, Style::default().fg(theme.text_muted)),
            Span::styled(
                self.value.clone(),
                Style::default().fg(theme.text_secondary),
            ),
        ])
    }

    /// Produce a `Line` with a compact layout (no padding, tighter spacing).
    ///
    /// Layout: ` label: value`
    #[allow(dead_code)]
    pub(crate) fn line_compact(&self, theme: &Theme) -> Line<'static> {
        Line::from(vec![
            Span::styled(
                format!(" {}: ", self.label),
                Style::default().fg(theme.text_muted),
            ),
            Span::styled(
                self.value.clone(),
                Style::default().fg(theme.text_secondary),
            ),
        ])
    }
}
