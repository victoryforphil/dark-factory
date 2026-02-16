----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/query/projects
- Keywords: moon, moonrepo, docs, monorepo, build, commands, query, projects
- Summary: Use the `moon query projects` sub-command to query information about all projects in the project
----

Source: https://moonrepo.dev/docs/commands/query/projects

# query projects

Use the `moon query projects` sub-command to query information about all projects in the project
graph. The project list can be filtered by passing a [query statement](/docs/concepts/query-lang) as
an argument, or by using [options](#options) arguments.

```
# Find all projects$ moon query projects# Find all projects with an id that matches "react"$ moon query projects --id react$ moon query projects "project~react"# Find all projects with a `lint` or `build` task$ moon query projects --tasks "lint|build"$ moon query projects "task=[lint,build]"
```

This will output a list of projects as JSON. The output has the following structure:

```
{	projects: Project[],	options: QueryOptions,}
```

### Affected projects

This command can also be used to query for affected projects, based on the state of the VCS working
tree. For advanced control, you can also pass the results of `moon query changed-files` to stdin.

```
# Find all affected projects$ moon query projects --affected# Find all affected projects using the results of another query$ moon query changed-files | moon query projects --affected
```

### Arguments

- `[query]` - An optional [query statement](/docs/concepts/query-lang) to filter projects with. When provided, all [filter options](#filters) are ignored. v1.4.0

### Options

#### Affected

- `--affected` - Filter projects that have been affected by touched files.

- `--downstream` - Include downstream dependents of queried projects. Supports "none" (default), "direct", "deep". v1.29.0

- `--upstream` - Include upstream dependencies of queried projects. Supports "none", "direct", "deep" (default). v1.29.0

#### Filters

All option values are case-insensitive regex patterns.

- `--alias ` - Filter projects that match this alias.

- `--id ` - Filter projects that match this ID/name.

- `--language ` - Filter projects of this programming language.

- `--layer ` - Filter project of this layer.

- `--source ` - Filter projects that match this source path.

- `--stack ` - Filter projects of the tech stack.

- `--tags ` - Filter projects that have the following tags.

- `--tasks ` - Filter projects that have the following tasks.

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
