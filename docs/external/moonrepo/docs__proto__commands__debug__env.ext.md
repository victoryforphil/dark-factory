----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/debug/env
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, debug, env
- Summary: The `proto debug env` command will print information about your current proto environment. Primarily
----

Source: https://moonrepo.dev/docs/proto/commands/debug/env

# debug env

v0.26.0

The `proto debug env` command will print information about your current proto environment. Primarily
the store location, relevant file paths, and environment variables.

```
$ proto debug envStore ─────────────────────────────────────────────────────────────────────  Root: /Users/name/.proto  Bins: /Users/name/.proto/bin  Shims: /Users/name/.proto/shims  Plugins: /Users/name/.proto/plugins  Tools: /Users/name/.proto/tools  Temp: /Users/name/.proto/tempEnvironment ───────────────────────────────────────────────────────────────  Proto version: 0.44.0  Operating system: macos  Architecture: arm64  Config sources:    - /Users/name/Projects/example/.prototools    - /Users/name/.proto/.prototools  Virtual paths:    /userhome = /Users/name    /proto = /Users/name/.proto  Environment variables:    PROTO_APP_LOG = proto=info,schematic=info,starbase=info,warpgate=info,extism::pdk=info    PROTO_HOME = /Users/name/.proto    PROTO_OFFLINE_TIMEOUT = 750    PROTO_VERSION = 0.44.0
```

### Options

- `--json` - Print the list in JSON format.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
