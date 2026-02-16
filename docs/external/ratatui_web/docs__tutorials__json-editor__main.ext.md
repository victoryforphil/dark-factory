----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/json-editor/main
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, json editor, main
- Summary: The `main` file in many ratatui applications is simply a place to store the startup loop, and
----

Source: https://ratatui.rs/tutorials/json-editor/main

# Main.rs

The `main` file in many ratatui applications is simply a place to store the startup loop, and
occasionally event handling. See more ways to handle events in
[Event Handling](/concepts/event-handling/)

In this application, we will be using our `main` function to run the startup steps, and start the
main loop. We will also put our main loop logic and event handling in this file.

## Main

[Section titled “Main”](#main)

In our main function, we will set up the terminal, create an application state and run our
application, and finally reset the terminal to the state we found it in.

### Application pre-run steps

[Section titled “Application pre-run steps”](#application-pre-run-steps)

Because a `ratatui` application takes the whole screen, and captures all of the keyboard input, we
need some boilerplate at the beginning of our `main` function.

- ``` use ratatui::crossterm::event::EnableMouseCapture;use ratatui::crossterm::execute;use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};use std::io; ``` ``` fn main() -> Result&#x3C;(), Box&#x3C;dyn Error>> { // setup terminal enable_raw_mode()?; let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?; // --snip-- ``` You might notice that we are using `stderr` for our output. This is because we want to allow the user to pipe their completed json to other programs like `ratatui-tutorial > output.json`. To do this, we are using the fact that `stderr` is piped differently than `stdout`. We render output to `stderr`, and print our completed json to `stdout`. For more information, please read the [crossterm documentation](https://docs.rs/crossterm/latest/crossterm/) ### State creation, and loop starting [Section titled “State creation, and loop starting”](#state-creation-and-loop-starting) Now that we have prepared the terminal for our application to run, it is time to actually run it. First, we need to create an instance of our `App` to hold all of the program’s state, and then we will call our function which handles the event and draw loop. ``` // --snip-- let backend = CrosstermBackend::new(stderr); let mut terminal = Terminal::new(backend)?; // create app and run it let mut app = App::new(); let res = run_app(&#x26;mut terminal, &#x26;mut app); // --snip-- ``` ### Application post-run steps [Section titled “Application post-run steps”](#application-post-run-steps) Since our `ratatui` application has changed the state of the user’s terminal with our [pre-run boilerplate](#application-pre-run-steps), we need to undo what we have done, and put the terminal back to the way we found it. Most of these functions will simply be the inverse of what we have done above. ``` use ratatui::crossterm::event::DisableMouseCapture;use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen}; ``` ``` // --snip-- // restore terminal disable_raw_mode()?; execute!( terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture )?; terminal.show_cursor()?; // --snip-- ``` When an application exits without running this closing boilerplate, the terminal will act very strange, and the user will usually have to end the terminal session and start a new one. Thus it is important that we handle our error in such a way that we can call this last piece. ``` // --snip-- if let Ok(do_print) = res { if do_print { app.print_json()?; } } else if let Err(err) = res { println!("{err:?}"); } Ok(())} ``` The if statement at the end of boilerplate checks if the `run_app` function errored, or if it returned an `Ok` state. If it returned an `Ok` state, we need to check if we should print the json. If we call our print function before we call `execute!(LeaveAlternateScreen)`, our prints will be rendered on an old screen and lost when we leave the alternate screen. (For more information on how this works, read the [Crossterm documentation](https://docs.rs/crossterm/latest/crossterm/terminal/struct.LeaveAlternateScreen.html)) So, altogether, our finished function should looks like this: ``` fn main() -> Result&#x3C;(), Box&#x3C;dyn Error>> { // setup terminal enable_raw_mode()?; let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?; let backend = CrosstermBackend::new(stderr); let mut terminal = Terminal::new(backend)?; // create app and run it let mut app = App::new(); let res = run_app(&#x26;mut terminal, &#x26;mut app); // restore terminal disable_raw_mode()?; execute!( terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture )?; terminal.show_cursor()?; if let Ok(do_print) = res { if do_print { app.print_json()?; } } else if let Err(err) = res { println!("{err:?}"); } Ok(())} ``` ## `run_app` [Section titled “run_app”](#run_app) In this function, we will start to do the actual logic. ### Method signature [Section titled “Method signature”](#method-signature) Let’s start with the method signature: ``` fn run_app&#x3C;B: Backend>(terminal: &#x26;mut Terminal&#x3C;B>, app: &#x26;mut App) -> io::Result&#x3C;bool> { // --snip-- ``` You’ll notice that we make this function generic across the `ratatui::backend::Backend`. In previous sections we hardcoded the `CrosstermBackend`. This trait approach allows us to make our code backend agnostic. This method accepts an object of type `Terminal` which implements the `ratatui::backend::Backend` trait. This trait includes the three (four counting the `TestBackend`) officially supported backends included in `ratatui`. It allows for 3rd party backends to be implemented. `run_app` also requires a mutable borrow to an application state object, as defined in this project. Finally, the `run_app` returns an `io::Result&#x3C;bool>` that indicates if there was an io error with the `Err` state, and an `Ok(true)` or `Ok(false)` that indicates if the program should print out the finished json. ### UI Loop [Section titled “UI Loop”](#ui-loop) Because `ratatui` requires us to implement our own event/ui loop, we will simply use the following code to update our main loop. ``` // --snip-- loop { terminal.draw(|f| ui(f, app))?; // --snip-- ``` Let’s unpack that `draw` call really quick. `terminal` is the `Terminal&#x3C;Backend>` that we take as an argument,

- `draw` is the `ratatui` command to draw a `Frame` to the terminal[1](#user-content-fn-note).

- `|f| ui(f, &#x26;app)` tells `draw` that we want to take `f: &#x3C;Frame>` and pass it to our function `ui`, and `ui` will draw to that `Frame`.

Notice that we also pass an immutable borrow of our application state to the `ui` function. This
will be important later.

### Event handling

[Section titled “Event handling”](#event-handling)

Now that we have started our app , and have set up the UI rendering, we will implement the event
handling.

#### Polling

[Section titled “Polling”](#polling)

Because we are using `crossterm`, we can simply poll for keyboard events with

```
if let Event::Key(key) = event::read()? {    dbg!(key.code)}
```

and then match the results.

Alternatively, we can set up a thread to run in the background to poll and send `Event`s, but let’s
keep things simple here for the sake of illustration.

Note that the process for polling events will vary on the backend you are utilizing, and you will
need to refer to the documentation of that backend for more information.

#### Main Screen

[Section titled “Main Screen”](#main-screen)

We will start with the keybinds and event handling for the `CurrentScreen::Main`.

```
// --snip--        if let Event::Key(key) = event::read()? {            if key.kind == event::KeyEventKind::Release {                // Skip events that are not KeyEventKind::Press                continue;            }            match app.current_screen {                CurrentScreen::Main => match key.code {                    KeyCode::Char('e') => {                        app.current_screen = CurrentScreen::Editing;                        app.currently_editing = Some(CurrentlyEditing::Key);                    }                    KeyCode::Char('q') => {                        app.current_screen = CurrentScreen::Exiting;                    }                    _ => {}                },                // --snip--
```

After matching to the `Main` enum variant, we match the event. When the user is in the main screen,
there are only two keybinds, and the rest are ignored.

In this case, `KeyCode::Char('e')` changes the current screen to `CurrentScreen::Editing` and sets
the `CurrentlyEditing` to a `Some` and notes that the user should be editing the `Key` value field,
as opposed to the `Value` field.

`KeyCode::Char('q')` is straightforward, as it simply switches the application to the `Exiting`
screen, and allows the ui and future event handling runs to do the rest.

#### Exiting

[Section titled “Exiting”](#exiting)

The next handler we will prepare, will handle events while the application is on the
`CurrentScreen::Exiting`. The job of this screen is to ask if the user wants to exit without
outputting the json. It is simply a `y/n` question, so that is all we listen for. We also add an
alternate exit key with `q`. If the user chooses to output the json, we return an `Ok(true)` that
indicates that our `main` function should call `app.print_json()` to perform the serialization and
printing for us after resetting the terminal to normal.

```
// --snip--                CurrentScreen::Exiting => match key.code {                    KeyCode::Char('y') => {                        return Ok(true);                    }                    KeyCode::Char('n') | KeyCode::Char('q') => {                        return Ok(false);                    }                    _ => {}                },                // --snip--
```

#### Editing

[Section titled “Editing”](#editing)

Our final handler will be a bit more involved, as we will be changing the state of internal
variables.

We would like the `Enter` key to serve two purposes. When the user is editing the `Key`, we want the
enter key to switch the focus to editing the `Value`. However, if the `Value` is what is being
currently edited, `Enter` will save the key-value pair, and return to the `Main` screen.

```
// --snip--                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {                    match key.code {                        KeyCode::Enter => {                            if let Some(editing) = &#x26;app.currently_editing {                                match editing {                                    CurrentlyEditing::Key => {                                        app.currently_editing = Some(CurrentlyEditing::Value);                                    }                                    CurrentlyEditing::Value => {                                        app.save_key_value();                                        app.current_screen = CurrentScreen::Main;                                    }                                }                            }                        }                        // --snip--
```

When `Backspace` is pressed, we need to first determine if the user is editing a `Key` or a `Value`,
then `pop()` the endings of those strings accordingly.

```
// --snip--                        KeyCode::Backspace => {                            if let Some(editing) = &#x26;app.currently_editing {                                match editing {                                    CurrentlyEditing::Key => {                                        app.key_input.pop();                                    }                                    CurrentlyEditing::Value => {                                        app.value_input.pop();                                    }                                }                            }                        }                        // --snip--
```

When `Escape` is pressed, we want to quit editing.

```
// --snip--                        KeyCode::Esc => {                            app.current_screen = CurrentScreen::Main;                            app.currently_editing = None;                        }                        // --snip--
```

When `Tab` is pressed, we want the currently editing selection to switch.

```
// --snip--                        KeyCode::Tab => {                            app.toggle_editing();                        }                        // --snip--
```

And finally, if the user types a valid character, we want to capture that, and add it to the string
that is the final key or value.

```
// --snip--                        KeyCode::Char(value) => {                            if let Some(editing) = &#x26;app.currently_editing {                                match editing {                                    CurrentlyEditing::Key => {                                        app.key_input.push(value);                                    }                                    CurrentlyEditing::Value => {                                        app.value_input.push(value);                                    }                                }                            }                        }                        // --snip--
```

Altogether, the event loop should look like this:

```
// --snip--        if let Event::Key(key) = event::read()? {            if key.kind == event::KeyEventKind::Release {                // Skip events that are not KeyEventKind::Press                continue;            }            match app.current_screen {                CurrentScreen::Main => match key.code {                    KeyCode::Char('e') => {                        app.current_screen = CurrentScreen::Editing;                        app.currently_editing = Some(CurrentlyEditing::Key);                    }                    KeyCode::Char('q') => {                        app.current_screen = CurrentScreen::Exiting;                    }                    _ => {}                },                CurrentScreen::Exiting => match key.code {                    KeyCode::Char('y') => {                        return Ok(true);                    }                    KeyCode::Char('n') | KeyCode::Char('q') => {                        return Ok(false);                    }                    _ => {}                },                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {                    match key.code {                        KeyCode::Enter => {                            if let Some(editing) = &#x26;app.currently_editing {                                match editing {                                    CurrentlyEditing::Key => {                                        app.currently_editing = Some(CurrentlyEditing::Value);                                    }                                    CurrentlyEditing::Value => {                                        app.save_key_value();                                        app.current_screen = CurrentScreen::Main;                                    }                                }                            }                        }                        KeyCode::Backspace => {                            if let Some(editing) = &#x26;app.currently_editing {                                match editing {                                    CurrentlyEditing::Key => {                                        app.key_input.pop();                                    }                                    CurrentlyEditing::Value => {                                        app.value_input.pop();                                    }                                }                            }                        }                        KeyCode::Esc => {                            app.current_screen = CurrentScreen::Main;                            app.currently_editing = None;                        }                        KeyCode::Tab => {                            app.toggle_editing();                        }                        KeyCode::Char(value) => {                            if let Some(editing) = &#x26;app.currently_editing {                                match editing {                                    CurrentlyEditing::Key => {                                        app.key_input.push(value);                                    }                                    CurrentlyEditing::Value => {                                        app.value_input.push(value);                                    }                                }                            }                        }                        _ => {}                    }                }                _ => {}            }        }        // --snip--
```

## Footnotes

[Section titled “Footnotes”](#footnote-label)

- Technically this is the command to the `Terminal&#x3C;Backend>`, but that only matters on the `TestBackend`. [↩](#user-content-fnref-note)

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/json-editor/main.md)

 [Previous App.rs](/tutorials/json-editor/app/) [Next UI.rs](/tutorials/json-editor/ui/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
