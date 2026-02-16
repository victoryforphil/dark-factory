use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::theme::ComponentThemeLike;

pub struct KeyBind {
    pub key: &'static str,
    pub action: &'static str,
}

impl KeyBind {
    pub const fn new(key: &'static str, action: &'static str) -> Self {
        Self { key, action }
    }
}

pub struct KeyHintBar<'a> {
    binds: &'a [KeyBind],
    separator: &'a str,
}

impl<'a> KeyHintBar<'a> {
    pub fn new(binds: &'a [KeyBind]) -> Self {
        Self {
            binds,
            separator: " \u{2502} ",
        }
    }

    pub fn separator(mut self, sep: &'a str) -> Self {
        self.separator = sep;
        self
    }

    fn key_style(theme: &impl ComponentThemeLike) -> Style {
        Style::default()
            .fg(theme.key_hint_key_fg())
            .bg(theme.key_hint_key_bg())
            .add_modifier(Modifier::BOLD)
    }

    fn action_style(theme: &impl ComponentThemeLike) -> Style {
        Style::default().fg(theme.key_hint_action_fg())
    }

    fn sep_style(theme: &impl ComponentThemeLike) -> Style {
        Style::default().fg(theme.key_hint_bracket_fg())
    }

    pub fn line(&self, theme: &impl ComponentThemeLike) -> Line<'static> {
        let mut spans: Vec<Span<'static>> = Vec::new();

        for (i, bind) in self.binds.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(
                    self.separator.to_string(),
                    Self::sep_style(theme),
                ));
            }
            spans.push(Span::styled(
                format!(" {} ", bind.key),
                Self::key_style(theme),
            ));
            spans.push(Span::styled(
                format!(" {}", bind.action),
                Self::action_style(theme),
            ));
        }

        Line::from(spans)
    }

    pub fn lines_wrapped(
        &self,
        max_width: u16,
        theme: &impl ComponentThemeLike,
    ) -> Vec<Line<'static>> {
        let mut result: Vec<Line<'static>> = Vec::new();
        let mut current_spans: Vec<Span<'static>> = Vec::new();
        let mut current_width: u16 = 0;

        for (i, bind) in self.binds.iter().enumerate() {
            let entry_width = (bind.key.len() + 2 + bind.action.len() + 1) as u16;
            let sep_width = if i > 0 {
                self.separator.len() as u16
            } else {
                0
            };
            let total = sep_width + entry_width;

            if current_width > 0 && current_width + total > max_width {
                result.push(Line::from(std::mem::take(&mut current_spans)));
                current_width = 0;
            }

            if current_width > 0 {
                current_spans.push(Span::styled(
                    self.separator.to_string(),
                    Self::sep_style(theme),
                ));
                current_width += sep_width;
            }

            current_spans.push(Span::styled(
                format!(" {} ", bind.key),
                Self::key_style(theme),
            ));
            current_spans.push(Span::styled(
                format!(" {}", bind.action),
                Self::action_style(theme),
            ));
            current_width += entry_width;
        }

        if !current_spans.is_empty() {
            result.push(Line::from(current_spans));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyBind, KeyHintBar};
    use crate::theme::ComponentTheme;

    #[test]
    fn wraps_on_small_width() {
        let theme = ComponentTheme::default();
        let binds = [
            KeyBind::new("q", "Quit"),
            KeyBind::new("r", "Refresh"),
            KeyBind::new("v", "View"),
        ];
        let lines = KeyHintBar::new(&binds).lines_wrapped(12, &theme);
        assert!(lines.len() > 1);
    }
}
