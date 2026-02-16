----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/render/overwrite-regions
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, render, overwrite regions
- Summary: Use the [`Clear`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html) widget to clear areas of the screen to avoid style and symbols from leaking from
----

Source: https://ratatui.rs/recipes/render/overwrite-regions

# Popups (overwrite regions)

TLDR

Use the [`Clear`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html) widget to clear areas of the screen to avoid style and symbols from leaking from
previously rendered widgets.

Ratatui renders text in the order that the application writes to the buffer. This means that earlier
instructions will be overwritten by later ones. However, it’s important to note that widgets do not
always clear every cell in the area that they are rendering to. This may cause symbols and styles
that were previously rendered to the buffer to “bleed” through into the cells that are rendered on
top of those cells.

The following code exhibits this problem:

```
use lipsum::lipsum;use ratatui::{    backend::CrosstermBackend,    crossterm::{        event::{self, Event},        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},        ExecutableCommand,    },    layout::Rect,    style::{Style, Stylize},    widgets::{Block, Borders, Paragraph, Wrap},    Frame, Terminal,};
// -- snip --
fn ui(frame: &#x26;mut Frame) {    let area = frame.area();    let background_text = Paragraph::new(lipsum(1000))        .wrap(Wrap { trim: true })        .light_blue()        .italic()        .on_black();    frame.render_widget(background_text, area);
    // take up a third of the screen vertically and half horizontally    let popup_area = Rect {        x: area.width / 4,        y: area.height / 3,        width: area.width / 2,        height: area.height / 3,    };    let bad_popup = Paragraph::new("Hello world!")        .wrap(Wrap { trim: true })        .style(Style::new().yellow())        .block(            Block::new()                .title("Without Clear")                .title_style(Style::new().white().bold())                .borders(Borders::ALL)                .border_style(Style::new().red()),        );    frame.render_widget(bad_popup, popup_area);}
```

Notice that the background color (black in this case), the italics, and the lorem ipsum background
text show through the popup.

This problem is easy to prevent by rendering a [`Clear`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html) widget prior to rendering the main popup.
Here is an example of how to use this technique to create a `Popup` widget:

```
use derive_setters::Setters;use lipsum::lipsum;use ratatui::{    backend::CrosstermBackend,    buffer::Buffer,    crossterm::{        event::{self, Event},        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},        ExecutableCommand,    },    layout::Rect,    style::{Style, Stylize},    text::{Line, Text},    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},    Frame, Terminal,};
#[derive(Debug, Default, Setters)]struct Popup&#x3C;'a> {    #[setters(into)]    title: Line&#x3C;'a>,    #[setters(into)]    content: Text&#x3C;'a>,    border_style: Style,    title_style: Style,    style: Style,}
impl Widget for Popup&#x3C;'_> {    fn render(self, area: Rect, buf: &#x26;mut Buffer) {        // ensure that all cells under the popup are cleared to avoid leaking content        Clear.render(area, buf);        let block = Block::new()            .title(self.title)            .title_style(self.title_style)            .borders(Borders::ALL)            .border_style(self.border_style);        Paragraph::new(self.content)            .wrap(Wrap { trim: true })            .style(self.style)            .block(block)            .render(area, buf);    }}
```

We can use the new `Popup` widget with the following code:

```
let popup = Popup::default()        .content("Hello world!")        .style(Style::new().yellow())        .title("With Clear")        .title_style(Style::new().white().bold())        .border_style(Style::new().red());    frame.render_widget(popup, popup_area);
```

Which results in the following:

Notice that the background is set to the default background and there are no italics or symbols from
the background text.

Full source for this article is available at
[https://github.com/ratatui/ratatui-website/tree/main/code/recipes/how-to-overwrite-regions](https://github.com/ratatui/ratatui-website/tree/main/code/recipes/how-to-overwrite-regions)

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/render/overwrite-regions.md)

 [Previous Styling Text](/recipes/render/style-text/) [Next Use Widgets](/recipes/widgets/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
