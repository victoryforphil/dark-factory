----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/counter-async-app/conclusion
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, counter async app, conclusion
- Summary: We touched on the basic framework for building an `async` application with Ratatui, namely using
----

Source: https://ratatui.rs/tutorials/counter-async-app/conclusion

# Conclusion

We touched on the basic framework for building an `async` application with Ratatui, namely using
`tokio` and `crossterm`’s async features to create an `Event` and `Action` enum that contain
`Render` variants. We also saw how we could use `tokio` channels to send `Action`s to run domain
specific async operations concurrently.

There’s more information in the documentation for a template that covers setting up a
[`Component` based architecture](/concepts/application-patterns/component-architecture/).

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/counter-async-app/conclusion.md)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
