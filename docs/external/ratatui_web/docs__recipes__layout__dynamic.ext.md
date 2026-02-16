----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/layout/dynamic
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, layout, dynamic
- Summary: With real world applications, the content can often be dynamic. For example, a chat application may
----

Source: https://ratatui.rs/recipes/layout/dynamic

# Create Dynamic layouts

With real world applications, the content can often be dynamic. For example, a chat application may
need to resize the chat input area based on the number of incoming messages. To achieve this, you
can generate layouts dynamically:

```
fn get_layout_based_on_messages(msg_count: usize, f: &#x26;Frame) -> Rc&#x3C;[Rect]> {    let msg_percentage = if msg_count > 50 { 80 } else { 50 };
    Layout::default()        .direction(Direction::Vertical)        .constraints([            Constraint::Percentage(msg_percentage),            Constraint::Percentage(100 - msg_percentage),        ])        .split(f.area())}
```

You can even update the layout based on some user input or command:

```
match action {    Action::IncreaseSize => {        current_percentage += 5;        if current_percentage > 95 {            current_percentage = 95;        }    },    Action::DecreaseSize => {        current_percentage -= 5;        if current_percentage &#x3C; 5 {            current_percentage = 5;        }    },    _ => {}}
let chunks = Layout::default()    .direction(Direction::Horizontal)    .constraints([        Constraint::Percentage(current_percentage),        Constraint::Percentage(100 - current_percentage),    ])    .split(f.area());
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/layout/dynamic.md)

 [Previous Collapse Borders](/recipes/layout/collapse-borders/) [Next Render UIs](/recipes/render/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
