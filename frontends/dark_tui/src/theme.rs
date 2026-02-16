use dark_tui_components::ComponentThemeLike;
use ratatui::style::Color;
use serde::Deserialize;
use std::path::Path;

// ---------------------------------------------------------------------------
// TOML-level color representation
// ---------------------------------------------------------------------------

/// A color as represented in TOML: `[r, g, b]` for RGB or a named string.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum TomlColor {
    Rgb([u8; 3]),
    Named(String),
}

impl TomlColor {
    fn to_ratatui(&self) -> Color {
        match self {
            TomlColor::Rgb([r, g, b]) => Color::Rgb(*r, *g, *b),
            TomlColor::Named(name) => match name.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "magenta" => Color::Magenta,
                "cyan" => Color::Cyan,
                "white" => Color::White,
                "darkgray" | "dark_gray" => Color::DarkGray,
                "lightred" | "light_red" => Color::LightRed,
                "lightgreen" | "light_green" => Color::LightGreen,
                "lightyellow" | "light_yellow" => Color::LightYellow,
                "lightblue" | "light_blue" => Color::LightBlue,
                "lightmagenta" | "light_magenta" => Color::LightMagenta,
                "lightcyan" | "light_cyan" => Color::LightCyan,
                "gray" => Color::Gray,
                "reset" => Color::Reset,
                _ => Color::Reset,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// TOML-level theme structure (mirrors the file layout)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
struct TomlTheme {
    #[serde(default)]
    pill: Option<TomlPillColors>,
    #[serde(default)]
    key_hint: Option<TomlKeyHintColors>,
    #[serde(default)]
    entity: Option<TomlEntityColors>,
    #[serde(default)]
    pane: Option<TomlPaneColors>,
    #[serde(default)]
    table: Option<TomlTableColors>,
    #[serde(default)]
    catalog: Option<TomlCatalogColors>,
    #[serde(default)]
    text: Option<TomlTextColors>,
    #[serde(default)]
    header: Option<TomlHeaderColors>,
    #[serde(default)]
    footer: Option<TomlFooterColors>,
}

#[derive(Debug, Clone, Deserialize)]
struct TomlPillColors {
    ok_fg: Option<TomlColor>,
    ok_bg: Option<TomlColor>,
    warn_fg: Option<TomlColor>,
    warn_bg: Option<TomlColor>,
    err_fg: Option<TomlColor>,
    err_bg: Option<TomlColor>,
    info_fg: Option<TomlColor>,
    info_bg: Option<TomlColor>,
    muted_fg: Option<TomlColor>,
    muted_bg: Option<TomlColor>,
    accent_fg: Option<TomlColor>,
    accent_bg: Option<TomlColor>,
}

#[derive(Debug, Clone, Deserialize)]
struct TomlKeyHintColors {
    key_fg: Option<TomlColor>,
    key_bg: Option<TomlColor>,
    action_fg: Option<TomlColor>,
    bracket_fg: Option<TomlColor>,
}

#[derive(Debug, Clone, Deserialize)]
struct TomlEntityColors {
    product: Option<TomlColor>,
    variant: Option<TomlColor>,
    actor: Option<TomlColor>,
}

#[derive(Debug, Clone, Deserialize)]
struct TomlPaneColors {
    focused_border: Option<TomlColor>,
    unfocused_border: Option<TomlColor>,
}

#[derive(Debug, Clone, Deserialize)]
struct TomlTableColors {
    header_fg: Option<TomlColor>,
    highlight_fg: Option<TomlColor>,
    highlight_bg_product: Option<TomlColor>,
    highlight_bg_variant: Option<TomlColor>,
    highlight_bg_actor: Option<TomlColor>,
}

#[derive(Debug, Clone, Deserialize)]
struct TomlCatalogColors {
    connector: Option<TomlColor>,
}

#[derive(Debug, Clone, Deserialize)]
struct TomlTextColors {
    primary: Option<TomlColor>,
    secondary: Option<TomlColor>,
    muted: Option<TomlColor>,
    error: Option<TomlColor>,
    status_normal: Option<TomlColor>,
}

#[derive(Debug, Clone, Deserialize)]
struct TomlHeaderColors {
    border: Option<TomlColor>,
}

#[derive(Debug, Clone, Deserialize)]
struct TomlFooterColors {
    border: Option<TomlColor>,
}

// ---------------------------------------------------------------------------
// Runtime Theme — flat struct with ratatui::Color values, used everywhere
// ---------------------------------------------------------------------------

/// Holds every semantic color used across the TUI.
///
/// Loaded once at startup from an optional TOML file, then shared immutably
/// through the render tree via `&Theme`.
#[derive(Debug, Clone)]
pub struct Theme {
    // -- StatusPill colors --
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

    // -- KeyHintBar colors --
    pub key_hint_key_fg: Color,
    pub key_hint_key_bg: Color,
    pub key_hint_action_fg: Color,
    pub key_hint_bracket_fg: Color,

    // -- Entity identity colors (product=cyan, variant=green, actor=magenta) --
    pub entity_product: Color,
    pub entity_variant: Color,
    pub entity_actor: Color,

    // -- Pane border colors --
    pub pane_focused_border: Color,
    pub pane_unfocused_border: Color,

    // -- Table header/highlight colors --
    pub table_header_fg: Color,
    pub table_highlight_fg: Color,
    pub table_highlight_bg_product: Color,
    pub table_highlight_bg_variant: Color,
    pub table_highlight_bg_actor: Color,

    // -- Catalog (unified view) colors --
    pub catalog_connector: Color,

    // -- General text colors --
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_error: Color,
    pub text_status_normal: Color,

    // -- Header/Footer panel borders --
    pub header_border: Color,
    pub footer_border: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // StatusPill — muted soft palette
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

            // KeyHintBar
            key_hint_key_fg: Color::Rgb(220, 220, 220),
            key_hint_key_bg: Color::Rgb(55, 55, 70),
            key_hint_action_fg: Color::Rgb(150, 150, 160),
            key_hint_bracket_fg: Color::Rgb(90, 90, 110),

            // Entity identity
            entity_product: Color::Cyan,
            entity_variant: Color::Green,
            entity_actor: Color::Magenta,

            // Pane borders
            pane_focused_border: Color::Cyan,
            pane_unfocused_border: Color::DarkGray,

            // Table
            table_header_fg: Color::White,
            table_highlight_fg: Color::Black,
            table_highlight_bg_product: Color::Cyan,
            table_highlight_bg_variant: Color::Green,
            table_highlight_bg_actor: Color::Magenta,

            // Catalog
            catalog_connector: Color::DarkGray,

            // Text
            text_primary: Color::White,
            text_secondary: Color::Gray,
            text_muted: Color::DarkGray,
            text_error: Color::Rgb(240, 160, 150),
            text_status_normal: Color::Rgb(150, 150, 160),

            // Header/Footer
            header_border: Color::Cyan,
            footer_border: Color::DarkGray,
        }
    }
}

impl Theme {
    /// Load a theme from a TOML file. Returns the default theme on any error.
    pub fn load(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(contents) => match toml::from_str::<TomlTheme>(&contents) {
                Ok(parsed) => Self::from_toml(parsed),
                Err(e) => {
                    eprintln!("theme: parse error in {}: {e}", path.display());
                    Self::default()
                }
            },
            Err(e) => {
                eprintln!("theme: could not read {}: {e}", path.display());
                Self::default()
            }
        }
    }

    /// Build a `Theme` by overlaying parsed TOML values onto the default.
    fn from_toml(t: TomlTheme) -> Self {
        let d = Self::default();

        let c = |opt: &Option<TomlColor>, fallback: Color| -> Color {
            opt.as_ref().map(|tc| tc.to_ratatui()).unwrap_or(fallback)
        };

        let pill = t.pill.as_ref();
        let kh = t.key_hint.as_ref();
        let ent = t.entity.as_ref();
        let pane = t.pane.as_ref();
        let tbl = t.table.as_ref();
        let cat = t.catalog.as_ref();
        let txt = t.text.as_ref();
        let hdr = t.header.as_ref();
        let ftr = t.footer.as_ref();

        Self {
            pill_ok_fg: c(&pill.and_then(|p| p.ok_fg.clone()), d.pill_ok_fg),
            pill_ok_bg: c(&pill.and_then(|p| p.ok_bg.clone()), d.pill_ok_bg),
            pill_warn_fg: c(&pill.and_then(|p| p.warn_fg.clone()), d.pill_warn_fg),
            pill_warn_bg: c(&pill.and_then(|p| p.warn_bg.clone()), d.pill_warn_bg),
            pill_err_fg: c(&pill.and_then(|p| p.err_fg.clone()), d.pill_err_fg),
            pill_err_bg: c(&pill.and_then(|p| p.err_bg.clone()), d.pill_err_bg),
            pill_info_fg: c(&pill.and_then(|p| p.info_fg.clone()), d.pill_info_fg),
            pill_info_bg: c(&pill.and_then(|p| p.info_bg.clone()), d.pill_info_bg),
            pill_muted_fg: c(&pill.and_then(|p| p.muted_fg.clone()), d.pill_muted_fg),
            pill_muted_bg: c(&pill.and_then(|p| p.muted_bg.clone()), d.pill_muted_bg),
            pill_accent_fg: c(&pill.and_then(|p| p.accent_fg.clone()), d.pill_accent_fg),
            pill_accent_bg: c(&pill.and_then(|p| p.accent_bg.clone()), d.pill_accent_bg),

            key_hint_key_fg: c(&kh.and_then(|k| k.key_fg.clone()), d.key_hint_key_fg),
            key_hint_key_bg: c(&kh.and_then(|k| k.key_bg.clone()), d.key_hint_key_bg),
            key_hint_action_fg: c(&kh.and_then(|k| k.action_fg.clone()), d.key_hint_action_fg),
            key_hint_bracket_fg: c(
                &kh.and_then(|k| k.bracket_fg.clone()),
                d.key_hint_bracket_fg,
            ),

            entity_product: c(&ent.and_then(|e| e.product.clone()), d.entity_product),
            entity_variant: c(&ent.and_then(|e| e.variant.clone()), d.entity_variant),
            entity_actor: c(&ent.and_then(|e| e.actor.clone()), d.entity_actor),

            pane_focused_border: c(
                &pane.and_then(|p| p.focused_border.clone()),
                d.pane_focused_border,
            ),
            pane_unfocused_border: c(
                &pane.and_then(|p| p.unfocused_border.clone()),
                d.pane_unfocused_border,
            ),

            table_header_fg: c(&tbl.and_then(|t| t.header_fg.clone()), d.table_header_fg),
            table_highlight_fg: c(
                &tbl.and_then(|t| t.highlight_fg.clone()),
                d.table_highlight_fg,
            ),
            table_highlight_bg_product: c(
                &tbl.and_then(|t| t.highlight_bg_product.clone()),
                d.table_highlight_bg_product,
            ),
            table_highlight_bg_variant: c(
                &tbl.and_then(|t| t.highlight_bg_variant.clone()),
                d.table_highlight_bg_variant,
            ),
            table_highlight_bg_actor: c(
                &tbl.and_then(|t| t.highlight_bg_actor.clone()),
                d.table_highlight_bg_actor,
            ),

            catalog_connector: c(&cat.and_then(|c| c.connector.clone()), d.catalog_connector),

            text_primary: c(&txt.and_then(|t| t.primary.clone()), d.text_primary),
            text_secondary: c(&txt.and_then(|t| t.secondary.clone()), d.text_secondary),
            text_muted: c(&txt.and_then(|t| t.muted.clone()), d.text_muted),
            text_error: c(&txt.and_then(|t| t.error.clone()), d.text_error),
            text_status_normal: c(
                &txt.and_then(|t| t.status_normal.clone()),
                d.text_status_normal,
            ),

            header_border: c(&hdr.and_then(|h| h.border.clone()), d.header_border),
            footer_border: c(&ftr.and_then(|f| f.border.clone()), d.footer_border),
        }
    }

    /// Resolve the entity highlight background color for table row selection.
    pub fn table_highlight_bg_for(&self, entity: EntityKind) -> Color {
        match entity {
            EntityKind::Product => self.table_highlight_bg_product,
            EntityKind::Variant => self.table_highlight_bg_variant,
            EntityKind::Actor => self.table_highlight_bg_actor,
        }
    }

    /// Resolve the entity identity color (used for focused borders, card outlines).
    pub fn entity_color(&self, entity: EntityKind) -> Color {
        match entity {
            EntityKind::Product => self.entity_product,
            EntityKind::Variant => self.entity_variant,
            EntityKind::Actor => self.entity_actor,
        }
    }
}

/// Distinguishes entity types for color lookups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Product,
    Variant,
    Actor,
}

impl ComponentThemeLike for Theme {
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
