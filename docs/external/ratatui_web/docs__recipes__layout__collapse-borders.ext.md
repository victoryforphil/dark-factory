----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/layout/collapse-borders
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, layout, collapse borders
- Summary: A common layout for applications is to split up the screen into panes, with borders around each
----

Source: https://ratatui.rs/recipes/layout/collapse-borders

# Collapse borders in a layout

A common layout for applications is to split up the screen into panes, with borders around each
pane. Often this leads to making UIs that look disconnected. E.g., the following layout:

Created by the following code:

- ``` fn draw(frame: &#x26;mut Frame) { // create a layout that splits the screen into 2 equal columns and the right column // into 2 equal rows let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area()); let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2]).areas(right); frame.render_widget(Block::bordered().title("Left Block"), left); frame.render_widget(Block::bordered().title("Top Right Block"), top_right); frame.render_widget(Block::bordered().title("Bottom Right Block"), bottom_right);} ``` We can do better though, by collapsing borders. E.g.: Starting with Ratatui 0.30, collapsing borders has become much easier thanks to the new `merge_borders` method and `Spacing::Overlap`. The recipe is simple: Import `Spacing` and `MergeStrategy`. ``` use ratatui::{ layout::{Constraint, Layout, Spacing}, symbols::merge::MergeStrategy, widgets::Block, DefaultTerminal, Frame,}; ```

- Use `Spacing::Overlap(1)` in your layout to make borders overlap. ``` // Use Spacing::Overlap(1) to make the borders overlap let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]) .spacing(Spacing::Overlap(1)) .areas(frame.area()); let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2]) .spacing(Spacing::Overlap(1)) .areas(right); ```

- Add `.merge_borders(MergeStrategy::Exact)` to your blocks to automatically merge borders (see [`MergeStrategy` documentation](https://docs.rs/ratatui/latest/ratatui/symbols/merge/enum.MergeStrategy.html#variants) for details about the different strategies). ``` // Use merge_borders(MergeStrategy::Exact) to automatically handle border merging let left_block = Block::bordered() .title("Left Block") .merge_borders(MergeStrategy::Exact); let top_right_block = Block::bordered() .title("Top Right Block") .merge_borders(MergeStrategy::Exact); let bottom_right_block = Block::bordered() .title("Bottom Right Block") .merge_borders(MergeStrategy::Exact); ```

Setting `merge_borders` to `MergeStrategy::Exact` or `MergeStrategy::Fuzzy` automatically handles
all the complex border joining logic. The `Spacing::Overlap(1)` ensures that adjacent borders occupy
the same space, allowing them to be merged.

Tip

This new approach in Ratatui 0.30 replaces the complex manual border management that was required in
earlier versions. If you’re using an older version of Ratatui, you’ll need to use custom border sets
and selective border rendering as described in the previous version of this recipe.

The full code for this example is available at
[https://github.com/ratatui/ratatui-website/blob/main/code/recipes/how-to-collapse-borders](https://github.com/ratatui/ratatui-website/blob/main/code/recipes/how-to-collapse-borders)

collapse-borders.rs

```
use std::time::Duration;
use color_eyre::Result;use ratatui::crossterm::event::{self, Event};use ratatui::{    layout::{Constraint, Layout, Spacing},    symbols::merge::MergeStrategy,    widgets::Block,    DefaultTerminal, Frame,};
/// This example shows how to use the new Ratatui v0.30 border merging feature to collapse borders/// between widgets./// See https://ratatui.rs/how-to/layout/collapse-borders for more infofn main() -> Result&#x3C;()> {    color_eyre::install()?;    let terminal = ratatui::init();    let result = run(terminal);    ratatui::restore();    result}
fn run(mut terminal: DefaultTerminal) -> Result&#x3C;()> {    loop {        terminal.draw(draw)?;        if key_pressed()? {            return Ok(());        }    }}
fn key_pressed() -> Result&#x3C;bool> {    Ok(event::poll(Duration::from_millis(16))? &#x26;&#x26; matches!(event::read()?, Event::Key(_)))}
fn draw(frame: &#x26;mut Frame) {    // create a layout that splits the screen into 2 equal columns and the right column    // into 2 equal rows
    // Use Spacing::Overlap(1) to make the borders overlap    let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])        .spacing(Spacing::Overlap(1))        .areas(frame.area());    let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2])        .spacing(Spacing::Overlap(1))        .areas(right);
    // Use merge_borders(MergeStrategy::Exact) to automatically handle border merging    let left_block = Block::bordered()        .title("Left Block")        .merge_borders(MergeStrategy::Exact);
    let top_right_block = Block::bordered()        .title("Top Right Block")        .merge_borders(MergeStrategy::Exact);
    let bottom_right_block = Block::bordered()        .title("Bottom Right Block")        .merge_borders(MergeStrategy::Exact);
    frame.render_widget(left_block, left);    frame.render_widget(top_right_block, top_right);    frame.render_widget(bottom_right_block, bottom_right);}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/layout/collapse-borders.md)

 [Previous Center a Widget](/recipes/layout/center-a-widget/) [Next Dynamic Layouts](/recipes/layout/dynamic/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
