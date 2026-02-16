----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/status
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, status
- Summary: The `proto status` command will list all tools that are currently active for a target directory,
----

Source: https://moonrepo.dev/docs/proto/commands/status

# status

v0.34.0

The `proto status` command will list all tools that are currently active for a target directory,
what versions of those tools are resolved to, and the configuration file in which they are defined.

```
$ proto status╭───────────────────────────────────────────────────────────────────────────────────────────────────────╮│ Tool      Configured Resolved  Installed                           Config                             ││───────────────────────────────────────────────────────────────────────────────────────────────────────││ bun       1.1.42     1.1.42    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                bun/1.1.42                                                             ││ deno      1.43.1     1.43.1    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                deno/1.43.1                                                            ││ node      23.5.0     23.5.0    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                node/23.5.0                                                            ││ npm       ~10.7      10.7.0    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                npm/10.7.0                                                             ││ python    3.12.0     3.12.0    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                python/3.12.0                                                          ││ yarn      3.6.3      3.6.3     /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                yarn/3.6.3                                                             │╰───────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

By default, this command does not check tools for versions pinned in the global
`~/.proto/.prototools` file. Pass `--config-mode all` to include them.

### Options

- `--json` - Print the list in JSON format.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
