----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /templates/component/action-rs
- Keywords: ratatui, rust, tui, terminal ui, docs, templates, component, action rs
- Summary: Defines the `Action` enum that represents actions that can be performed by the app.
----

Source: https://ratatui.rs/templates/component/action-rs

# Action.rs

Defines the `Action` enum that represents actions that can be performed by the app.

Tip

The `Action` pattern is the concept of “reified method calls”. You can learn a lot more about this
pattern from the excellent
[http://gameprogrammingpatterns.com](http://gameprogrammingpatterns.com/command.html).

These are also typically called `Action`s or `Message`s.

Note

It should come as no surprise that building a terminal user interface using `ratatui` (i.e. an
immediate mode rendering library) has a lot of similarities with game development or user interface
libraries. For example, you’ll find these domains all have their own version of “input handling”,
“event loop” and “draw” step.

If you are coming to `ratatui` with a background in `Elm` or `React`, or if you are looking for a
framework that extends the `ratatui` library to provide a more standard UI design paradigm, you can
check out [`tui-realm`](https://github.com/veeso/tui-realm/) for a more featureful out of the box
experience.

```
pub enum Action {    Tick,    Render,    Resize(u16, u16),    Suspend,    Resume,    Quit,    ClearScreen,    Error(String),    Help,}
```

Full code for the `action.rs` file is:

```
use serde::{Deserialize, Serialize};use strum::Display;
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]pub enum Action {    Tick,    Render,    Resize(u16, u16),    Suspend,    Resume,    Quit,    ClearScreen,    Error(String),    Help,}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/templates/component/action-rs.md)

 [Previous Tui.rs](/templates/component/tui-rs/) [Next Components.rs](/templates/component/components-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
