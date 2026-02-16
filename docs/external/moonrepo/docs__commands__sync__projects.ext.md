----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/sync/projects
- Keywords: moon, moonrepo, docs, monorepo, build, commands, sync, projects
- Summary: The `moon sync projects` command will force sync all projects in the workspace to help achieve a
----

Source: https://moonrepo.dev/docs/commands/sync/projects

# sync projects

v1.8.0

The `moon sync projects` command will force sync all projects in the workspace to help achieve a
[healthy repository state](/docs/faq#what-should-be-considered-the-source-of-truth). This applies
the following:

- Ensures cross-project dependencies are linked based on [`dependsOn`](/docs/config/project#dependson).

- Ensures language specific configuration files are present and accurate (`package.json`, `tsconfig.json`, etc).

- Ensures root configuration and project configuration are in sync.

- Any additional language specific semantics that may be required.

```
$ moon sync projects
```

This command should rarely be ran, as [`moon run`](/docs/commands/run) will sync affected projects
automatically! However, when migrating or refactoring, manual syncing may be necessary.

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
