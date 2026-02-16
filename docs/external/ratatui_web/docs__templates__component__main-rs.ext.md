----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /templates/component/main-rs
- Keywords: ratatui, rust, tui, terminal ui, docs, templates, component, main rs
- Summary: In this section, let’s just cover the contents of `main.rs`, `build.rs` and `utils.rs`.
----

Source: https://ratatui.rs/templates/component/main-rs

# Main.rs

In this section, let’s just cover the contents of `main.rs`, `build.rs` and `utils.rs`.

The `main.rs` file is the entry point of the application. Here’s the complete `main.rs` file:

- ``` use clap::Parser;use cli::Cli;use color_eyre::Result; use crate::app::App; mod action;mod app;mod cli;mod components;mod config;mod errors;mod logging;mod tui; #[tokio::main]async fn main() -> Result&#x3C;()> { crate::errors::init()?; crate::logging::init()?; let args = Cli::parse(); let mut app = App::new(args.tick_rate, args.frame_rate)?; app.run().await?; Ok(())} ``` In essence, the `main` function creates an instance of `App` and calls `App.run()`, which runs the “`handle event` -> `update state` -> `draw`” loop. We will talk more about this in a later section. This `main.rs` file incorporates some key features that are not necessarily related to `ratatui`, but in my opinion, essential for any Terminal User Interface (TUI) program: Command Line Argument Parsing (`clap`)

- XDG Base Directory Specification

- Logging

- Panic Handler

These are described in more detail in the [`config.rs`], [`cli.rs`], [`errors.rs`] and
[`logging.rs`] files.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/templates/component/main-rs.md)

 [Previous Project Structure](/templates/component/project-structure/) [Next Tui.rs](/templates/component/tui-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
