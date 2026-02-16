----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/plugin/add
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, plugin, add
- Summary: ` command will add the provided ID and plugin locator string to
----

Source: https://moonrepo.dev/docs/proto/commands/plugin/add

# plugin add

v0.23.0

The `proto plugin add
` command will add the provided ID and plugin locator string to
the `[plugins]` section of a chosen `.prototools`.

```
$ proto plugin add node "https://github.com/moonrepo/node-plugin/releases/latest/download/node_plugin.wasm"
```

Learn more about [plugin locator strings](/docs/proto/plugins#enabling-plugins).

### Arguments

- `` - ID of the tool.

- `` - How to locate the plugin.

### Options

- `--to` - [Location of `.prototools`](/docs/proto/config#locations) to update. v0.41.0

- `--type` - Type of plugin to add, either `tool` (default) or `backend`. v0.52.0

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
