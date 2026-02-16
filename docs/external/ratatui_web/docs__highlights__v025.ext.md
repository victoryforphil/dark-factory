----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /highlights/v025
- Keywords: ratatui, rust, tui, terminal ui, docs, highlights, v025
- Summary: [https://github.com/ratatui/ratatui/releases/tag/v0.25.0](https://github.com/ratatui/ratatui/releases/tag/v0.25.0)
----

Source: https://ratatui.rs/highlights/v025

# v0.25.0

[https://github.com/ratatui/ratatui/releases/tag/v0.25.0](https://github.com/ratatui/ratatui/releases/tag/v0.25.0)

⚠️ See the [breaking changes](https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md) for
this release.

## Ratatui Website 📚

[Section titled “Ratatui Website 📚”](#ratatui-website)

We revamped our entire website ([ratatui.rs](https://ratatui.rs)) with [Astro](https://astro.build/)
and updated the documentation/tutorials.

Here are some of the sections that we recommend checking out:

- [Tutorials](https://ratatui.rs/tutorials/)

- [Concepts](https://ratatui.rs/concepts/)

- [How-to](https://ratatui.rs/recipes/)

- [Apps/Widgets Showcase](https://ratatui.rs/showcase/)

All the code is available at [https://github.com/ratatui/ratatui-website](https://github.com/ratatui/ratatui-website)

Check out the [good first issues here](https://github.com/ratatui/ratatui-website/issues) if you are
interested in contributing to our website!

## Awesome Ratatui 🦀

[Section titled “Awesome Ratatui 🦀”](#awesome-ratatui)

We created an awesome list for curating the awesome applications/libraries built with `ratatui`!

See: [https://github.com/ratatui/awesome-ratatui](https://github.com/ratatui/awesome-ratatui)

If you have something built with/for Ratatui, feel free to
[add it to this list](https://github.com/ratatui/awesome-ratatui/blob/main/contributing.md)!

## Ratatui Templates 📝

[Section titled “Ratatui Templates 📝”](#ratatui-templates)

We gathered the available `ratatui` templates under a single repository:
[https://github.com/ratatui/templates](https://github.com/ratatui/templates)

Currently, we have two templates:

- Simple: A template for bootstrapping a simple `ratatui` application quickly.

- Async: A more advanced template which uses crates such as `tokio` and `tracing` for creating an async `ratatui` application.

They can be used with [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) as
follows:

- Terminal window ``` cargo generate ratatui/ratatui-template ``` This repository is still under construction so feel free to take a look and help us out. Additionally, we really appreciate if you have another template and interested in contributing it. ## New/Updated Examples 🆕 [Section titled “New/Updated Examples 🆕”](#newupdated-examples) We added a new example ([`ratatui-logo`](https://github.com/ratatui/ratatui/blob/main/examples/ratatui-logo.rs)) which shows how to render a big text using half blocks. We also updated the [`colors_rgb`](https://github.com/ratatui/ratatui/blob/main/examples/colors_rgb.rs) example to add animation and a FPS counter. Your browser does not support the video tag. ## List: Better Construction ✨ [Section titled “List: Better Construction ✨”](#list-better-construction) `List::new` now accepts `IntoIterator&#x3C;Item = Into&#x3C;ListItem>>` which means you can build a `List` like following: ``` List::new(["Item 1", "Item 2"]) ``` However, this change will throw a compilation error for `IntoIterator`s with an indeterminate item (e.g. empty vectors): ``` let list = List::new(vec![]); // instead, this should be:let list = List::default(); ``` ## List: Line Alignment 📏 [Section titled “List: Line Alignment 📏”](#list-line-alignment) The `List` widget now respects the alignment of `Line`s and renders them as expected. For example: ``` let items = [ Line::from("Left").alignment(Alignment::Left), Line::from("Center").alignment(Alignment::Center), Line::from("Right").alignment(Alignment::Right),] ``` Will result in a list: ``` Left Center Right ``` ## List: `start_corner` -> `direction` [Section titled “List: start_corner -> direction”](#list-start_corner---direction) The previous name `start_corner` did not communicate clearly the intent of the method. A new method `List::direction` and a new enum `ListDirection` were added. `List::start_corner` is now deprecated. ## Layout: Better Construction ✨ [Section titled “Layout: Better Construction ✨”](#layout-better-construction) We added a convenience function to create a layout with a direction and a list of constraints which are the most common parameters that would be generally configured using the builder pattern. ``` let layout = Layout::new(Direction::Horizontal, [ Constraint::Percentage(50), Constraint::Percentage(50),]); ``` The constraints can be passed in as any iterator of constraints. ## Layout: Allow Configuring Fill 🖌️ [Section titled “Layout: Allow Configuring Fill 🖌️”](#layout-allow-configuring-fill-️) The layout split will generally fill the remaining area when `split()` is called. With this feature, we now allow the caller to configure how any extra space is allocated to the `Rect`s. This is useful for cases where the caller wants to have a fixed size for one of the `Rect`s, and have the other `Rect`s fill the remaining space. To use this feature, add `unstable` or `unstable-segment-size` feature flag to `Cargo.toml`. The reason for this is that the exact name for the types is still being bikeshedded. The default behavior is `SegmentSize::LastTakesRemainder`, which gives the last segment the remaining space. ``` Layout::new( Direction::Horizontal, [Constraint::Percentage(50), Constraint::Percentage(25)],).split(frame.size()); ``` `SegmentSize::None` will disable this behavior. ``` Layout::new( Direction::Horizontal, [Constraint::Percentage(50), Constraint::Percentage(25)],).segment_size(layout::SegmentSize::None).split(frame.size()); ``` To configure the layout to fill the remaining space evenly, use `Layout::segment_size(SegmentSize::EvenDistribution)`. See the documentation for `Layout::segment_size()` and `layout::SegmentSize` for more information. ## Table: Segment Size 🗃️ [Section titled “Table: Segment Size 🗃️”](#table-segment-size-️) Similarly to the previous entry, this works just like `Layout::segment_size` but for tables: ``` let widths = [Constraint::Min(10), Constraint::Min(10), Constraint::Min(10)];let table = Table::new([]) .widths(widths) .segment_size(SegmentSize::LastTakesRemainder); ``` It controls how to distribute extra space to an underconstrained table. The default, legacy behavior is to leave the extra space unused. The new options are `LastTakesRemainder` which gets all space to the rightmost column that can used it, and `EvenDistribution` which divides it amongst all columns. ## Table: Width Improvements ✨ [Section titled “Table: Width Improvements ✨”](#table-width-improvements) ### `Table::new()` now requires specifying the widths of the columns [Section titled “Table::new() now requires specifying the widths of the columns”](#tablenew-now-requires-specifying-the-widths-of-the-columns) Previously, `Table`s could be constructed without widths. In almost all cases this is an error so we made the `widths` parameter mandatory. Which means you need to update your existing code to: ``` Table::new(rows).widths(widths)Table::new(rows, widths) ``` For ease of automated replacement, in cases where the amount of code broken by this change is large or complex, it may be convenient to replace `Table::new` with `Table::default().rows`: ``` Table::new(rows).block(block).widths(widths);Table::default().rows(rows).block(block).widths(widths) ``` ### `Table::widths()` now accepts `AsRef&#x3C;[Constraint]>` [Section titled “Table::widths() now accepts AsRef&#x3C;[Constraint]>”](#tablewidths-now-accepts-asrefconstraint) This allows passing an array, slice or Vec of constraints, which is more ergonomic than requiring this to always be a slice. ``` Table::default().widths([Constraint::Length(5), Constraint::Length(5)]);Table::default().widths(&#x26;[Constraint::Length(5), Constraint::Length(5)]); // widths could also be computed at runtimelet widths = vec![Constraint::Length(5), Constraint::Length(5)];Table::default().widths(widths.clone());Table::default().widths(&#x26;widths); ``` ## Constraint Helpers 🆘 [Section titled “Constraint Helpers 🆘”](#constraint-helpers) We added helper methods that convert from iterators of `u16` values to the specific `Constraint` type. This makes it easy to create constraints like: ``` // a fixed layoutlet constraints = Constraint::from_lengths([10, 20, 10]); // a centered layoutlet constraints = Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]);let constraints = Constraint::from_percentages([25, 50, 25]); // a centered layout with a minimum sizelet constraints = Constraint::from_mins([0, 100, 0]); // a sidebar / main layout with maximum sizeslet constraints = Constraint::from_maxes([30, 200]); ``` ## Paragraph: `line_count` &#x26; `line_width` 📏 [Section titled “Paragraph: line_count &#x26; line_width 📏”](#paragraph-line_count--line_width) We now have a new unstable feature which helps getting information about a rendered paragraph Enable the `unstable` or `unstable-rendered-line-info` feature flag to get access to the following methods: `Paragraph::line_count`: Takes a bounding width and returns the number of lines necessary to display the full paragraph when rendered. The motivation for this method is to use it with scrollbars. If a paragraph has wrapped text, it is currently impossible to get the number of lines it will need.

- `Paragraph::line_width`: Returns the smallest line width needed to fully display every line of a paragraph. This is simply the maximum line widths.

Together, these two methods can be used to calculate the minimum bounding box for a paragraph.
Please note that they are experimental and may change in the future.

## Rect: `offset` 📐

[Section titled “Rect: offset 📐”](#rect-offset)

Rect has a new method called `offset` and it can be used to create a new Rect that is moved by the
amount specified in the x and y direction. These values can be positive or negative. This is useful
for manual layout tasks.

```
let rect = area.offset(Offset { x: 10, y -10 });
```

## `IntoIterator` for Constraints 🔄

[Section titled “IntoIterator for Constraints 🔄”](#intoiterator-for-constraints)

`Layout` and `Table` now accept `IntoIterator` for constraints with an Item that is
`AsRef&#x3C;Constraint>`.

This allows pretty much any collection of constraints to be passed to the layout functions including
arrays, vectors, slices, and iterators (without having to call collect() on them).

```
let layout = Layout::default().constraints([Constraint::Min(0)]);let layout = Layout::default().constraints(&#x26;[Constraint::Min(0)]);let layout = Layout::default().constraints(vec![Constraint::Min(0)]);let layout = Layout::default().constraints([Constraint::Min(0)].iter().filter(|_| true));let layout = Layout::default().constraints([1,2,3].iter().map(|&#x26;c| Constraint::Length(c)));
```

## Chart: Legend Position 📊

[Section titled “Chart: Legend Position 📊”](#chart-legend-position)

`Chart` has a new setter method called `legend_position`:

```
let chart: Chart = Chart::new(vec![])    .legend_position(Some(LegendPosition::TopLeft));
```

See `Chart::LegendPosition` for other possible values.

## Default Highlight Style 🎨

[Section titled “Default Highlight Style 🎨”](#default-highlight-style)

Previously the default `highlight_style` was set to `Style::default()`, which meant that the
highlight style was the same as the normal style.

This means that for example a `Tabs` widget in the default configuration would not show any
indication of the selected tab.

Now we set the `highlight_style` to the reversed style (`Style::new().reversed()`) as default.

## Tab: Custom Padding 📇

[Section titled “Tab: Custom Padding 📇”](#tab-custom-padding)

The Tab widget now contains `padding_left` and `padding_right` properties.

Those values can be set with functions `padding_left()`, `padding_right()`, and `padding()` which
all accept `Into&#x3C;Line>`.

For convenience, you can also use the `padding` method to set left/right padding accordingly:

```
// A space on either side of the tabs.let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding(" ", " ");
/// Nothing on either side of the tabs.let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding("", "");
```

## Deprecate `Cell::symbol` 🚫

[Section titled “Deprecate Cell::symbol 🚫”](#deprecate-cellsymbol)

The `Cell::symbol` field is now accessible via a getter method (`symbol()`). This will allow us to
make future changes to the Cell internals such as replacing `String` with `compact_str`.

```
cell.symbolcell.symbol()
```

## Remove Deprecated Items 💀

[Section titled “Remove Deprecated Items 💀”](#remove-deprecated-items)

We removed `Axis::title_style` and `Buffer::set_background` which were deprecated since `0.10`.

## Other 💼

[Section titled “Other 💼”](#other)

- Add new setter methods: `Span::content`, `Span::style`

- Implement `From` trait for `crossterm`/`termion` to `Style` related structs

- Rewrite the `insert_before` method on `Terminal` to fix a bug and remove the height cap

- Add `WrappedLine` struct to reflow module

- Add documentation to `Sparkline`, `Chart`, `ListItem` and many other parts of the codebase

- Reorganize the modules/documentation to respect the natural reading order

- Remove unnecessary dynamic dispatch and heap allocation from widgets

- Add Vim key bindings (movement) to the examples

- Add `#[must_use]` to fluent setter methods

- Create a [security policy](https://github.com/ratatui/ratatui/blob/main/SECURITY.md)

- Bump MSRV to `1.70.0`

And lastly, we welcome [@Valentin271](https://github.com/Valentin271) on board as a maintainer! 🥳

## New Socials 🌐

[Section titled “New Socials 🌐”](#new-socials)

We created the following social media presences for `ratatui`:

- [Mastodon](https://fosstodon.org/@ratatui_rs)

- [Twitter/X](https://twitter.com/ratatui_rs)

- [LinkedIn](https://www.linkedin.com/company/ratatui-rs)

Give us a follow for the latest `ratatui` news &#x26; other cool stuff!

## GitHub Sponsors 💖

[Section titled “GitHub Sponsors 💖”](#github-sponsors)

We have enabled GitHub Sponsors for our organization: [https://github.com/sponsors/ratatui](https://github.com/sponsors/ratatui)

Our mission is to empower developers to create more interactive and visually rich command-line
terminal user interfaces (TUIs). By sponsoring Ratatui, you’re not just funding a project; you’re
helping make the ecosystem of terminal applications in Rust better.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/highlights/v0.25.md)

 [Previous v0.26.0](/highlights/v026/) [Next v0.24.0](/highlights/v024/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
