----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /examples/apps/minimal
- Keywords: ratatui, rust, tui, terminal ui, docs, examples, apps, minimal
- Summary: Demonstrates a minimal hello world. Source
----

Source: https://ratatui.rs/examples/apps/minimal

# Minimal Hello World

Demonstrates a minimal hello world. Source
[minimal.rs](https://github.com/ratatui/ratatui/blob/main/examples/apps/minimal/src/main.rs).

run

```
git clone https://github.com/ratatui/ratatui.git --branch latestcd ratatuicargo run --example=minimal --features=crossterm
```

minimal.rs

```
//! # [Ratatui] Minimal example//!//! This is a bare minimum example. There are many approaches to running an application loop, so//! this is not meant to be prescriptive. See the [examples] folder for more complete examples.//! In particular, the [hello-world] example is a good starting point.//!//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples//! [hello-world]: https://github.com/ratatui-org/ratatui/blob/main/examples/hello_world.rs//!//! The latest version of this example is available in the [examples] folder in the repository.//!//! Please note that the examples are designed to be run against the `main` branch of the Github//! repository. This means that you may not be able to compile with the latest release version on//! crates.io, or the one that you have installed locally.//!//! See the [examples readme] for more information on finding examples that match the version of the//! library you are using.//!//! [Ratatui]: https://github.com/ratatui/ratatui//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
use crossterm::event::{self, Event};use ratatui::{text::Text, Frame};
fn main() {    let mut terminal = ratatui::init();    loop {        terminal.draw(draw).expect("failed to draw frame");        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {            break;        }    }    ratatui::restore();}
fn draw(frame: &#x26;mut Frame) {    let text = Text::raw("Hello World!");    frame.render_widget(text, frame.area());}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/examples/Apps/minimal.md)

 [Previous Inline Viewport](/examples/apps/inline/) [Next Panic Hooks](/examples/apps/panic/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
