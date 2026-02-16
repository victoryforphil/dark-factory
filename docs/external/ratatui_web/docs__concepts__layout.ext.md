----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /concepts/layout
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, layout
- Summary: The coordinate system in Ratatui runs left to right, top to bottom, with the origin `(0, 0)` in the top left corner of the terminal. The x and y coordinates are represented by u16 values and are generally listed in that order in most pla...
----

Source: https://ratatui.rs/concepts/layout

The coordinate system in Ratatui runs left to right, top to bottom, with the origin `(0, 0)` in the top left corner of the terminal. The x and y coordinates are represented by u16 values and are generally listed in that order in most places.

Layouts and widgets form the basis of the UI in Ratatui. Layouts dictate the structure of the interface, dividing the screen into various sections using constraints, while widgets fill these sections with content.

When rendering widgets to the screen, you first need to define the area where the widget will be displayed. This area is represented by a rectangle with a specific height and width in the buffer. You can specify this rectangle as an absolute position and size, or you can use the [`Layout`](https://docs.rs/ratatui/latest/ratatui/layout/struct.Layout.html) struct to divide the terminal window dynamically based on constraints such as `Length`, `Min`, `Max`, `Ratio`, `Percentage`.

The following example renders “Hello world!” 10 times, by manually calculating the areas to render within.

`for i in 0..10 {    let area = Rect::new(0, i, frame.area().width, 1);    frame.render_widget(Paragraph::new("Hello world!"), area);}`

The Layout struct
-----------------

[Section titled “The Layout struct”](http://ratatui.rs/concepts/layout#the-layout-struct)

A simple example of using the layout struct might look like this:

`use ratatui::prelude::*;let layout = Layout::default()    .direction(Direction::Vertical)    .constraints(vec![        Constraint::Percentage(50),        Constraint::Percentage(50),    ])    .split(frame.area());`

In this example, we have indicated that we want to split the available space vertically into two equal parts, allocating 50% of the screen height to each. The [`Layout::split`](https://docs.rs/ratatui/latest/ratatui/layout/struct.Layout.html#method.split) function takes the total size of the terminal window as an argument, returned by the [`Frame::area()`] method, and then calculates the appropriate size and placement for each rectangle based on the specified constraints.

Once you have defined your layout (or a set of nested layouts), you can use one of the rectangle areas derived from such layout to render your widget. This can be achieved by calling either the [`Frame::render_widget`](https://docs.rs/ratatui/latest/ratatui/terminal/struct.Frame.html#method.render_widget) or [`frame::render_stateful_widget`](https://docs.rs/ratatui/latest/ratatui/terminal/struct.Frame.html#method.render_stateful_widget) methods:

`frame.render_widget(    Paragraph::new("Top")        .block(Block::new().borders(Borders::ALL)),    layout[0]);frame.render_widget(    Paragraph::new("Bottom")        .block(Block::new().borders(Borders::ALL)),    layout[1]);`

This might look something like:

In this example, two `Paragraph` widgets are generated, named “Top” and “Bottom.” These widgets are then rendered in the first and second areas (`layout[0]` and `layout[1]`) of the split buffer, respectively. It’s important to note that layouts return an indexed list of rectangles, defined by their respective constraints. In this case, `layout[0]` refers to the top half of the screen, and `layout[1]` refers to the bottom half.

One of the important concepts to understand is that layouts can be nested. This means you can define another Layout within a rectangle of an outer layout. This nested layouts allow complex and flexible UI designs to be built while still maintaining control over how your grid of widgets resize with the terminal window.

Here’s how you might use nested layouts:

`let outer_layout = Layout::default()    .direction(Direction::Vertical)    .constraints(vec![        Constraint::Percentage(50),        Constraint::Percentage(50),    ])    .split(f.area());let inner_layout = Layout::default()    .direction(Direction::Horizontal)    .constraints(vec![        Constraint::Percentage(25),        Constraint::Percentage(75),    ])    .split(outer_layout[1]);`

In this situation, the terminal window is initially split vertically into two equal parts by `outer_layout`. Then, `inner_layout` splits the second rectangle of `outer_layout` horizontally, creating two areas that are 25% and 75% of the width of the original rectangle, respectively.

Rendering some Paragraphs of text into the above layouts produces the following:

`frame.render_widget(    Paragraph::new("outer 0")        .block(Block::new().borders(Borders::ALL)),    outer_layout[0]);frame.render_widget(    Paragraph::new("inner 0")        .block(Block::new().borders(Borders::ALL)),    inner_layout[0]);frame.render_widget(    Paragraph::new("inner 1")        .block(Block::new().borders(Borders::ALL)),    inner_layout[1]);`

This enables you to divide the terminal window into multiple sections of varying sizes, giving you the flexibility to create complex and adaptive graphical interfaces.

[`Constraint`](https://docs.rs/ratatui/latest/ratatui/layout/enum.Constraint.html)s dictate the size and arrangement of components within layouts. The Ratatui framework provides several constraint types for fine-tuning your user interface’s layout:

*   `Constraint::Length(u16)`: This constraint specifies a specific number of rows or columns that a rectangle should take up. Note that this is determined by absolute size and is not responsive to the overall terminal window size.

*   `Constraint::Percentage(u16)`: This constraint offers a size relative to the size of the parent layout or the terminal window itself. For instance, `Constraint::Percentage(50)` signifies that a rectangle should take up half of its parent’s size.

*   `Constraint::Ratio(u16, u16)`: Utilizing ratios offers an even finer granularity for splitting your layout. For instance, `Constraint::Ratio(1, 3)` will allocate 1/3rd of the parent’s size to this constraint.

*   `Constraint::Min(u16)`: Immerses a minimum limit to the size of a component. If a `Min` constraint is ensured with a `Percentage` or `Ratio`, the component will never shrink below the specified minimum size.

*   `Constraint::Max(u16)`: Limits the maximum size of a component. Similar to `Min`, if mixed with `Percentage` or `Ratio`, the component will never exceed the specified maximum size.

*   `Constraint::Fill(u16)`: Applies the scaling factor proportional to all other `Constraint::Fill` elements to fill excess space.

The element will only expand or fill into excess available space, proportionally matching other `Constraint::Fill` elements while satisfying all other constraints.

Constraints can be mixed and matched within a layout to create dynamic and adjustable interfaces. These constraints can be used when defining the layout for an application:

`let layout = Layout::default()    .direction(Direction::Horizontal)    .constraints([        Constraint::Length(10),        Constraint::Percentage(70),        Constraint::Min(5),    ]    .into_iter())    .split(frame.area());`

In this example, the initial `Length` constraint cause the first rectangle to have a width of 10 characters. The next rectangle will be 70% of the **total width**. The final rectangle will take up the remaining space, but will never be smaller than 5 characters.

Note that the order in which you specify your constraints is the order in which they will apply to the screen space.

By default, the split method allocates any remaining space in the area to the last area of the layout. To avoid this, add an unused `Min(0)` constraint as the last constraint.

Ratatui uses a constraint solver algorithm called [Cassowary](https://crates.io/crates/cassowary) in order to determine the right size for the rects. In some cases, not every constraint will be possible to achieve, and the solver can return an arbitrary solution that is close to fulfilling the constraints. The specific result is non-deterministic when this occurs.

The [`Flex`](https://docs.rs/ratatui/latest/ratatui/layout/enum.Flex.html) system determines how elements are positioned when the constraints don’t perfectly fill the available area.

### Alignment Strategies

[Section titled “Alignment Strategies”](http://ratatui.rs/concepts/layout#alignment-strategies)

| Flex Variant | Behavior |
| --- | --- |
| `Flex::Legacy` | Fills the available space within the container, putting excess space into the last element. |
| `Flex::Start` | Aligns items to the start of the container. |
| `Flex::End` | Aligns items to the end of the container. |
| `Flex::Center` | Centers items within the container. |
| `Flex::SpaceBetween` | Adds excess space between each element. |
| `Flex::SpaceAround` | Adds excess space around each element. |

Here is how two `Length(20)` elements are positioned across the same 80-cell area using different `Flex` strategies:

#### Flex::SpaceAround

[Section titled “Flex::SpaceAround”](http://ratatui.rs/concepts/layout#flexspacearound)

#### Flex::SpaceBetween

[Section titled “Flex::SpaceBetween”](http://ratatui.rs/concepts/layout#flexspacebetween)

Other Layout approaches
-----------------------

[Section titled “Other Layout approaches”](http://ratatui.rs/concepts/layout#other-layout-approaches)

There are a few PoCs of using [Taffy](https://crates.io/crates/taffy) for creating layouts that use flexbox / grid algorithms (similar to CSS) to layout rects. This can work nicely, but is not built in to Ratatui (yet). See [taffy in ratatui](https://github.com/search?q=repo%3Aratatui%2Fratatui%20taffy&type=pullrequests) for more details.

*   [FAQ: How do I avoid panics due to out of range calls on the Buffer?](https://ratatui.rs/faq#how-do-i-avoid-panics-due-to-out-of-range-calls-on-the-buffer)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
