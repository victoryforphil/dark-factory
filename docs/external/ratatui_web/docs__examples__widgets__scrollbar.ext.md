----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /examples/widgets/scrollbar
- Keywords: ratatui, rust, tui, terminal ui, docs, examples, widgets, scrollbar
- Summary: Demonstrates the [`Scrollbar`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Scrollbar.html)
----

Source: https://ratatui.rs/examples/widgets/scrollbar

# Scrollbar

Demonstrates the [`Scrollbar`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Scrollbar.html)
widget.

run

```
git clone https://github.com/ratatui/ratatui.git --branch latestcd ratatuicargo run --example=scrollbar --features=crossterm
```

scrollbar.rs

```
//! # [Ratatui] Scrollbar example//!//! The latest version of this example is available in the [examples] folder in the repository.//!//! Please note that the examples are designed to be run against the `main` branch of the Github//! repository. This means that you may not be able to compile with the latest release version on//! crates.io, or the one that you have installed locally.//!//! See the [examples readme] for more information on finding examples that match the version of the//! library you are using.//!//! [Ratatui]: https://github.com/ratatui/ratatui//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
#![warn(clippy::pedantic)]
use std::time::{Duration, Instant};
use color_eyre::Result;use ratatui::{    crossterm::event::{self, Event, KeyCode},    layout::{Alignment, Constraint, Layout, Margin},    style::{Color, Style, Stylize},    symbols::scrollbar,    text::{Line, Masked, Span},    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},    DefaultTerminal, Frame,};
#[derive(Default)]struct App {    pub vertical_scroll_state: ScrollbarState,    pub horizontal_scroll_state: ScrollbarState,    pub vertical_scroll: usize,    pub horizontal_scroll: usize,}
fn main() -> Result&#x3C;()> {    color_eyre::install()?;    let terminal = ratatui::init();    let app_result = App::default().run(terminal);    ratatui::restore();    app_result}
impl App {    fn run(mut self, mut terminal: DefaultTerminal) -> Result&#x3C;()> {        let tick_rate = Duration::from_millis(250);        let mut last_tick = Instant::now();        loop {            terminal.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());            if event::poll(timeout)? {                if let Event::Key(key) = event::read()? {                    match key.code {                        KeyCode::Char('q') => return Ok(()),                        KeyCode::Char('j') | KeyCode::Down => {                            self.vertical_scroll = self.vertical_scroll.saturating_add(1);                            self.vertical_scroll_state =                                self.vertical_scroll_state.position(self.vertical_scroll);                        }                        KeyCode::Char('k') | KeyCode::Up => {                            self.vertical_scroll = self.vertical_scroll.saturating_sub(1);                            self.vertical_scroll_state =                                self.vertical_scroll_state.position(self.vertical_scroll);                        }                        KeyCode::Char('h') | KeyCode::Left => {                            self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);                            self.horizontal_scroll_state = self                                .horizontal_scroll_state                                .position(self.horizontal_scroll);                        }                        KeyCode::Char('l') | KeyCode::Right => {                            self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);                            self.horizontal_scroll_state = self                                .horizontal_scroll_state                                .position(self.horizontal_scroll);                        }                        _ => {}                    }                }            }            if last_tick.elapsed() >= tick_rate {                last_tick = Instant::now();            }        }    }
    #[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]    fn draw(&#x26;mut self, frame: &#x26;mut Frame) {        let area = frame.area();
        // Words made "loooong" to demonstrate line breaking.        let s =            "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";        let mut long_line = s.repeat(usize::from(area.width) / s.len() + 4);        long_line.push('\n');
        let chunks = Layout::vertical([            Constraint::Min(1),            Constraint::Percentage(25),            Constraint::Percentage(25),            Constraint::Percentage(25),            Constraint::Percentage(25),        ])        .split(area);
        let text = vec![            Line::from("This is a line "),            Line::from("This is a line   ".red()),            Line::from("This is a line".on_dark_gray()),            Line::from("This is a longer line".crossed_out()),            Line::from(long_line.clone()),            Line::from("This is a line".reset()),            Line::from(vec![                Span::raw("Masked text: "),                Span::styled(Masked::new("password", '*'), Style::new().fg(Color::Red)),            ]),            Line::from("This is a line "),            Line::from("This is a line   ".red()),            Line::from("This is a line".on_dark_gray()),            Line::from("This is a longer line".crossed_out()),            Line::from(long_line.clone()),            Line::from("This is a line".reset()),            Line::from(vec![                Span::raw("Masked text: "),                Span::styled(Masked::new("password", '*'), Style::new().fg(Color::Red)),            ]),        ];        self.vertical_scroll_state = self.vertical_scroll_state.content_length(text.len());        self.horizontal_scroll_state = self.horizontal_scroll_state.content_length(long_line.len());
        let create_block = |title: &#x26;'static str| Block::bordered().gray().title(title.bold());
        let title = Block::new()            .title_alignment(Alignment::Center)            .title("Use h j k l or ◄ ▲ ▼ ► to scroll ".bold());        frame.render_widget(title, chunks[0]);
        let paragraph = Paragraph::new(text.clone())            .gray()            .block(create_block("Vertical scrollbar with arrows"))            .scroll((self.vertical_scroll as u16, 0));        frame.render_widget(paragraph, chunks[1]);        frame.render_stateful_widget(            Scrollbar::new(ScrollbarOrientation::VerticalRight)                .begin_symbol(Some("↑"))                .end_symbol(Some("↓")),            chunks[1],            &#x26;mut self.vertical_scroll_state,        );
        let paragraph = Paragraph::new(text.clone())            .gray()            .block(create_block(                "Vertical scrollbar without arrows, without track symbol and mirrored",            ))            .scroll((self.vertical_scroll as u16, 0));        frame.render_widget(paragraph, chunks[2]);        frame.render_stateful_widget(            Scrollbar::new(ScrollbarOrientation::VerticalLeft)                .symbols(scrollbar::VERTICAL)                .begin_symbol(None)                .track_symbol(None)                .end_symbol(None),            chunks[2].inner(Margin {                vertical: 1,                horizontal: 0,            }),            &#x26;mut self.vertical_scroll_state,        );
        let paragraph = Paragraph::new(text.clone())            .gray()            .block(create_block(                "Horizontal scrollbar with only begin arrow &#x26; custom thumb symbol",            ))            .scroll((0, self.horizontal_scroll as u16));        frame.render_widget(paragraph, chunks[3]);        frame.render_stateful_widget(            Scrollbar::new(ScrollbarOrientation::HorizontalBottom)                .thumb_symbol("🬋")                .end_symbol(None),            chunks[3].inner(Margin {                vertical: 0,                horizontal: 1,            }),            &#x26;mut self.horizontal_scroll_state,        );
        let paragraph = Paragraph::new(text.clone())            .gray()            .block(create_block(                "Horizontal scrollbar without arrows &#x26; custom thumb and track symbol",            ))            .scroll((0, self.horizontal_scroll as u16));        frame.render_widget(paragraph, chunks[4]);        frame.render_stateful_widget(            Scrollbar::new(ScrollbarOrientation::HorizontalBottom)                .thumb_symbol("░")                .track_symbol(Some("─")),            chunks[4].inner(Margin {                vertical: 0,                horizontal: 1,            }),            &#x26;mut self.horizontal_scroll_state,        );    }}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/examples/Widgets/scrollbar.md)

 [Previous Paragraph](/examples/widgets/paragraph/) [Next Sparkline](/examples/widgets/sparkline/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
