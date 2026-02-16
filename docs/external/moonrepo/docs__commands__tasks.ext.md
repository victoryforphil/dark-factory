----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/tasks
- Keywords: moon, moonrepo, docs, monorepo, build, commands, tasks
- Summary: The `moon tasks` command will list all tasks available in the workspace as a table of information.
----

Source: https://moonrepo.dev/docs/commands/tasks

# tasks

v2.0.0

The `moon tasks` command will list all tasks available in the workspace as a table of information.

```
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮│Task                          Command          Type        Preset      Toolchains                                Description                                                           ││───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────││website:build                 docusaurus       build                   typescript, javascript, node, yarn        Builds the Docusaurus app.                                            ││website:format                prettier         test                    javascript, node, yarn                                                                                          ││website:format-write          prettier         test                    javascript, node, yarn                                                                                          ││website:lint                  eslint           test                    javascript, node, yarn                                                                                          ││website:lint-fix              eslint           test                    javascript, node, yarn                                                                                          ││website:start                 docusaurus       run         server      typescript, javascript, node, yarn                                                                              ││website:test                  jest             test                    javascript, node, yarn                                                                                          ││website:typecheck             tsc              test                    typescript, javascript, node, yarn                                                                              │╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

info

Use [`moon query tasks`](/docs/commands/query/tasks) for advanced querying and filtering of tasks.

### Arguments

- `[id]` - Filter tasks to a specific project ID.

### Options

- `--json` - Print the projects as JSON.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
