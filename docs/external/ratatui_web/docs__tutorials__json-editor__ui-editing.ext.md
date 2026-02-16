----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/json-editor/ui-editing
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, json editor, ui editing
- Summary: Now that the `Main` screen is rendered, we now need to check if the `Editing` popup needs to be
----

Source: https://ratatui.rs/tutorials/json-editor/ui-editing

# UI - Editing Popup

Now that the `Main` screen is rendered, we now need to check if the `Editing` popup needs to be
rendered. Since the `ratatui` renderer simply writes over the cells within a `Rect` on a
`render_widget`, we simply need to give `render_widget` an area on top of our `Main` screen to
create the appearance of a popup.

## Popup area and title

[Section titled “Popup area and title”](#popup-area-and-title)

The first thing we will do, is draw the `Block` that will contain the popup. We will give this
`Block` a title to display as well to explain to the user what it is.

```
if let Some(editing) = &#x26;app.currently_editing {        let popup_block = Block::default()            .title("Enter a new key-value pair")            .borders(Borders::NONE)            .style(Style::default().bg(Color::DarkGray));
        let area = centered_rect(60, 25, frame.area());        frame.render_widget(popup_block, area);
```

## Popup contents

[Section titled “Popup contents”](#popup-contents)

Now that we have where our popup is going to go, we can create the layout for the popup, and create
and draw the widgets inside of it.

First, we will create split the `Rect` given to us by `centered_rect`, and create a layout from it.
Note the use of `margin(1)`, which gives a 1 space margin around any layout block, meaning our new
blocks and widgets don’t overwrite anything from the first popup block.

```
let popup_chunks = Layout::default()            .direction(Direction::Horizontal)            .margin(1)            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])            .split(area);
```

Now that we have the layout for where we want to display the keys and values, we will actually
create the blocks and paragraphs to show what the user has already entered.

```
let mut key_block = Block::default().title("Key").borders(Borders::ALL);        let mut value_block = Block::default().title("Value").borders(Borders::ALL);
        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
        match editing {            CurrentlyEditing::Key => key_block = key_block.style(active_style),            CurrentlyEditing::Value => value_block = value_block.style(active_style),        };
        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);        frame.render_widget(key_text, popup_chunks[0]);
        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);        frame.render_widget(value_text, popup_chunks[1]);    }
```

Note that we are declaring the blocks as variables, and then adding extra styling to the block the
user is currently editing. Then we create the `Paragraph` widgets, and assign the blocks with those
variables. Also note how we used the `popup_chunks` layout instead of the `popup_block` layout to
render these widgets into.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/json-editor/ui-editing.md)

 [Previous UI - Main screen](/tutorials/json-editor/ui-main/) [Next UI - Exit Popup](/tutorials/json-editor/ui-exit/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
