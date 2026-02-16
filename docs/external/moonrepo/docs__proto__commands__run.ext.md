----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/run
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, run
- Summary: The `proto run  [version]` (or `proto r`) command will run a tool after
----

Source: https://moonrepo.dev/docs/proto/commands/run

# run

The `proto run  [version]` (or `proto r`) command will run a tool after
[detecting a version](/docs/proto/detection) from the environment.

```
# Run and detect version from environment$ proto run bun# Run with explicit version$ proto run bun 0.5.3# Run with version from environment variable$ PROTO_BUN_VERSION=0.5.3 proto run bun
```

Arguments can be passed to the underlying tool binary by providing additional arguments after `--`.

```
$ proto run bun -- run ./script.ts# When using the binary on PATH$ bun run ./script.ts
```

### Arguments

- `` - Type of tool.

- `[version]` - Version of tool. If not provided, will attempt to detect the version from the environment.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
