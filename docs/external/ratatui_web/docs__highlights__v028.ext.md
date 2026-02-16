----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /highlights/v028
- Keywords: ratatui, rust, tui, terminal ui, docs, highlights, v028
- Summary: [https://github.com/ratatui/ratatui/releases/tag/v0.28.0](https://github.com/ratatui/ratatui/releases/tag/v0.28.0)
----

Source: https://ratatui.rs/highlights/v028

# v0.28.0

[https://github.com/ratatui/ratatui/releases/tag/v0.28.0](https://github.com/ratatui/ratatui/releases/tag/v0.28.0)

⚠️ See the [breaking changes](https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md) for
this release.

## Crossterm 0.28.0 ⬆️

[Section titled “Crossterm 0.28.0 ⬆️”](#crossterm-0280-️)

Crossterm is updated to version
[0.28.0](https://github.com/crossterm-rs/crossterm/blob/master/CHANGELOG.md#version-028), which is a
semver incompatible version with the previous version (`0.27.0`). Ratatui re-exports the version of
crossterm that it is compatible with under `ratatui::crossterm`, which can be used to avoid
incompatible versions in your dependency list.

See [this issue](https://github.com/ratatui/ratatui/issues/1298) for more information.

## Chart: Add GraphType::Bar 📊

[Section titled “Chart: Add GraphType::Bar 📊”](#chart-add-graphtypebar)

We have introduced a new variant to `GraphType`, named `Bar`, which is designed to draw a bar for
each point in the dataset:

- ``` let chart = Chart::new(vec![Dataset::default() .data(&#x26;data) .marker(symbols::Marker::Dot) .graph_type(GraphType::Bar)]); ``` ## Async Example 📚 [Section titled “Async Example 📚”](#async-example) We added a new example which demonstrates how to use Ratatui with widgets that fetch data asynchronously. The code is available [here](https://github.com/ratatui/ratatui/tree/main/examples/async.rs). ## Barchart: Support Lines 📈 [Section titled “Barchart: Support Lines 📈”](#barchart-support-lines) Previously, `Axis::labels` accepted a `Vec&#x3C;Span>`. To make it more flexible, we have changed it to accept a vector of any type that can be converted into a [`Line`] (e.g., `&#x26;str`, `String`, `&#x26;Line`, `Span`, etc.). This means any code using conversion methods that infer the type will need to be rewritten as follows. ``` Axis::default().labels(vec!["a".into(), "b".into()])Axis::default().labels(["a", "b"]) ``` ## Terminal: `try_draw` ✨ [Section titled “Terminal: try_draw ✨”](#terminal-try_draw) We have added a new method to `Terminal` called `try_draw`, which functions similarly to `Terminal::draw` but allows the render callback to be a function or closure that returns a `Result` instead of nothing (`()`). This makes it easier to handle fallible rendering methods using the `?` operator: ``` terminal.try_draw(|frame| { some_method_that_can_fail()?; another_fallible_method()?; Ok(())})?; ``` The method returns `Result::Ok` with a `CompletedFrame` if successful, or `Result::Err` with the `std::io::Error` that caused the failure. ## Terminal: Make `terminal` module private 🔒 [Section titled “Terminal: Make terminal module private 🔒”](#terminal-make-terminal-module-private) The terminal module is now private and can not be used directly. The types under this module are exported from the root of the crate. ``` use ratatui::terminal::{CompletedFrame, Frame, Terminal, TerminalOptions, ViewPort};use ratatui::{CompletedFrame, Frame, Terminal, TerminalOptions, ViewPort}; ``` This simplifies the public API, making it more user-friendly for those unfamiliar with Rust’s re-exports and avoiding clashes with other modules named `terminal` in backend code. ## Backend: Add `get/set_cursor_position()` 📍 [Section titled “Backend: Add get/set_cursor_position() 📍”](#backend-add-getset_cursor_position) If you implement the `Backend` trait yourself, you need to update the implementation and add the `get/set_cursor_position` methods, which indicates more clearly what about the cursor to set. These new methods return/accept `Into&#x3C;Position>` which can be either a `Position` or a `(u16, u16)` tuple. ``` backend.set_cursor_position(Position { x: 0, y: 20 })?;let position = backend.get_cursor_position()?; terminal.set_cursor_position((0, 20))?;let position = terminal.set_cursor_position()?; ``` You can remove the `get/set_cursor` methods from your implementation as they are deprecated and a default implementation for them exists. ## Buffer: Add `cell`, `cell_mut` and `index` 🧩 [Section titled “Buffer: Add cell, cell_mut and index 🧩”](#buffer-add-cell-cell_mut-and-index) Buffer used to access elements with `buf.get(x, y)` or `buf.get_mut(x, y)`. Now, we have added support for index operators and introduced `buf.cell()` and `buf.cell_mut()` methods. These new methods use `Into&#x3C;Position>` for coordinates, making them easier to use and safer by returning `Option&#x3C;&#x26;Cell>` and `Option&#x3C;&#x26;mut Cell>`, which helps avoid panics (yay). ``` let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10)); // Access cellslet cell = buf[(0, 0)];let cell = buf[Position::new(0, 0)]; // Get symbollet symbol = buf.cell((0, 0)).map(|cell| cell.symbol());let symbol = buf.cell(Position::new(0, 0)).map(|cell| cell.symbol()); // Set symbolbuf[(0, 0)].set_symbol("🐀");buf[Position::new(0, 0)].set_symbol("🐀");buf.cell_mut((0, 0)).map(|cell| cell.set_symbol("🐀"));buf.cell_mut(Position::new(0, 0)).map(|cell| cell.set_symbol("🐀")); ``` The existing `get()` and `get_mut()` methods are now marked as deprecated. ## Frame: Rename `size()` to `area()` 🔄 [Section titled “Frame: Rename size() to area() 🔄”](#frame-rename-size-to-area) It is just the more correct name. 🧀 `Frame::size` is now deprecated. ## Text: Add `Add` and `AddAssign` implementations ✏️ [Section titled “Text: Add Add and AddAssign implementations ✏️”](#text-add-add-and-addassign-implementations-️) You can now combine Line, Span, and Text types together while inferring their types! ``` let line = Span::raw("Red").red() + Span::raw("blue").blue();let line = Line::raw("Red").red() + Span::raw("blue").blue();let line = Line::raw("Red").red() + Line::raw("Blue").blue();let text = Line::raw("Red").red() + Line::raw("Blue").blue();let text = Text::raw("Red").red() + Line::raw("Blue").blue(); let mut line = Line::raw("Red").red();line += Span::raw("Blue").blue(); let mut text = Text::raw("Red").red();text += Line::raw("Blue").blue(); line.extend(vec![Span::raw("1"), Span::raw("2"), Span::raw("3")]); ``` ## Text: Remove unnecessary lifetime 🔧 [Section titled “Text: Remove unnecessary lifetime 🔧”](#text-remove-unnecessary-lifetime) The [`ToText`](https://docs.rs/ratatui/latest/ratatui/text/trait.ToText.html) trait no longer has a lifetime parameter. This change simplifies the trait and makes it easier to implement. ## List/Table: New Scroll Methods 🔽 [Section titled “List/Table: New Scroll Methods 🔽”](#listtable-new-scroll-methods) We have implemented new `scroll_down_by(u16)` and `scroll_up_by(u16)` methods for both `ListState` and `TableState`, which allow you to scroll through the items by a specified number of positions. ``` let mut state = ListState::default();state.select(Some(2));state.scroll_down_by(4);assert_eq!(state.selected, Some(6)); let mut state = TableState::default();state.select(Some(3));state.scroll_up_by(3);assert_eq!(state.selected, Some(0)); ``` ## Table: Navigation Methods 🧭 [Section titled “Table: Navigation Methods 🧭”](#table-navigation-methods) You can now navigate in the `Table` widget by using the following methods! ``` let mut state = TableState::default();state.select_first();state.select_next();state.select_previous();state.select_last(); ``` This is the equivalent API as in `ListState`. ## Tracking Benchmarks ⏲️ [Section titled “Tracking Benchmarks ⏲️”](#tracking-benchmarks-️) We started using [Bencher.dev](https://bencher.dev/) to track benchmarks over time and easily catch any regressions. You can view our benchmarks at [https://bencher.dev/console/projects/ratatui](https://bencher.dev/console/projects/ratatui). For discussions about future improvements, check out the [tracking issue](https://github.com/ratatui/ratatui/issues/1092). ## Check Semver Violations 🚦 [Section titled “Check Semver Violations 🚦”](#check-semver-violations) We have started experimenting with [`cargo-semver-checks`](https://github.com/obi1kenobi/cargo-semver-checks) in our CI to lint our API for semver violations! See the [PR](https://github.com/ratatui/ratatui/pull/1166) for related discussion. ## Other 💼 [Section titled “Other 💼”](#other) Return `Size` from `Backend::size` instead of `Rect` ([#1254](https://github.com/ratatui/ratatui/pull/1254))

- Remove unnecessary synchronization in `Layout` cache ([#1245](https://github.com/ratatui/ratatui/pull/1245)) `Layout::init_cache` no longer returns bool and takes a `NonZeroUsize` instead of `usize`

- Remove unnecessary allocations when creating `Line`s ([#1237](https://github.com/ratatui/ratatui/pull/1237))

- Avoid extra allocations when rendering `List` &#x26; `Table`([#1244](https://github.com/ratatui/ratatui/pull/1244) &#x26; [#1242](https://github.com/ratatui/ratatui/pull/1242))

- Add `Size::ZERO` and `Position::ORIGIN` constants to `Layout` ([#1253](https://github.com/ratatui/ratatui/pull/1253))

- Enable serde for `Margin`, `Position`, `Rect` and `Size` ([#1255](https://github.com/ratatui/ratatui/pull/1255))

- Prevent area mismatch in `TestBackend` (changes the serde representation) ([#1252](https://github.com/ratatui/ratatui/pull/1252))

- Allow removing all the axis labels in `Chart` ([#1282](https://github.com/ratatui/ratatui/pull/1282))

- Only apply style to first line when rendering a `Line` ([#1247](https://github.com/ratatui/ratatui/pull/1247))

- Ensure emojis are rendered ([#1258](https://github.com/ratatui/ratatui/pull/1258))

- Add Code of Conduct ([#1279](https://github.com/ratatui/ratatui/pull/1279))

“If you are what you eat, then I only want to eat the good stuff.” – Remy

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/highlights/v0.28.md)

 [Previous v0.29.0](/highlights/v029/) [Next v0.27.0](/highlights/v027/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
