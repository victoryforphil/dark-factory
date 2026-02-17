use ratatui::style::Color;

/// Default theme palette used by shared components.
#[derive(Debug, Clone)]
pub struct ComponentTheme {
    pub pill_ok_fg: Color,
    pub pill_ok_bg: Color,
    pub pill_warn_fg: Color,
    pub pill_warn_bg: Color,
    pub pill_err_fg: Color,
    pub pill_err_bg: Color,
    pub pill_info_fg: Color,
    pub pill_info_bg: Color,
    pub pill_muted_fg: Color,
    pub pill_muted_bg: Color,
    pub pill_accent_fg: Color,
    pub pill_accent_bg: Color,
    pub key_hint_key_fg: Color,
    pub key_hint_key_bg: Color,
    pub key_hint_action_fg: Color,
    pub key_hint_bracket_fg: Color,
    pub pane_focused_border: Color,
    pub pane_unfocused_border: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
}

impl Default for ComponentTheme {
    fn default() -> Self {
        Self {
            pill_ok_fg: Color::Rgb(180, 230, 180),
            pill_ok_bg: Color::Rgb(30, 60, 30),
            pill_warn_fg: Color::Rgb(230, 210, 140),
            pill_warn_bg: Color::Rgb(60, 50, 20),
            pill_err_fg: Color::Rgb(240, 160, 150),
            pill_err_bg: Color::Rgb(70, 25, 25),
            pill_info_fg: Color::Rgb(150, 190, 230),
            pill_info_bg: Color::Rgb(25, 40, 65),
            pill_muted_fg: Color::Rgb(140, 140, 140),
            pill_muted_bg: Color::Rgb(40, 40, 40),
            pill_accent_fg: Color::Rgb(160, 220, 230),
            pill_accent_bg: Color::Rgb(25, 55, 60),
            key_hint_key_fg: Color::Rgb(220, 220, 220),
            key_hint_key_bg: Color::Rgb(55, 55, 70),
            key_hint_action_fg: Color::Rgb(150, 150, 160),
            key_hint_bracket_fg: Color::Rgb(90, 90, 110),
            pane_focused_border: Color::Cyan,
            pane_unfocused_border: Color::DarkGray,
            text_secondary: Color::Gray,
            text_muted: Color::DarkGray,
        }
    }
}

/// Theme contract consumed by shared components.
pub trait ComponentThemeLike {
    /// Foreground color for success pills.
    fn pill_ok_fg(&self) -> Color;
    /// Background color for success pills.
    fn pill_ok_bg(&self) -> Color;
    /// Foreground color for warning pills.
    fn pill_warn_fg(&self) -> Color;
    /// Background color for warning pills.
    fn pill_warn_bg(&self) -> Color;
    /// Foreground color for error pills.
    fn pill_err_fg(&self) -> Color;
    /// Background color for error pills.
    fn pill_err_bg(&self) -> Color;
    /// Foreground color for info pills.
    fn pill_info_fg(&self) -> Color;
    /// Background color for info pills.
    fn pill_info_bg(&self) -> Color;
    /// Foreground color for muted pills.
    fn pill_muted_fg(&self) -> Color;
    /// Background color for muted pills.
    fn pill_muted_bg(&self) -> Color;
    /// Foreground color for accent pills.
    fn pill_accent_fg(&self) -> Color;
    /// Background color for accent pills.
    fn pill_accent_bg(&self) -> Color;
    /// Foreground color for key labels in key hint bars.
    fn key_hint_key_fg(&self) -> Color;
    /// Background color for key labels in key hint bars.
    fn key_hint_key_bg(&self) -> Color;
    /// Foreground color for action labels in key hint bars.
    fn key_hint_action_fg(&self) -> Color;
    /// Foreground color for separators in key hint bars.
    fn key_hint_bracket_fg(&self) -> Color;
    /// Border color for focused pane blocks.
    fn pane_focused_border(&self) -> Color;
    /// Border color for unfocused pane blocks.
    fn pane_unfocused_border(&self) -> Color;
    /// Secondary text color.
    fn text_secondary(&self) -> Color;
    /// Muted text color.
    fn text_muted(&self) -> Color;
}

impl ComponentThemeLike for ComponentTheme {
    fn pill_ok_fg(&self) -> Color {
        self.pill_ok_fg
    }
    fn pill_ok_bg(&self) -> Color {
        self.pill_ok_bg
    }
    fn pill_warn_fg(&self) -> Color {
        self.pill_warn_fg
    }
    fn pill_warn_bg(&self) -> Color {
        self.pill_warn_bg
    }
    fn pill_err_fg(&self) -> Color {
        self.pill_err_fg
    }
    fn pill_err_bg(&self) -> Color {
        self.pill_err_bg
    }
    fn pill_info_fg(&self) -> Color {
        self.pill_info_fg
    }
    fn pill_info_bg(&self) -> Color {
        self.pill_info_bg
    }
    fn pill_muted_fg(&self) -> Color {
        self.pill_muted_fg
    }
    fn pill_muted_bg(&self) -> Color {
        self.pill_muted_bg
    }
    fn pill_accent_fg(&self) -> Color {
        self.pill_accent_fg
    }
    fn pill_accent_bg(&self) -> Color {
        self.pill_accent_bg
    }
    fn key_hint_key_fg(&self) -> Color {
        self.key_hint_key_fg
    }
    fn key_hint_key_bg(&self) -> Color {
        self.key_hint_key_bg
    }
    fn key_hint_action_fg(&self) -> Color {
        self.key_hint_action_fg
    }
    fn key_hint_bracket_fg(&self) -> Color {
        self.key_hint_bracket_fg
    }
    fn pane_focused_border(&self) -> Color {
        self.pane_focused_border
    }
    fn pane_unfocused_border(&self) -> Color {
        self.pane_unfocused_border
    }
    fn text_secondary(&self) -> Color {
        self.text_secondary
    }
    fn text_muted(&self) -> Color {
        self.text_muted
    }
}
