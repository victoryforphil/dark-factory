----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/sync/code-owners
- Keywords: moon, moonrepo, docs, monorepo, build, commands, sync, code owners
- Summary: The `moon sync code-owners` command will manually sync code owners, by aggregating all owners from
----

Source: https://moonrepo.dev/docs/commands/sync/code-owners

# sync code-owners

v1.8.0

The `moon sync code-owners` command will manually sync code owners, by aggregating all owners from
projects, and generating a single `CODEOWNERS` file. Refer to the official
[code owners](/docs/guides/codeowners) guide for more information.

```
$ moon sync code-owners
```

### Options

- `--clean` - Clean and remove previously generated file.

- `--force` - Bypass cache and force create file.

### Configuration

- [`codeowners`](/docs/config/workspace#codeowners) in `.moon/workspace.yml`

- [`owners`](/docs/config/project#owners) in `moon.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
