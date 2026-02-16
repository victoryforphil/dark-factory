----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/json-editor/ui-main
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, json editor, ui main
- Summary: Because we want the `Main` screen to be rendered behind the editing popup, we will draw it first,
----

Source: https://ratatui.rs/tutorials/json-editor/ui-main

# UI - Main screen

Because we want the `Main` screen to be rendered behind the editing popup, we will draw it first,
and then have additional logic about our popups

## Our layout

[Section titled “Our layout”](#our-layout)

Now that we have our `Frame`, we can actually begin drawing widgets onto it. We will begin by
creating our layout.

```
let chunks = Layout::default()        .direction(Direction::Vertical)        .constraints([            Constraint::Length(3),            Constraint::Min(1),            Constraint::Length(3),        ])        .split(frame.area());
```

The variable `chunks` now contains a length 3 array of `Rect` objects that contain the top left
corner of their space, and their size. We will use these later, after we prepare our widgets.

## The title

[Section titled “The title”](#the-title)

The title is an important piece for any application. It helps the user understand what they can do
and where they are. To create our title, we are going to use a `Paragraph` widget (which is used to
display only text), and we are going to tell that `Paragraph` we want a border all around it by
giving it a `Block` with borders enabled. See [Block recipes](/recipes/widgets/block/) and
[Paragraph recipes](/recipes/widgets/paragraph/) for more information about `Block` and `Paragraph`.

```
let title_block = Block::default()        .borders(Borders::ALL)        .style(Style::default());
    let title = Paragraph::new(Text::styled(        "Create New Json",        Style::default().fg(Color::Green),    ))    .block(title_block);
    frame.render_widget(title, chunks[0]);
```

In this code, the first thing we do, is create a `Block` with all borders enabled, and the default
style. Next, we created a paragraph widget with the text “Create New Json” styled green. See
[Paragraph recipes](/recipes/widgets/paragraph/) for more information about creating paragraphs and
[Styling text recipes](/recipes/render/style-text/) for styling text. Finally, we call
`render_widget` on our `Frame`, and give it the widget we want to render it, and the `Rect`
representing where it needs to go and what size it should be. (this is the way all widgets are
drawn)

## The list of existing pairs

[Section titled “The list of existing pairs”](#the-list-of-existing-pairs)

We would also like the user to be able to see any key-value pairs that they have already entered.
For this, we will be using another widget, the `List`. The list is what it sounds like - it creates
a new line of text for each `ListItem`, and it supports passing in a state so you can implement
selecting items on the list with little extra work. We will not be implementing selection, as we
simply want the user to be able to see what they have already entered.

```
let mut list_items = Vec::&#x3C;ListItem>::new();
    for key in app.pairs.keys() {        list_items.push(ListItem::new(Line::from(Span::styled(            format!("{: &#x3C;25} : {}", key, app.pairs.get(key).unwrap()),            Style::default().fg(Color::Yellow),        ))));    }
    let list = List::new(list_items);
    frame.render_widget(list, chunks[1]);
```

For more information on Line, Span, and Style see
[Displaying Text recipes](/recipes/render/display-text/)

In this piece of the function, we create a vector of `ListItem`s, and populate it with styled and
formatted key-value pairs. Finally, we create the `List` widget, and render it.

## The bottom navigational bar

[Section titled “The bottom navigational bar”](#the-bottom-navigational-bar)

It can help new users of your application to see hints about what keys they can press. For this, we
are going to implement two bars, and another layout. These two bars will contain information on 1)
the current screen (`Main`, `Editing`, and `Exiting`), and 2) what keybinds are available.

Here, we will create a `Vec` of `Span` which will be converted later into a single line by the
`Paragraph`. (A `Span` is different from a `Line`, because a `Span` indicates a section of `Text`
with a style applied, and doesn’t end with a newline)

```
let current_navigation_text = vec![        // The first half of the text        match app.current_screen {            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),            CurrentScreen::Editing => {                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))            }            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),        }        .to_owned(),        // A white divider bar to separate the two sections        Span::styled(" | ", Style::default().fg(Color::White)),        // The final section of the text, with hints on what the user is editing        {            if let Some(editing) = &#x26;app.currently_editing {                match editing {                    CurrentlyEditing::Key => {                        Span::styled("Editing Json Key", Style::default().fg(Color::Green))                    }                    CurrentlyEditing::Value => {                        Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))                    }                }            } else {                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))            }        },    ];
    let mode_footer = Paragraph::new(Line::from(current_navigation_text))        .block(Block::default().borders(Borders::ALL));
```

Next, we are also going to make a hint in the navigation bar with available keys. This one does not
have several sections of text with different styles, and is thus less code.

```
let current_keys_hint = {        match app.current_screen {            CurrentScreen::Main => Span::styled(                "(q) to quit / (e) to make new pair",                Style::default().fg(Color::Red),            ),            CurrentScreen::Editing => Span::styled(                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",                Style::default().fg(Color::Red),            ),            CurrentScreen::Exiting => Span::styled(                "(q) to quit / (e) to make new pair",                Style::default().fg(Color::Red),            ),        }    };
    let key_notes_footer =        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));
```

Finally, we are going to create our first nested layout. Because the `Layout.split` function
requires a `Rect`, and not a `Frame`, we can pass one of our chunks from the previous layout as the
space for the new layout. If you remember the bottom most section from the above graphic:

We will create a new layout in this space by passing it (`chunks[2]`) as the parameter for `split`.

```
let footer_chunks = Layout::default()        .direction(Direction::Horizontal)        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])        .split(chunks[2]);
```

The visual equivalent of this code is:

And now we can render our footer paragraphs in the appropriate spaces.

```
frame.render_widget(mode_footer, footer_chunks[0]);    frame.render_widget(key_notes_footer, footer_chunks[1]);
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/json-editor/ui-main.md)

 [Previous UI.rs](/tutorials/json-editor/ui/) [Next UI - Editing Popup](/tutorials/json-editor/ui-editing/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
