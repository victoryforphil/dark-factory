use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::theme::Theme;

/// A single key-action binding, e.g. `q` -> "Quit".
pub struct KeyBind {
    pub key: &'static str,
    pub action: &'static str,
}

impl KeyBind {
    pub const fn new(key: &'static str, action: &'static str) -> Self {
        Self { key, action }
    }
}

/// Renders a horizontal bar of `[key] Action` pairs separated by thin spacers.
///
/// Visual style: keys appear as ` key ` on a subtle dark background with bright
/// text, followed by the action label in muted gray. Pairs are separated by
/// a dim `|` divider.
///
/// Usage:
/// ```ignore
/// let hints = KeyHintBar::new(&[
///     KeyBind::new("q", "Quit"),
///     KeyBind::new("r", "Refresh"),
///     KeyBind::new("v", "View"),
/// ]);
/// let line = hints.line(theme);
/// frame.render_widget(Paragraph::new(line), area);
/// ```
pub struct KeyHintBar<'a> {
    binds: &'a [KeyBind],
    separator: &'a str,
}

impl<'a> KeyHintBar<'a> {
    pub fn new(binds: &'a [KeyBind]) -> Self {
        Self {
            binds,
            separator: " \u{2502} ", // thin ` │ ` divider
        }
    }

    #[allow(dead_code)]
    pub fn separator(mut self, sep: &'a str) -> Self {
        self.separator = sep;
        self
    }

    fn key_style(theme: &Theme) -> Style {
        Style::default()
            .fg(theme.key_hint_key_fg)
            .bg(theme.key_hint_key_bg)
            .add_modifier(Modifier::BOLD)
    }

    fn action_style(theme: &Theme) -> Style {
        Style::default().fg(theme.key_hint_action_fg)
    }

    fn sep_style(theme: &Theme) -> Style {
        Style::default().fg(theme.key_hint_bracket_fg)
    }

    /// Produce a `Line` of styled spans: ` key  action │  key  action │ ...`
    pub fn line(&self, theme: &Theme) -> Line<'static> {
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

    /// Produce multiple `Line`s, wrapping at the given max width.
    pub fn lines_wrapped(&self, max_width: u16, theme: &Theme) -> Vec<Line<'static>> {
        let mut result: Vec<Line<'static>> = Vec::new();
        let mut current_spans: Vec<Span<'static>> = Vec::new();
        let mut current_width: u16 = 0;

        for (i, bind) in self.binds.iter().enumerate() {
            // Calculate width of this entry: ` key ` + ` action` + separator
            let entry_width = (bind.key.len() + 2 + bind.action.len() + 1) as u16;
            let sep_width = if i > 0 {
                self.separator.len() as u16
            } else {
                0
            };
            let total = sep_width + entry_width;

            if current_width > 0 && current_width + total > max_width {
                // Wrap to next line
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
