----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /concepts/rendering/under-the-hood
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, rendering, under the hood
- Summary: You may have read in previous sections that Ratatui is an immediate mode rendering library. But what does that really mean? And how is it implemented? In this section, we will discuss how Ratatui renders a widget to the screen, starting ...
----

Source: https://ratatui.rs/concepts/rendering/under-the-hood

You may have read in previous sections that Ratatui is an immediate mode rendering library. But what does that really mean? And how is it implemented? In this section, we will discuss how Ratatui renders a widget to the screen, starting with the `Terminal`’s `draw` method and ending with your chosen backend library.

To render an UI in Ratatui, your application calls the [`Terminal::draw()`](https://github.com/ratatui/ratatui/blob/e5caf170c8c304b952cbff7499fd4da17ab154ea/src/terminal.rs#L325-L360) method. This method takes a [closure](https://doc.rust-lang.org/stable/book/ch13-01-closures.html) which accepts an instance of [`Frame`](https://github.com/ratatui/ratatui/blob/e5caf170c8c304b952cbff7499fd4da17ab154ea/src/terminal.rs#L566-L578). Inside the `draw` method, applications can call [`Frame::render_widget()`](https://github.com/ratatui/ratatui/blob/88ae3485c2c540b4ee630ab13e613e84efa7440a/src/terminal.rs#L596) to render the state of a widget within the available renderable area. We only discuss the `Frame::render_widget()` on this page but this discussion about rendering applies equally to [`Frame::render_stateful_widget()`](https://github.com/ratatui/ratatui/blob/88ae3485c2c540b4ee630ab13e613e84efa7440a/src/terminal.rs#L628).

As an example, here is the `terminal.draw()` call for a simple “hello world” with Ratatui.

`terminal.draw(|frame| {    frame.render_widget(Paragraph::new("Hello World!"), frame.area());});`

The closure gets an argument `frame` of type `&mut Frame`.

`frame.area()` returns a `Rect` that represents the total renderable area. `Frame` also holds a reference to an intermediate buffer which it can render widgets to using the `render_widget()` method. At the end of the `draw` method (after the closure returns), Ratatui persists the content of the buffer to the terminal. Let’s walk through more specifics in the following sections.

In Ratatui, the `frame.render_widget()` method calls a `Widget::render()` method on the type-erased struct that implements the [`Widget`](https://github.com/ratatui/ratatui/blob/e5caf170c8c304b952cbff7499fd4da17ab154ea/src/widgets.rs#L107-L112) trait.

`pub trait Widget {    /// Draws the current state of the widget in the given buffer.    fn render(self, area: Rect, buf: &mut Buffer);}`

Any struct (inside Ratatui or third party crates) can implement the `Widget` trait, making an instance of that struct renderable to the terminal. The `Widget::render()` method is the only method required to make a struct a renderable widget.

In the `Paragraph` example above, `frame.render_widget()` calls the [`Widget::render()` method implemented for `Paragraph`](https://github.com/ratatui/ratatui/blob/88ae3485c2c540b4ee630ab13e613e84efa7440a/src/widgets/paragraph.rs#L213-L214). You can take a look at other widgets’ `render` methods for examples of how to draw content.

As a simple example, let’s take a look at the builtin `Clear` widget. The `Clear` widget resets the style information of every cell in the buffer back to the defaults. Here is the full implementation for the `Clear` widget:

`pub struct Clear;impl Widget for Clear {    fn render(self, area: Rect, buf: &mut Buffer) {        for x in area.left()..area.right() {            for y in area.top()..area.bottom() {                buf.get_mut(x, y).reset();            }        }    }}`

In the `Clear` widget example above, when the application calls the `Frame::render_widget()` method, it will call the `Clear`’s `Widget::render()` method passing it the area (a `Rect` value) and a mutable reference to the frame’s [`Buffer`](https://github.com/ratatui/ratatui/blob/e5caf170c8c304b952cbff7499fd4da17ab154ea/src/buffer.rs#L149-L157). You can see that the `render` loops through the entire area and calls `buf.get_mut(x, y).reset()`. Here we only use one of the many methods on `Buffer`, i.e. `get_mut(x, y)` which returns a `Cell` and `reset()` is a method on [`Cell`](https://github.com/ratatui/ratatui/blob/e5caf170c8c304b952cbff7499fd4da17ab154ea/src/buffer.rs#L15-L26).

A [`Buffer`](https://github.com/ratatui/ratatui/blob/e5caf170c8c304b952cbff7499fd4da17ab154ea/src/buffer.rs#L149-L157) represents a rectangular area that covers the Terminal’s [`Viewport`](https://github.com/ratatui/ratatui/blob/88ae3485c2c540b4ee630ab13e613e84efa7440a/src/terminal.rs#L41-L65) which the application can draw into by manipulating its contents. A `Buffer` contains a collection of [`Cell`](https://github.com/ratatui/ratatui/blob/e5caf170c8c304b952cbff7499fd4da17ab154ea/src/buffer.rs#L15-L26)s to represent the rows and columns of the terminal’s display area. As we saw in the `Clear` example above, widgets interact with these `Cell`s using `Buffer` methods.

Here’s a visual representation of a `Buffer` that is 12 `Cell`s wide and 4 `Cell`s tall.

In Ratatui, a `Cell` struct is the smallest renderable unit of code. Each `Cell` tracks symbol and style information (foreground color, background color, modifiers etc). `Cell`s are similar to a “pixel” in a graphical UI. Terminals generally render text so that each individual cell takes up space approximately twice as high as it is wide. A `Cell` in Ratatui should usually contain 1 wide string content.

`Buffer` implements methods to write text, set styles on particular areas and manipulate individual cells. For example,

*   `buf.get_mut(0, 0)` will return a `Cell` with the symbol and style information for row = 0 and col = 0.
*   `buf.set_string(0, 0, "Hello World!", Style::default())` will render `hello world` into the `Buffer` starting at row = 0 and col = 0 with the style set to default for all those cells.

These methods allow any implementation of the `Widget` trait to write into different parts of the `Buffer`.

Every time your application calls `terminal.draw(|frame| ...)`, Ratatui passes into the closure a new instance of [`Frame`](https://github.com/ratatui/ratatui/blob/e5caf170c8c304b952cbff7499fd4da17ab154ea/src/terminal.rs#L566-L578) which contains a mutable reference to an instance of `Buffer`. Ratatui widgets render to this intermediate buffer before any information is written to the terminal and any content rendered to a `Buffer` is only stored in the `Buffer` that is attached to the frame during the `draw` call. This is in contrast to using a library like `crossterm` directly, where writing text to terminal can occur immediately.

After the closure provided to the `draw` method finishes, the `draw` method calls [`Terminal::flush()`](https://github.com/ratatui/ratatui/blob/e5caf170c8c304b952cbff7499fd4da17ab154ea/src/terminal.rs#L253-L263). `flush()` writes the content of the buffer to the terminal. Ratatui uses a double buffer approach. It calculates a `diff` between the current buffer and the previous buffer to figure out what content to write to the terminal screen efficiently. After `flush()`, Ratatui swaps the buffers and the next time it calls `terminal.draw(|frame| ...)` it constructs `Frame` with the other `Buffer`.

Because all widgets render to the same `Buffer` within a single `terminal.draw(|frame| ...)` call, rendering of different widgets may overwrite the same `Cell` in the buffer. This means the order in which widgets are rendered will affect the final UI.

For example, in this `draw` example below, `"content1"` will be overwritten by `"content2"` which will be overwritten by `"content3"` in `Buffer`, and Ratatui will only ever write out `"content3"` to the terminal:

`terminal.draw(|frame| {    frame.render_widget(Paragraph::new("content1"), frame.area());    frame.render_widget(Paragraph::new("content2"), frame.area());    frame.render_widget(Paragraph::new("content3"), frame.area());})`

Before a new `Frame` is constructed, Ratatui wipes the current buffer clean. Because of this, when an application calls `terminal.draw()` it must draw all the widgets it expects to be rendered to the terminal, and not just a part of the frame. The diffing algorithm in Ratatui ensures efficient writing to the terminal screen.

In summary, the application calls `terminal.draw(|frame| ...)`, and the terminal constructs a frame that is passed to the closure provided by the application. The closure draws each widget to the buffer by calling the `Frame::render_widget`, which in turn calls each widget’s render method. Finally, Ratatui writes the contents of the buffer to the terminal.

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
