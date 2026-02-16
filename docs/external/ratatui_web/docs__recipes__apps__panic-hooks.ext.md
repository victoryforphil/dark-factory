----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/apps/panic-hooks
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, apps, panic hooks
- Summary: When building TUIs with `ratatui`, it’s vital to ensure that if your application encounters a panic,
----

Source: https://ratatui.rs/recipes/apps/panic-hooks

# Setup Panic Hooks

When building TUIs with `ratatui`, it’s vital to ensure that if your application encounters a panic,
it gracefully returns to the original terminal state. This prevents the terminal from getting stuck
in a modified state, which can be quite disruptive for users.

The rust standard library allows applications to setup a panic hook that runs whenever a panic
occurs. Ratatui applications should use this to disable raw mode and return the main screen.

Given the following application that panics after a 1 second delay as a basis, we can implement the
hooks for each backend.

- main.rs ``` pub fn main() -> io::Result&#x3C;()> { init_panic_hook(); let mut tui = init_tui()?; tui.draw(|frame| frame.render_widget(Span::from("Hello, world!"), frame.area()))?; sleep(Duration::from_secs(1)); panic!("This is a panic!");} ``` ## Crossterm [Section titled “Crossterm”](#crossterm) Restoring the terminal state in an app that uses the `CrosstermBackend` is pretty simple. The `init_panic_hook` method saves a copy of the current hook, and then sets up a new hook that restores the terminal to the original state before calling the original hook. It’s important to avoid panicking while restoring the terminal state, otherwise the original panic reason might be lost. In your own app, this might be supplemented with logging to a file or similar. main.rs ``` 26 collapsed linesuse std::{ io::{self, stdout}, panic::{set_hook, take_hook}, thread::sleep, time::Duration,}; use ratatui::{ backend::{Backend, CrosstermBackend}, crossterm::{ execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, }, text::Span, Terminal,}; pub fn main() -> io::Result&#x3C;()> { init_panic_hook(); let mut tui = init_tui()?; tui.draw(|frame| frame.render_widget(Span::from("Hello, world!"), frame.area()))?; sleep(Duration::from_secs(1)); panic!("This is a panic!");} pub fn init_panic_hook() { let original_hook = take_hook(); set_hook(Box::new(move |panic_info| { // intentionally ignore errors here since we're already in a panic let _ = restore_tui(); original_hook(panic_info); }));} pub fn init_tui() -> io::Result&#x3C;Terminal&#x3C;impl Backend>> { enable_raw_mode()?; execute!(stdout(), EnterAlternateScreen)?; Terminal::new(CrosstermBackend::new(stdout()))} pub fn restore_tui() -> io::Result&#x3C;()> { disable_raw_mode()?; execute!(stdout(), LeaveAlternateScreen)?; Ok(())} ``` ## Termion [Section titled “Termion”](#termion) Termion requires a bit more effort, as the code for enabling and disabling raw mode is only available on the `RawTerminal` type. The type stores a copy of the terminal state when constructed and then restores that when dropped. It has a `suspend_raw_mode` function that temporarily restores the terminal state. To make it possible for the `init_tui` method to see the terminal in a cooked state (the opposite of raw), the `init_panic_hook` method needs to create a `RawTerminal` which will be used in the panic hook, and immediately suspend raw mode. Termion provides a similar wrapper type for the alternate screen, but this type doesn’t implement a method to leave the alternate screen except when dropped. Apps should use `ToAlternateScreen` / `ToMainScreen` instead of the `IntoAlternateScreen` wrapper. Also make sure to call `stdout().flush`, to make this change take effect. main.rs ``` 23 collapsed linesuse std::{ io::{self, stdout, Write}, panic::{set_hook, take_hook}, thread::sleep, time::Duration,}; use ratatui::{ backend::{Backend, TermionBackend}, termion::{ raw::IntoRawMode, screen::{ToAlternateScreen, ToMainScreen}, }, text::Span, Terminal,}; pub fn main() -> io::Result&#x3C;()> { init_panic_hook()?; let mut tui = init_tui()?; tui.draw(|frame| frame.render_widget(Span::from("Hello, world!"), frame.area()))?; sleep(Duration::from_secs(1)); panic!("This is a panic!");} pub fn init_panic_hook() -> io::Result&#x3C;()> { let raw_output = stdout().into_raw_mode()?; raw_output.suspend_raw_mode()?; let original_hook = take_hook(); set_hook(Box::new(move |panic_info| { // intentionally ignore errors here since we're already in a panic let _ = raw_output.suspend_raw_mode(); let _ = restore_tui(); original_hook(panic_info); })); Ok(())} pub fn init_tui() -> io::Result&#x3C;Terminal&#x3C;impl Backend>> { let mut stdout = stdout().into_raw_mode()?; write!(stdout, "{}", ToAlternateScreen)?; stdout.flush()?; Terminal::new(TermionBackend::new(stdout))} pub fn restore_tui() -> io::Result&#x3C;()> { write!(stdout(), "{}", ToMainScreen)?; stdout().flush()?; Ok(())} ``` For more discussion on this, see: [https://github.com/ratatui/ratatui/issues/1005](https://github.com/ratatui/ratatui/issues/1005)

- [https://gitlab.redox-os.org/redox-os/termion/-/issues/176](https://gitlab.redox-os.org/redox-os/termion/-/issues/176)

## Termwiz

[Section titled “Termwiz”](#termwiz)

Termwiz is a little more difficult as the methods to disable raw mode and exit the alternate screen
require mutable access to the terminal instance.

```
// TODO
```

## Conclusion

[Section titled “Conclusion”](#conclusion)

As a general rule, you want to take the original panic hook and execute it after cleaning up the
terminal. In the next sections we will discuss some third party packages that can help give better
output for handling errors and panics.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/apps/panic-hooks.md)

 [Previous Terminal and Event Handler](/recipes/apps/terminal-and-event-handler/) [Next color_eyre Error Hooks](/recipes/apps/color-eyre/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
