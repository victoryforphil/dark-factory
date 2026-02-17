use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, ResultsViewMode};

use dark_tui_components::{KeyBind, KeyHintBar};

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
    KeyBind::new("o", "Poll actor"),
    KeyBind::new("x", "Clone"),
    KeyBind::new("d", "Delete"),
    KeyBind::new("m", "Import"),
    KeyBind::new("g", "Move"),
    KeyBind::new("i", "Init"),
    KeyBind::new("n", "Spawn"),
    KeyBind::new("a", "Attach"),
    KeyBind::new("t", "Chat"),
    KeyBind::new("c", "Compose"),
];

/// Extra key hints for viz mode.
const VIZ_KEYS: &[KeyBind] = &[KeyBind::new("0", "Reset pan")];

/// Extra key hints while composing a chat message.
const CHAT_COMPOSE_KEYS: &[KeyBind] =
    &[KeyBind::new("Enter", "Send"), KeyBind::new("Esc", "Cancel")];

/// Extra key hints while editing the clone variant form.
const CLONE_FORM_KEYS: &[KeyBind] = &[
    KeyBind::new("Enter", "Clone"),
    KeyBind::new("Tab", "Field"),
    KeyBind::new("Esc", "Cancel"),
];

const DELETE_FORM_KEYS: &[KeyBind] = &[
    KeyBind::new("Space", "Toggle remove"),
    KeyBind::new("Enter", "Delete"),
    KeyBind::new("Esc", "Cancel"),
];

const MOVE_FORM_KEYS: &[KeyBind] = &[KeyBind::new("Enter", "Move"), KeyBind::new("Esc", "Cancel")];

/// Result of clicking on a key hint - maps to app action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyHintAction {
    Quit,
    Focus,
    Select,
    Refresh,
    View,
    Filter,
    Poll,
    PollActor,
    Clone,
    Delete,
    Import,
    Move,
    Init,
    Spawn,
    Attach,
    Chat,
    Compose,
    ResetPan,
    Send,
    Cancel,
    ToggleRemove,
    #[allow(dead_code)]
    FieldNav,
}

impl KeyHintAction {
    /// Maps a key string to the corresponding action, or None if not clickable.
    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "q" => Some(Self::Quit),
            "Tab" => Some(Self::Focus),
            "j/k" => Some(Self::Select),
            "r" => Some(Self::Refresh),
            "v" => Some(Self::View),
            "f" => Some(Self::Filter),
            "p" => Some(Self::Poll),
            "o" => Some(Self::PollActor),
            "x" => Some(Self::Clone),
            "d" => Some(Self::Delete),
            "m" => Some(Self::Import),
            "g" => Some(Self::Move),
            "i" => Some(Self::Init),
            "n" => Some(Self::Spawn),
            "a" => Some(Self::Attach),
            "t" => Some(Self::Chat),
            "c" => Some(Self::Compose),
            "0" => Some(Self::ResetPan),
            "Enter" => Some(Self::Send),
            "Esc" => Some(Self::Cancel),
            "Space" => Some(Self::ToggleRemove),
            _ => None,
        }
    }
}

/// Horizontal key-hint bar rendered between header and body.
///
/// Combines all available key bindings into a single compact line
/// (wrapping to a second line on narrow terminals). Viz-mode extra
/// bindings appear only when the 2D catalog view is active.
pub(crate) struct KeyBarPanel;

impl KeyBarPanel {
    fn active_keys(app: &App) -> Vec<&'static KeyBind> {
        let mut all_keys: Vec<&KeyBind> = Vec::with_capacity(12);
        all_keys.extend(CORE_KEYS.iter());
        all_keys.extend(ACTION_KEYS.iter());

        if app.results_view_mode() == ResultsViewMode::Viz {
            all_keys.extend(VIZ_KEYS.iter());
        }

        if app.is_chat_composing() {
            all_keys.extend(CHAT_COMPOSE_KEYS.iter());
        }

        if app.is_clone_form_open() {
            all_keys.extend(CLONE_FORM_KEYS.iter());
        }

        if app.is_delete_variant_form_open() {
            all_keys.extend(DELETE_FORM_KEYS.iter());
        }

        if app.is_move_actor_form_open() {
            all_keys.extend(MOVE_FORM_KEYS.iter());
        }

        all_keys
    }

    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();

        let all_keys = Self::active_keys(app);

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

    /// Hit test: returns the key hint action at the given position, or None if not on a key hint.
    /// The area is the key bar bounds, and row/col are absolute terminal coordinates.
    pub(crate) fn hit_test(area: Rect, app: &App, row: u16, col: u16) -> Option<KeyHintAction> {
        // Check if click is within the key bar area
        if col < area.x || col >= area.x + area.width || row < area.y || row >= area.y + area.height
        {
            return None;
        }

        let local_row = row.saturating_sub(area.y);
        let local_col = col.saturating_sub(area.x);

        // Build the same key list as render() to calculate positions
        let all_keys: Vec<(&'static str, &'static str)> = Self::active_keys(app)
            .into_iter()
            .map(|kb| (kb.key, kb.action))
            .collect();

        // Calculate which key was clicked by tracking column positions
        // Format: " key " (2 + key_len) + separator + " action" (1 + action_len)
        let separator = " \u{2502} ";

        let mut current_col: u16 = 0;
        let mut current_row: u16 = 0;

        for (key, action) in &all_keys {
            let key_width = (key.len() + 2) as u16; // " key "
            let sep_width = if current_col > 0 {
                separator.len() as u16
            } else {
                0
            };
            let action_width = (action.len() + 1) as u16; // " action"

            let entry_start = current_col;
            let entry_end = current_col + sep_width + key_width + action_width;

            // Check if click is within this entry
            if local_row == current_row && local_col >= entry_start && local_col < entry_end {
                // Check if click is on the key part (the first part of the entry)
                let key_start = current_col + sep_width;
                let key_end = key_start + key_width;

                if local_col >= key_start && local_col < key_end {
                    return KeyHintAction::from_key(key);
                }
            }

            // Move to next position
            let entry_width = if current_col > 0 { separator.len() } else { 0 }
                + key.len()
                + 2
                + action.len()
                + 1;

            current_col += entry_width as u16;

            // Wrap to next row if needed
            if current_col > area.width && area.width > 0 {
                current_col = 0;
                current_row += 1;
            }
        }

        None
    }
}
