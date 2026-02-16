----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /examples/style/modifiers
- Keywords: ratatui, rust, tui, terminal ui, docs, examples, style, modifiers
- Summary: [`Modifiers`](https://docs.rs/ratatui/latest/ratatui/style/struct.Modifier.html)
----

Source: https://ratatui.rs/examples/style/modifiers

# Modifiers

Demonstrates the style
[`Modifiers`](https://docs.rs/ratatui/latest/ratatui/style/struct.Modifier.html)

run

```
git clone https://github.com/ratatui/ratatui.git --branch latestcd ratatuicargo run --example=modifiers --features=crossterm
```

modifiers.rs

```
//! # [Ratatui] Modifiers example//!//! The latest version of this example is available in the [examples] folder in the repository.//!//! Please note that the examples are designed to be run against the `main` branch of the Github//! repository. This means that you may not be able to compile with the latest release version on//! crates.io, or the one that you have installed locally.//!//! See the [examples readme] for more information on finding examples that match the version of the//! library you are using.//!//! [Ratatui]: https://github.com/ratatui/ratatui//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
// This example is useful for testing how your terminal emulator handles different modifiers.// It will render a grid of combinations of foreground and background colors with all// modifiers applied to them.
use std::{error::Error, iter::once, result};
use itertools::Itertools;use ratatui::{    crossterm::event::{self, Event, KeyCode, KeyEventKind},    layout::{Constraint, Layout},    style::{Color, Modifier, Style, Stylize},    text::Line,    widgets::Paragraph,    DefaultTerminal, Frame,};
type Result&#x3C;T> = result::Result&#x3C;T, Box&#x3C;dyn Error>>;
fn main() -> Result&#x3C;()> {    color_eyre::install()?;    let terminal = ratatui::init();    let app_result = run(terminal);    ratatui::restore();    app_result}
fn run(mut terminal: DefaultTerminal) -> Result&#x3C;()> {    loop {        terminal.draw(draw)?;        if let Event::Key(key) = event::read()? {            if key.kind == KeyEventKind::Press &#x26;&#x26; key.code == KeyCode::Char('q') {                return Ok(());            }        }    }}
fn draw(frame: &#x26;mut Frame) {    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);    let [text_area, main_area] = vertical.areas(frame.area());    frame.render_widget(        Paragraph::new("Note: not all terminals support all modifiers")            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),        text_area,    );    let layout = Layout::vertical([Constraint::Length(1); 50])        .split(main_area)        .iter()        .flat_map(|area| {            Layout::horizontal([Constraint::Percentage(20); 5])                .split(*area)                .to_vec()        })        .collect_vec();
    let colors = [        Color::Black,        Color::DarkGray,        Color::Gray,        Color::White,        Color::Red,    ];    let all_modifiers = once(Modifier::empty())        .chain(Modifier::all().iter())        .collect_vec();    let mut index = 0;    for bg in &#x26;colors {        for fg in &#x26;colors {            for modifier in &#x26;all_modifiers {                let modifier_name = format!("{modifier:11?}");                let padding = (" ").repeat(12 - modifier_name.len());                let paragraph = Paragraph::new(Line::from(vec![                    modifier_name.fg(*fg).bg(*bg).add_modifier(*modifier),                    padding.fg(*fg).bg(*bg).add_modifier(*modifier),                    // This is a hack to work around a bug in VHS which is used for rendering the                    // examples to gifs. The bug is that the background color of a paragraph seems                    // to bleed into the next character.                    ".".black().on_black(),                ]));                frame.render_widget(paragraph, layout[index]);                index += 1;            }        }    }}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/examples/Style/modifiers.md)

 [Previous Colors (RGB)](/examples/style/colors_rgb/) [Next Widget Examples](/examples/widgets/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
