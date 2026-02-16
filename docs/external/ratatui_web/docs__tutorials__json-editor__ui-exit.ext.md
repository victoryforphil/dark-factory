----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/json-editor/ui-exit
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, json editor, ui exit
- Summary: We have a way for the user to view their already entered key-value pairs, and we have a way for the
----

Source: https://ratatui.rs/tutorials/json-editor/ui-exit

# UI - Exit Popup

We have a way for the user to view their already entered key-value pairs, and we have a way for the
user to enter new ones. The last screen we need to create, is the exit/confirmation screen.

In this screen, we are asking the user if they want to output the key-value pairs they have entered
in the `stdout` pipe, or close without outputting anything.

```
if let CurrentScreen::Exiting = app.current_screen {        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn        let popup_block = Block::default()            .title("Y/N")            .borders(Borders::NONE)            .style(Style::default().bg(Color::DarkGray));
        let exit_text = Text::styled(            "Would you like to output the buffer as json? (y/n)",            Style::default().fg(Color::Red),        );        // the `trim: false` will stop the text from being cut off when over the edge of the block        let exit_paragraph = Paragraph::new(exit_text)            .block(popup_block)            .wrap(Wrap { trim: false });
        let area = centered_rect(60, 25, frame.area());        frame.render_widget(exit_paragraph, area);    }
```

The only thing in this part that we haven’t done before, is use the
[`Clear`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html) widget. This is a
special widget that does what the name suggests --- it clears everything in the space it is
rendered.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/json-editor/ui-exit.md)

 [Previous UI - Editing Popup](/tutorials/json-editor/ui-editing/) [Next Closing Thoughts](/tutorials/json-editor/closing-thoughts/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
