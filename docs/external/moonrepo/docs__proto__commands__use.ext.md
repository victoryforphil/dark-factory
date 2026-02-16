----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/use
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, use
- Summary: This command has been deprecated and its functionality was merged into [`proto install`](/docs/proto/commands/install)
----

Source: https://moonrepo.dev/docs/proto/commands/use

# use

danger

This command has been deprecated and its functionality was merged into [`proto install`](/docs/proto/commands/install)
in v0.39. Use that command instead!

The `proto use` (or `proto u`) command will download and install all tools and plugins from all
parent [`.prototools`](/docs/proto/config) configuration files, and any [versions detected](/docs/proto/detection) in
the current working directory (if not defined in `.prototools`).

```
$ proto use
```

This command does not install tools for versions pinned in the global `~/.proto/.prototools`
file.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
