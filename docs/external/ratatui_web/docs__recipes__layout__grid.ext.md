----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/layout/grid
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, layout, grid
- Summary: [Section titled “Problem”](#problem)
----

Source: https://ratatui.rs/recipes/layout/grid

# Grid Layout

## Problem

[Section titled “Problem”](#problem)

You want to create a grid layout for your TUI application, where widgets are arranged in a grid-like
structure.

## Solution

[Section titled “Solution”](#solution)

To create a grid layout, you can use the `Layout` struct to define the horizontal and vertical
constraints of the rows and columns. Combine these constraints with iterator methods to create a
grid layout.

## Example

[Section titled “Example”](#example)

Given the following grid struct:

```
struct Grid {    cols: usize,    rows: usize,}
```

With the following render method:

```
impl Widget for Grid {    fn render(self, area: Rect, buf: &#x26;mut Buffer) {        let col_constraints = (0..self.cols).map(|_| Constraint::Length(9));        let row_constraints = (0..self.rows).map(|_| Constraint::Length(3));        let horizontal = Layout::horizontal(col_constraints).spacing(1);        let vertical = Layout::vertical(row_constraints).spacing(1);
        let rows = vertical.split(area);        let cells = rows.iter().flat_map(|&#x26;row| horizontal.split(row).to_vec());
        for (i, cell) in cells.enumerate() {            Paragraph::new(format!("Area {:02}", i + 1))                .block(Block::bordered())                .render(cell, buf);        }    }}
```

The output will look like this:

```
┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐│Area 01│ │Area 02│ │Area 03│ │Area 04│└───────┘ └───────┘ └───────┘ └───────┘
┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐│Area 05│ │Area 06│ │Area 07│ │Area 08│└───────┘ └───────┘ └───────┘ └───────┘
┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐│Area 09│ │Area 10│ │Area 11│ │Area 12│└───────┘ └───────┘ └───────┘ └───────┘
┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐│Area 13│ │Area 14│ │Area 15│ │Area 16│└───────┘ └───────┘ └───────┘ └───────┘
```

In Ratatui 0.30, we introduce a few [new methods on Rect](https://github.com/ratatui/ratatui/pull/1909), which removes the need to bind rows to
satisfy the borrow checker, and simplifies this to a single line of code:

```
let cells = area.layout_vec(&#x26;vertical).iter().flat_map(|row| row.layout_vec(&#x26;horizontal));
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/layout/grid.md)

 [Previous UI Layout](/recipes/layout/) [Next Center a Widget](/recipes/layout/center-a-widget/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
