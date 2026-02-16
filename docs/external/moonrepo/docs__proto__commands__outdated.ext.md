----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/outdated
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, outdated
- Summary: The `proto outdated` command will load all [`.prototools`](/docs/proto/config) files and check for newer
----

Source: https://moonrepo.dev/docs/proto/commands/outdated

# outdated

v0.19.0

The `proto outdated` command will load all [`.prototools`](/docs/proto/config) files and check for newer
(matching configured range) and latest versions of each configured tool. Will also include the
configuration file in which the version has been configured.

```
$ proto outdated╭───────────────────────────────────────────────────────────────────────╮│ Tool      Current Newest  Latest  Config                              ││───────────────────────────────────────────────────────────────────────││ bun       1.1.42  1.1.42  1.1.42  /Users/name/.proto/.prototools      ││ node      23.5.0  23.5.0  23.5.0  /Users/name/.proto/.prototools      ││ npm       10.7.0  10.7.0  11.0.0  /Users/name/.proto/.prototools      ││ rust      1.83.0  1.83.0  1.83.0  /Users/name/.proto/.prototools      ││ yarn      3.6.3   3.8.7   4.5.1   /Users/name/.proto/.prototools      │╰───────────────────────────────────────────────────────────────────────╯
```

By default, this command does not check tools for versions pinned in the global
`~/.proto/.prototools` file. Pass `--config-mode all` to include them.

### Options

- `--json` - Print the list in JSON format.

- `--latest` - When updating versions with `--update`, use the latest version instead of newest.

- `--update` - Update and write newest/latest versions to their respective configuration.

- `--yes` - Avoid and confirm all prompts. v0.44.0

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
