use ratatui::style::{Color, Style};
use ratatui::text::Span;

use crate::theme::ComponentThemeLike;

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

    pub fn ok(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_ok_fg(), theme.pill_ok_bg())
    }

    pub fn warn(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_warn_fg(), theme.pill_warn_bg())
    }

    pub fn error(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_err_fg(), theme.pill_err_bg())
    }

    pub fn info(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_info_fg(), theme.pill_info_bg())
    }

    pub fn muted(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_muted_fg(), theme.pill_muted_bg())
    }

    pub fn accent(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_accent_fg(), theme.pill_accent_bg())
    }

    pub fn span(&self) -> Span<'static> {
        Span::styled(
            format!(" {} ", self.label),
            Style::default().fg(self.fg).bg(self.bg),
        )
    }

    pub fn span_compact(&self) -> Span<'static> {
        Span::styled(self.label.clone(), Style::default().fg(self.fg).bg(self.bg))
    }
}

#[cfg(test)]
mod tests {
    use super::StatusPill;
    use crate::theme::ComponentTheme;

    #[test]
    fn span_has_padded_label() {
        let theme = ComponentTheme::default();
        let pill = StatusPill::ok("clean", &theme);
        assert_eq!(pill.span().content, " clean ");
        assert_eq!(pill.span_compact().content, "clean");
    }
}
