----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /concepts/widgets
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, widgets
- Summary: Widgets are the building blocks of user interfaces in Ratatui. They are used to create and manage the layout and style of the terminal interface. Widgets can be combined and nested to create complex UIs, and can be easily customized to s...
----

Source: https://ratatui.rs/concepts/widgets

Widgets are the building blocks of user interfaces in Ratatui. They are used to create and manage the layout and style of the terminal interface. Widgets can be combined and nested to create complex UIs, and can be easily customized to suit the needs of your application.

Ratatui provides a wide variety of built-in widgets that can be used to quickly create UIs. These widgets include:

*   [`Block`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html)|[`Example`](https://ratatui.rs/examples/widgets/block): A basic Widget that draws a block with optional borders, titles, and styles.
*   [`BarChart`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html)|[`Example`](https://ratatui.rs/examples/widgets/barchart): Displays multiple datasets as bars with optional grouping.
*   [`Calendar`](https://docs.rs/ratatui/latest/ratatui/widgets/calendar/struct.Monthly.html)|[`Example`](https://ratatui.rs/examples/widgets/calendar): Displays a single month.
*   [`Canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/struct.Canvas.html)|[`Example`](https://ratatui.rs/examples/widgets/canvas): Draws arbitrary shapes using drawing characters.
*   [`Chart`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Chart.html)|[`Example`](https://ratatui.rs/examples/widgets/chart): Displays multiple datasets as a lines or scatter graph.
*   [`Clear`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html): Clears the area it occupies. Useful to render over previously drawn widgets.
*   [`Gauge`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Gauge.html)|[`Example`](https://ratatui.rs/examples/widgets/gauge): Displays progress percentage using block characters.
*   [`LineGauge`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.LineGauge.html): Display progress as a line.
*   [`List`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html)|[`Example`](https://ratatui.rs/examples/widgets/list): Displays a list of items and allows selection.
*   [`Paragraph`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html)|[`Example`](https://ratatui.rs/examples/widgets/paragraph): Displays a paragraph of optionally styled and wrapped text.
*   [`Scrollbar`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Scrollbar.html)|[`Example`](https://ratatui.rs/examples/widgets/scrollbar): Displays a scrollbar.
*   [`Sparkline`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Sparkline.html)|[`Example`](https://ratatui.rs/examples/widgets/sparkline): Display a single data set as a sparkline.
*   [`Table`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html)|[`Example`](https://ratatui.rs/examples/widgets/table): Displays multiple rows and columns in a grid and allows selection.
*   [`Tabs`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Tabs.html)|[`Example`](https://ratatui.rs/examples/widgets/tabs): Displays a tab bar and allows selection.

Additionally, [`String`](https://doc.rust-lang.org/std/string/struct.String.html), [`&str`](https://doc.rust-lang.org/std/primitive.str.html), [`Span`](https://docs.rs/ratatui/latest/ratatui/text/struct.Span.html), [`Line`](https://docs.rs/ratatui/latest/ratatui/text/struct.Line.html), and [`Text`](https://docs.rs/ratatui/latest/ratatui/text/struct.Text.html) can be used as widgets (though it’s common to use `Paragraph` instead of these directly).

For more information on these widgets, you can view the [Widgets API docs](https://docs.rs/ratatui/latest/ratatui/widgets/index.html) and the [Widget showcase](https://ratatui.rs/showcase/widgets/). Additionally, there are several third-party widgets available that can be used with Ratatui, which can be found on the [third-party widgets showcase](https://ratatui.rs/showcase/third-party-widgets/) and in the [Awesome Ratatui](https://github.com/ratatui/awesome-ratatui) repository.

In Ratatui, widgets are implemented as Rust traits, which allow for easy implementation and extension. The two main traits for widgets are [`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) and [`StatefulWidget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html), which provide the basic functionality for rendering and managing the state of a Widget.

Additionally, the [`WidgetRef`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.WidgetRef.html) and [`StatefulWidgetRef`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidgetRef.html) traits allow for rendering widgets by reference, which can be useful for storing and rendering collections of widgets. The latter two traits were added in Ratatui 0.26 and are at the time of writing, gated by an unstable feature flag, so there may be limited third party use of these traits. All the internal widgets have been updated to implement the ref traits and there is also a blanket implementation of `Widget for &T where T: WidgetRef`.

The [`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) trait is the most basic trait for widgets in Ratatui. It provides the basic functionality for rendering a Widget onto a buffer.

`pub trait Widget {    fn render(self, area: Rect, buf: &mut Buffer);}`

The [`StatefulWidget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html) trait is similar to the [`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) trait, but also includes a state that can be managed and updated during rendering.

`pub trait StatefulWidget {    type State;    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);}`

### WidgetRef and StatefulWidgetRef

[Section titled “WidgetRef and StatefulWidgetRef”](http://ratatui.rs/concepts/widgets#widgetref-and-statefulwidgetref)

The [`WidgetRef`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.WidgetRef.html) trait allows for rendering a Widget by reference instead of consuming the widget, which can be useful for storing and rendering individual or collections of widgets.

The [`StatefulWidgetRef`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidgetRef.html) trait is similar to the [`WidgetRef`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.WidgetRef.html) trait, but also includes a state that can be managed and updated during rendering.

These two traits were introduced in Ratatui 0.26.0 to help avoid a shortcoming that meant that widgets were always consumed on rendering while not breaking all code that has previously been built with that assumption. These two widgets are currently marked as unstable and gated behind the `unstable-widget-ref` feature flag.

`pub trait WidgetRef {    fn render_ref(&self, area: Rect, buf: &mut Buffer);}pub trait StatefulWidgetRef {    type State;    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State);}`

In Ratatui, widgets are used to create and manage the layout and style of the terminal interface. widgets can be combined and nested to create complex UIs, and can be easily customized to suit the needs of your application.

To use widgets in your application, you will typically use the [`Frame`](https://docs.rs/ratatui/latest/ratatui/struct.Frame.html) type, which has two methods for rendering widgets: [`render_widget`](https://docs.rs/ratatui/latest/ratatui/struct.Frame.html#method.render_widget) and [`render_stateful_widget`](https://docs.rs/ratatui/latest/ratatui/struct.Frame.html#method.render_stateful_widget) (and the corresponding `_ref` methods). These methods are the entry points for an application to draw widgets and are usually called from the closure that is passed to the [`Terminal::draw`] method.

Here’s an example of using the `render_widget` method to draw a Widget:

`terminal.draw(|frame| {    frame.render_widget(some_widget, frame.area());});`

And here’s an example of using the `render_stateful_widget` method to draw a StatefulWidget:

`terminal.draw(|frame| {    frame.render_stateful_widget(some_stateful_widget, frame.area(), &mut some_state);});`

These methods internally call the `render` function on the [`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) or [`StatefulWidget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html) trait, which will then call the `render` function on the specific [`WidgetRef`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.WidgetRef.html) or [`StatefulWidgetRef`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidgetRef.html) that you have implemented.

A common compositional pattern is to only have a single root widget (the `App` struct in the example below) that is passed to `Frame::render_widget()` and then within that and other widgets, you call the render methods directly passing in the area which you want to render the widgets to.

`#[derive(Default)]struct App {    // ...    should_quit: bool}fn main() {    let app = App::default();    // ...    while !app.should_quit {        terminal.draw(|frame| {            frame.render_widget(&app, frame.area())        })    }}impl Widget for &App {    fn render(self, area: Rect, buf: &mut Buffer) {        // ...        MyHeaderWidget::new("Header text")            .render(Rect::new(0, 0, area.width, 1), buf);    }}`

In Ratatui, widgets are implemented as Rust traits, which allow for easy implementation and extension. The two main traits for widgets are [`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) and [`StatefulWidget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html), which provide the basic functionality for rendering and managing the state of a Widget.

Here’s an example of implementing the [`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) trait for a simple greeting Widget:

`struct GreetingWidget {    name: String,}impl Widget for GreetingWidget {    fn render(self, area: Rect, buf: &mut Buffer) {        let greeting = format!("Hello, {}!", self.name);        buf.set_string(area.x, area.y, greeting, Style::default());    }}`

In this example, the `GreetingWidget` struct has a single field, `name`, which is a `String`. The `render` function takes a `Rect` and a mutable reference to a `Buffer`, and sets the string at the specified coordinates in the buffer.

Widgets are not restricted to just calling methods on `Buffer`. They can also create and render other widgets within their `render` method. For example, instead of directly calling methods on `buf`, a Widget can create a `Line` widget with a vector of `Span`s, where the `Span` for the name is styled. This `Line` widget can then be rendered within the Widget’s `render` method.

Here’s the same greeting example using nested widgets:

`struct GreetingWidget {    name: String,}impl Widget for GreetingWidget {    fn render(self, area: Rect, buf: &mut Buffer) {        let hello = Span::raw("Hello, ");        let name = Span::styled(self.name, Modifier::BOLD);        let line = Line::from(vec![hello, name]);        line.render(area, buf);    }}`

This approach allows for composing and reusing widgets. For example, the `Line` widget can be used in other widgets or even in other parts of the application. Additionally, it allows for easier testing and maintenance of widgets as the code related to rendering is organized in a consistent place (the `impl Widget` blocks).

In some situations, a widget might need to be able to mutate some extra state while rendering itself. An example of this is how the built-in `List` widget works. During rendering the `List` updates the scroll position in the state to ensure that the selected item is visible in the rendering area.

Here’s an example of implementing the `StatefulWidget` trait for a frame count Widget:

`struct FrameCountWidget {    style: Style,}impl StatefulWidget for FrameCountWidget {    type State = i32;    fn render(self, area: Rect, buf: &mut Buffer, state: &mut i32) {        *state += 1;        let text = format!("Frame count: {state}");        Line::styled(text, self.style).render(area, buf);    }}`

In this example, the `FrameCount` widget increments the state every time it is rendered, counting the number of frames.

In Ratatui 0.26.0, we added the [`WidgetRef`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.WidgetRef.html) (and similarly [`StatefulWidgetRef`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidgetRef.html)) traits. These allow widgets to be created that are rendering by reference, which can be useful for storing a widget which can be rendered multiple times instead of constructing it on every frame. This also makes it easy to create dynamic collections of widgets (e.g. Panes in a layout) by boxing the widgets using `Box<dyn T>`.

Implementing `WidgetRef` / `StatefulWidgetRef` is similar to implementing the consuming versions of the traits (with the difference being that the method name is `render_ref` and self is a reference).

`struct GreetingWidget {    name: String,}impl WidgetRef for GreetingWidget {    fn render_ref(&self, area: Rect, buf: &mut Buffer) {        let hello = Span::raw("Hello, ");        let name = Span::styled(self.name, Modifier::BOLD);        let line = Line::from(vec![hello, name]);        line.render(area, buf);    }}`

This can be useful as a way to store the widgets between frames. E.g.:

`struct App {    greeting: GreetingWidget,}// and then later:frame.render_widget_ref(&app.greeting, area);`

In situations where there is a collection of widgets of different types, or a single widget that doesn’t have a type known at compile time, the ref traits make it possible to store widgets as trait objects by using [`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html).

E.g. the following code shows how you might have two different widgets stored in a `Vec` that are later rendered without knowing the type of each widget at runtime.

`struct Greeting { ... }struct Farewell { ... }impl WidgetRef for Greeting { ... }impl WidgetRef for Farewell { ... }let widgets: Vec<Box<dyn WidgetRef>> = vec![    Box::new(Greeting { name: "alice".into() }),    Box::new(Farewell { name: "bob".into() })];for widget in widgets {    widget.render_ref(area, buf);}`

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
