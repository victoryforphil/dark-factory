----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/render/style-text
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, render, style text
- Summary: Styling enhances user experience by adding colors, emphasis, and other visual aids. In `ratatui`,
----

Source: https://ratatui.rs/recipes/render/style-text

# Styling Text

Styling enhances user experience by adding colors, emphasis, and other visual aids. In `ratatui`,
the primary tool for this is the `ratatui::style::Style` struct.

`ratatui::style::Style` provides a set of methods to apply styling attributes to your text. These
styles can then be applied to various text structures like `Text`, `Span`, and `Line` (as well as
other non text structures).

Common styling attributes include:

- Foreground and Background Colors (`fg` and `bg`)

- Modifiers (like `bold`, `italic`, and `underline`)

- Basic Color Styling Setting the foreground (text color) and background: ``` let styled_text = Span::styled( "Hello, Ratatui!", Style::default().fg(Color::Red).bg(Color::Yellow)); ```

- Using `Modifiers` Making text bold or italic: ``` let bold_text = Span::styled( "This is bold", Style::default().add_modifier(Modifier::BOLD)); let italic_text = Span::styled( "This is italic", Style::default().add_modifier(Modifier::ITALIC)); ``` You can also combine multiple modifiers: ``` let bold_italic_text = Span::styled( "This is bold and italic", Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)); ```

- Styling within a Line You can mix and match different styled spans within a single line: ``` let mixed_line = Line::from(vec![ Span::styled("This is mixed", Style::default().fg(Color::Green)), Span::styled("styling", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)), Span::from("!"),]); ```

This is what it would look like if you rendered a `Paragraph` with different styles for each line:

```
fn ui(_: &#x26;App, f: &#x26;mut Frame) {  let styled_text = Span::styled("Hello, Ratatui!", Style::default().fg(Color::Red).bg(Color::Yellow));  let bold_text = Span::styled("This is bold", Style::default().add_modifier(Modifier::BOLD));  let italic_text = Span::styled("This is italic", Style::default().add_modifier(Modifier::ITALIC));  let bold_italic_text =    Span::styled("This is bold and italic", Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC));  let mixed_line = vec![    Span::styled("This is mixed", Style::default().fg(Color::Green)),    Span::styled("styling", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),    Span::from("!"),  ];  let text: Vec&#x3C;Line&#x3C;'_>> =    vec![styled_text.into(), bold_text.into(), italic_text.into(), bold_italic_text.into(), mixed_line.into()];  f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL)), f.area());}
```

Here’s the HTML representation of the above styling:

Hello, Ratatui!

This is bold

This is italic

This is bold and italic

        This is mixed
        styling
        !

Tip

You can also create instances of `Color` from a string:

```
use std::str::FromStr;
let color: Color = Color::from_str("blue").unwrap();assert_eq!(color, Color::Blue);
let color: Color = Color::from_str("#FF0000").unwrap();assert_eq!(color, Color::Rgb(255, 0, 0));
let color: Color = Color::from_str("10").unwrap();assert_eq!(color, Color::Indexed(10));
```

You can read more about the
[`Color` enum](https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html) and
[`Modifier`](https://docs.rs/ratatui/latest/ratatui/style/struct.Modifier.html) in the reference
documentation online.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/render/style-text.md)

 [Previous Displaying Text](/recipes/render/display-text/) [Next Popups (overwrite regions)](/recipes/render/overwrite-regions/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
