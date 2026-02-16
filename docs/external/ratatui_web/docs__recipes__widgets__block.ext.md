----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/widgets/block
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, widgets, block
- Summary: The `Block` widget serves as a foundational building block for structuring and framing other
----

Source: https://ratatui.rs/recipes/widgets/block

# Block

The `Block` widget serves as a foundational building block for structuring and framing other
widgets. It’s essentially a container that can have borders, a title, and other styling elements to
enhance the aesthetics and structure of your terminal interface. This page provides an in-depth
exploration of the `Block` widget.

## Basic Usage

[Section titled “Basic Usage”](#basic-usage)

The simplest use case for a `Block` is to create a container with borders:

```
let b = Block::default()    .borders(Borders::ALL);f.render_widget(b, chunks[0]);
```

## Titles

[Section titled “Titles”](#titles)

A common use case for Block is to give a section of the UI a title or a label:

```
let b = Block::default()    .title("Header")    .borders(Borders::ALL);f.render_widget(b, chunks[0]);
```

You can also use the `Line` struct for better positioning or multiple titles.

```
let b = Block::default()    .title(Line::from("Left Title").left_aligned())    .title(Line::from("Middle Title").centered())    .title(Line::from("Right Title").right_aligned())    .borders(Borders::ALL);f.render_widget(b, chunks[0]);
```

## Border style

[Section titled “Border style”](#border-style)

Block provides flexibility in both the borders style and type:

```
let b = Block::default()    .title("Styled Header")    .border_style(Style::default().fg(Color::Magenta))    .border_type(BorderType::Rounded)    .borders(Borders::ALL);f.render_widget(b, chunks[0]);
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/widgets/block.md)

 [Previous Use Widgets](/recipes/widgets/) [Next Paragraph](/recipes/widgets/paragraph/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
