----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/plugin/search
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, plugin, search
- Summary: The `proto plugin search ` command will search for plugins provided by the community, based
----

Source: https://moonrepo.dev/docs/proto/commands/plugin/search

# plugin search

v0.36.0

The `proto plugin search ` command will search for plugins provided by the community, based
on the provided query string. Built-in plugins are not searchable.

```
$ proto plugin search moonSearch results for: moonLearn more about plugins: https://moonrepo.dev/docs/proto/plugins╭──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮│ Plugin      Author    Format Description             Locator                                                             ││──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────││ moon        moonrepo  TOML   moon is a multi-        https://raw.githubusercontent.com/moonrepo/moon/master/proto-       ││                              language build system   plugin.toml                                                         ││                              and codebase management                                                                     ││                              tool.                                                                                       │╰──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

### Arguments

- `` - Query string to match against.

### Options

- `--json` - Print the results in JSON format.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
