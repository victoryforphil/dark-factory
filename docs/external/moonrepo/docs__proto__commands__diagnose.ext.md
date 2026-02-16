----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/diagnose
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, diagnose
- Summary: The `proto diagnose` command will diagnose your proto installation for any potential issues. Issues
----

Source: https://moonrepo.dev/docs/proto/commands/diagnose

# diagnose

v0.37.0

The `proto diagnose` command will diagnose your proto installation for any potential issues. Issues
are categorized into errors and warnings, with the former being a must fix, and the latter being a
maybe fix (depending on your usage of proto).

```
$ proto diagnoseShell: zshShell profile: /Users/name/.zshrcErrors ────────────────────────────────────────────────────────────────────  - Issue: Bin directory /Users/name/.proto/bin was found BEFORE the shims directory /Users/name/.proto/shims on PATH    Resolution: Ensure the shims path comes before the bin path in your shell    Comment: Runtime version detection will not work correctly unless shims are used
```

### Options

- `--shell` - The shell to diagnose (will detect automatically).

- `--json` - Print the diagnosis in JSON format.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
