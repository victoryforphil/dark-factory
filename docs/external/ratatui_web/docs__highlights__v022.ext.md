----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /highlights/v022
- Keywords: ratatui, rust, tui, terminal ui, docs, highlights, v022
- Summary: [https://github.com/ratatui/ratatui/releases/tag/v0.22.0](https://github.com/ratatui/ratatui/releases/tag/v0.22.0)
----

Source: https://ratatui.rs/highlights/v022

# v0.22.0

[https://github.com/ratatui/ratatui/releases/tag/v0.22.0](https://github.com/ratatui/ratatui/releases/tag/v0.22.0)

Note

reposted from [https://blog.orhun.dev/ratatui-0-22-0/](https://blog.orhun.dev/ratatui-0-22-0/)

## Prelude

[Section titled “Prelude”](#prelude)

We now have a `prelude` module! This allows users of the library to easily use `ratatui` without a
huge amount of imports.

- ``` use ratatui::prelude::*; ``` Aside from the main types that are used in the library, this `prelude` also re-exports several modules to make it easy to qualify types that would otherwise collide. For example: ``` use ratatui::{prelude::*, widgets::*}; #[derive(Debug, Default, PartialEq, Eq)]struct Line; assert_eq!(Line::default(), Line);assert_eq!(text::Line::default(), ratatui::text::Line::from(vec![])); ``` ## New widget: Scrollbar [Section titled “New widget: Scrollbar”](#new-widget-scrollbar) A scrollbar widget has been added which can be used with any `Rect`. It can also be customized with different styles and symbols. Here are the components of a `Scrollbar`: ``` &#x3C;--▮------->^ ^ ^ ^│ │ │ └ end│ │ └──── track│ └──────── thumb└─────────── begin ``` To use it, render it as a stateful widget along with `ScrollbarState`: ``` frame.render_stateful_widget( Scrollbar::default() .orientation(ScrollbarOrientation::VerticalRight) .begin_symbol(Some("↑")) .end_symbol(Some("↓")), rect, &#x26;mut scrollbar_state,); ``` Will result in: ``` ┌scrollbar──────────────────↑│This is a longer line ║│Veeeeeeeeeeeeeeeery looo█│This is a line ║└───────────────────────────↓ ``` ## Block: support multiple titles [Section titled “Block: support multiple titles”](#block-support-multiple-titles) `Block` widget now supports having more than one title via `Title` widget. Each title will be rendered with a single space separating titles that are in the same position or alignment. When both centered and non-centered titles are rendered, the centered space is calculated based on the full width of the block, rather than the leftover width. You can provide various types as the title, including strings, string slices, borrowed strings (`Cow&#x3C;str>`), spans, or vectors of spans (`Vec&#x3C;Span>`). It can be used as follows: ``` Block::default() .borders(Borders::ALL) .title("Title") // By default in the top right corner .title(Title::from("Left").alignment(Alignment::Left)) .title(Title::from("Center").alignment(Alignment::Center)) .title(Title::from("Bottom").position(Position::Bottom)) .title( Title::from("Bottom center") .alignment(Alignment::Center) .position(Position::Bottom), ); ``` Results in: ``` ┌Title─Left──Center─────────────┐│ ││ ││ │└Bottom───Bottom center─────────┘ ``` ## Barchart: support groups [Section titled “Barchart: support groups”](#barchart-support-groups) `Barchart` has been improved to support adding multiple bars from different data sets. This can be done by using the newly added `Bar` and `BarGroup` objects. See the [barchart example](https://github.com/ratatui/ratatui/blob/main/examples/barchart.rs) for more information and implementation details. ## Stylization shorthands [Section titled “Stylization shorthands”](#stylization-shorthands) It is possible to use style shorthands for `str`, `Span`, and `Paragraph`. A crazy example would be: ``` "hello" .on_black() .black() .bold() .underline() .dimmed() .slow_blink() .crossed_out() .reversed() ``` This especially helps with concise styling: ``` assert_eq!( "hello".red().on_blue().bold(), Span::styled("hello", Style::default().fg(Color::Red).bg(Color::Blue).add_modifier(Modifier::BOLD))) ``` ## Stylize everything [Section titled “Stylize everything”](#stylize-everything) All widgets can be styled now (i.e. `set_style`) `Styled` trait is implemented for all the remaining widgets, including: `Barchart`

- `Chart` (including `Axis` and `Dataset`)

- `Gauge` and `LineGauge`

- `List` and `ListItem`

- `Sparkline`

- `Table`, `Row`, and `Cell`

- `Tabs`

- `Style`

## Constant styles

[Section titled “Constant styles”](#constant-styles)

`Style`s can be constructed in a `const` context as follows:

```
const DEFAULT_MODIFIER: Modifier = Modifier::BOLD.union(Modifier::ITALIC);const EMPTY: Modifier = Modifier::empty();
const DEFAULT_STYLE: Style = Style::with(DEFAULT_MODIFIER, EMPTY)    .fg(Color::Red)    .bg(Color::Black);
```

## More colors formats

[Section titled “More colors formats”](#more-colors-formats)

It is now possible to parse hyphenated color names like `light-red` via `Color::from_str`.

Additionally, all colors from the
[ANSI color table](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) are supported (though some
names are not exactly the same).

- `gray` is sometimes called `white` - this is not supported as we use `white` for bright white

- `gray` is sometimes called `silver` - this is supported

- `darkgray` is sometimes called `light black` or `bright black` (both are supported)

- `white` is sometimes called `light white` or `bright white` (both are supported)

- we support `bright` and `light` prefixes for all colors

- we support `"-"`, `"_"`, and `" "` as separators for all colors

- we support both `gray` and `grey` spellings

For example:

```
use ratatui::style::Color;use std::str::FromStr;
assert_eq!(Color::from_str("red"), Ok(Color::Red));assert_eq!("red".parse(), Ok(Color::Red));assert_eq!("lightred".parse(), Ok(Color::LightRed));assert_eq!("light red".parse(), Ok(Color::LightRed));assert_eq!("light-red".parse(), Ok(Color::LightRed));assert_eq!("light_red".parse(), Ok(Color::LightRed));assert_eq!("lightRed".parse(), Ok(Color::LightRed));assert_eq!("bright red".parse(), Ok(Color::LightRed));assert_eq!("bright-red".parse(), Ok(Color::LightRed));assert_eq!("silver".parse(), Ok(Color::Gray));assert_eq!("dark-grey".parse(), Ok(Color::DarkGray));assert_eq!("dark gray".parse(), Ok(Color::DarkGray));assert_eq!("light-black".parse(), Ok(Color::DarkGray));assert_eq!("white".parse(), Ok(Color::White));assert_eq!("bright white".parse(), Ok(Color::White));
```

## Integrations

[Section titled “Integrations”](#integrations)

Following tools are now integrated into the repository:

- [`cargo-husky`](https://github.com/rhysd/cargo-husky): git pre-push hooks

- [`bacon`](https://github.com/Canop/bacon): background code checks / coverage

- [`commitizen`](https://github.com/commitizen/cz-cli): conventional commits

- [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny): linting dependencies

- [`typos`](https://github.com/crate-ci/typos): spell checker

## Other

[Section titled “Other”](#other)

- Benchmarks added for the `Paragraph` widget

- Added underline colors support for `crossterm` backend

- Mark some of the low-level functions of `Block`, `Layout` and `Rect` as `const`

- The project license has been updated to acknowledge `ratatui` developers

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/highlights/v0.22.md)

 [Previous v0.23.0](/highlights/v023/) [Next v0.21.0](/highlights/v021/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
