----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /installation/feature-flags
- Keywords: ratatui, rust, tui, terminal ui, docs, installation, feature flags
- Summary: As ratatui grows and evolves, this list may change, so make sure to check the
----

Source: https://ratatui.rs/installation/feature-flags

# Feature Flags

As ratatui grows and evolves, this list may change, so make sure to check the
[main repo](https://github.com/ratatui/ratatui) if you are unsure.

## Backend Selection

[Section titled “Backend Selection”](#backend-selection)

For most cases, the default `crossterm` backend is the correct choice. See
[Backends](/concepts/backends/) for more information. However, this can be changed to termion or
termwiz

Terminal window

```
# Defaults to crosstermcargo add ratatui
# For termion, unset the default crossterm feature and select the termion featurecargo add ratatui --no-default-features --features=termioncargo add termion
# For termwiz, unset the default crossterm feature and select the termwiz featurecargo add ratatui --no-default-features --features=termwizcargo add termwiz
```

## All-Widgets

[Section titled “All-Widgets”](#all-widgets)

As of v0.21, the only widget in this feature group is the `calendar` widget, which can be enabled
with the `widget-calendar` feature.

Note

Starting from v0.30.0, the `all-widgets` feature is enabled by default. It can be disabled to save
on compile time.

Terminal window

```
cargo add ratatui --no-default-features --features=all-widgets
```

## Widget-Calendar

[Section titled “Widget-Calendar”](#widget-calendar)

This feature enables the calendar widget, which requires the `time` crate. It is enabled by default
as part of the `all-widgets` feature.

Terminal window

```
cargo add ratatui --no-default-features --features=widget-calendar
```

## Serde

[Section titled “Serde”](#serde)

Enables serialization and deserialization of style and color types using the Serde crate. This is
useful if you want to save themes to a file.

Terminal window

```
cargo add ratatui --features serde
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/installation/feature-flags.md)

 [Previous Installation](/installation/) [Next Tutorials](/tutorials/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
