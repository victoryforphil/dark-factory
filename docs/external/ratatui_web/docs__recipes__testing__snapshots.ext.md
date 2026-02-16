----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/testing/snapshots
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, testing, snapshots
- Summary: Snapshot tests allow you to skip the tedious process of writing exact tests by capturing reference
----

Source: https://ratatui.rs/recipes/testing/snapshots

# Testing with insta snapshots

Snapshot tests allow you to skip the tedious process of writing exact tests by capturing reference
values once. and then using them in all future tests runs. It’s easy to use
[insta](https://insta.rs/) and [cargo-insta](https://crates.io/crates/cargo-insta) to write snapshot
tests for Ratatui apps and widgets.

### 1. Add Dependencies

[Section titled “1. Add Dependencies”](#1-add-dependencies)

First, make sure to install cargo-insta and include the [`insta`](https://crates.io/crates/insta) crate in your `Cargo.toml`:

- Terminal window ``` cargo install cargo-instacargo add insta --dev ``` ### 2. Structuring Your Application [Section titled “2. Structuring Your Application”](#2-structuring-your-application) Let’s assume you have a simple application that implements the `App` struct, which is responsible for your TUI’s core logic and rendering. To test this with insta snapshots, you’ll use a [`TestBackend`](https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html) from Ratatui to capture the output in a test environment. Here’s the structure of your app and test: ``` #[derive(Default)]pub struct App { /* Your app struct */ } impl Widget for App { /* Implement the Widget trait */ } // Now in tests module:#[cfg(test)]mod tests { use super::App; use insta::assert_snapshot; use ratatui::{backend::TestBackend, Terminal}; #[test] fn test_render_app() { let app = App::default(); let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap(); terminal .draw(|frame| frame.render_widget(&#x26;app, frame.area())) .unwrap(); assert_snapshot!(terminal.backend()); }} ``` ### 3. Running the Test [Section titled “3. Running the Test”](#3-running-the-test) When you run the test (`cargo test`), the output of the `Terminal::draw()` method will be compared against a snapshot. If this is the first time running the test or the output has changed, [`insta`](https://crates.io/crates/insta) will create a snapshot file under the `snapshots/` directory. For example, after running the test, a new snapshot file might be created at: ``` snapshots/demo2__tests__render_app.snap ``` This file will store the visual representation of your terminal as a string: ``` ---source: examples/demo2/main.rsexpression: terminal.backend()---"Ratatui Recipe Email Traceroute Weather ""▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄▄███▄▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄███████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄█████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀████████████▄▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ ▀▀▀▀▀▀▀▀▀▀▀▀▀███████████▀▀▀▀▄▄██████▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ ──────── Ratatui ───────── ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀███▀▄█▀▀████████▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ - cooking up terminal user ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄▄▄▄▀▄████████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ interfaces - ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀████████████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀███▀██████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ Ratatui is a Rust crate ▀▀▀▀▀▀▀▀▀▀▀▀▀▄▀▀▄▀▀▀█████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ that provides widgets ▀▀▀▀▀▀▀▀▀▀▀▄▀ ▄ ▀▄▀█████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ (e.g. Paragraph, Table) ▀▀▀▀▀▀▀▀▀▄▀ ▀▀ ▀▄▀███████▄▄▄▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ and draws them to the ▀▀▀▀▀▀▀▄▀ ▄▄ ▀▄▀█████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀ ▀▀▀▀▀▄▀ ▀▀ ▀▄▀██▀▀▀███▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█ ▀▄▀▀▀▄██▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄ ▀▄▀█▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀""▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"" H/← Left L/→ Right K/↑ Up J/↓ Down D/Del Destroy Q/Esc Quit " ``` In the snapshot, each line represents a row of the terminal, capturing the rendered state of your TUI. The next time you run your tests, the output will be compared to this file to detect any unintentional changes. NoteAsserting with color is not supported as of now. See [https://github.com/ratatui/ratatui/issues/1402](https://github.com/ratatui/ratatui/issues/1402) ### 4. Handling Snapshot Changes [Section titled “4. Handling Snapshot Changes”](#4-handling-snapshot-changes) When changes to the UI are intentional, you can update the snapshot by running: Terminal window ``` cargo insta review ``` This command allows you to review the differences and accept the new snapshot if desired. Tip If your UI changes often, consider reviewing snapshots after significant updates to avoid constant failures in CI/CD pipelines.

- Use a consistent terminal size (`80x20` in this example) to ensure reproducible results.

Check out the [cargo-insta documentation](https://insta.rs/docs/) for more details on managing snapshot tests.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/testing/snapshots.md)

 [Previous Testing Apps](/recipes/testing/) [Next Debugging Widget State](/recipes/testing/debug-widget-state/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
