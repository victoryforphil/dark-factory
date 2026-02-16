----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /highlights/v023
- Keywords: ratatui, rust, tui, terminal ui, docs, highlights, v023
- Summary: [https://github.com/ratatui/ratatui/releases/tag/v0.23.0](https://github.com/ratatui/ratatui/releases/tag/v0.23.0)
----

Source: https://ratatui.rs/highlights/v023

# v0.23.0

[https://github.com/ratatui/ratatui/releases/tag/v0.23.0](https://github.com/ratatui/ratatui/releases/tag/v0.23.0)

Note

reposted from [https://blog.orhun.dev/ratatui-0-23-0/](https://blog.orhun.dev/ratatui-0-23-0/)

## Coolify everything 😎

[Section titled “Coolify everything 😎”](#coolify-everything)

We already had a cool name and a logo, and now we have a cool description as well:

- ``` ratatui: A Rust library to build rich terminal user interfaces or dashboards.ratatui: A Rust library that's all about cooking up terminal user interfaces. ``` We also renamed our organization from `tui-rs-revival` to `ratatui`: [https://github.com/ratatui/ratatui](https://github.com/ratatui/ratatui)

## Barchart: horizontal bars

[Section titled “Barchart: horizontal bars”](#barchart-horizontal-bars)

You can now render the bars horizontally for the `Barchart` widget. This is especially useful in
some cases to make more efficient use of the available space.

Simply use the `Direction` attribute for rendering horizontal bars:

```
let mut barchart = BarChart::default()    .block(Block::default().title("Data1").borders(Borders::ALL))    .bar_width(1)    .group_gap(1)    .bar_gap(0)    .direction(Direction::Horizontal);
```

Here is an example of what you can do with the `Barchart` widget (see the bottom right for
horizontal bars):

## Voluntary skipping capability for Sixel

[Section titled “Voluntary skipping capability for Sixel”](#voluntary-skipping-capability-for-sixel)

[Sixel](https://en.wikipedia.org/wiki/Sixel) is a bitmap graphics format supported by terminals.
“Sixel mode” is entered by sending the sequence `ESC+Pq`. The “String Terminator” sequence `ESC+\`
exits the mode.

`Cell` widget now has a `set_skip` method that allows the cell to be skipped when copying (diffing)
the buffer to the screen. This is helpful when it is necessary to prevent the buffer from
overwriting a cell that is covered by an image from some terminal graphics protocol such as Sixel,
iTerm, Kitty, etc.

See the pull request for more information:
[https://github.com/ratatui/ratatui/pull/215](https://github.com/ratatui/ratatui/pull/215)

In this context, there is also an experimental image rendering crate:
[ratatui-image](https://github.com/benjajaja/ratatui-image)

## Table/List: Highlight spacing

[Section titled “Table/List: Highlight spacing”](#tablelist-highlight-spacing)

We added a new property called `HighlightSpacing` to the `Table` and `List` widgets and it can be
optionally set via calling `highlight_spacing` function.

Before this option was available, selecting a row in the table when no row was selected previously
made the tables layout change (the same applies to unselecting) by adding the width of the
“highlight symbol” in the front of the first column. The idea is that we want this behaviour to be
configurable with this newly added option.

```
let list = List::new(items)    .highlight_symbol(">>")    .highlight_spacing(HighlightSpacing::Always);
```

Right now, there are 3 variants:

- `Always`: Always add spacing for the selection symbol column.

- `WhenSelected`: Only add spacing for the selection symbol column if a row is selected.

- `Never`: Never add spacing to the selection symbol column, regardless of whether something is selected or not.

## Table: support line alignment

[Section titled “Table: support line alignment”](#table-support-line-alignment)

```
let table = Table::new(vec![        Row::new(vec![Line::from("Left").alignment(Alignment::Left)]),        Row::new(vec![Line::from("Center").alignment(Alignment::Center)]),        Row::new(vec![Line::from("Right").alignment(Alignment::Right)]),    ])    .widths(&#x26;[Constraint::Percentage(100)]);
```

Now results in:

```
Left       Center               Right
```

## Scrollbar: optional track symbol

[Section titled “Scrollbar: optional track symbol”](#scrollbar-optional-track-symbol)

The track symbol in the `Scrollbar` is now optional, simplifying composition with other widgets. It
also makes it easier to use the `Scrollbar` in tandem with a block with special block characters.

One breaking change is that `track_symbol` needs to be set in the following way now:

```
let scrollbar = Scrollbar::default().track_symbol("-");let scrollbar = Scrollbar::default().track_symbol(Some("-"));
```

It also makes it possible to render a custom track that is composed out of multiple differing track
symbols.

## `symbols::scrollbar` module

[Section titled “symbols::scrollbar module”](#symbolsscrollbar-module)

The symbols and sets are moved from `widgets::scrollbar` to `symbols::scrollbar`. This makes it
consistent with the other symbol sets. We also made the `scrollbar` module private.

Since this is a breaking change, you need to update your code to add an import for
`ratatui::symbols::scrollbar::*` (or the specific symbols you need).

## Alpha releases

[Section titled “Alpha releases”](#alpha-releases)

The alpha releases (i.e. pre-releases) are created *every Saturday* and they are automated with
the help of
[this GitHub Actions workflow](https://github.com/ratatui/ratatui/blob/main/.github/workflows/cd.yml).
This is especially useful if you want to test `ratatui` or use unstable/experimental features before
we hit a stable release.

The versioning scheme is `v&#x3C;version>-alpha.&#x3C;num>`, for example:
[v0.22.1-alpha.2](https://github.com/ratatui/ratatui/releases/tag/v0.22.1-alpha.2)

Additionally, see the following issue for possible contributions in the context of alpha releases
and documentation:
[https://github.com/ratatui/ratatui/issues/412](https://github.com/ratatui/ratatui/issues/412)

## Example GIFs

[Section titled “Example GIFs”](#example-gifs)

We added GIFs for each example in the `examples/` directory and added a `README.md` for preview.
This should make it easier to see what each example does without having to run it.

See:
[https://github.com/ratatui/ratatui/blob/main/examples/README.md](https://github.com/ratatui/ratatui/blob/main/examples/README.md)

One thing to note here is that we used [vhs](https://github.com/charmbracelet/vhs) for generating
GIFs from a set of instructions. For example:

```
# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.# To run this script, install vhs and run `vhs ./examples/demo.tape`Output "target/demo.gif"Set Theme "OceanicMaterial"Set Width 1200Set Height 1200Set PlaybackSpeed 0.5HideType "cargo run --example demo"EnterSleep 2sShowSleep 1sDown@1s 12RightSleep 4sRightSleep 4s
```

Results in:

We also host these GIFs at [https://vhs.charm.sh](https://vhs.charm.sh) but there is an issue about
moving everything to GitHub. If you are interested in contributing regarding this, see
[https://github.com/ratatui/ratatui/issues/401](https://github.com/ratatui/ratatui/issues/401)

## Common traits

[Section titled “Common traits”](#common-traits)

With the help of [strum](https://crates.io/crates/strum) crate, we added `Display` and `FromStr`
implementation to enum types.

Also, we implemented common traits such as `Debug`, `Default`, `Clone`, `Copy`, `Eq`, `PartialEq`,
`Ord`, `PartialOrd`, `Hash` to the structs/enums where possible.

## Test coverage 🧪

[Section titled “Test coverage 🧪”](#test-coverage)

`ratatui` now has [90% test coverage](https://app.codecov.io/gh/ratatui/ratatui)!

Shoutout to everyone who added tests/benchmarks for various widgets made this possible.

## No unsafe ⚠️

[Section titled “No unsafe ⚠️”](#no-unsafe-️)

We now forbid [unsafe code](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) in `ratatui`.
Also, see [this discussion](https://github.com/ratatui/ratatui/discussions/66) we had in the past
about using `unsafe` code for optimization purposes.

## The book 📕

[Section titled “The book 📕”](#the-book)

We are working on a book for more in-depth `ratatui` documentation and usage examples, you can read
it at [https://ratatui.rs/](https://ratatui.rs/)

Repository:
[https://github.com/ratatui/ratatui-website](https://github.com/ratatui/ratatui-website)

## Other

[Section titled “Other”](#other)

- Expand serde attributes for `TestBuffer` for de/serializing the whole test buffer.

- Add weak constraints to make `Rect`s closer to each other in size.

- Simplify `Layout::split` function.

- Various bug fixes and improvements in Barchart, Block, Layout and other widgets.

- Add documentation to various widgets and improve existing documentation.

- Add examples for colors and modifiers.

- We created a Matrix bridge at [#ratatui:matrix.org](https://matrix.to/#/#ratatui:matrix.org).

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/highlights/v0.23.md)

 [Previous v0.24.0](/highlights/v024/) [Next v0.22.0](/highlights/v022/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
