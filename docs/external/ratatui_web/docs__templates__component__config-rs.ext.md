----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /templates/component/config-rs
- Keywords: ratatui, rust, tui, terminal ui, docs, templates, component, config rs
- Summary: At the moment, our keys are hard coded into the app.
----

Source: https://ratatui.rs/templates/component/config-rs

# Config.rs

At the moment, our keys are hard coded into the app.

- ``` impl Component for Home { fn handle_key_events(&#x26;mut self, key: KeyEvent) -> Action { match self.mode { Mode::Normal | Mode::Processing => { match key.code { KeyCode::Char('q') => Action::Quit, KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit, KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit, KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Suspend, KeyCode::Char('?') => Action::ToggleShowHelp, KeyCode::Char('j') => Action::ScheduleIncrement, KeyCode::Char('k') => Action::ScheduleDecrement, KeyCode::Char('/') => Action::EnterInsert, _ => Action::Tick, } }, Mode::Insert => { match key.code { KeyCode::Esc => Action::EnterNormal, KeyCode::Enter => Action::EnterNormal, _ => { self.input.handle_event(&#x26;crossterm::event::Event::Key(key)); Action::Update }, } }, } } ``` If a user wants to press `Up` and `Down` arrow key to `ScheduleIncrement` and `ScheduleDecrement`, the only way for them to do it is having to make changes to the source code and recompile the app. It would be better to provide a way for users to set up a configuration file that maps key presses to actions. For example, assume we want a user to be able to set up a keyevents-to-actions mapping in a `config.toml` file like below: ``` [keymap]"q" = "Quit""j" = "ScheduleIncrement""k" = "ScheduleDecrement""l" = "ToggleShowHelp""/" = "EnterInsert""ESC" = "EnterNormal""Enter" = "EnterNormal""Ctrl-d" = "Quit""Ctrl-c" = "Quit""Ctrl-z" = "Suspend" ``` We can set up a `Config` struct using [the excellent `config` crate](https://docs.rs/config/0.13.3/config/): ``` use std::collections::HashMap;use std::path::PathBuf; use color_eyre::eyre::Result;use ratatui::crossterm::event::KeyEvent;use serde::Deserialize; use crate::action::Action; #[derive(Clone, Debug, Deserialize, Default)]pub struct AppConfig { #[serde(default)] pub data_dir: PathBuf, #[serde(default)] pub config_dir: PathBuf,} #[derive(Clone, Debug, Default, Deref, DerefMut)]pub struct KeyBindings(pub HashMap&#x3C;Mode, HashMap&#x3C;Vec&#x3C;KeyEvent>, Action>>); #[derive(Clone, Debug, Default, Deserialize)]pub struct Config { #[serde(default, flatten)] pub config: AppConfig, #[serde(default)] pub keybindings: KeyBindings, #[serde(default)] pub styles: Styles,} ``` ## Key Bindings and Styles [Section titled “Key Bindings and Styles”](#key-bindings-and-styles) We are using `serde` to deserialize from a TOML file. Now the default `KeyEvent` serialized format is not very user friendly, so let’s implement our own version: ``` #[derive(Clone, Debug, Default, Deref, DerefMut)]pub struct KeyBindings(pub HashMap&#x3C;Mode, HashMap&#x3C;Vec&#x3C;KeyEvent>, Action>>); impl&#x3C;'de> Deserialize&#x3C;'de> for KeyBindings { fn deserialize&#x3C;D>(deserializer: D) -> Result&#x3C;Self, D::Error> where D: Deserializer&#x3C;'de>, { let parsed_map = HashMap::&#x3C;Mode, HashMap&#x3C;String, Action>>::deserialize(deserializer)?; let keybindings = parsed_map .into_iter() .map(|(mode, inner_map)| { let converted_inner_map = inner_map .into_iter() .map(|(key_str, cmd)| (parse_key_sequence(&#x26;key_str).unwrap(), cmd)) .collect(); (mode, converted_inner_map) }) .collect(); Ok(KeyBindings(keybindings)) }} ``` Now all we need to do is implement a `parse_key_event` function. [You can check the source code for an example of this implementation](https://github.com/ratatui/templates/blob/main/component/template/src/config.rs#L150-L154). TipYou can create different keyevent presses to map to different actions based on the mode of the app by adding more sections into the toml configuration file. And in the `handle_key_events` we get the `Action` that should to be performed from the `HashMap` directly. ``` impl App { fn handle_key_events(&#x26;mut self, key: KeyEvent) -> Result&#x3C;()> { let action_tx = self.action_tx.clone(); let Some(keymap) = self.config.keybindings.get(&#x26;self.mode) else { return Ok(()); }; match keymap.get(&#x26;vec![key]) { Some(action) => { info!("Got action: {action:?}"); action_tx.send(action.clone())?; } _ => { // If the key was not handled as a single key action, // then consider it for multi-key combinations. self.last_tick_key_events.push(key); // Check for multi-key combinations if let Some(action) = keymap.get(&#x26;self.last_tick_key_events) { info!("Got action: {action:?}"); action_tx.send(action.clone())?; } } } Ok(()) }} ``` In the template, it is set up to handle `Vec&#x3C;KeyEvent>` mapped to an `Action`. This allows you to map for example: `&#x3C;g>&#x3C;j>` to `Action::GotoBottom`

- `&#x3C;g>&#x3C;k>` to `Action::GotoTop`

Here’s the JSON configuration we use for the template:

```
{  "keybindings": {    "Home": {      "&#x3C;q>": "Quit", // Quit the application      "&#x3C;j>": "ScheduleIncrement",      "&#x3C;k>": "ScheduleDecrement",      "&#x3C;l>": "ToggleShowHelp",      "&#x3C;/>": "EnterInsert",      "&#x3C;Ctrl-d>": "Quit", // Another way to quit      "&#x3C;Ctrl-c>": "Quit", // Yet another way to quit      "&#x3C;Ctrl-z>": "Suspend", // Suspend the application    },  },}
```

Similarly, we have a `Styles` struct that parses custom styles from a config file.

```
#[derive(Clone, Debug, Default, Deref, DerefMut)]pub struct Styles(pub HashMap&#x3C;Mode, HashMap&#x3C;String, Style>>);
impl&#x3C;'de> Deserialize&#x3C;'de> for Styles {    fn deserialize&#x3C;D>(deserializer: D) -> Result&#x3C;Self, D::Error>    where        D: Deserializer&#x3C;'de>,    {        let parsed_map = HashMap::&#x3C;Mode, HashMap&#x3C;String, String>>::deserialize(deserializer)?;
        let styles = parsed_map            .into_iter()            .map(|(mode, inner_map)| {                let converted_inner_map = inner_map                    .into_iter()                    .map(|(str, style)| (str, parse_style(&#x26;style)))                    .collect();                (mode, converted_inner_map)            })            .collect();
        Ok(Styles(styles))    }}
```

There are some helper functions in the `config.rs` file that you can use to parse the styles and
keybinds.

## XDG Directories

[Section titled “XDG Directories”](#xdg-directories)

The template has two main directories that are used for storing configuration files and data files.

Using the directories crate, we can get the XDG directories for the current user. This allows us to
store the configuration and data files in a platform-agnostic way.

```
lazy_static! {    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();    pub static ref DATA_FOLDER: Option&#x3C;PathBuf> =        env::var(format!("{}_DATA", PROJECT_NAME.clone()))            .ok()            .map(PathBuf::from);    pub static ref CONFIG_FOLDER: Option&#x3C;PathBuf> =        env::var(format!("{}_CONFIG", PROJECT_NAME.clone()))            .ok()            .map(PathBuf::from);}
// -- snip --pub fn get_data_dir() -> PathBuf {    let directory = if let Some(s) = DATA_FOLDER.clone() {        s    } else if let Some(proj_dirs) = project_directory() {        proj_dirs.data_local_dir().to_path_buf()    } else {        PathBuf::from(".").join(".data")    };    directory}
pub fn get_config_dir() -> PathBuf {    let directory = if let Some(s) = CONFIG_FOLDER.clone() {        s    } else if let Some(proj_dirs) = project_directory() {        proj_dirs.config_local_dir().to_path_buf()    } else {        PathBuf::from(".").join(".config")    };    directory}
fn project_directory() -> Option&#x3C;ProjectDirs> {    ProjectDirs::from("com", "kdheepak", env!("CARGO_PKG_NAME")) // Replace kdheepak with your name/project name.}
```

## Final Code

[Section titled “Final Code”](#final-code)

```
use std::{collections::HashMap, env, path::PathBuf};
use color_eyre::Result;use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};use derive_deref::{Deref, DerefMut};use directories::ProjectDirs;use lazy_static::lazy_static;use ratatui::style::{Color, Modifier, Style};use serde::{Deserialize, de::Deserializer};use tracing::error;
use crate::{action::Action, app::Mode};
const CONFIG: &#x26;str = include_str!("../.config/config.json5");
#[derive(Clone, Debug, Deserialize, Default)]pub struct AppConfig {    #[serde(default)]    pub data_dir: PathBuf,    #[serde(default)]    pub config_dir: PathBuf,}
#[derive(Clone, Debug, Default, Deserialize)]pub struct Config {    #[serde(default, flatten)]    pub config: AppConfig,    #[serde(default)]    pub keybindings: KeyBindings,    #[serde(default)]    pub styles: Styles,}
lazy_static! {    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();    pub static ref DATA_FOLDER: Option&#x3C;PathBuf> =        env::var(format!("{}_DATA", PROJECT_NAME.clone()))            .ok()            .map(PathBuf::from);    pub static ref CONFIG_FOLDER: Option&#x3C;PathBuf> =        env::var(format!("{}_CONFIG", PROJECT_NAME.clone()))            .ok()            .map(PathBuf::from);}
impl Config {    pub fn new() -> Result&#x3C;Self, config::ConfigError> {        let default_config: Config = json5::from_str(CONFIG).unwrap();        let data_dir = get_data_dir();        let config_dir = get_config_dir();        let mut builder = config::Config::builder()            .set_default("data_dir", data_dir.to_str().unwrap())?            .set_default("config_dir", config_dir.to_str().unwrap())?;
        let config_files = [            ("config.json5", config::FileFormat::Json5),            ("config.json", config::FileFormat::Json),            ("config.yaml", config::FileFormat::Yaml),            ("config.toml", config::FileFormat::Toml),            ("config.ini", config::FileFormat::Ini),        ];        let mut found_config = false;        for (file, format) in &#x26;config_files {            let source = config::File::from(config_dir.join(file))                .format(*format)                .required(false);            builder = builder.add_source(source);            if config_dir.join(file).exists() {                found_config = true            }        }        if !found_config {            error!("No configuration file found. Application may not behave as expected");        }
        let mut cfg: Self = builder.build()?.try_deserialize()?;
        for (mode, default_bindings) in default_config.keybindings.iter() {            let user_bindings = cfg.keybindings.entry(*mode).or_default();            for (key, cmd) in default_bindings.iter() {                user_bindings                    .entry(key.clone())                    .or_insert_with(|| cmd.clone());            }        }        for (mode, default_styles) in default_config.styles.iter() {            let user_styles = cfg.styles.entry(*mode).or_default();            for (style_key, style) in default_styles.iter() {                user_styles.entry(style_key.clone()).or_insert(*style);            }        }
        Ok(cfg)    }}
pub fn get_data_dir() -> PathBuf {    let directory = if let Some(s) = DATA_FOLDER.clone() {        s    } else if let Some(proj_dirs) = project_directory() {        proj_dirs.data_local_dir().to_path_buf()    } else {        PathBuf::from(".").join(".data")    };    directory}
pub fn get_config_dir() -> PathBuf {    let directory = if let Some(s) = CONFIG_FOLDER.clone() {        s    } else if let Some(proj_dirs) = project_directory() {        proj_dirs.config_local_dir().to_path_buf()    } else {        PathBuf::from(".").join(".config")    };    directory}
fn project_directory() -> Option&#x3C;ProjectDirs> {    ProjectDirs::from("com", "kdheepak", env!("CARGO_PKG_NAME"))}
#[derive(Clone, Debug, Default, Deref, DerefMut)]pub struct KeyBindings(pub HashMap&#x3C;Mode, HashMap&#x3C;Vec&#x3C;KeyEvent>, Action>>);
impl&#x3C;'de> Deserialize&#x3C;'de> for KeyBindings {    fn deserialize&#x3C;D>(deserializer: D) -> Result&#x3C;Self, D::Error>    where        D: Deserializer&#x3C;'de>,    {        let parsed_map = HashMap::&#x3C;Mode, HashMap&#x3C;String, Action>>::deserialize(deserializer)?;
        let keybindings = parsed_map            .into_iter()            .map(|(mode, inner_map)| {                let converted_inner_map = inner_map                    .into_iter()                    .map(|(key_str, cmd)| (parse_key_sequence(&#x26;key_str).unwrap(), cmd))                    .collect();                (mode, converted_inner_map)            })            .collect();
        Ok(KeyBindings(keybindings))    }}
fn parse_key_event(raw: &#x26;str) -> Result&#x3C;KeyEvent, String> {    let raw_lower = raw.to_ascii_lowercase();    let (remaining, modifiers) = extract_modifiers(&#x26;raw_lower);    parse_key_code_with_modifiers(remaining, modifiers)}
fn extract_modifiers(raw: &#x26;str) -> (&#x26;str, KeyModifiers) {    let mut modifiers = KeyModifiers::empty();    let mut current = raw;
    loop {        match current {            rest if rest.starts_with("ctrl-") => {                modifiers.insert(KeyModifiers::CONTROL);                current = &#x26;rest[5..];            }            rest if rest.starts_with("alt-") => {                modifiers.insert(KeyModifiers::ALT);                current = &#x26;rest[4..];            }            rest if rest.starts_with("shift-") => {                modifiers.insert(KeyModifiers::SHIFT);                current = &#x26;rest[6..];            }            _ => break, // break out of the loop if no known prefix is detected        };    }
    (current, modifiers)}
fn parse_key_code_with_modifiers(    raw: &#x26;str,    mut modifiers: KeyModifiers,) -> Result&#x3C;KeyEvent, String> {    let c = match raw {        "esc" => KeyCode::Esc,        "enter" => KeyCode::Enter,        "left" => KeyCode::Left,        "right" => KeyCode::Right,        "up" => KeyCode::Up,        "down" => KeyCode::Down,        "home" => KeyCode::Home,        "end" => KeyCode::End,        "pageup" => KeyCode::PageUp,        "pagedown" => KeyCode::PageDown,        "backtab" => {            modifiers.insert(KeyModifiers::SHIFT);            KeyCode::BackTab        }        "backspace" => KeyCode::Backspace,        "delete" => KeyCode::Delete,        "insert" => KeyCode::Insert,        "f1" => KeyCode::F(1),        "f2" => KeyCode::F(2),        "f3" => KeyCode::F(3),        "f4" => KeyCode::F(4),        "f5" => KeyCode::F(5),        "f6" => KeyCode::F(6),        "f7" => KeyCode::F(7),        "f8" => KeyCode::F(8),        "f9" => KeyCode::F(9),        "f10" => KeyCode::F(10),        "f11" => KeyCode::F(11),        "f12" => KeyCode::F(12),        "space" => KeyCode::Char(' '),        "hyphen" => KeyCode::Char('-'),        "minus" => KeyCode::Char('-'),        "tab" => KeyCode::Tab,        c if c.len() == 1 => {            let mut c = c.chars().next().unwrap();            if modifiers.contains(KeyModifiers::SHIFT) {                c = c.to_ascii_uppercase();            }            KeyCode::Char(c)        }        _ => return Err(format!("Unable to parse {raw}")),    };    Ok(KeyEvent::new(c, modifiers))}
pub fn key_event_to_string(key_event: &#x26;KeyEvent) -> String {    let char;    let key_code = match key_event.code {        KeyCode::Backspace => "backspace",        KeyCode::Enter => "enter",        KeyCode::Left => "left",        KeyCode::Right => "right",        KeyCode::Up => "up",        KeyCode::Down => "down",        KeyCode::Home => "home",        KeyCode::End => "end",        KeyCode::PageUp => "pageup",        KeyCode::PageDown => "pagedown",        KeyCode::Tab => "tab",        KeyCode::BackTab => "backtab",        KeyCode::Delete => "delete",        KeyCode::Insert => "insert",        KeyCode::F(c) => {            char = format!("f({c})");            &#x26;char        }        KeyCode::Char(' ') => "space",        KeyCode::Char(c) => {            char = c.to_string();            &#x26;char        }        KeyCode::Esc => "esc",        KeyCode::Null => "",        KeyCode::CapsLock => "",        KeyCode::Menu => "",        KeyCode::ScrollLock => "",        KeyCode::Media(_) => "",        KeyCode::NumLock => "",        KeyCode::PrintScreen => "",        KeyCode::Pause => "",        KeyCode::KeypadBegin => "",        KeyCode::Modifier(_) => "",    };
    let mut modifiers = Vec::with_capacity(3);
    if key_event.modifiers.intersects(KeyModifiers::CONTROL) {        modifiers.push("ctrl");    }
    if key_event.modifiers.intersects(KeyModifiers::SHIFT) {        modifiers.push("shift");    }
    if key_event.modifiers.intersects(KeyModifiers::ALT) {        modifiers.push("alt");    }
    let mut key = modifiers.join("-");
    if !key.is_empty() {        key.push('-');    }    key.push_str(key_code);
    key}
pub fn parse_key_sequence(raw: &#x26;str) -> Result&#x3C;Vec&#x3C;KeyEvent>, String> {    if raw.chars().filter(|c| *c == '>').count() != raw.chars().filter(|c| *c == '&#x3C;').count() {        return Err(format!("Unable to parse `{}`", raw));    }    let raw = if !raw.contains(">&#x3C;") {        let raw = raw.strip_prefix('&#x3C;').unwrap_or(raw);        let raw = raw.strip_prefix('>').unwrap_or(raw);        raw    } else {        raw    };    let sequences = raw        .split(">&#x3C;")        .map(|seq| {            if let Some(s) = seq.strip_prefix('&#x3C;') {                s            } else if let Some(s) = seq.strip_suffix('>') {                s            } else {                seq            }        })        .collect::&#x3C;Vec&#x3C;_>>();
    sequences.into_iter().map(parse_key_event).collect()}
#[derive(Clone, Debug, Default, Deref, DerefMut)]pub struct Styles(pub HashMap&#x3C;Mode, HashMap&#x3C;String, Style>>);
impl&#x3C;'de> Deserialize&#x3C;'de> for Styles {    fn deserialize&#x3C;D>(deserializer: D) -> Result&#x3C;Self, D::Error>    where        D: Deserializer&#x3C;'de>,    {        let parsed_map = HashMap::&#x3C;Mode, HashMap&#x3C;String, String>>::deserialize(deserializer)?;
        let styles = parsed_map            .into_iter()            .map(|(mode, inner_map)| {                let converted_inner_map = inner_map                    .into_iter()                    .map(|(str, style)| (str, parse_style(&#x26;style)))                    .collect();                (mode, converted_inner_map)            })            .collect();
        Ok(Styles(styles))    }}
pub fn parse_style(line: &#x26;str) -> Style {    let (foreground, background) =        line.split_at(line.to_lowercase().find("on ").unwrap_or(line.len()));    let foreground = process_color_string(foreground);    let background = process_color_string(&#x26;background.replace("on ", ""));
    let mut style = Style::default();    if let Some(fg) = parse_color(&#x26;foreground.0) {        style = style.fg(fg);    }    if let Some(bg) = parse_color(&#x26;background.0) {        style = style.bg(bg);    }    style = style.add_modifier(foreground.1 | background.1);    style}
fn process_color_string(color_str: &#x26;str) -> (String, Modifier) {    let color = color_str        .replace("grey", "gray")        .replace("bright ", "")        .replace("bold ", "")        .replace("underline ", "")        .replace("inverse ", "");
    let mut modifiers = Modifier::empty();    if color_str.contains("underline") {        modifiers |= Modifier::UNDERLINED;    }    if color_str.contains("bold") {        modifiers |= Modifier::BOLD;    }    if color_str.contains("inverse") {        modifiers |= Modifier::REVERSED;    }
    (color, modifiers)}
fn parse_color(s: &#x26;str) -> Option&#x3C;Color> {    let s = s.trim_start();    let s = s.trim_end();    if s.contains("bright color") {        let s = s.trim_start_matches("bright ");        let c = s            .trim_start_matches("color")            .parse::&#x3C;u8>()            .unwrap_or_default();        Some(Color::Indexed(c.wrapping_shl(8)))    } else if s.contains("color") {        let c = s            .trim_start_matches("color")            .parse::&#x3C;u8>()            .unwrap_or_default();        Some(Color::Indexed(c))    } else if s.contains("gray") {        let c = 232            + s.trim_start_matches("gray")                .parse::&#x3C;u8>()                .unwrap_or_default();        Some(Color::Indexed(c))    } else if s.contains("rgb") {        let red = (s.as_bytes()[3] as char).to_digit(10).unwrap_or_default() as u8;        let green = (s.as_bytes()[4] as char).to_digit(10).unwrap_or_default() as u8;        let blue = (s.as_bytes()[5] as char).to_digit(10).unwrap_or_default() as u8;        let c = 16 + red * 36 + green * 6 + blue;        Some(Color::Indexed(c))    } else if s == "bold black" {        Some(Color::Indexed(8))    } else if s == "bold red" {        Some(Color::Indexed(9))    } else if s == "bold green" {        Some(Color::Indexed(10))    } else if s == "bold yellow" {        Some(Color::Indexed(11))    } else if s == "bold blue" {        Some(Color::Indexed(12))    } else if s == "bold magenta" {        Some(Color::Indexed(13))    } else if s == "bold cyan" {        Some(Color::Indexed(14))    } else if s == "bold white" {        Some(Color::Indexed(15))    } else if s == "black" {        Some(Color::Indexed(0))    } else if s == "red" {        Some(Color::Indexed(1))    } else if s == "green" {        Some(Color::Indexed(2))    } else if s == "yellow" {        Some(Color::Indexed(3))    } else if s == "blue" {        Some(Color::Indexed(4))    } else if s == "magenta" {        Some(Color::Indexed(5))    } else if s == "cyan" {        Some(Color::Indexed(6))    } else if s == "white" {        Some(Color::Indexed(7))    } else {        None    }}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/templates/component/config-rs.md)

 [Previous Components/home.rs](/templates/component/components-home-rs/) [Next App.rs](/templates/component/app-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
