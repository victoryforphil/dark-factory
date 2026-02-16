----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/query/tasks
- Keywords: moon, moonrepo, docs, monorepo, build, commands, query, tasks
- Summary: Use the `moon query tasks` sub-command to query task information for all projects in the project
----

Source: https://moonrepo.dev/docs/commands/query/tasks

# query tasks

Use the `moon query tasks` sub-command to query task information for all projects in the project
graph. The tasks list can be filtered by passing a [query statement](/docs/concepts/query-lang) as
an argument, or by using [options](#options) arguments.

```
# Find all tasks grouped by project$ moon query tasks# Find all tasks from projects with an id that matches "react"$ moon query tasks --id react$ moon query tasks "task~react"
```

This will output a list of projects as JSON. The output has the following structure:

```
{	tasks: Record>,	options: QueryOptions,}
```

### Arguments

- `[query]` - An optional [query statement](/docs/concepts/query-lang) to filter projects with. When provided, all [filter options](#filters) are ignored. v1.4.0

### Options

#### Affected

- `--affected` - Filter tasks that have been affected by touched files.

- `--downstream` - Include downstream dependents of queried tasks. Supports "none" (default), "direct", "deep". v1.30.0

- `--upstream` - Include upstream dependencies of queried tasks. Supports "none", "direct", "deep" (default). v1.30.0

#### Filtersv1.30.0

All option values are case-insensitive regex patterns.

- `--command ` - Filter tasks that match this command.

- `--id ` - Filter tasks that match this ID.

- `--project ` - Filter tasks that belong to this project.

- `--script ` - Filter tasks that match this script.

- `--toolchain ` - Filter tasks of this toolchain. v1.31.0

- `--type ` - Filter tasks of this type.

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

- [`tasks`](/docs/config/project#tasks) in `moon.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
