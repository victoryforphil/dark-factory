----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/versions
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, versions
- Summary: The `proto versions ` command will list available versions by resolving versions from the
----

Source: https://moonrepo.dev/docs/proto/commands/versions

# versions

v0.44.0

The `proto versions ` command will list available versions by resolving versions from the
tool's remote release manifest. Furthermore, if a version has been installed locally, it will be
denoted with a timestamp.

```
$ proto versions node...22.0.022.1.022.2.022.3.022.4.022.4.122.5.0 - installed 12/25/2422.5.122.6.022.7.022.8.022.9.022.10.022.11.022.12.023.0.023.1.023.2.023.3.023.4.0 - installed 12/19/2423.5.0 - installed 12/25/24
```

### Arguments

- `` - Type of tool.

### Options

- `--aliases` - Include aliases in the list.

- `--installed` - Only display installed versions.

- `--json` - Print the versions and aliases in JSON format.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
