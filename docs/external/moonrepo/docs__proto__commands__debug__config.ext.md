----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/debug/config
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, debug, config
- Summary: The `proto debug config` command will list all `.prototools` configuration files (in TOML format)
----

Source: https://moonrepo.dev/docs/proto/commands/debug/config

# debug config

v0.25.0

The `proto debug config` command will list all `.prototools` configuration files (in TOML format)
that have been loaded, in order of precedence, with the final merged configuration printed at the
end.

```
$ proto debug config/Users/name/.proto/.prototools ───────────────────────────────────────────  node = "20.0.0"  npm = "bundled"  [tools.node.aliases]  stable = "~20"  [settings]  auto-clean = falseFinal configuration ───────────────────────────────────────────────────────  node = "20.0.0"  npm = "bundled"  [tools.node.aliases]  stable = "~20"  [plugins.tools]  node = "https://github.com/moonrepo/node-plugin/releases/download/v0.6.1/node_plugin.wasm"  [settings]  auto-clean = false  auto-install = false  detect-strategy = "first-available"  [settings.http]  allow-invalid-certs = false  proxies = []
```

### Options

- `--json` - Print the list in JSON format.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
