use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, ResultsViewMode};

use super::super::components::{KeyBind, KeyHintBar};

/// Core navigation + view keys (always visible).
const CORE_KEYS: &[KeyBind] = &[
    KeyBind::new("q", "Quit"),
    KeyBind::new("Tab", "Focus"),
    KeyBind::new("j/k", "Select"),
    KeyBind::new("r", "Refresh"),
    KeyBind::new("v", "View"),
    KeyBind::new("f", "Filter"),
];

/// Action keys (always visible).
const ACTION_KEYS: &[KeyBind] = &[
    KeyBind::new("p", "Poll"),
    KeyBind::new("i", "Init"),
    KeyBind::new("n", "Spawn"),
    KeyBind::new("a", "Attach"),
];

/// Extra key hints for viz mode.
const VIZ_KEYS: &[KeyBind] = &[KeyBind::new("0", "Reset pan")];

/// Horizontal key-hint bar rendered between header and body.
///
/// Combines all available key bindings into a single compact line
/// (wrapping to a second line on narrow terminals). Viz-mode extra
/// bindings appear only when the 2D catalog view is active.
pub(crate) struct KeyBarPanel;

impl KeyBarPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();

        // Build combined key list depending on view mode.
        let mut all_keys: Vec<&KeyBind> = Vec::with_capacity(12);
        all_keys.extend(CORE_KEYS.iter());
        all_keys.extend(ACTION_KEYS.iter());

        if app.results_view_mode() == ResultsViewMode::Viz {
            all_keys.extend(VIZ_KEYS.iter());
        }

        // Build owned KeyBind vec for the bar (KeyHintBar expects a slice).
        let owned: Vec<KeyBind> = all_keys
            .into_iter()
            .map(|kb| KeyBind::new(kb.key, kb.action))
            .collect();

        let bar = KeyHintBar::new(&owned);
        let lines = bar.lines_wrapped(area.width, theme);

        // Render up to the available height.
        let visible_lines: Vec<_> = lines.into_iter().take(area.height as usize).collect();
        let widget = Paragraph::new(visible_lines).style(Style::default());

        frame.render_widget(widget, area);
    }
}
