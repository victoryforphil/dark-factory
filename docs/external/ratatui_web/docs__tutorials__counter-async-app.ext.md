----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/counter-async-app
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, counter async app
- Summary: In the previous counter app, we had a purely sequential blocking application. There are times when
----

Source: https://ratatui.rs/tutorials/counter-async-app

# Async Counter App

In the previous counter app, we had a purely sequential blocking application. There are times when
you may be interested in running IO operations or compute asynchronously.

For this tutorial, we will build a single file version of an async TUI using
[tokio](https://tokio.rs/). This tutorial section is a simplified version of the
[async ratatui counter app](https://github.com/ratatui/async-template/tree/main/ratatui-counter).

## Installation

[Section titled “Installation”](#installation)

Here’s an example of the `Cargo.toml` file required for this tutorial:

```
[package]name = "ratatui-counter-async-app"version = "0.1.0"edition = "2021"
[dependencies]color-eyre = "0.6.2"crossterm = { version = "0.28.0", features = ["event-stream"] }ratatui = "0.28.0"tokio = { version = "1.32.0", features = ["full"] }tokio-util = "0.7.9"futures = "0.3.28"
```

Note

If you were already using `crossterm` before, note that now you’ll need to add
`features = ["event-stream"]` to use crossterm’s async features.

You can use `cargo add` from the command line to add the above dependencies in one go:

Terminal window

```
cargo add ratatui crossterm color-eyre tokio tokio-util futures --features tokio/full,crossterm/event-stream
```

## Setup

[Section titled “Setup”](#setup)

Let’s take the single file multiple function example from the counter app from earlier:

```
fn main() -> Result&#x3C;()> {  // setup terminal  startup()?;
  let result = run();
  // teardown terminal before unwrapping Result of app run  shutdown()?;
  result?;
  Ok(())}
```

Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks
needed for writing network applications. We recommend you read the
[Tokio documentation](https://tokio.rs/tokio/tutorial) to learn more.

For the setup for this section of the tutorial, we are going to make just one change. We are going
to make our `main` function a `tokio` entry point.

```
#[tokio::main]async fn main() -> Result&#x3C;()> {  // setup terminal  startup()?;
  let result = run();
  // teardown terminal before unwrapping Result of app run  shutdown()?;
  result?;
  Ok(())}
```

Adding this `#[tokio::main]` macro allows us to spawn tokio tasks within `main`. At the moment,
there are no `async` functions other than `main` and we are not using `.await` anywhere yet. We will
change that in the following sections. But first, we let us introduce the `Action` enum.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/counter-async-app/index.md)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
