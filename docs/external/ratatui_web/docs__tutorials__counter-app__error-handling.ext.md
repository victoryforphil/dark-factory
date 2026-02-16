----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/counter-app/error-handling
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, counter app, error handling
- Summary: This tutorial is outdated since Ratatui 0.28.1.
----

Source: https://ratatui.rs/tutorials/counter-app/error-handling

# Counter App Error Handling

Caution

This tutorial is outdated since Ratatui 0.28.1.

We introduced the `ratatui::init` and `ratatui::restore` methods in Ratatui 0.28.1, which
automatically setup panic hooks. That removes the need for the `tui` module here. We will update the
tutorial soon.

Source Code

Full source code is available at:
[https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/counter-app-error-handling](https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/counter-app-error-handling).

In the previous section, you created a [basic counter app](../basic-app/) that responds to the user
pressing the Left and Right arrow keys to control the value of a counter. This tutorial will
start with that code and add error and panic handling.

A quick reminder of where we left off in the basic app:

Cargo.toml (click to expand)

```
# -- snip --
[dependencies]ratatui = "0.30.0"crossterm = "0.29.0"
```

main.rs (click to expand)

```
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};use ratatui::{    buffer::Buffer,    layout::Rect,    style::Stylize,    symbols::border,    text::{Line, Text},    widgets::{Block, Paragraph, Widget},    DefaultTerminal, Frame,};
fn main() -> io::Result&#x3C;()> {    ratatui::run(|terminal| App::default().run(terminal))}
#[derive(Debug, Default)]pub struct App {    counter: u8,    exit: bool,}
impl App {    /// runs the application's main loop until the user quits    pub fn run(&#x26;mut self, terminal: &#x26;mut DefaultTerminal) -> io::Result&#x3C;()> {        while !self.exit {            terminal.draw(|frame| self.draw(frame))?;            self.handle_events()?;        }        Ok(())    }
    fn draw(&#x26;self, frame: &#x26;mut Frame) {        frame.render_widget(self, frame.area());    }
    /// updates the application's state based on user input    fn handle_events(&#x26;mut self) -> io::Result&#x3C;()> {        match event::read()? {            // it's important to check that the event is a key press event as            // crossterm also emits key release and repeat events on Windows.            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {                self.handle_key_event(key_event)            }            _ => {}        };        Ok(())    }
    fn handle_key_event(&#x26;mut self, key_event: KeyEvent) {        match key_event.code {            KeyCode::Char('q') => self.exit(),            KeyCode::Left => self.decrement_counter(),            KeyCode::Right => self.increment_counter(),            _ => {}        }    }
    fn exit(&#x26;mut self) {        self.exit = true;    }
    fn increment_counter(&#x26;mut self) {        self.counter += 1;    }
    fn decrement_counter(&#x26;mut self) {        self.counter -= 1;    }}
impl Widget for &#x26;App {    fn render(self, area: Rect, buf: &#x26;mut Buffer) {        let title = Line::from(" Counter App Tutorial ".bold());        let instructions = Line::from(vec![            " Decrement ".into(),            "&#x3C;Left>".blue().bold(),            " Increment ".into(),            "&#x3C;Right>".blue().bold(),            " Quit ".into(),            "&#x3C;Q> ".blue().bold(),        ]);        let block = Block::bordered()            .title(title.centered())            .title_bottom(instructions.centered())            .border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec![            "Value: ".into(),            self.counter.to_string().yellow(),        ])]);
        Paragraph::new(counter_text)            .centered()            .block(block)            .render(area, buf);    }}
#[cfg(test)]mod tests {
    use super::*;    use ratatui::style::Style;
    #[test]    fn render() {        let app = App::default();        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
        app.render(buf.area, &#x26;mut buf);
        let mut expected = Buffer::with_lines(vec![            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",            "┃                    Value: 0                    ┃",            "┃                                                ┃",            "┗━ Decrement &#x3C;Left> Increment &#x3C;Right> Quit &#x3C;Q> ━━┛",        ]);        let title_style = Style::new().bold();        let counter_style = Style::new().yellow();        let key_style = Style::new().blue().bold();        expected.set_style(Rect::new(14, 0, 22, 1), title_style);        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);        expected.set_style(Rect::new(13, 3, 6, 1), key_style);        expected.set_style(Rect::new(30, 3, 7, 1), key_style);        expected.set_style(Rect::new(43, 3, 4, 1), key_style);
        assert_eq!(buf, expected);    }
    #[test]    fn handle_key_event() {        let mut app = App::default();        app.handle_key_event(KeyCode::Right.into());        assert_eq!(app.counter, 1);
        app.handle_key_event(KeyCode::Left.into());        assert_eq!(app.counter, 0);
        let mut app = App::default();        app.handle_key_event(KeyCode::Char('q').into());        assert!(app.exit);    }}
```

## The problem

[Section titled “The problem”](#the-problem)

The app you built in the previous section has an intentional error in that causes the app to panic
when the user presses the Left arrow key when the Counter is already at 0. When this happens,
the main function does not have a chance to restore the terminal state before it exits.

src/main.rs (from basic app)

```
fn main() -> io::Result&#x3C;()> {    ratatui::run(|terminal| App::default().run(terminal))}
```

The application’s default panic handler runs and displays the details messed up. This is because raw
mode stops the terminal from interpreting newlines in the usual way. The shell prompt is also
rendered at the wrong place.

To recover from this, on a macOS or Linux console, run the `reset` command. On a Windows console you
may need to restart the console.

## Setup Hooks

[Section titled “Setup Hooks”](#setup-hooks)

There are two ways that a rust application can fail. The rust book chapter on [error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
explains this in better detail.

Rust groups errors into two major categories: recoverable and unrecoverable errors. For a
recoverable error, such as a file not found error, we most likely just want to report the
problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs,
like trying to access a location beyond the end of an array, and so we want to immediately stop
the program. — [https://doc.rust-lang.org/book/ch09-00-error-handling.html](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

One approach that makes it easy to show unhandled errors is to use the [color-eyre](https://crates.io/crates/color-eyre) crate to augment
the error reporting hooks. In a ratatui application that’s running on the [alternate screen](/concepts/backends/alternate-screen/) in [raw mode](/concepts/backends/raw-mode/), it’s important to restore the terminal before displaying these errors to the user.

Add the `color-eyre` crate

add color-eyre

```
cargo add color-eyre
```

Update the `main` function’s return value to [`color_eyre::Result&#x3C;()>`](https://docs.rs/eyre/latest/eyre/type.Result.html) and call the the
[`color_eyre::install`](https://docs.rs/color-eyre/latest/color_eyre/fn.install.html) function. We can also add an error message that helps your app user
understand what to do if restoring the terminal does fail.

main.rs

```
use color_eyre::{    eyre::{bail, WrapErr},    Result,};
fn main() -> Result&#x3C;()> {    color_eyre::install()?;    let mut terminal = tui::init()?;    let app_result = App::default().run(&#x26;mut terminal);    if let Err(err) = tui::restore() {        eprintln!(            "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}"        );    }    app_result}
```

Next, update the `tui::init()` function to replace the panic hook with one that first restores the
terminal before printing the panic information. This will ensure that both panics and unhandled
errors (i.e. any `Result::Err`s that bubble up to the top level of the main function) are both
displayed on the terminal correctly when the application exits.

tui.rs

```
/// Initialize the terminalpub fn init() -> io::Result&#x3C;Tui> {    execute!(stdout(), EnterAlternateScreen)?;    enable_raw_mode()?;    set_panic_hook();    Terminal::new(CrosstermBackend::new(stdout()))}
fn set_panic_hook() {    let hook = std::panic::take_hook();    std::panic::set_hook(Box::new(move |panic_info| {        let _ = restore(); // ignore any errors as we are already failing        hook(panic_info);    }));}
```

## Using color_eyre

[Section titled “Using color_eyre”](#using-color_eyre)

Color eyre works by adding extra information to Results. You can add context to the errors by
calling `wrap_err` (defined on the `color_eyre::eyre::WrapErr` trait).

Update the `App::run` function to add some information about the update function failing and change
the return value.

main.rs

```
impl App {    /// runs the application's main loop until the user quits    pub fn run(&#x26;mut self, terminal: &#x26;mut tui::Tui) -> Result&#x3C;()> {        while !self.exit {            terminal.draw(|frame| self.render_frame(frame))?;            self.handle_events().wrap_err("handle events failed")?;        }        Ok(())    }}
```

Tip

[Good Rust API error messages](https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err) are generally lower case, without trailing punctuation and generally
concise. Your app might choose to provide more detail than this convention as the errors are usually
user-facing instead of developer-facing.

## Creating a recoverable error

[Section titled “Creating a recoverable error”](#creating-a-recoverable-error)

The tutorial needs a synthetic error to show how we can handle recoverable errors. Change
`handle_key_event` to return a `color_eyre::Result` and make sure the calls to increment and
decrement calls have the `?` operator to propagate the error to the caller.

main.rs

```
impl App {    fn handle_key_event(&#x26;mut self, key_event: KeyEvent) -> Result&#x3C;()> {        match key_event.code {            KeyCode::Char('q') => self.exit(),            KeyCode::Left => self.decrement_counter()?,            KeyCode::Right => self.increment_counter()?,            _ => {}        }        Ok(())    }}
```

Let’s add an error that occurs when the counter is above 2. Also change both methods’ return types.
Add the new error to the `increment_counter` method. You can use the `bail!` macro for this:

main.rs

```
impl App {    fn decrement_counter(&#x26;mut self) -> Result&#x3C;()> {        self.counter -= 1;        Ok(())    }
    fn increment_counter(&#x26;mut self) -> Result&#x3C;()> {        self.counter += 1;        if self.counter > 2 {            bail!("counter overflow");        }        Ok(())    }}
```

In the `handle_events` method, add some extra information about which key caused the failure and
update the return value.

main.rs

```
impl App {    /// updates the application's state based on user input    fn handle_events(&#x26;mut self) -> Result&#x3C;()> {        match event::read()? {            // it's important to check that the event is a key press event as            // crossterm also emits key release and repeat events on Windows.            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self                .handle_key_event(key_event)                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),            _ => Ok(()),        }    }}
```

Update the tests for this method to unwrap the calls to handle_key_events. This will cause the test
to fail if an error is returned.

main.rs

```
mod tests {    #[test]    fn handle_key_event() {        let mut app = App::default();        app.handle_key_event(KeyCode::Right.into()).unwrap();        assert_eq!(app.counter, 1);
        app.handle_key_event(KeyCode::Left.into()).unwrap();        assert_eq!(app.counter, 0);
        let mut app = App::default();        app.handle_key_event(KeyCode::Char('q').into()).unwrap();        assert!(app.exit);    }}
```

Add tests for the panic and overflow conditions

main.rs

```
mod tests {    #[test]    #[should_panic(expected = "attempt to subtract with overflow")]    fn handle_key_event_panic() {        let mut app = App::default();        let _ = app.handle_key_event(KeyCode::Left.into());    }
    #[test]    fn handle_key_event_overflow() {        let mut app = App::default();        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());        assert_eq!(            app.handle_key_event(KeyCode::Right.into())                .unwrap_err()                .to_string(),            "counter overflow"        );    }}
```

Run the tests:

run tests

```
cargo test
```

```
running 4 teststhread 'tests::handle_key_event_panic' panicked at code/counter-app-error-handling/src/main.rs:94:9:attempt to subtract with overflowtest tests::handle_key_event ... okstack backtrace:
test tests::handle_key_event_overflow ... oktest tests::render ... ok20 collapsed lines   0: rust_begin_unwind             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/std/src/panicking.rs:645:5   1: core::panicking::panic_fmt             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:72:14   2: core::panicking::panic             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:144:5   3: counter_app_error_handling::App::decrement_counter             at ./src/main.rs:94:9   4: counter_app_error_handling::App::handle_key_event             at ./src/main.rs:79:30   5: counter_app_error_handling::tests::handle_key_event_panic             at ./src/main.rs:200:17   6: counter_app_error_handling::tests::handle_key_event_panic::{{closure}}             at ./src/main.rs:198:32   7: core::ops::function::FnOnce::call_once             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/ops/function.rs:250:5   8: core::ops::function::FnOnce::call_once             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/ops/function.rs:250:5note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.test tests::handle_key_event_panic - should panic ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## The Finished App

[Section titled “The Finished App”](#the-finished-app)

Putting this altogether, you should now have the following files.

main.rs (click to expand)

```
use color_eyre::{    eyre::{bail, WrapErr},    Result,};use ratatui::{    buffer::Buffer,    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},    layout::Rect,    style::Stylize,    symbols::border,    text::{Line, Text},    widgets::{Block, Borders, Paragraph, Widget},    Frame,};
mod tui;
fn main() -> Result&#x3C;()> {    color_eyre::install()?;    let mut terminal = tui::init()?;    let app_result = App::default().run(&#x26;mut terminal);    if let Err(err) = tui::restore() {        eprintln!(            "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}"        );    }    app_result}
#[derive(Debug, Default)]pub struct App {    counter: u8,    exit: bool,}
impl App {    /// runs the application's main loop until the user quits    pub fn run(&#x26;mut self, terminal: &#x26;mut tui::Tui) -> Result&#x3C;()> {        while !self.exit {            terminal.draw(|frame| self.render_frame(frame))?;            self.handle_events().wrap_err("handle events failed")?;        }        Ok(())    }
    fn render_frame(&#x26;self, frame: &#x26;mut Frame) {        frame.render_widget(self, frame.area());    }
    /// updates the application's state based on user input    fn handle_events(&#x26;mut self) -> Result&#x3C;()> {        match event::read()? {            // it's important to check that the event is a key press event as            // crossterm also emits key release and repeat events on Windows.            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self                .handle_key_event(key_event)                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),            _ => Ok(()),        }    }
    fn handle_key_event(&#x26;mut self, key_event: KeyEvent) -> Result&#x3C;()> {        match key_event.code {            KeyCode::Char('q') => self.exit(),            KeyCode::Left => self.decrement_counter()?,            KeyCode::Right => self.increment_counter()?,            _ => {}        }        Ok(())    }
    fn exit(&#x26;mut self) {        self.exit = true;    }
    fn decrement_counter(&#x26;mut self) -> Result&#x3C;()> {        self.counter -= 1;        Ok(())    }
    fn increment_counter(&#x26;mut self) -> Result&#x3C;()> {        self.counter += 1;        if self.counter > 2 {            bail!("counter overflow");        }        Ok(())    }}
impl Widget for &#x26;App {    fn render(self, area: Rect, buf: &#x26;mut Buffer) {        let title = Line::from(" Counter App Tutorial ".bold());        let instructions = Line::from(vec![            " Decrement ".into(),            "&#x3C;Left>".blue().bold(),            " Increment ".into(),            "&#x3C;Right>".blue().bold(),            " Quit ".into(),            "&#x3C;Q> ".blue().bold(),        ]);        let block = Block::default()            .title(title.centered())            .title_bottom(instructions.centered())            .borders(Borders::ALL)            .border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec![            "Value: ".into(),            self.counter.to_string().yellow(),        ])]);
        Paragraph::new(counter_text)            .centered()            .block(block)            .render(area, buf);    }}
#[cfg(test)]mod tests {    use ratatui::style::Style;
    use super::*;
    #[test]    fn render() {        let app = App::default();        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
        app.render(buf.area, &#x26;mut buf);
        let mut expected = Buffer::with_lines(vec![            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",            "┃                    Value: 0                    ┃",            "┃                                                ┃",            "┗━ Decrement &#x3C;Left> Increment &#x3C;Right> Quit &#x3C;Q> ━━┛",        ]);        let title_style = Style::new().bold();        let counter_style = Style::new().yellow();        let key_style = Style::new().blue().bold();        expected.set_style(Rect::new(14, 0, 22, 1), title_style);        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);        expected.set_style(Rect::new(13, 3, 6, 1), key_style);        expected.set_style(Rect::new(30, 3, 7, 1), key_style);        expected.set_style(Rect::new(43, 3, 4, 1), key_style);
        assert_eq!(buf, expected);    }
    #[test]    fn handle_key_event() {        let mut app = App::default();        app.handle_key_event(KeyCode::Right.into()).unwrap();        assert_eq!(app.counter, 1);
        app.handle_key_event(KeyCode::Left.into()).unwrap();        assert_eq!(app.counter, 0);
        let mut app = App::default();        app.handle_key_event(KeyCode::Char('q').into()).unwrap();        assert!(app.exit);    }
    #[test]    #[should_panic(expected = "attempt to subtract with overflow")]    fn handle_key_event_panic() {        let mut app = App::default();        let _ = app.handle_key_event(KeyCode::Left.into());    }
    #[test]    fn handle_key_event_overflow() {        let mut app = App::default();        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());        assert_eq!(            app.handle_key_event(KeyCode::Right.into())                .unwrap_err()                .to_string(),            "counter overflow"        );    }}
```

tui.rs (click to expand)

```
use std::io::{self, stdout, Stdout};
use ratatui::{    backend::CrosstermBackend,    crossterm::{        execute,        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},    },    Terminal,};
/// A type alias for the terminal type used in this applicationpub type Tui = Terminal&#x3C;CrosstermBackend&#x3C;Stdout>>;
/// Initialize the terminalpub fn init() -> io::Result&#x3C;Tui> {    execute!(stdout(), EnterAlternateScreen)?;    enable_raw_mode()?;    set_panic_hook();    Terminal::new(CrosstermBackend::new(stdout()))}
fn set_panic_hook() {    let hook = std::panic::take_hook();    std::panic::set_hook(Box::new(move |panic_info| {        let _ = restore(); // ignore any errors as we are already failing        hook(panic_info);    }));}
/// Restore the terminal to its original statepub fn restore() -> io::Result&#x3C;()> {    execute!(stdout(), LeaveAlternateScreen)?;    disable_raw_mode()?;    Ok(())}
```

## Handling Panics

[Section titled “Handling Panics”](#handling-panics)

Experiment to see what happens when the application panics. The application has an intentional bug
where it uses `u8` for the counter field, but doesn’t guard against decrementing this below 0. Run
the app and press the Left arrow key.

To get more information about where the error occurred, add `RUST_BACKTRACE=full` before the
command.

## Handling Errors

[Section titled “Handling Errors”](#handling-errors)

Experiment to see what happens when the application returns an unhandled error as a result. The app
will cause this to happen when the counter increases past 2. Run the app and press the Right arrow 3
times.

To get more information about where the error occurred, add `RUST_BACKTRACE=full` before the
command.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/counter-app/error-handling.md)

 [Previous Basic App](/tutorials/counter-app/basic-app/) [Next JSON Editor](/tutorials/json-editor/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
