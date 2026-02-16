----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/json-editor/closing-thoughts
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, json editor, closing thoughts
- Summary: This tutorial should get you started with a basic understanding of the flow of a `ratatui` program.
----

Source: https://ratatui.rs/tutorials/json-editor/closing-thoughts

# Closing Thoughts

This tutorial should get you started with a basic understanding of the flow of a `ratatui` program.
However, this is only one way to create a `ratatui` application. Because `ratatui` is relatively
low level compared to other UI frameworks, almost any application model can be implemented. You can
explore more of these in
[Concepts: Application Patterns](/concepts/application-patterns/the-elm-architecture/) and get some
inspiration for what model will work best for your application.

## Finished Files

[Section titled “Finished Files”](#finished-files)

You can find the finished project used for the tutorial on
[GitHub](https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/json-editor). The code
is also shown at the bottom of this page.

You can test this application by yourself by running:

Terminal window

```
cargo run > test.json
```

and double checking the output.

### Main.rs

[Section titled “Main.rs”](#mainrs)

```
use std::{error::Error, io};
use ratatui::{    backend::{Backend, CrosstermBackend},    crossterm::{        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},        execute,        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},    },    Terminal,};
mod app;mod ui;use crate::{    app::{App, CurrentScreen, CurrentlyEditing},    ui::ui,};
fn main() -> Result&#x3C;(), Box&#x3C;dyn Error>> {    // setup terminal    enable_raw_mode()?;    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;    let backend = CrosstermBackend::new(stderr);    let mut terminal = Terminal::new(backend)?;
    // create app and run it    let mut app = App::new();    let res = run_app(&#x26;mut terminal, &#x26;mut app);
    // restore terminal    disable_raw_mode()?;    execute!(        terminal.backend_mut(),        LeaveAlternateScreen,        DisableMouseCapture    )?;    terminal.show_cursor()?;
    if let Ok(do_print) = res {        if do_print {            app.print_json()?;        }    } else if let Err(err) = res {        println!("{err:?}");    }
    Ok(())}
fn run_app&#x3C;B: Backend>(terminal: &#x26;mut Terminal&#x3C;B>, app: &#x26;mut App) -> io::Result&#x3C;bool> {    loop {        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {            if key.kind == event::KeyEventKind::Release {                // Skip events that are not KeyEventKind::Press                continue;            }            match app.current_screen {                CurrentScreen::Main => match key.code {                    KeyCode::Char('e') => {                        app.current_screen = CurrentScreen::Editing;                        app.currently_editing = Some(CurrentlyEditing::Key);                    }                    KeyCode::Char('q') => {                        app.current_screen = CurrentScreen::Exiting;                    }                    _ => {}                },                CurrentScreen::Exiting => match key.code {                    KeyCode::Char('y') => {                        return Ok(true);                    }                    KeyCode::Char('n') | KeyCode::Char('q') => {                        return Ok(false);                    }                    _ => {}                },                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {                    match key.code {                        KeyCode::Enter => {                            if let Some(editing) = &#x26;app.currently_editing {                                match editing {                                    CurrentlyEditing::Key => {                                        app.currently_editing = Some(CurrentlyEditing::Value);                                    }                                    CurrentlyEditing::Value => {                                        app.save_key_value();                                        app.current_screen = CurrentScreen::Main;                                    }                                }                            }                        }                        KeyCode::Backspace => {                            if let Some(editing) = &#x26;app.currently_editing {                                match editing {                                    CurrentlyEditing::Key => {                                        app.key_input.pop();                                    }                                    CurrentlyEditing::Value => {                                        app.value_input.pop();                                    }                                }                            }                        }                        KeyCode::Esc => {                            app.current_screen = CurrentScreen::Main;                            app.currently_editing = None;                        }                        KeyCode::Tab => {                            app.toggle_editing();                        }                        KeyCode::Char(value) => {                            if let Some(editing) = &#x26;app.currently_editing {                                match editing {                                    CurrentlyEditing::Key => {                                        app.key_input.push(value);                                    }                                    CurrentlyEditing::Value => {                                        app.value_input.push(value);                                    }                                }                            }                        }                        _ => {}                    }                }                _ => {}            }        }    }}
```

### App.rs

[Section titled “App.rs”](#apprs)

```
use std::collections::HashMap;
pub enum CurrentScreen {    Main,    Editing,    Exiting,}
pub enum CurrentlyEditing {    Key,    Value,}
pub struct App {    pub key_input: String,              // the currently being edited json key.    pub value_input: String,            // the currently being edited json value.    pub pairs: HashMap&#x3C;String, String>, // The representation of our key and value pairs with serde Serialize support    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.    pub currently_editing: Option&#x3C;CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.}
impl App {    pub fn new() -> App {        App {            key_input: String::new(),            value_input: String::new(),            pairs: HashMap::new(),            current_screen: CurrentScreen::Main,            currently_editing: None,        }    }
    pub fn save_key_value(&#x26;mut self) {        self.pairs            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();        self.value_input = String::new();        self.currently_editing = None;    }
    pub fn toggle_editing(&#x26;mut self) {        if let Some(edit_mode) = &#x26;self.currently_editing {            match edit_mode {                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),            };        } else {            self.currently_editing = Some(CurrentlyEditing::Key);        }    }
    pub fn print_json(&#x26;self) -> serde_json::Result&#x3C;()> {        let output = serde_json::to_string(&#x26;self.pairs)?;        println!("{output}");        Ok(())    }}
```

### UI.rs

[Section titled “UI.rs”](#uirs)

```
use ratatui::{    layout::{Constraint, Direction, Layout, Rect},    style::{Color, Style},    text::{Line, Span, Text},    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},    Frame,};
use crate::app::{App, CurrentScreen, CurrentlyEditing};
pub fn ui(frame: &#x26;mut Frame, app: &#x26;App) {    // Create the layout sections.    let chunks = Layout::default()        .direction(Direction::Vertical)        .constraints([            Constraint::Length(3),            Constraint::Min(1),            Constraint::Length(3),        ])        .split(frame.area());
    let title_block = Block::default()        .borders(Borders::ALL)        .style(Style::default());
    let title = Paragraph::new(Text::styled(        "Create New Json",        Style::default().fg(Color::Green),    ))    .block(title_block);
    frame.render_widget(title, chunks[0]);    let mut list_items = Vec::&#x3C;ListItem>::new();
    for key in app.pairs.keys() {        list_items.push(ListItem::new(Line::from(Span::styled(            format!("{: &#x3C;25} : {}", key, app.pairs.get(key).unwrap()),            Style::default().fg(Color::Yellow),        ))));    }
    let list = List::new(list_items);
    frame.render_widget(list, chunks[1]);    let current_navigation_text = vec![        // The first half of the text        match app.current_screen {            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),            CurrentScreen::Editing => {                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))            }            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),        }        .to_owned(),        // A white divider bar to separate the two sections        Span::styled(" | ", Style::default().fg(Color::White)),        // The final section of the text, with hints on what the user is editing        {            if let Some(editing) = &#x26;app.currently_editing {                match editing {                    CurrentlyEditing::Key => {                        Span::styled("Editing Json Key", Style::default().fg(Color::Green))                    }                    CurrentlyEditing::Value => {                        Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))                    }                }            } else {                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))            }        },    ];
    let mode_footer = Paragraph::new(Line::from(current_navigation_text))        .block(Block::default().borders(Borders::ALL));
    let current_keys_hint = {        match app.current_screen {            CurrentScreen::Main => Span::styled(                "(q) to quit / (e) to make new pair",                Style::default().fg(Color::Red),            ),            CurrentScreen::Editing => Span::styled(                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",                Style::default().fg(Color::Red),            ),            CurrentScreen::Exiting => Span::styled(                "(q) to quit / (e) to make new pair",                Style::default().fg(Color::Red),            ),        }    };
    let key_notes_footer =        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));
    let footer_chunks = Layout::default()        .direction(Direction::Horizontal)        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])        .split(chunks[2]);
    frame.render_widget(mode_footer, footer_chunks[0]);    frame.render_widget(key_notes_footer, footer_chunks[1]);
    if let Some(editing) = &#x26;app.currently_editing {        let popup_block = Block::default()            .title("Enter a new key-value pair")            .borders(Borders::NONE)            .style(Style::default().bg(Color::DarkGray));
        let area = centered_rect(60, 25, frame.area());        frame.render_widget(popup_block, area);
        let popup_chunks = Layout::default()            .direction(Direction::Horizontal)            .margin(1)            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])            .split(area);
        let mut key_block = Block::default().title("Key").borders(Borders::ALL);        let mut value_block = Block::default().title("Value").borders(Borders::ALL);
        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
        match editing {            CurrentlyEditing::Key => key_block = key_block.style(active_style),            CurrentlyEditing::Value => value_block = value_block.style(active_style),        };
        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);        frame.render_widget(key_text, popup_chunks[0]);
        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);        frame.render_widget(value_text, popup_chunks[1]);    }
    if let CurrentScreen::Exiting = app.current_screen {        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn        let popup_block = Block::default()            .title("Y/N")            .borders(Borders::NONE)            .style(Style::default().bg(Color::DarkGray));
        let exit_text = Text::styled(            "Would you like to output the buffer as json? (y/n)",            Style::default().fg(Color::Red),        );        // the `trim: false` will stop the text from being cut off when over the edge of the block        let exit_paragraph = Paragraph::new(exit_text)            .block(popup_block)            .wrap(Wrap { trim: false });
        let area = centered_rect(60, 25, frame.area());        frame.render_widget(exit_paragraph, area);    }}
/// helper function to create a centered rect using up certain percentage of the available rect `r`fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {    // Cut the given rectangle into three vertical pieces    let popup_layout = Layout::default()        .direction(Direction::Vertical)        .constraints([            Constraint::Percentage((100 - percent_y) / 2),            Constraint::Percentage(percent_y),            Constraint::Percentage((100 - percent_y) / 2),        ])        .split(r);
    // Then cut the middle vertical piece into three width-wise pieces    Layout::default()        .direction(Direction::Horizontal)        .constraints([            Constraint::Percentage((100 - percent_x) / 2),            Constraint::Percentage(percent_x),            Constraint::Percentage((100 - percent_x) / 2),        ])        .split(popup_layout[1])[1] // Return the middle chunk}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/json-editor/closing-thoughts.md)

 [Previous UI - Exit Popup](/tutorials/json-editor/ui-exit/) [Next Videos](/tutorials/videos/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
