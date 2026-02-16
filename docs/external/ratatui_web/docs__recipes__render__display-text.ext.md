----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/render/display-text
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, render, display text
- Summary: This page covers how text displaying works. It will cover `Span`, `Line`, and `Text`, and how these
----

Source: https://ratatui.rs/recipes/render/display-text

# Displaying Text

This page covers how text displaying works. It will cover `Span`, `Line`, and `Text`, and how these
can be created, styled, displayed, altered, and such.

## `Span`

[Section titled “Span”](#span)

A `Span` is a styled segment of text. You can think of it as a substring with its own unique style.
It is the most basic unit of displaying text in `ratatui`.

The examples below assume the following imports:

- ``` use ratatui::{prelude::*, widgets::*}; ``` A `Span` consists of “content” and a “style” for the content. And a `Span` can be created in a few different ways. using `Span::raw`: ``` fn ui(_app: &#x26;App, f: &#x26;mut Frame) { let span = Span::raw("This is text that is not styled"); // --snip--} ```

- using `Span::styled`: ``` fn ui(_app: &#x26;App, f: &#x26;mut Frame) { let span = Span::styled("This is text that will be yellow", Style::default().fg(Color::Yellow)); // --snip--} ```

- using the `Stylize` trait: ``` fn ui(_app: &#x26;App, f: &#x26;mut Frame) { let span = "This is text that will be yellow".yellow(); // --snip--} ```

A `Span` is the basic building block for any styled text, and can be used anywhere text is
displayed.

## `Line`

[Section titled “Line”](#line)

The next building block that we are going to talk about is a `Line`. A `Line` represents a cluster
of graphemes, where each unit in the cluster can have its own style. You can think of an instance of
the `Line` struct as essentially a collection of `Span` objects, i.e. `Vec&#x3C;Span>`.

Since each `Line` struct consists of multiple `Span` objects, this allows for varied styling in a
row of words, phrases or sentences.

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {    let line = Line::from(vec![        "hello".red(),        " ".into(),        "world".red().bold()    ]);    // --snip--}
```

A `Line` can be constructed directly from content, where the content is `Into&#x3C;Cow&#x3C;'a, &#x26;str>>`.

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {    let line = Line::from("hello world");    // --snip--}
```

You can even style a full line directly:

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {    let line = Line::styled("hello world", Style::default().fg(Color::Yellow));    // --snip--}
```

And you can use the `Stylize` trait on the line directly by using `into()`:

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {    let line: Line = "hello world".yellow().into();    // --snip--}
```

Finally, you can also align it using `alignment()` or the shorthand methods `left_aligned()`,
`centered()` and `right_aligned()`. Widgets using `Line` internally will generally respect this.

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {    let line = Line::from("hello world").alignment(Alignment::Center);    let line = Line::from("hello world").centered(); // shorthand    // --snip--}
```

## `Text`

[Section titled “Text”](#text)

`Text` is the final building block of outputting text. A `Text` object represents a collection of
`Line`s.

Most widgets accept content that can be converted to `Text`.

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {    let span1 = "hello".red();    let span2 = "world".red().bold();    let line = Line::from(vec![span1, " ".into(), span2]);    let text = Text::from(line);    f.render_widget(Paragraph::new(text).block(Block::bordered()), f.area());}
```

Here’s an HTML representation of what you’d get in the terminal:

    hello
    world

Often code like the one above can be simplified:

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {    let line: Line = vec![        "hello".red(),        " ".into(),        "world".red().bold()    ].into();    f.render_widget(Paragraph::new(line).block(Block::bordered()), f.area());}
```

This is because in this case, Rust is able to infer the types and convert them appropriately.

`Text` instances can be created using the `raw` or `styled` constructors too.

Something that you might find yourself doing pretty often for a `Paragraph` is wanting to have
multiple lines styled differently. This is one way you might go about that:

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {    let text = vec![        "hello world 1".into(),        "hello world 2".blue().into(),        Line::from(vec!["hello".green(), " ".into(), "world".green().bold(), "3".into()]),    ];    f.render_widget(Paragraph::new(text).block(Block::bordered()), f.area());}
```

        hello world 1

        hello world 2

        hello
        world 3

We will talk more about styling in the next section.

As with `Line`, a `Text` can be aligned with `alignment()` or the shorthand methods
`left_aligned()`, `centered()` and `right_aligned()`. Widgets using `Text` internally will generally
respect this. Note in the example below, you can override the alignment for a particular line.

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {    let text= Text::from(vec![        Line::from("hello world 1").left_aligned(),        Line::from("hello world 2"),        Line::from("hello world 3").right_aligned(),    ]).centered();    f.render_widget(Paragraph::new(text).block(Block::bordered()), f.area());}
```

hello world 1

hello world 2

hello world 3

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/render/display-text.md)

 [Previous Render UIs](/recipes/render/) [Next Styling Text](/recipes/render/style-text/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
