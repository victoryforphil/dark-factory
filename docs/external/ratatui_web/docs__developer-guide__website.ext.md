----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /developer-guide/website
- Keywords: ratatui, rust, tui, terminal ui, docs, developer guide, website
- Summary: [ratatui.rs](https://ratatui.rs) is built with [`Astro`](https://astro.build/) and
----

Source: https://ratatui.rs/developer-guide/website

# Contributing to Ratatui Website

[ratatui.rs](https://ratatui.rs) is built with [`Astro`](https://astro.build/) and
[`Starlight`](https://starlight.astro.build).

The source is available from the
[ratatui/ratatui-website](https://github.com/ratatui/ratatui-website) GitHub repository.

If you would like to contribute, you can make a fork and clone the repository. Make sure you run the
following [`git lfs`](../git-guide/) commands before making a PR.

- install git lfs ``` git lfs installgit lfs pull ``` To build and run the site locally: install necessary packages ``` npm install ``` run site ``` npm run dev ``` Feel free to make contributions and submit a PR if you’d like to improve the documentation. ## Some Guidelines [Section titled “Some Guidelines”](#some-guidelines) Prefer links from the root rather than using multiple levels of parent links. (e.g. `/concepts/backends/comparison/` instead of `../../backends/comparison/`).

- Prefer to add the last slash on links

## Astro and Starlight features

[Section titled “Astro and Starlight features”](#astro-and-starlight-features)

Starlight supports the full range of Markdown syntax in `.md` files as well as meta information for
titles and descriptions in YAML frontmatter.

See [starlight](https://starlight.astro.build/guides/authoring-content/) for more information on how to author content in markdown.

## Custom components

[Section titled “Custom components”](#custom-components)

The website uses custom components and features to make it easier to generate high quality
documentation that is more maintainable.

### LinkBadge

[Section titled “LinkBadge”](#linkbadge)

Use the `LinkBadge` component:

```
import LinkBadge from "/src/components/LinkBadge.astro";
&#x3C;LinkBadge text="default" href="" variant="default" />&#x3C;LinkBadge text="outline" href="" variant="outline" />&#x3C;LinkBadge text="note" href="" variant="note" />&#x3C;LinkBadge text="danger" href="" variant="danger" />&#x3C;LinkBadge text="success" href="" variant="success" />&#x3C;LinkBadge text="caution" href="" variant="caution" />&#x3C;LinkBadge text="tip" href="" variant="tip" />
```

This renders as:

default
outline
note
danger
success
caution
tip

### Code include

[Section titled “Code include”](#code-include)

Use the `remark-include-code` plugin to include code from a test file:

```
```rust{{#include @code/tutorials/hello-ratatui/src/main.rs}}```
```

This renders as:

```
use ratatui::{DefaultTerminal, Frame};
fn main() -> color_eyre::Result&#x3C;()> {    color_eyre::install()?;    ratatui::run(app)?;    Ok(())}
fn app(terminal: &#x26;mut DefaultTerminal) -> std::io::Result&#x3C;()> {    loop {        terminal.draw(render)?;        if crossterm::event::read()?.is_key_press() {            break Ok(());        }    }}
fn render(frame: &#x26;mut Frame) {    frame.render_widget("hello world", frame.area());}
```

### svgbob

[Section titled “svgbob”](#svgbob)

Draw diagrams with [`svgbob`](https://github.com/ivanceras/svgbob):

```
```svgbob ,-------------. |Get Key Event| `-----+-------'       |       | ,-----v------. |Update State| `-----+------'       |       |   ,---v----.   | Render |   `--------'```
```

This renders as:

### mermaidjs

[Section titled “mermaidjs”](#mermaidjs)

Draw diagrams with [`mermaidjs`](https://mermaid.js.org/):

```
```mermaidsequenceDiagramparticipant Userparticipant TUI Application
User->>TUI Application: Input/Event/MessageTUI Application->>TUI Application: Update (based on Model and Message)TUI Application->>TUI Application: Render View (from Model)TUI Application-->>User: Display UI```
```

This renders as:

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/developer-guide/website.mdx)

 [Previous Contributing to Ratatui](/developer-guide/ratatui/) [Next Git guide](/developer-guide/git-guide/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
