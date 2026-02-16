use ratatui::style::{Color, Style};
use ratatui::text::Span;

use crate::theme::Theme;

/// A colored inline label/pill: ` label ` with foreground + background.
///
/// Usage:
/// ```ignore
/// let span = StatusPill::new("clean", theme.pill_ok_fg, theme.pill_ok_bg).span();
/// let line = Line::from(vec![span, Span::raw(" "), other_span]);
/// ```
///
/// Semantic constructors provide consistent color mappings from theme:
/// - `ok(label, theme)` — soft green for healthy/clean states
/// - `warn(label, theme)` — soft amber for attention/dirty states
/// - `error(label, theme)` — soft red for errors/drift
/// - `info(label, theme)` — soft blue for neutral information
/// - `muted(label, theme)` — dim gray for secondary metadata
/// - `accent(label, theme)` — soft cyan for highlighted/focused labels
pub struct StatusPill {
    label: String,
    fg: Color,
    bg: Color,
}

impl StatusPill {
    pub fn new(label: impl Into<String>, fg: Color, bg: Color) -> Self {
        Self {
            label: label.into(),
            fg,
            bg,
        }
    }

    // --- Semantic constructors (theme-driven) ---

    /// Soft green pill for healthy/clean/active states.
    pub fn ok(label: impl Into<String>, theme: &Theme) -> Self {
        Self::new(label, theme.pill_ok_fg, theme.pill_ok_bg)
    }

    /// Soft amber pill for attention/dirty/warning states.
    pub fn warn(label: impl Into<String>, theme: &Theme) -> Self {
        Self::new(label, theme.pill_warn_fg, theme.pill_warn_bg)
    }

    /// Soft red pill for error/drift/critical states.
    pub fn error(label: impl Into<String>, theme: &Theme) -> Self {
        Self::new(label, theme.pill_err_fg, theme.pill_err_bg)
    }

    /// Soft blue pill for neutral informational labels.
    pub fn info(label: impl Into<String>, theme: &Theme) -> Self {
        Self::new(label, theme.pill_info_fg, theme.pill_info_bg)
    }

    /// Dim gray pill for secondary/muted metadata.
    pub fn muted(label: impl Into<String>, theme: &Theme) -> Self {
        Self::new(label, theme.pill_muted_fg, theme.pill_muted_bg)
    }

    /// Soft cyan pill for highlighted/focused labels.
    pub fn accent(label: impl Into<String>, theme: &Theme) -> Self {
        Self::new(label, theme.pill_accent_fg, theme.pill_accent_bg)
    }

    /// Produce a styled `Span` with padded text: ` label `.
    pub fn span(&self) -> Span<'static> {
        Span::styled(
            format!(" {} ", self.label),
            Style::default().fg(self.fg).bg(self.bg),
        )
    }

    /// Produce a styled `Span` without padding (compact): `label`.
    #[allow(dead_code)]
    pub fn span_compact(&self) -> Span<'static> {
        Span::styled(self.label.clone(), Style::default().fg(self.fg).bg(self.bg))
    }
}
