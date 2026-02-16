----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /highlights/v024
- Keywords: ratatui, rust, tui, terminal ui, docs, highlights, v024
- Summary: [https://github.com/ratatui/ratatui/releases/tag/v0.24.0](https://github.com/ratatui/ratatui/releases/tag/v0.24.0)
----

Source: https://ratatui.rs/highlights/v024

# v0.24.0

[https://github.com/ratatui/ratatui/releases/tag/v0.24.0](https://github.com/ratatui/ratatui/releases/tag/v0.24.0)

⚠️ We created a [breaking changes](https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md)
document for easily going through the breaking changes in each version.

## Ratatui Website/Book 📚

[Section titled “Ratatui Website/Book 📚”](#ratatui-websitebook)

The site you are browsing right now (`ratatui.rs`) is the new homepage of `ratatui`! For now, we
host the book here which contains a bunch of useful tutorials, concepts and FAQ sections and there
is a [plan](https://github.com/ratatui/ratatui-website/issues/115) to create a landing page
[pretty soon](https://github.com/ratatui/ratatui-website/pull/116)!

All the code is available at [https://github.com/ratatui/ratatui-website](https://github.com/ratatui/ratatui-website)

## Frame: no more generics 🚫

[Section titled “Frame: no more generics 🚫”](#frame-no-more-generics)

If you were using the `Frame` type with a `Backend` parameter before:

- ``` fn draw&#x3C;B: Backend>(frame: &#x26;mut Frame&#x3C;B>) { // ...} ``` You no longer need to provide a generic over backend (`B`): ``` fn draw(frame: &#x26;mut Frame) { // ...} ``` ## New Demo / Examples ✨ [Section titled “New Demo / Examples ✨”](#new-demo--examples) We have a new kick-ass demo! To try it out: Terminal window ``` cargo run --example=demo2 --features="crossterm widget-calendar" ``` The code is available [here](https://github.com/ratatui/ratatui/tree/main/examples/demo2). We also have a new example demonstrating how to create a custom widget. Terminal window ``` cargo run --example=custom_widget --features=crossterm ``` The code is available [here](https://github.com/ratatui/ratatui/blob/main/examples/custom_widget.rs). Lastly, we added an example to demonstrate RGB color options: Terminal window ``` cargo run --example=colors_rgb --features=crossterm ``` The code is available [here](https://github.com/ratatui/ratatui/blob/main/examples/colors_rgb.rs). ## Window Size API 🪟 [Section titled “Window Size API 🪟”](#window-size-api) A new method called `window_size` is added for retrieving the window size. It returns a struct called `WindowSize` that contains both pixels (width, height) and columns/rows information. ``` let stdout = std::io::stdout();let mut backend = CrosstermBackend::new(stdout);println!("{:#?}", backend.window_size()?; ``` Outputs: ``` WindowSize { columns_rows: Size { width: 240, height: 30, }, pixels: Size { width: 0, height: 0, },} ``` With this new API, the goal is to improve image handling in terminal emulators by sharing terminal size and layout information, enabling precise image placement and resizing within rectangles. See the pull request for more information: [https://github.com/ratatui/ratatui/pull/276](https://github.com/ratatui/ratatui/pull/276) ## BarChart: Render smol charts 📊 [Section titled “BarChart: Render smol charts 📊”](#barchart-render-smol-charts) We had a bug where the `BarChart` widget doesn’t render labels that are full width. Now this is fixed and we are able to render charts smaller than 3 lines! For example, here is how `BarChart` is rendered and resized from a single line to 4 lines in order: If you set `bar_width(2)` for 3 lines, then it is rendered as: ## Block: custom symbols for borders 🛡️ [Section titled “Block: custom symbols for borders 🛡️”](#block-custom-symbols-for-borders-️) The `Block` widget has a new method called `border_set` that can be used to specify the symbols that are going to be used for the borders. ``` Block::default() .borders(Borders::ALL) .border_set(border::Set { top_left: "1", top_right: "2", bottom_left: "3", bottom_right: "4", vertical_left: "L", vertical_right: "R", horizontal_top: "T", horizontal_bottom: "B", }) ``` When rendered: There are also 2 new border types added (`QuadrantInside`, `QuadrantOutside`). See the available border types at [https://docs.rs/ratatui/latest/ratatui/widgets/block/enum.BorderType.html](https://docs.rs/ratatui/latest/ratatui/widgets/block/enum.BorderType.html) Also, there are breaking changes to note here: `BorderType::to_line_set` is renamed to `to_border_set`

- `BorderType::line_symbols` is renamed to `border_symbols`

## Canvas: half block marker 🖼️

[Section titled “Canvas: half block marker 🖼️”](#canvas-half-block-marker-️)

A new marker named `HalfBlock` is added to `Canvas` widget along with the associated
`HalfBlockGrid`.

The idea is to use half blocks to draw a grid of “pixels” on the screen. Because we can set two
colors per cell, and because terminal cells are about twice as tall as they are wide, we can draw a
grid of half blocks that looks like a grid of square pixels.

```
Canvas::default()    .marker(Marker::HalfBlock)    .x_bounds([0.0, 10.0])    .y_bounds([0.0, 10.0])    .paint(|context| {        context.draw(&#x26;Rectangle {            x: 0.0,            y: 0.0,            width: 10.0,            height: 10.0,            color: Color::Red,        });    });
```

Rendered as:

```
█▀▀▀▀▀▀▀▀██        ██        ██        ██        ██        ██        ██        ██        ██▄▄▄▄▄▄▄▄█
```

## Line: raw constructor 📝

[Section titled “Line: raw constructor 📝”](#line-raw-constructor)

You can simply construct `Line` widgets from strings using `raw` (similar to `Span::raw` and
`Text::raw`):

```
let line = Line::raw("test content");
```

One thing to note here is that multi-line content is converted to multiple spans with the new lines
removed.

## Rect: is empty? 🛍️

[Section titled “Rect: is empty? 🛍️”](#rect-is-empty-️)

With the newly added `is_empty` method, you can check if a `Rect` has any area or not:

```
assert!(!Rect::new(1, 2, 3, 4).is_empty());assert!(Rect::new(1, 2, 0, 4).is_empty());assert!(Rect::new(1, 2, 3, 0).is_empty());
```

## Layout: LRU cache 📚

[Section titled “Layout: LRU cache 📚”](#layout-lru-cache)

The layout cache now uses a [`LruCache`](https://docs.rs/lru/latest/lru/struct.LruCache.html) with
default size set to 16 entries. Previously the cache was backed by a `HashMap`, and was able to grow
without bounds as a new entry was added for every new combination of layout parameters.

We also added a new method called `init_cache` for changing the cache size if necessary:

```
Layout::init_cache(10);
```

This will only have an effect if it is called prior to any calls to `layout::split()`.

## Backend: optional underline colors 🎨

[Section titled “Backend: optional underline colors 🎨”](#backend-optional-underline-colors)

Windows 7 doesn’t support the underline color attribute, so we need to make it optional. For that,
we added a feature fla called `underline-color` and enabled it as default.

It can be disabled as follows for applications that supports Windows 7:

```
ratatui = { version = "0.24.0", default-features = false, features = ["crossterm"] }
```

## Stylized strings ✨

[Section titled “Stylized strings ✨”](#stylized-strings)

Although the [`Stylize`](https://docs.rs/ratatui/latest/ratatui/style/trait.Stylize.html) trait is
already implemented for `&#x26;str` which extends to `String`, it is not implemented for `String` itself.

So we added an implementation of `Stylize` for `String` that returns `Span&#x3C;'static>` which makes the
following code compile just fine instead of failing with a temporary value error:

```
let s = format!("hello {name}!", "world").red();
```

This may break some code that expects to call `Stylize` methods on `String` values and then use the
String value later. For example, following code will now fail to compile because the `String` is
consumed by `set_style` instead of a slice being created and consumed.

```
let s = String::from("hello world");let line = Line::from(vec![s.red(), s.green()]); // fails to compile
```

Simply clone the `String` to fix the compilation error:

```
let s = String::from("hello world");let line = Line::from(vec![s.clone().red(), s.green()]);
```

## Spans: RIP 💀

[Section titled “Spans: RIP 💀”](#spans-rip)

The `Spans` was deprecated and replaced with a more ergonomic `Line` type in 0.21.0 and now it is
removed.

Long live `Line`!

## Other 💼

[Section titled “Other 💼”](#other)

- Simplified the internal implementation of `BarChart` and add benchmarks

- Add documentation to various places including `Block`, `Gauge`, `Table`, `BarChart`, etc.

- Used modern modules syntax throughout the codebase (`xxx/mod.rs` -> `xxx.rs`)

- Added `buffer_mut` method to `Frame`

- Integrated `Dependabot` for dependency updates and bump dependencies

- Refactored examples

- Fixed arithmetic overflow edge cases

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/highlights/v0.24.md)

 [Previous v0.25.0](/highlights/v025/) [Next v0.23.0](/highlights/v023/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
