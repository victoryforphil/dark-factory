use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::ui::command_palette::toolbar_bindings;

use dark_tui_components::{KeyBind, KeyHintBar};

/// Extra key hints while composing a chat message.
const CHAT_COMPOSE_KEYS: &[KeyBind] =
    &[KeyBind::new("Enter", "Send"), KeyBind::new("Esc", "Cancel")];

/// Extra key hints while editing the clone variant form.
const CLONE_FORM_KEYS: &[KeyBind] = &[
    KeyBind::new("Enter", "Clone"),
    KeyBind::new("Tab", "Field"),
    KeyBind::new("Esc", "Cancel"),
];

const BRANCH_FORM_KEYS: &[KeyBind] = &[
    KeyBind::new("Enter", "Switch"),
    KeyBind::new("Tab", "Complete"),
    KeyBind::new("Esc", "Cancel"),
];

const DELETE_FORM_KEYS: &[KeyBind] = &[
    KeyBind::new("Space", "Toggle remove"),
    KeyBind::new("Enter", "Delete"),
    KeyBind::new("Esc", "Cancel"),
];

const MOVE_FORM_KEYS: &[KeyBind] = &[KeyBind::new("Enter", "Move"), KeyBind::new("Esc", "Cancel")];

const INIT_PRODUCT_FORM_KEYS: &[KeyBind] =
    &[KeyBind::new("Enter", "Init"), KeyBind::new("Esc", "Cancel")];

/// Result of clicking on a key hint - maps to app action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyHintAction {
    Quit,
    Focus,
    Select,
    Refresh,
    View,
    Density,
    Filter,
    ToggleInspector,
    Poll,
    SwitchBranch,
    PollActor,
    Clone,
    Delete,
    Import,
    Move,
    Init,
    Spawn,
    Attach,
    Chat,
    CoreLogs,
    Compose,
    ResetPan,
    Send,
    Cancel,
    ToggleRemove,
    #[allow(dead_code)]
    FieldNav,
}

#[derive(Debug, Clone)]
pub struct KeyHoverToken {
    pub row: u16,
    pub col: u16,
    pub width: u16,
    pub text: String,
}

#[derive(Debug, Clone, Copy)]
struct HitBind {
    key: &'static str,
    action: &'static str,
    row: u16,
    key_start: u16,
    key_width: u16,
}

impl KeyHintAction {
    /// Maps a key string to the corresponding action, or None if not clickable.
    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "q" => Some(Self::Quit),
            "Tab" => Some(Self::Focus),
            "↑/↓" => Some(Self::Select),
            "r" => Some(Self::Refresh),
            "v" => Some(Self::View),
            "z" => Some(Self::Density),
            "f" => Some(Self::Filter),
            "s" => Some(Self::ToggleInspector),
            "p" => Some(Self::Poll),
            "o" => Some(Self::PollActor),
            "x" => Some(Self::Clone),
            "w" => Some(Self::SwitchBranch),
            "d" => Some(Self::Delete),
            "m" => Some(Self::Import),
            "g" => Some(Self::Move),
            "i" => Some(Self::Init),
            "n" => Some(Self::Spawn),
            "a" => Some(Self::Attach),
            "t" => Some(Self::Chat),
            "l" => Some(Self::CoreLogs),
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
    fn active_keys(app: &App) -> Vec<KeyBind> {
        let mut all_keys: Vec<KeyBind> = toolbar_bindings(app)
            .into_iter()
            .map(|binding| KeyBind::new(binding.key, binding.label))
            .collect();

        if app.is_chat_composing() {
            all_keys.extend(
                CHAT_COMPOSE_KEYS
                    .iter()
                    .map(|binding| KeyBind::new(binding.key, binding.action)),
            );
        }

        if app.is_clone_form_open() {
            all_keys.extend(
                CLONE_FORM_KEYS
                    .iter()
                    .map(|binding| KeyBind::new(binding.key, binding.action)),
            );
        }

        if app.is_branch_form_open() {
            all_keys.extend(
                BRANCH_FORM_KEYS
                    .iter()
                    .map(|binding| KeyBind::new(binding.key, binding.action)),
            );
        }

        if app.is_delete_variant_form_open() {
            all_keys.extend(
                DELETE_FORM_KEYS
                    .iter()
                    .map(|binding| KeyBind::new(binding.key, binding.action)),
            );
        }

        if app.is_move_actor_form_open() {
            all_keys.extend(
                MOVE_FORM_KEYS
                    .iter()
                    .map(|binding| KeyBind::new(binding.key, binding.action)),
            );
        }

        if app.is_init_product_form_open() {
            all_keys.extend(
                INIT_PRODUCT_FORM_KEYS
                    .iter()
                    .map(|binding| KeyBind::new(binding.key, binding.action)),
            );
        }

        all_keys
    }

    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();

        let owned = Self::active_keys(app);

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
        Self::hit_test_bind(area, app, row, col).and_then(|bind| KeyHintAction::from_key(bind.key))
    }

    pub(crate) fn hover_hint(area: Rect, app: &App, row: u16, col: u16) -> Option<String> {
        let bind = Self::hit_test_bind(area, app, row, col)?;
        Some(format!("Click: {}", bind.action))
    }

    pub(crate) fn hover_token(area: Rect, app: &App, row: u16, col: u16) -> Option<KeyHoverToken> {
        let bind = Self::hit_test_bind(area, app, row, col)?;
        Some(KeyHoverToken {
            row: area.y + bind.row,
            col: area.x + bind.key_start,
            width: bind.key_width,
            text: format!(" {} ", bind.key),
        })
    }

    fn hit_test_bind(area: Rect, app: &App, row: u16, col: u16) -> Option<HitBind> {
        // Check if click is within the key bar area
        if col < area.x || col >= area.x + area.width || row < area.y || row >= area.y + area.height
        {
            return None;
        }

        let local_row = row.saturating_sub(area.y);
        let local_col = col.saturating_sub(area.x);

        let separator = " \u{2502} ";
        let sep_width = display_width(separator);

        let mut current_width: u16 = 0;
        let mut current_row: u16 = 0;

        for bind in Self::active_keys(app) {
            let entry_width = display_width(bind.key) + 2 + display_width(bind.action) + 1;
            let total_width = if current_width > 0 {
                sep_width + entry_width
            } else {
                entry_width
            };

            if current_width > 0 && current_width + total_width > area.width {
                current_row += 1;
                current_width = 0;
            }

            if current_row >= area.height {
                return None;
            }

            let text_start = current_width + if current_width > 0 { sep_width } else { 0 };
            let text_end = text_start + entry_width;
            let key_start = text_start;
            let key_width = display_width(bind.key) + 2;

            if local_row == current_row && local_col >= text_start && local_col < text_end {
                return Some(HitBind {
                    key: bind.key,
                    action: bind.action,
                    row: current_row,
                    key_start,
                    key_width,
                });
            }

            current_width += total_width;
        }

        None
    }
}

fn display_width(value: &str) -> u16 {
    value.chars().count() as u16
}
