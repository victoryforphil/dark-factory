----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/apps/migrate-from-tui-rs
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, apps, migrate from tui rs
- Summary: [Ratatui](https://github.com/tui-rs-revival/ratatui) is a fork of
----

Source: https://ratatui.rs/recipes/apps/migrate-from-tui-rs

# Migrate from tui-rs

[Ratatui](https://github.com/tui-rs-revival/ratatui) is a fork of
[tui-rs](https://github.com/fdehau/tui-rs/), created to continue maintenance of the project.

Several options are available to migrate apps and libs:

- Fully replace `tui-rs` with `ratatui` (preferred approach)

- Use `ratatui` as a drop in replacement aliased as `tui`

- Support both `tui` and `ratatui`

## Fully replace Tui with Ratatui

[Section titled “Fully replace Tui with Ratatui”](#fully-replace-tui-with-ratatui)

Most new code should use the following. To take this approach to migration requires find and replace
`tui::`->`ratatui::` on the entire codebase.

- ``` ratatui = { version = "0.28.0" }crossterm = { version = "0.28.0" } ``` ## Drop in replacement [Section titled “Drop in replacement”](#drop-in-replacement) The simplest approach to migrating to `ratatui` is to use it as drop in replacement for tui and update the terminal libraries used (`crossterm` / `termion`). E.g.: ``` tui = { package = "ratatui", version = "0.28.0", features = ["crossterm"] }crossterm = { version = "0.28.0" } ``` ## Support both tui and ratatui [Section titled “Support both tui and ratatui”](#support-both-tui-and-ratatui) For more complex scenarios where a library (or in some cases an app) needs to support both ratatui and maintain existing support for tui, it may be feasible to use feature flags to select which library to use. See [tui-logger](https://github.com/gin66/tui-logger) for an example of this approach. ## Backwards compatibility and breaking changes [Section titled “Backwards compatibility and breaking changes”](#backwards-compatibility-and-breaking-changes) [BREAKING-CHANGES.md](https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md)

- PRs tagged with the [breaking changes](https://github.com/ratatui/ratatui/pulls?q=is%3Apr+label%3A%22breaking+change%22+is%3Aclosed) label

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/apps/migrate-from-tui-rs.md)

 [Previous Better Panic Hooks](/recipes/apps/better-panic/) [Next Spawn External Editor (Vim)](/recipes/apps/spawn-vim/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
