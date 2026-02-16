----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /highlights/v0262
- Keywords: ratatui, rust, tui, terminal ui, docs, highlights, v0262
- Summary: [https://github.com/ratatui/ratatui/releases/tag/v0.26.2](https://github.com/ratatui/ratatui/releases/tag/v0.26.2)
----

Source: https://ratatui.rs/highlights/v0262

# v0.26.2

[https://github.com/ratatui/ratatui/releases/tag/v0.26.2](https://github.com/ratatui/ratatui/releases/tag/v0.26.2)

## MSRV: 1.74.0 🦀

[Section titled “MSRV: 1.74.0 🦀”](#msrv-1740)

The minimum supported Rust version of Ratatui is updated from `1.70.0` to `1.74.0`.

## List: Scroll Padding 📜

[Section titled “List: Scroll Padding 📜”](#list-scroll-padding)

We introduced a new method for `List` which allows a certain number of items be kept visible above
and below the currently selected item while scrolling.

- ``` let list = List::new(items).scroll_padding(1); ``` Demo of the new behavior (visible on the left side) ## Text: Construct from Iterator 🏗️ [Section titled “Text: Construct from Iterator 🏗️”](#text-construct-from-iterator-️) `Line` and `Text` widgets now implement `FromIterator` which means you can: Construct `Line` from an iterator of `Span`

```
let line = Line::from_iter(vec!["Hello".blue(), " world!".green()]);let line: Line = iter::once("Hello".blue())    .chain(iter::once(" world!".green()))    .collect();
```

- Construct `Text` from an iterator of `Line`

```
let text = Text::from_iter(vec!["The first line", "The second line"]);let text: Text = iter::once("The first line")    .chain(iter::once("The second line"))    .collect();
```

## Text: Push Methods 📥

[Section titled “Text: Push Methods 📥”](#text-push-methods)

We added the following methods to the `Text` and `Line` structs:

- `Text::push_line`

- `Text::push_span`

- `Line::push_span`

This allows for adding lines and spans to a text object without having to call methods on the fields
directly, which is useful for incremental construction of text objects.

For example:

```
let mut line = Line::from("Hello, ");line.push_span(Span::raw("world!"));line.push_span(" How are you?");
```

## Implement Widget for strings 🧶

[Section titled “Implement Widget for strings 🧶”](#implement-widget-for-strings)

`Widget` is now implemented for `&#x26;str` and `String`, which makes it easier to render strings with no
styles as widgets.

Example usage:

```
terminal.draw(|f| f.render_widget("Hello World!", f.size()))?;
```

## Span: Rename Methods 🔄

[Section titled “Span: Rename Methods 🔄”](#span-rename-methods)

The following Span methods are renamed accordingly to the Rust method naming conventions.

Deprecated usage:

- `Span::to_centered_line`

- `Span::to_left_aligned_line`

- `Span::to_right_aligned_line`

New usage:

- `Span::into_centered_line`

- `Span::into_left_aligned_line`

- `Span::into_right_aligned_line`

## Funding 🧀

[Section titled “Funding 🧀”](#funding)

We are happy to share that we have received a funding donation from [Radicle](https://radicle.xyz)!

You can read about the details [here](https://blog.orhun.dev/open-source-funding-with-ratatui).

## Other 💼

[Section titled “Other 💼”](#other)

- Marked various functions as const ([#951](https://github.com/ratatui/ratatui/pull/951))

- Respect alignment on Line truncation ([#987](https://github.com/ratatui/ratatui/pull/987))

- Don’t render scrollbar on zero length track ([#964](https://github.com/ratatui/ratatui/pull/964))

- Fix panic when rendering Text out of bounds ([#997](https://github.com/ratatui/ratatui/pull/997))

- Fix highlight_symbol overflow ([#949](https://github.com/ratatui/ratatui/pull/949))

- Fix Scrollbar thumb not being visible on long lists ([#959](https://github.com/ratatui/ratatui/pull/959))

- Ensure that paragraph correctly renders styled text ([#992](https://github.com/ratatui/ratatui/pull/992))

- Applied clippy (pedantic) suggestions

And lastly, we welcome [@EdJoPaTo](https://github.com/EdJoPaTo) on board as a maintainer! 🥳

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/highlights/v0.26.2.md)

 [Previous v0.26.3](/highlights/v0263/) [Next v0.26.0](/highlights/v026/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
