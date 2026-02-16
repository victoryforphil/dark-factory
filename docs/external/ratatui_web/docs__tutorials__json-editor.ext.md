----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/json-editor
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, json editor
- Summary: Now that we have covered some of the basics of a [Hello Ratatui](/tutorials/hello-ratatui) and [Counter](/tutorials/counter-app) apps, we are ready to
----

Source: https://ratatui.rs/tutorials/json-editor

# JSON Editor

Now that we have covered some of the basics of a [Hello Ratatui](/tutorials/hello-ratatui) and [Counter](/tutorials/counter-app) apps, we are ready to
build and manage something more involved.

In this tutorial, we will be creating an application that gives the user a simple interface to enter
key-value pairs, which will be converted and printed to `stdout` in json. The purpose of this
application will be to give the user an interface to create correct json, instead of having to worry
about commas and brackets themselves.

Here’s a gif of what it will look like if you run this:

## Initialization

[Section titled “Initialization”](#initialization)

Go ahead and set up a new rust project with

Terminal window

```
cargo new ratatui-json-editor
```

and put the following in the `Cargo.toml`:

```
[dependencies]ratatui = "0.29.0"serde = { version = "1.0.219", features = ["derive"] }serde_json = "1.0.140"
```

or the latest version of these libraries.

## Filestructure

[Section titled “Filestructure”](#filestructure)

Now create two files inside of `src/` so it looks like this:

```
src├── main.rs├── ui.rs└── app.rs
```

This follows a common approach to small applications in `ratatui`, where we have a state file, a UI
file, and the main file to tie it all together.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/json-editor/index.md)

 [Previous Error Handling](/tutorials/counter-app/error-handling/) [Next App.rs](/tutorials/json-editor/app/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
