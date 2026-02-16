----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/layout/center-a-widget
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, layout, center a widget
- Summary: [Section titled “Problem”](#problem)
----

Source: https://ratatui.rs/recipes/layout/center-a-widget

# How to Center Widgets

## Problem

[Section titled “Problem”](#problem)

You want to center a widget within some area of your TUI’s layout.

## Solution

[Section titled “Solution”](#solution)

To center a widget in any area, create a [`Rect`](https://docs.rs/ratatui/latest/ratatui/struct.Rect.html) that is centered within the area. You can
calculate the x and y positions of the widget by subtracting the widget width and height from the
enclosing area’s width and height, respectively, and dividing by 2.

More simply, you can use the `.centered_vertically()` and `.centered_horizontally()` methods on
[`Rect`](https://docs.rs/ratatui/latest/ratatui/struct.Rect.html).

### Centering horizontally

[Section titled “Centering horizontally”](#centering-horizontally)

- ``` use ratatui::layout::{Constraint, Rect}; fn center_horizontal(area: Rect, width: u16) -> Rect { area.centered_horizontally(Constraint::Length(width))} ``` ### Centering vertically [Section titled “Centering vertically”](#centering-vertically) ``` use ratatui::layout::{Constraint, Rect}; fn center_vertical(area: Rect, height: u16) -> Rect { area.centered_vertically(Constraint::Length(height))} ``` ### Centering both horizontally and vertically [Section titled “Centering both horizontally and vertically”](#centering-both-horizontally-and-vertically) You can use the `.centered` method to get a centered `Rect`. ``` 13 collapsed lines/// Centers a [`Rect`] within another [`Rect`] using the provided [`Constraint`]s.////// # Examples////// ```rust/// use ratatui::layout::{Constraint, Rect};////// let area = Rect::new(0, 0, 100, 100);/// let horizontal = Constraint::Percentage(20);/// let vertical = Constraint::Percentage(30);////// let centered = center(area, horizontal, vertical);/// ```fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect { area.centered(horizontal, vertical)} ``` ### Centering a widget [Section titled “Centering a widget”](#centering-a-widget) You can use these methods to draw any widget centered on the containing area. ``` fn render(frame: &#x26;mut Frame) { let text = Text::raw("Hello world!"); let area = frame.area().centered( Constraint::Length(text.width() as u16), Constraint::Length(1), ); frame.render_widget(text, area);} ``` ### Popups [Section titled “Popups”](#popups) A common use case for this feature is to create a popup style dialog block. For this, typically, you’ll want to use the [`Clear`] widget to clear the popup area before rendering your content to it. The following is an example of how you might do that: ``` fn render_popup(frame: &#x26;mut Frame) { let area = frame.area().centered( Constraint::Percentage(20), Constraint::Length(3), // top and bottom border + content ); let popup = Paragraph::new("Popup content").block(Block::bordered().title("Popup")); frame.render_widget(Clear, area); frame.render_widget(popup, area);} ``` ## Summary [Section titled “Summary”](#summary) Center a widget by placing it inside a `Rect` that sits in the middle of the area. Compute that rect by hand or use the `.centered`, `.centered_horizontally()`, and `.centered_vertically()` helpers on [`Rect`](https://docs.rs/ratatui/latest/ratatui/struct.Rect.html), then render the widget (popups included) into it. NoteThere is no method for vertically aligning text within an area yet. We recommend prewrapping the text using the [textwrap crate](https://crates.io/crates/textwrap) and then using the line count to work out where to render the text. Full code for this recipe is available in the website repo at: [https://github.com/ratatui/ratatui-website/blob/main/code/recipes/how-to-misc/src/layout.rs](https://github.com/ratatui/ratatui-website/blob/main/code/recipes/how-to-misc/src/layout.rs) ## See also [Section titled “See also”](#see-also) There are several third party widget libraries for making popups easy to use: [tui-popup](https://crates.io/crates/tui-popup)

- [tui-confirm-dialog](https://crates.io/crates/tui-confirm-dialog)

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/layout/center-a-widget.md)

 [Previous Grid Layout](/recipes/layout/grid/) [Next Collapse Borders](/recipes/layout/collapse-borders/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
