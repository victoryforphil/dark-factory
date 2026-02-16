----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/alias
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, alias
- Summary: The `proto alias   ` (or `proto a`) command will define a custom alias that
----

Source: https://moonrepo.dev/docs/proto/commands/alias

# alias

The `proto alias   ` (or `proto a`) command will define a custom alias that
maps to a specific version for the provided tool. Aliases can be used anywhere a version is
accepted.

```
$ proto alias node work 16.16
```

By default this will update the local [`./.prototools`](/docs/proto/config) file. Pass `--to` to customize
the location.

### Arguments

- `` - Type of tool.

- `` - Name of the alias. Supports alphanumeric chars.

- `` - Version to map to the alias.

## Options

- `--to` - [Location of `.prototools`](/docs/proto/config#locations) to update. Supports `global`, `local`, and `user`. v0.41.0

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
