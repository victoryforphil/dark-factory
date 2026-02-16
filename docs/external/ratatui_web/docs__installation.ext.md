----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /installation
- Keywords: ratatui, rust, tui, terminal ui, docs, installation
- Summary: `ratatui` is a standard rust crate and can be installed into your app using the following command:
----

Source: https://ratatui.rs/installation

# Installation

`ratatui` is a standard rust crate and can be installed into your app using the following command:

Terminal window

```
cargo add ratatui
```

or by adding the following to your `Cargo.toml` file:

```
[dependencies]ratatui = "0.30.0"
```

By default, `ratatui` enables the `crossterm` feature, but it’s possible to alternatively use
`termion`, or `termwiz` instead by enabling the appropriate feature and disabling the default
features. See [Backend](/concepts/backends/) for more information.

Note

Before Ratatui 0.27.0, it was necessary to import a backend crate that matched the backend feature.
In 0.27.0 Ratatui now exports the backend crates at the root to make it easier to ensure a matching
version of the backend crate is used.

For Termion:

Terminal window

```
cargo add ratatui --no-default-features --features termion
```

or in your `Cargo.toml`:

```
[dependencies]ratatui = { version = "0.30.0", default-features = false, features = ["termion"] }
```

For Termwiz:

Terminal window

```
cargo add ratatui --no-default-features --features termwiz
```

or in your `Cargo.toml`:

```
[dependencies]ratatui = { version = "0.30.0", default-features = false, features = ["termwiz"] }
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/installation/index.md)

  [Next Feature Flags](/installation/feature-flags/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
