----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/apps/color-eyre
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, apps, color eyre
- Summary: Full source code is available at:
----

Source: https://ratatui.rs/recipes/apps/color-eyre

# Use `color_eyre` with Ratatui

Source Code

Full source code is available at:
[https://github.com/ratatui/ratatui-website/tree/main/code/how-to-color_eyre/](https://github.com/ratatui/ratatui-website/tree/main/code/how-to-color_eyre/)

The [`color_eyre`](https://crates.io/crates/color-eyre) crate provides error report handlers for panics and errors. It displays the
reports formatted and in color. To use these handlers, a Ratatui app needs to restore the terminal
before displaying the errors.

## Installation

[Section titled “Installation”](#installation)

First add the crate to your `Cargo.toml`

- add color_eyre to Cargo.toml ``` cargo add color_eyre ``` Call the [`color_eyre::install`](https://docs.rs/color-eyre/latest/color_eyre/fn.install.html) method from your main function and update the return value to [`color_eyre::Result&#x3C;()>`](https://docs.rs/eyre/latest/eyre/type.Result.html). main.rs ``` fn main() -> color_eyre::Result&#x3C;()> { color_eyre::install()?;9 collapsed lines let terminal = tui::init()?; let result = run(terminal).wrap_err("run failed"); if let Err(err) = tui::restore() { eprintln!( "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}" ); } result} ``` In your terminal initialization function, add some new code that replaces rusts default panic handler with one that restores the terminal before displaying the panic details. This will be used by both panics and unhandled errors that fall through to the end of the program. tui.rs ``` /// Initialize the terminalpub fn init() -> io::Result&#x3C;ratatui::Terminal&#x3C;CrosstermBackend&#x3C;Stdout>>> { execute!(stdout(), EnterAlternateScreen)?; enable_raw_mode()?; set_panic_hook(); Terminal::new(CrosstermBackend::new(stdout()))} fn set_panic_hook() { let hook = std::panic::take_hook(); std::panic::set_hook(Box::new(move |panic_info| { let _ = restore(); // ignore any errors as we are already failing hook(panic_info); }));} ``` ## Usage [Section titled “Usage”](#usage) In your application, wrap errors with extra context as needed: Add the following import: main.rs ``` use color_eyre::eyre::WrapErr; ``` Call wrap_err from methods that can fail with an error. main.rs ``` fn main() -> color_eyre::Result&#x3C;()> { color_eyre::install()?; let terminal = tui::init()?; let result = run(terminal).wrap_err("run failed"); if let Err(err) = tui::restore() { eprintln!( "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}" ); } result} ``` ## Demo [Section titled “Demo”](#demo) Full code main.rs ``` use std::panic; use color_eyre::eyre::WrapErr;use color_eyre::eyre::bail;use ratatui::{ backend::Backend, crossterm::event::{self, Event, KeyCode, KeyEvent}, widgets::Paragraph, Terminal,}; mod tui; fn main() -> color_eyre::Result&#x3C;()> { color_eyre::install()?; let terminal = tui::init()?; let result = run(terminal).wrap_err("run failed"); if let Err(err) = tui::restore() { eprintln!( "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}" ); } result} fn run(mut terminal: Terminal&#x3C;impl Backend>) -> color_eyre::Result&#x3C;()> { loop { terminal.draw(|frame| { let message = "Press &#x3C;Q> to quit, &#x3C;P> to panic, or &#x3C;E> to error"; frame.render_widget(Paragraph::new(message), frame.area()); })?; match event::read()? { Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => break, Event::Key(KeyEvent { code: KeyCode::Char('p'), .. }) => panic!("User triggered panic"), Event::Key(KeyEvent { code: KeyCode::Char('e'), .. }) => bail!("user triggered error"), _ => {} } } Ok(())} ``` tui.rs ``` use std::io::{self, stdout, Stdout}; use ratatui::{ backend::CrosstermBackend, crossterm::{ execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, }, Terminal,}; /// Initialize the terminalpub fn init() -> io::Result&#x3C;ratatui::Terminal&#x3C;CrosstermBackend&#x3C;Stdout>>> { execute!(stdout(), EnterAlternateScreen)?; enable_raw_mode()?; set_panic_hook(); Terminal::new(CrosstermBackend::new(stdout()))} fn set_panic_hook() { let hook = std::panic::take_hook(); std::panic::set_hook(Box::new(move |panic_info| { let _ = restore(); // ignore any errors as we are already failing hook(panic_info); }));} /// Restore the terminal to its original statepub fn restore() -> io::Result&#x3C;()> { execute!(stdout(), LeaveAlternateScreen)?; disable_raw_mode()?; Ok(())} ``` ### Panic [Section titled “Panic”](#panic) With `RUST_BACKTRACE=full`: ### Error [Section titled “Error”](#error) With `RUST_BACKTRACE=full`: ### Normal exit [Section titled “Normal exit”](#normal-exit) ## Further Steps [Section titled “Further Steps”](#further-steps) See the `color_eyre` [docs](https://docs.rs/color_eyre/latest/color_eyre) and [examples](https://github.com/eyre-rs/eyre/blob/master/color-eyre/examples/) for more advanced setups. E.g.: [Capturing span traces](https://github.com/eyre-rs/eyre/blob/master/color-eyre/examples/usage.rs)

- [Configuring an automatic issue url](https://github.com/eyre-rs/eyre/blob/master/color-eyre/examples/github_issue.rs)

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/apps/color-eyre.md)

 [Previous Setup Panic Hooks](/recipes/apps/panic-hooks/) [Next Better Panic Hooks](/recipes/apps/better-panic/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
