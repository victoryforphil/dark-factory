use ratatui::style::Style;
use ratatui::text::{Line, Span};

use crate::theme::ComponentThemeLike;

/// Label/value pair renderer used in metadata panes.
pub struct LabeledField {
    label: String,
    value: String,
}

impl LabeledField {
    /// Creates a labeled field.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }

    /// Renders the field as a fixed-width aligned row.
    pub fn line(&self, theme: &impl ComponentThemeLike) -> Line<'static> {
        let label_width = 12;
        let padded_label = format!("  {:width$}", self.label, width = label_width);

        Line::from(vec![
            Span::styled(padded_label, Style::default().fg(theme.text_muted())),
            Span::styled(
                self.value.clone(),
                Style::default().fg(theme.text_secondary()),
            ),
        ])
    }

    /// Renders the field as a compact `label: value` row.
    pub fn line_compact(&self, theme: &impl ComponentThemeLike) -> Line<'static> {
        Line::from(vec![
            Span::styled(
                format!(" {}: ", self.label),
                Style::default().fg(theme.text_muted()),
            ),
            Span::styled(
                self.value.clone(),
                Style::default().fg(theme.text_secondary()),
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::LabeledField;
    use crate::theme::ComponentTheme;

    #[test]
    fn compact_line_renders_label_and_value() {
        let theme = ComponentTheme::default();
        let line = LabeledField::new("Branch", "main").line_compact(&theme);
        let text = line
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>();
        assert_eq!(text, " Branch: main");
    }
}
