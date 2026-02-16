----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/unpin
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, unpin
- Summary: The `proto unpin ` command will unpin a version of a tool.
----

Source: https://moonrepo.dev/docs/proto/commands/unpin

# unpin

v0.36.0

The `proto unpin ` command will unpin a version of a tool.

```
$ proto unpin go$ proto unpin node --tool-native
```

By default this will update the local [`./.prototools`](/docs/proto/config) file. Pass `--from` to customize
the location.

### Arguments

- `` - Type of tool.

### Options

- `--from` - [Location of `.prototools`](/docs/proto/config#locations) to update. Supports `global`, `local`, and `user`. v0.41.0

- `--tool-native` - Use a tool specific location, like the `devEngines` field in the `package.json` for JavaScript tools. v0.55.0

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
