----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/json-editor/ui
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, json editor, ui
- Summary: Finally we come to the last piece of the puzzle, and also the hardest part when you are just
----

Source: https://ratatui.rs/tutorials/json-editor/ui

# UI.rs

Finally we come to the last piece of the puzzle, and also the hardest part when you are just
starting out creating `ratatui` TUIs --- the UI. We created a very simple UI with just one widget in
the previous tutorial, but here we’ll explore some more sophisticated layouts.

Note

If you have created a UI before, you should know that the UI code can take up much more space than
you think it should, and this is no exception. We will only briefly cover all the functionality
available in `ratatui` and how the core of `ratatui` design works.

There will be links to more resources where they are covered in depth in the following sections.

## Layout basics

[Section titled “Layout basics”](#layout-basics)

Our first step is to grasp how we render widgets onto the terminal.

In essence: Widgets are constructed and then drawn onto the screen using a `Frame`, which is placed
within a specified `Rect`.

Now, envision a scenario where we wish to divide our renderable `Rect` area into three distinct
areas. For this, we can use the `Layout` functionality in `ratatui`.

- ``` let chunks = Layout::default() .direction(Direction::Vertical) .constraints([ Constraint::Length(3), Constraint::Min(1), Constraint::Length(3), ]) .split(frame.area()); ``` This can be likened to partitioning a large rectangle into smaller sections. TipFor a better understanding of layouts and constraints, refer to the concepts page on [Layout](/concepts/layout/). In the example above, you can read the instructions aloud like this: Take the area `f.area()` (which is a rectangle), and cut it into three vertical pieces (making horizontal cuts).

- The first section will be 3 lines tall

- The second section should never be smaller than one line tall, but can expand if needed.

- The final section should also be 3 lines tall

For those visual learners, I have the following graphic:

Now that we have that out of the way, let us create the TUI for our application.

## The function signature

[Section titled “The function signature”](#the-function-signature)

Our UI function needs two things to successfully create our UI elements. The `Frame` which contains
the size of the terminal at render time (this is important, because it allows us to take resizable
terminals into account), and the application state.

```
pub fn ui(frame: &#x26;mut Frame, app: &#x26;App) {
```

Before we proceed, let’s implement a `centered_rect` helper function. This code is adapted from the
[popup example](https://ratatui.rs/examples/apps/popup/).

```
/// helper function to create a centered rect using up certain percentage of the available rect `r`fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {    // Cut the given rectangle into three vertical pieces    let popup_layout = Layout::default()        .direction(Direction::Vertical)        .constraints([            Constraint::Percentage((100 - percent_y) / 2),            Constraint::Percentage(percent_y),            Constraint::Percentage((100 - percent_y) / 2),        ])        .split(r);
    // Then cut the middle vertical piece into three width-wise pieces    Layout::default()        .direction(Direction::Horizontal)        .constraints([            Constraint::Percentage((100 - percent_x) / 2),            Constraint::Percentage(percent_x),            Constraint::Percentage((100 - percent_x) / 2),        ])        .split(popup_layout[1])[1] // Return the middle chunk}
```

This will be useful for the later subsections.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/json-editor/ui.md)

 [Previous Main.rs](/tutorials/json-editor/main/) [Next UI - Main screen](/tutorials/json-editor/ui-main/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
