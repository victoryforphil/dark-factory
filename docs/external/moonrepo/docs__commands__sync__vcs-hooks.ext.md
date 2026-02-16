----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/sync/vcs-hooks
- Keywords: moon, moonrepo, docs, monorepo, build, commands, sync, vcs hooks
- Summary: The `moon sync vcs-hooks` command will manually sync hooks for the configured
----

Source: https://moonrepo.dev/docs/commands/sync/vcs-hooks

# sync vcs-hooks

v1.9.0

The `moon sync vcs-hooks` command will manually sync hooks for the configured
[VCS](/docs/config/workspace#vcs), by generating and referencing hook scripts from the
[`vcs.hooks`](/docs/config/workspace#hooks) setting. Refer to the official
[VCS hooks](/docs/guides/vcs-hooks) guide for more information.

```
$ moon sync vcs-hooks
```

### Options

- `--clean` - Clean and remove previously generated hooks.

- `--force` - Bypass cache and force create hooks.

### Configuration

- [`vcs.hooks`](/docs/config/workspace#hooks) in `.moon/workspace.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
