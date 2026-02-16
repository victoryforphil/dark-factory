----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/upgrade
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, upgrade
- Summary: The `proto upgrade` (or `proto up`) command can be used to upgrade your current proto binary to the
----

Source: https://moonrepo.dev/docs/proto/commands/upgrade

# upgrade

The `proto upgrade` (or `proto up`) command can be used to upgrade your current proto binary to the
latest version, or check if you're currently outdated.

```
$ proto upgrade# Up/downgrade to a specific version$ proto upgrade 0.39.0
```

info

The previous binary will be moved to `~/.proto/tools/proto/`, while the new binary will be
installed to `~/.proto/bin`.

### Arguments

- `` - The version of proto to explicitly upgrade or downgrade to. v0.39.3

### Options

- `--check` - Check if there's a new version without executing the upgrade.

- `--json` - Print the upgrade information as JSON.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
