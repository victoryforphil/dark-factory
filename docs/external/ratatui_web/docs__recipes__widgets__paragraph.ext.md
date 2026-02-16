----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/widgets/paragraph
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, widgets, paragraph
- Summary: The `Paragraph` widget provides a way to display text content in your terminal user interface. It
----

Source: https://ratatui.rs/recipes/widgets/paragraph

# Paragraph

The `Paragraph` widget provides a way to display text content in your terminal user interface. It
allows not only plain text display but also handling text wrapping, alignment, and styling. This
page will delve deeper into the functionality of the `Paragraph` widget.

## Usage

[Section titled “Usage”](#usage)

```
let p = Paragraph::new("Hello, World!");f.render_widget(p, chunks[0]);
```

## Styling and Borders

[Section titled “Styling and Borders”](#styling-and-borders)

You can also apply styles to your text and wrap it with a border:

```
let p = Paragraph::new("Hello, World!")    .style(Style::default().fg(Color::Yellow))    .block(        Block::default()            .borders(Borders::ALL)            .title("Title")            .border_type(BorderType::Rounded)    );f.render_widget(p, chunks[0]);
```

## Wrapping

[Section titled “Wrapping”](#wrapping)

The `Paragraph` widget will wrap the content based on the available width in its containing block.
You can also control the wrapping behavior using the `wrap` method:

```
let p = Paragraph::new("A very long text that might not fit the container...")    .wrap(Wrap { trim: true });f.render_widget(p, chunks[0]);
```

Setting `trim` to `true` will ensure that trailing whitespaces at the end of each line are removed.

## Alignment

[Section titled “Alignment”](#alignment)

```
let p = Paragraph::new("Centered Text")    .alignment(Alignment::Center);f.render_widget(p, chunks[0]);
```

## Styled Text

[Section titled “Styled Text”](#styled-text)

`Paragraph` supports rich text through `Span`, `Line`, and `Text`:

```
let mut lines = vec![];lines.push(Line::from(vec![    Span::styled("Hello ", Style::default().fg(Color::Yellow)),    Span::styled("World", Style::default().fg(Color::Blue).bg(Color::White)),]));lines.push(Line::from(vec![    Span::styled("Goodbye ", Style::default().fg(Color::Yellow)),    Span::styled("World", Style::default().fg(Color::Blue).bg(Color::White)),]));let text = Text::from(lines);let p = Paragraph::new(text);f.render_widget(p, chunks[0]);
```

## Scrolling

[Section titled “Scrolling”](#scrolling)

For long content, `Paragraph` supports scrolling:

```
let mut p = Paragraph::new("Lorem ipsum ...")    .scroll((1, 0));  // Scroll down by one linef.render_widget(p, chunks[0]);
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/widgets/paragraph.md)

 [Previous Block](/recipes/widgets/block/) [Next Create custom widgets](/recipes/widgets/custom/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
