----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /highlights/v029
- Keywords: ratatui, rust, tui, terminal ui, docs, highlights, v029
- Summary: We are excited to announce Ratatui
----

Source: https://ratatui.rs/highlights/v029

# v0.29.0

We are excited to announce Ratatui
[0.29.0](https://github.com/ratatui/ratatui/releases/tag/v0.29.0)! See the breaking changes for this
release [here](https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md).

Big shoutout to [@dekirsu](https://github.com/dekirisu) for the kickass animation above! We will
start improving our website soon!

## Sparkline: Empty bar style 📊

[Section titled “Sparkline: Empty bar style 📊”](#sparkline-empty-bar-style)

You can now distinguish between empty bars and bars with a value of 0 in the `Sparkline` widget.

Before:

After:

To achieve this, we added the `absent_value_style` and `absent_value_symbol` functions to the
`Sparkline` widget.

- ``` let widget = Sparkline::default() .absent_value_style(Style::default().fg(Color::Red)) // new! .absent_value_symbol(symbols::shade::FULL) // new! .data([ None, // absent, will be rendered as a red full block Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), ]);let buffer = render(widget, 12);let mut expected = Buffer::with_lines(["█▁▂▃▄▅▆▇█xxx"]);expected.set_style(Rect::new(0, 0, 1, 1), Style::default().fg(Color::Red));assert_eq!(buffer, expected); ``` Caution`Sparkline::data` takes `IntoIterator&#x3C;Item = SparklineBar>` instead of `&#x26;[u64]` and is no longer `const`. ## Overlapping layouts 🔄 [Section titled “Overlapping layouts 🔄”](#overlapping-layouts) TLDRYou can use `Layout::spacing(-1)` to create layouts with overlapping segments. `Layout::spacing` is now generic and can take: Zero or positive numbers, e.g. `Layout::spacing(1)` (current functionality)

- Negative number, e.g. `Layout::spacing(-1)` (new!)

- Variant of the `Spacing` (new!) `Spacing::Space`

- `Spacing::Overlap`

This allows creating layouts with a shared pixel for segments. When `spacing(negative_value)` is
used, spacing is ignored and all segments will be adjacent and have pixels overlapping.

Here is a snippet from the [implementation](https://github.com/ratatui/ratatui/pull/1398):

```
let (segments, spacers) = Layout::horizontal([Length(10), Length(10), Length(10)])    .flex(Flex::Center)    .spacing(-1) // new feature    .split_with_spacers(lower);
for segment in segments.iter() {    frame.render_widget(        crate::widgets::Block::bordered()            .border_set(crate::symbols::border::DOUBLE),        *segment,    );}for spacer in spacers.iter() {    frame.render_widget(crate::widgets::Block::bordered(), *spacer);}
```

You can see that drawing a border on top of an existing border overwrites it:

```
┌─────────┐╔════════╔════════╔════════╗┌─────────┐└─────────┘╚════════╚════════╚════════╝└─────────┘
```

Future versions will enhance border drawing by combining borders to handle overlaps better.

## Table: Support selecting columns and cells 🗃️

[Section titled “Table: Support selecting columns and cells 🗃️”](#table-support-selecting-columns-and-cells-️)

You can now select columns and cells in a `Table` widget!

  Your browser does not support the video tag.

To select a column or cell, use the `TableState` methods `select_column` and `select_cell`. We also
added `scroll_right_by` and `scroll_left_by` along with other convenience methods.

```
let mut state = TableState::new().with_selected_column(Some(1));state.select_first_column();state.select_next_column();state.select_previous_column();state.select_last_column();state.scroll_right_by(4);state.scroll_left_by(20);state.select_column(Some(1));state.select_cell(Some((1, 5)));
```

The selected column and cell styles can be set using `Table::column_highlight_style` and
`Table::cell_highlight_style`.

For example:

```
let table = Table::new(rows, [Constraint::Length(5); 3])    .highlight_symbol(">>")    .row_highlight_style(Style::new().red())    .column_highlight_style(Style::new().blue());
```

Caution

- The serialized output of the state will now include the `selected_column` field.

- The `Table::highlight_style` is now deprecated in favor of `Table::row_highlight_style`.

## Tabs: Support deselection 🚫

[Section titled “Tabs: Support deselection 🚫”](#tabs-support-deselection)

`Tabs::select()` now accepts `Into&#x3C;Option&#x3C;usize>>` instead of `usize`. This allows tabs to be
deselected by passing `None`.

```
let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).select(None);
```

However, this breaks any code already using parameter type inference:

```
let selected = 1u8; let tabs = Tabs::new(["A", "B"]).select(selected.into()) let tabs = Tabs::new(["A", "B"]).select(selected as usize)
```

## Terminal: Support scrolling regions 🖥️

[Section titled “Terminal: Support scrolling regions 🖥️”](#terminal-support-scrolling-regions-️)

The current implementation of `Terminal::insert_before` used to cause the viewport to flicker as
described [in this issue](https://github.com/ratatui/ratatui/issues/584).

We introduced a new crate feature called `scrolling-regions` to address this issue. This feature
uses terminal scrolling regions to implement `Terminal::insert_before` without flickering.

Note

Terminal scrolling regions (sometimes called “scroll regions”) allow a terminal to have its
scrolling region set to something other than the whole screen. When a scroll ANSI sequence is sent
to the terminal and it has a non-default scrolling region, the terminal will scroll just inside of
that region.

When the viewport takes up the entire screen, we create a scrolling region of just the top line
(could be more) of the viewport, then use that to draw the lines we want to output. When we’re done,
we scroll it up by one line, into the scrollback history, and then redraw the top line from the
viewport.

For achieving that, we added two new `Backend` methods: `scroll_region_up` and `scroll_region_down`.
These methods are implemented on all backends in the codebase.

To enable this feature for your `Viewport`, update your `Cargo.toml` as follows:

```
[dependencies]ratatui = { version = "0.29", features = ["scrolling-regions"] }
```

See the [implementation](https://github.com/ratatui/ratatui/pull/1341) for more details.

## Color: HSLuv support 🎨

[Section titled “Color: HSLuv support 🎨”](#color-hsluv-support)

After enabling the `palette` feature, you can now use the `Hsluv` struct to create colors in the
[HSLuv color space](https://www.hsluv.org/):

```
use ratatui::{palette::Hsluv, style::Color};
let color: Color = Color::from_hsluv(Hsluv::new(0.0, 100.0, 0.0));assert_eq!(color, Color::Rgb(0, 0, 0));
```

Note

Also, the `Color::from_hsl` method now accepts a `palette::Hsl` value instead of individual
components:

```
Color::from_hsl(360.0, 100.0, 100.0)Color::from_hsl(Hsl::new(360.0, 100.0, 100.0))
```

This means that you need to enable the `palette` feature in your `Cargo.toml`:

```
[dependencies]ratatui = { version = "0.29", features = ["palette"] }
```

## Canvas: draw example 🎨

[Section titled “Canvas: draw example 🎨”](#canvas-draw-example)

We extended the `Canvas` example to include a drawing feature. You can now draw on the canvas using
your mouse:

## Ratatui logo widget 🖼️

[Section titled “Ratatui logo widget 🖼️”](#ratatui-logo-widget-️)

We added a new widget called `RatatuiLogo` that can be used to render the Ratatui logo in the
terminal.

```
use ratatui::{Frame, widgets::RatatuiLogo};
fn draw(frame: &#x26;mut Frame) {    frame.render_widget(RatatuiLogo::tiny(), frame.area());  // 2x15 characters    frame.render_widget(RatatuiLogo::small(), frame.area()); // 2x27 characters}
```

Results in:

```
▛▚▗▀▖▜▘▞▚▝▛▐ ▌▌▛▚▐▀▌▐ ▛▜ ▌▝▄▘▌
█▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ ██▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █
```

You can also run the example using:

Terminal window

```
cargo run --example ratatui-logo
```

## Line: Implement `From&#x3C;Cow&#x3C;str>>` 📜

[Section titled “Line: Implement From&#x3C;Cow&#x3C;str>> 📜”](#line-implement-fromcowstr)

`Line` now implements `From&#x3C;Cow&#x3C;str>>` to allow for more flexible conversions.

```
let cow_str: Cow&#x3C;'static, str> = Cow::Borrowed("hello, world");let line = Line::from(cow_str);
```

As this adds an extra conversion, ambiguous inferred values may no longer compile. In that case, use
`Line::from(String::from(...))` instead.

## `Rect::area` now returns `u32` 📏

[Section titled “Rect::area now returns u32 📏”](#rectarea-now-returns-u32)

The `Rect::area()` function now returns a `u32` instead of a `u16` to allow for larger areas to be
calculated.

Previously, `Rect::new()` would clamp the rectangle’s total area to `u16::MAX`, maintaining its
aspect ratio. Now, it clamps the width and height separately to stay within `u16::MAX`.

## Deprecate `block::Title` ⚠️

[Section titled “Deprecate block::Title ⚠️”](#deprecate-blocktitle-️)

`ratatui::widgets::block::Title` is deprecated in favor of using `Line` to represent titles.

This removes an unnecessary layer of wrapping (string -> Span -> Line -> Title).

To update your code:

```
Block::new().title(Title::from("foo"));// becomes any ofBlock::new().title("foo");Block::new().title(Line::from("foo"));
Block::new().title(Title::from("foo").position(Position::TOP));// becomes any ofBlock::new().title_top("foo");Block::new().title_top(Line::from("foo"));
Block::new().title(Title::from("foo").position(Position::BOTTOM));// becomes any ofBlock::new().title_bottom("foo");Block::new().title_bottom(Line::from("foo"));
```

The `Title` struct will be removed in a future release of Ratatui (likely 0.31).

For more information see [this issue](https://github.com/ratatui/ratatui/issues/738).

## Better `Debug` output 🐞

[Section titled “Better Debug output 🐞”](#better-debug-output)

The Debug output for `Text`, `Line`, `Span`, and `Style` has been improved to be more concise and
easier to read.

For example, given this code:

```
Text::styled("Hello, world!", Color::Yellow).centered(),
```

The Debug output (`{:?}`) will now look like this:

Text::from(Line::from(“Hello, world!“)).yellow().centered()

## `DoubleEndedIterator` for `Columns` and `Rows` 🔄

[Section titled “DoubleEndedIterator for Columns and Rows 🔄”](#doubleendediterator-for-columns-and-rows)

You can now iterate over the columns and rows in a layout in reverse order!

```
let rect = Rect::new(0, 0, 3, 2);let mut columns = Columns::new(rect);
assert_eq!(columns.next_back(), Some(Rect::new(2, 0, 1, 2)));assert_eq!(columns.next_back(), Some(Rect::new(1, 0, 1, 2)));assert_eq!(columns.next_back(), Some(Rect::new(0, 0, 1, 2)));assert_eq!(columns.next_back(), None);assert_eq!(columns.next(), None);
```

Caution

We also removed the public fields from the `Columns`,`Rows`, and `Positions` iterators since they
were not intended to be public and should not have been accessed directly.

## Pin `unicode-width` 📌

[Section titled “Pin unicode-width 📌”](#pin-unicode-width)

We use the [`unicode-width`](https://crates.io/crates/unicode-width) crate to calculate the width of
characters. There was
[a controversial change](https://github.com/unicode-rs/unicode-width/issues/66) in `0.1.14` which
resulted in `0.1.13` being published as `0.2.0`. This also broke our tests:

```
assert_eq!("👩".width(), 2); // Womanassert_eq!("🔬".width(), 2); // Microscopeassert_eq!("👩‍🔬".width(), 4); // Woman scientist -> should be 4 but it expect 2
```

We decided to comply with these changes by pinning at `0.2.0` to avoid breaking applications when
there are breaking changes in the library.

See the discussion in [#1271](https://github.com/ratatui/ratatui/pull/1271)

## Check in Cargo.lock ✔️

[Section titled “Check in Cargo.lock ✔️”](#check-in-cargolock-️)

We added `Cargo.lock` to the repository due to the benefits it provides:

When kept up to date, this makes it possible to build any git version with the same versions of
crates that were used for any version, without it, you can only use the current versions. This
makes bugs in semver compatible code difficult to detect.

See:

- [https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control](https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control)

- [https://blog.rust-lang.org/2023/08/29/committing-lockfiles.html](https://blog.rust-lang.org/2023/08/29/committing-lockfiles.html)

## Other 💼

[Section titled “Other 💼”](#other)

- Remove unused dependencies detected with cargo-machete ([#1362](https://github.com/ratatui/ratatui/pull/1362))

- Remove the usage of prelude in doc examples ([#1390](https://github.com/ratatui/ratatui/pull/1390))

- Add benchmark for `Table` ([#1408](https://github.com/ratatui/ratatui/pull/1408))

- Implement size hints for `Rect` iterators ([#1420](https://github.com/ratatui/ratatui/pull/1420))

- Update README.md ([#1431](https://github.com/ratatui/ratatui/pull/1431) &#x26; [#1419](https://github.com/ratatui/ratatui/pull/1419))

- Fix viewport resizing and clearing ([#1353](https://github.com/ratatui/ratatui/pull/1353) &#x26; [#1427](https://github.com/ratatui/ratatui/pull/1427))

“Food will come, Remy. Food always comes to those who love to cook.” – Gusteau

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/highlights/v0.29.md)

 [Previous v0.30.0](/highlights/v030/) [Next v0.28.0](/highlights/v028/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
