----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/unalias
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, unalias
- Summary: The `proto unalias  ` (or `proto ua`) command will remove a custom alias for the
----

Source: https://moonrepo.dev/docs/proto/commands/unalias

# unalias

The `proto unalias  ` (or `proto ua`) command will remove a custom alias for the
provided tool.

```
$ proto unalias node work
```

By default this will update the local [`./.prototools`](/docs/proto/config) file. Pass `--from` to customize
the location.

### Arguments

- `` - Type of tool.

- `` - Name of the alias. Supports alphanumeric chars.

## Options

- `--from` - [Location of `.prototools`](/docs/proto/config#locations) to update. Supports `global`, `local`, and `user`. v0.41.0

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
