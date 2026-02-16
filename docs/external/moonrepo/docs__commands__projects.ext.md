----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/projects
- Keywords: moon, moonrepo, docs, monorepo, build, commands, projects
- Summary: The `moon projects` command will list all projects configured in the workspace as a table of
----

Source: https://moonrepo.dev/docs/commands/projects

# projects

v2.0.0

The `moon projects` command will list all projects configured in the workspace as a table of
information.

```
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮│Project          Source                    Stack             Layer             Toolchains                                Description                                                   ││───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────││types            packages/types            frontend          library           javascript, node, typescript, yarn                                                                      ││website          website                   frontend          application       javascript, node, typescript, yarn        A static website powered by Docusaurus.                       │╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

info

Use [`moon query projects`](/docs/commands/query/projects) for advanced querying and filtering of projects.

### Options

- `--json` - Print the projects as JSON.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
