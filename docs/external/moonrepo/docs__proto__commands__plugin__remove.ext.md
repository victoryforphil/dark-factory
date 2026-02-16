----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/plugin/remove
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, plugin, remove
- Summary: The `proto plugin remove ` command will remove the provided tool ID from the `[plugins]` section
----

Source: https://moonrepo.dev/docs/proto/commands/plugin/remove

# plugin remove

v0.23.0

The `proto plugin remove ` command will remove the provided tool ID from the `[plugins]` section
of the chosen (`.prototools`).

```
$ proto plugin remove node
```

Built-in plugins cannot be removed!

### Arguments

- `` - ID of the tool.

### Options

- `--from` - [Location of `.prototools`](/docs/proto/config#locations) to update. v0.41.0

- `--type` - Type of plugin to remove, either `tool` (default) or `backend`. v0.52.0

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
