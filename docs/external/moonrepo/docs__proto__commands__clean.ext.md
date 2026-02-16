----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/clean
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, clean
- Summary: The `proto clean` command can be used to uninstall stale and unused tools, plugins, and more. By
----

Source: https://moonrepo.dev/docs/proto/commands/clean

# clean

The `proto clean` command can be used to uninstall stale and unused tools, plugins, and more. By
default, it will remove items that haven't been used in the last 30 days.

```
$ proto clean
```

Furthermore, the command can be used to target a specific artifact type.

```
$ proto clean plugins
```

### Arguments

- `[target]` - Type of target. Accepts `cache`, `plugins`, `temp`, or `tools`. v0.44.0

### Options

- `--days` - Number of days before a tool is considered stale.

- `--json` - Print the clean result in JSON format. v0.44.0

- `--yes` - Avoid and confirm all prompts.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
