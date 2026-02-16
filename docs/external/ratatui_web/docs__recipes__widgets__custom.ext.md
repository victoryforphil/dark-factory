----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/widgets/custom
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, widgets, custom
- Summary: While Ratatui offers a rich set of pre-built widgets, there may be scenarios where you require a
----

Source: https://ratatui.rs/recipes/widgets/custom

# Create custom widgets

While Ratatui offers a rich set of pre-built widgets, there may be scenarios where you require a
unique component tailored to specific needs. In such cases, creating a custom widget becomes
invaluable. This page will guide you through the process of designing and implementing custom
widgets.

## `Widget` trait

[Section titled “Widget trait”](#widget-trait)

At the core of creating a custom widget is the `Widget` trait. Any struct that implements this trait
can be rendered using the framework’s drawing capabilities.

- ``` pub struct MyWidget { // Custom widget properties content: String,} impl Widget for MyWidget { fn render(self, area: Rect, buf: &#x26;mut Buffer) { // Rendering logic goes here }} ``` The `render` method must draw into the current `Buffer`. There are a number of methods implemented on `Buffer`. ``` impl Widget for MyWidget { fn render(self, area: Rect, buf: &#x26;mut Buffer) { buf.set_string(area.left(), area.top(), &#x26;self.content, Style::default().fg(Color::Green)); }} ``` For a given state, the `Widget` trait implements how that struct should be rendered. ``` pub struct Button { label: String, is_pressed: bool, style: Style, pressed_style: Option&#x3C;Style>,} impl Widget for Button { fn render(self, area: Rect, buf: &#x26;mut Buffer) { let style = if self.is_pressed { self.pressed_style.unwrap_or_else(|| Style::default().fg(Color::Blue)) } else { self.style }; buf.set_string(area.left(), area.top(), &#x26;self.label, style); }} ``` Ratatui also has a `StatefulWidget`. This is essentially a widget that can “remember” information between two draw calls. This is essential when you have interactive UI components, like lists, where you might need to remember which item was selected or how much the user has scrolled. Here’s a breakdown of the trait: ``` pub trait StatefulWidget { type State; fn render(self, area: Rect, buf: &#x26;mut Buffer, state: &#x26;mut Self::State);} ``` `type State`: This represents the type of the state that this widget will use to remember details between draw calls.

- `fn render(...)`: This method is responsible for drawing the widget on the terminal. Notably, it also receives a mutable reference to the state, allowing you to read from and modify the state as needed.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/widgets/custom.md)

 [Previous Paragraph](/recipes/widgets/paragraph/) [Next Testing Apps](/recipes/testing/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
