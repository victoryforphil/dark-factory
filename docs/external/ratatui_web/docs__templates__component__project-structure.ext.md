----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /templates/component/project-structure
- Keywords: ratatui, rust, tui, terminal ui, docs, templates, component, project structure
- Summary: The rust files in the `component` project are organized as follows:
----

Source: https://ratatui.rs/templates/component/project-structure

# Project Structure

The rust files in the `component` project are organized as follows:

Terminal window

```
$ tree.├── build.rs└── src    ├── action.rs    ├── app.rs    ├── cli.rs    ├── components    │   ├── fps.rs    │   └── home.rs    ├── components.rs    ├── config.rs    ├── errors.rs    ├── logging.rs    ├── main.rs    └── tui.rs
```

Once you have set up the project, you shouldn’t need to change the contents of anything outside the
`components` folder.

Let’s discuss the contents of the files in the `src` folder first, how these contents of these files
interact with each other and why they do what they are doing.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/templates/component/project-structure.md)

 [Previous Component Template](/templates/component/) [Next Main.rs](/templates/component/main-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
