----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /examples/apps/ratatui-logo
- Keywords: ratatui, rust, tui, terminal ui, docs, examples, apps, ratatui logo
- Summary: A fun example of using half blocks to render graphics.
----

Source: https://ratatui.rs/examples/apps/ratatui-logo

# Ratatui Logo

A fun example of using half blocks to render graphics.

run

```
git clone https://github.com/ratatui/ratatui.git --branch latestcd ratatuicargo run --example=ratatui-logo --features=crossterm
```

ratatui-logo.rs

```
//! # [Ratatui] Logo example//!//! The latest version of this example is available in the [examples] folder in the repository.//!//! Please note that the examples are designed to be run against the `main` branch of the Github//! repository. This means that you may not be able to compile with the latest release version on//! crates.io, or the one that you have installed locally.//!//! See the [examples readme] for more information on finding examples that match the version of the//! library you are using.//!//! [Ratatui]: https://github.com/ratatui/ratatui//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
use std::env::args;
use color_eyre::Result;use crossterm::event::{self, Event};use ratatui::{    layout::{Constraint, Layout},    widgets::{RatatuiLogo, RatatuiLogoSize},    DefaultTerminal, TerminalOptions, Viewport,};
fn main() -> Result&#x3C;()> {    color_eyre::install()?;    let terminal = ratatui::init_with_options(TerminalOptions {        viewport: Viewport::Inline(3),    });    let size = match args().nth(1).as_deref() {        Some("small") => RatatuiLogoSize::Small,        Some("tiny") => RatatuiLogoSize::Tiny,        _ => RatatuiLogoSize::default(),    };    let result = run(terminal, size);    ratatui::restore();    println!();    result}
fn run(mut terminal: DefaultTerminal, size: RatatuiLogoSize) -> Result&#x3C;()> {    loop {        terminal.draw(|frame| {            use Constraint::{Fill, Length};            let [top, bottom] = Layout::vertical([Length(1), Fill(1)]).areas(frame.area());            frame.render_widget("Powered by", top);            frame.render_widget(RatatuiLogo::new(size), bottom);        })?;        if matches!(event::read()?, Event::Key(_)) {            break Ok(());        }    }}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/examples/Apps/ratatui-logo.md)

 [Previous Popup](/examples/apps/popup/) [Next User Input](/examples/apps/user_input/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
