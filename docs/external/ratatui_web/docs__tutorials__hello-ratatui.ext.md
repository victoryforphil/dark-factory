----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/hello-ratatui
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, hello ratatui
- Summary: Code for this tutorial is available to view at
----

Source: https://ratatui.rs/tutorials/hello-ratatui

# Hello Ratatui

Note

Code for this tutorial is available to view at
[https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/hello-ratatui](https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/hello-ratatui)

This tutorial will lead you through creating a simple “Hello World” TUI app that displays some text
in the top-left corner of the screen and waits for the user to press any key to exit. It
demonstrates the tasks that any application developed with Ratatui needs to undertake.

We assume you have a basic understanding of the terminal, and have a text editor or IDE. If you
don’t have a preference, [VSCode](https://code.visualstudio.com/) with [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) makes a good default choice.

## Pre-requisites

[Section titled “Pre-requisites”](#pre-requisites)

### Install Rust

[Section titled “Install Rust”](#install-rust)

First install Rust if it is not already installed. See the [Installation](https://doc.rust-lang.org/book/ch01-01-installation.html) section of the official
Rust Book for more information. Most people use `rustup`, a command line tool for managing Rust
versions and associated tools. Ratatui requires at least Rust 1.74, but it’s generally a good idea
to work with the latest stable version if you can. Once you’ve installed Rust, verify it’s installed
by running:

check rust version

```
rustc --version
```

You should see output similar to the following (the exact version, date and commit hash will vary):

```
rustc 1.83.0 (90b35a623 2024-11-26)
```

### Install Cargo generate

[Section titled “Install Cargo generate”](#install-cargo-generate)

Ratatui has a few templates that make it easy to get started with a new project. [Cargo generate](https://cargo-generate.github.io/cargo-generate/) is
a developer tool to help you get up and running quickly with a new Rust project by leveraging a
pre-existing git repository as a template. We will use it to create a new Ratatui project.

Install `cargo-generate` by running the following command (or see the [installation instructions](https://cargo-generate.github.io/cargo-generate/installation.html)
for other approaches to installing cargo-generate.)

Terminal window

```
cargo install cargo-generate
```

## Create a New Project

[Section titled “Create a New Project”](#create-a-new-project)

Let’s create a new Rust project. In the terminal, navigate to a folder where you will store your
projects and run the following command to generate a new app using the simple ratatui template. (You
can find more information about this template in the [Hello World Template README](https://github.com/ratatui/templates/blob/main/hello-world/README.md))

create new rust project

```
cargo generate ratatui/templates hello-world
```

Note

The example code is licensed under the MIT license.

You will be prompted for a project name to use. Enter `hello-ratatui`.

create new rust project

```
$ cargo generate ratatui/templates⚠️   Favorite `ratatui/templates` not found in config, using it as a git repository: https://github.com/ratatui/templates.git✔ 🤷   Which sub-template should be expanded? · hello-world🤷   Project Name: hello-ratatui🔧   Destination: /Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui ...🔧   project-name: hello-ratatui ...🔧   Generating template ...🤷   Short description of the project: A Ratatui Hello World app🔧   Moving generated files into: `/Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui`...🔧   Initializing a fresh Git repository✨   Done! New project created /Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui
```

### Examine the Project

[Section titled “Examine the Project”](#examine-the-project)

The `cargo generate` command creates a new folder called `hello-ratatui` with a basic binary
application in it. If you examine the folders and files created this will look like:

```
hello-ratatui/├── src/│  └── main.rs├── Cargo.toml├── LICENSE└── README.md
```

The `Cargo.toml` file is filled with some default values and the necessary dependencies (Ratatui and
Crossterm), and one useful dependency (Color-eyre) for nicer error handling.

cargo.toml

```
[package]name = "hello-ratatui"version = "0.1.0"description = "A Ratatui Hello World app"authors = ["Josh McKinney &#x3C;[email&#160;protected]>"]license = "MIT"edition = "2024"
[dependencies]color-eyre = "0.6.3"crossterm = "0.29.0"ratatui = "0.30.0"
# Read the optimization guideline for more details: https://ratatui.rs/recipes/apps/release-your-app/#optimizations[profile.release]codegen-units = 1lto = trueopt-level = "s"strip = true
```

The generate command created a default `main.rs` that runs the app:

main.rs

```
use ratatui::{DefaultTerminal, Frame};
fn main() -> color_eyre::Result&#x3C;()> {    color_eyre::install()?;    ratatui::run(app)?;    Ok(())}
fn app(terminal: &#x26;mut DefaultTerminal) -> std::io::Result&#x3C;()> {    loop {        terminal.draw(render)?;        if crossterm::event::read()?.is_key_press() {            break Ok(());        }    }}
fn render(frame: &#x26;mut Frame) {    frame.render_widget("hello world", frame.area());}
```

Tip

In previous versions, the setup of an app was quite a bit more complex. Older Ratatui apps may have
code that includes a lot of boilerplate code to set up the app. Ratatui 0.28.1 has simplified this
process to just calling `ratatui::init()` and `ratatui::restore()`. Ratatui 0.30.0 made it even
simpler by introducing the `ratatui::run()` method, which handles those calls and can be used for
most applications.

### Run the App

[Section titled “Run the App”](#run-the-app)

Let’s build and execute the project. Run:

run the app

```
cd hello-ratatuicargo run
```

You should see the build output and then a TUI app with a `Hello world` message.

You can press any key to exit and go back to your terminal as it was before.

## Summary

[Section titled “Summary”](#summary)

Congratulations! 🎉 You have written a “hello world” terminal user interface with Ratatui. The
next sections will go into more detail about how Ratatui works.

The next tutorial, [Counter App](/tutorials/counter-app/), introduces some more interactivity, and a
more robust approach to arranging your application code.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/hello-ratatui/index.md)

 [Previous Tutorials](/tutorials/) [Next Counter App](/tutorials/counter-app/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
