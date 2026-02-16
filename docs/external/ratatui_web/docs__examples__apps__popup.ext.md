----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /examples/apps/popup
- Keywords: ratatui, rust, tui, terminal ui, docs, examples, apps, popup
- Summary: Demonstrates how to render a widget over the top of previously rendered widgets using the
----

Source: https://ratatui.rs/examples/apps/popup

# Popup

Demonstrates how to render a widget over the top of previously rendered widgets using the
[`Clear`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html) widget. Source
[popup.rs](https://github.com/ratatui/ratatui/blob/main/examples/apps/popup/src/main.rs).

run

```
git clone https://github.com/ratatui/ratatui.git --branch latestcd ratatuicargo run --example=popup --features=crossterm
```

popup.rs

```
//! # [Ratatui] Popup example//!//! The latest version of this example is available in the [examples] folder in the repository.//!//! Please note that the examples are designed to be run against the `main` branch of the Github//! repository. This means that you may not be able to compile with the latest release version on//! crates.io, or the one that you have installed locally.//!//! See the [examples readme] for more information on finding examples that match the version of the//! library you are using.//!//! [Ratatui]: https://github.com/ratatui/ratatui//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
// See also https://github.com/joshka/tui-popup and// https://github.com/sephiroth74/tui-confirm-dialog
use color_eyre::Result;use ratatui::{    crossterm::event::{self, Event, KeyCode, KeyEventKind},    layout::{Constraint, Flex, Layout, Rect},    style::Stylize,    widgets::{Block, Clear, Paragraph, Wrap},    DefaultTerminal, Frame,};
fn main() -> Result&#x3C;()> {    color_eyre::install()?;    let terminal = ratatui::init();    let app_result = App::default().run(terminal);    ratatui::restore();    app_result}
#[derive(Default)]struct App {    show_popup: bool,}
impl App {    fn run(mut self, mut terminal: DefaultTerminal) -> Result&#x3C;()> {        loop {            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {                if key.kind == KeyEventKind::Press {                    match key.code {                        KeyCode::Char('q') => return Ok(()),                        KeyCode::Char('p') => self.show_popup = !self.show_popup,                        _ => {}                    }                }            }        }    }
    fn draw(&#x26;self, frame: &#x26;mut Frame) {        let area = frame.area();
        let vertical = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);        let [instructions, content] = vertical.areas(area);
        let text = if self.show_popup {            "Press p to close the popup"        } else {            "Press p to show the popup"        };        let paragraph = Paragraph::new(text.slow_blink())            .centered()            .wrap(Wrap { trim: true });        frame.render_widget(paragraph, instructions);
        let block = Block::bordered().title("Content").on_blue();        frame.render_widget(block, content);
        if self.show_popup {            let block = Block::bordered().title("Popup");            let area = popup_area(area, 60, 20);            frame.render_widget(Clear, area); //this clears out the background            frame.render_widget(block, area);        }    }}
/// helper function to create a centered rect using up certain percentage of the available rect `r`fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);    let [area] = vertical.areas(area);    let [area] = horizontal.areas(area);    area}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/examples/Apps/popup.md)

 [Previous Panic Hooks](/examples/apps/panic/) [Next Ratatui Logo](/examples/apps/ratatui-logo/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
