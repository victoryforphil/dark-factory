----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /examples/widgets/block
- Keywords: ratatui, rust, tui, terminal ui, docs, examples, widgets, block
- Summary: Demonstrates the [`Block`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html) widget.
----

Source: https://ratatui.rs/examples/widgets/block

# Block

Demonstrates the [`Block`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html) widget.

run

```
git clone https://github.com/ratatui/ratatui.git --branch latestcd ratatuicargo run --example=block --features=crossterm
```

block.rs

```
//! # [Ratatui] Block example//!//! The latest version of this example is available in the [examples] folder in the repository.//!//! Please note that the examples are designed to be run against the `main` branch of the Github//! repository. This means that you may not be able to compile with the latest release version on//! crates.io, or the one that you have installed locally.//!//! See the [examples readme] for more information on finding examples that match the version of the//! library you are using.//!//! [Ratatui]: https://github.com/ratatui/ratatui//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
use color_eyre::Result;use ratatui::{    crossterm::event::{self, Event, KeyCode, KeyEventKind},    layout::{Alignment, Constraint, Layout, Rect},    style::{Style, Stylize},    text::Line,    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},    DefaultTerminal, Frame,};
fn main() -> Result&#x3C;()> {    color_eyre::install()?;    let terminal = ratatui::init();    let result = run(terminal);    ratatui::restore();    result}
fn run(mut terminal: DefaultTerminal) -> Result&#x3C;()> {    loop {        terminal.draw(draw)?;        if let Event::Key(key) = event::read()? {            if key.kind == KeyEventKind::Press &#x26;&#x26; key.code == KeyCode::Char('q') {                break Ok(());            }        }    }}
fn draw(frame: &#x26;mut Frame) {    let (title_area, layout) = calculate_layout(frame.area());
    render_title(frame, title_area);
    let paragraph = placeholder_paragraph();
    render_borders(&#x26;paragraph, Borders::ALL, frame, layout[0][0]);    render_borders(&#x26;paragraph, Borders::NONE, frame, layout[0][1]);    render_borders(&#x26;paragraph, Borders::LEFT, frame, layout[1][0]);    render_borders(&#x26;paragraph, Borders::RIGHT, frame, layout[1][1]);    render_borders(&#x26;paragraph, Borders::TOP, frame, layout[2][0]);    render_borders(&#x26;paragraph, Borders::BOTTOM, frame, layout[2][1]);
    render_border_type(&#x26;paragraph, BorderType::Plain, frame, layout[3][0]);    render_border_type(&#x26;paragraph, BorderType::Rounded, frame, layout[3][1]);    render_border_type(&#x26;paragraph, BorderType::Double, frame, layout[4][0]);    render_border_type(&#x26;paragraph, BorderType::Thick, frame, layout[4][1]);
    render_styled_block(&#x26;paragraph, frame, layout[5][0]);    render_styled_borders(&#x26;paragraph, frame, layout[5][1]);    render_styled_title(&#x26;paragraph, frame, layout[6][0]);    render_styled_title_content(&#x26;paragraph, frame, layout[6][1]);    render_multiple_titles(&#x26;paragraph, frame, layout[7][0]);    render_multiple_title_positions(&#x26;paragraph, frame, layout[7][1]);    render_padding(&#x26;paragraph, frame, layout[8][0]);    render_nested_blocks(&#x26;paragraph, frame, layout[8][1]);}
/// Calculate the layout of the UI elements.////// Returns a tuple of the title area and the main areas.fn calculate_layout(area: Rect) -> (Rect, Vec&#x3C;Vec&#x3C;Rect>>) {    let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);    let block_layout = Layout::vertical([Constraint::Max(4); 9]);    let [title_area, main_area] = main_layout.areas(area);    let main_areas = block_layout        .split(main_area)        .iter()        .map(|&#x26;area| {            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])                .split(area)                .to_vec()        })        .collect();    (title_area, main_areas)}
fn render_title(frame: &#x26;mut Frame, area: Rect) {    frame.render_widget(        Paragraph::new("Block example. Press q to quit")            .dark_gray()            .alignment(Alignment::Center),        area,    );}
fn placeholder_paragraph() -> Paragraph&#x3C;'static> {    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";    Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true })}
fn render_borders(paragraph: &#x26;Paragraph, border: Borders, frame: &#x26;mut Frame, area: Rect) {    let block = Block::new()        .borders(border)        .title(format!("Borders::{border:#?}"));    frame.render_widget(paragraph.clone().block(block), area);}
fn render_border_type(    paragraph: &#x26;Paragraph,    border_type: BorderType,    frame: &#x26;mut Frame,    area: Rect,) {    let block = Block::bordered()        .border_type(border_type)        .title(format!("BorderType::{border_type:#?}"));    frame.render_widget(paragraph.clone().block(block), area);}fn render_styled_borders(paragraph: &#x26;Paragraph, frame: &#x26;mut Frame, area: Rect) {    let block = Block::bordered()        .border_style(Style::new().blue().on_white().bold().italic())        .title("Styled borders");    frame.render_widget(paragraph.clone().block(block), area);}
fn render_styled_block(paragraph: &#x26;Paragraph, frame: &#x26;mut Frame, area: Rect) {    let block = Block::bordered()        .style(Style::new().blue().on_white().bold().italic())        .title("Styled block");    frame.render_widget(paragraph.clone().block(block), area);}
fn render_styled_title(paragraph: &#x26;Paragraph, frame: &#x26;mut Frame, area: Rect) {    let block = Block::bordered()        .title("Styled title")        .title_style(Style::new().blue().on_white().bold().italic());    frame.render_widget(paragraph.clone().block(block), area);}
fn render_styled_title_content(paragraph: &#x26;Paragraph, frame: &#x26;mut Frame, area: Rect) {    let title = Line::from(vec![        "Styled ".blue().on_white().bold().italic(),        "title content".red().on_white().bold().italic(),    ]);    let block = Block::bordered().title(title);    frame.render_widget(paragraph.clone().block(block), area);}
fn render_multiple_titles(paragraph: &#x26;Paragraph, frame: &#x26;mut Frame, area: Rect) {    let block = Block::bordered()        .title("Multiple".blue().on_white().bold().italic())        .title("Titles".red().on_white().bold().italic());    frame.render_widget(paragraph.clone().block(block), area);}
fn render_multiple_title_positions(paragraph: &#x26;Paragraph, frame: &#x26;mut Frame, area: Rect) {    let block = Block::bordered()        .title(Line::from("top left").left_aligned())        .title(Line::from("top center").centered())        .title(Line::from("top right").right_aligned())        .title_bottom(Line::from("bottom left").left_aligned())        .title_bottom(Line::from("bottom center").centered())        .title_bottom(Line::from("bottom right").right_aligned());    frame.render_widget(paragraph.clone().block(block), area);}
fn render_padding(paragraph: &#x26;Paragraph, frame: &#x26;mut Frame, area: Rect) {    let block = Block::bordered()        .padding(Padding::new(5, 10, 1, 2))        .title("Padding");    frame.render_widget(paragraph.clone().block(block), area);}
fn render_nested_blocks(paragraph: &#x26;Paragraph, frame: &#x26;mut Frame, area: Rect) {    let outer_block = Block::bordered().title("Outer block");    let inner_block = Block::bordered().title("Inner block");    let inner = outer_block.inner(area);    frame.render_widget(outer_block, area);    frame.render_widget(paragraph.clone().block(inner_block), inner);}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/examples/Widgets/block.md)

 [Previous Barchart](/examples/widgets/barchart/) [Next Calendar](/examples/widgets/calendar/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
