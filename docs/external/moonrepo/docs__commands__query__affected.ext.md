----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/query/affected
- Keywords: moon, moonrepo, docs, monorepo, build, commands, query, affected
- Summary: Use the `moon query affected` sub-command to query for all affected projects and tasks based on the
----

Source: https://moonrepo.dev/docs/commands/query/affected

# query affected

v2.0.0

Use the `moon query affected` sub-command to query for all affected projects and tasks based on the
state of the workspace and VCS.

```
# Return affected$ moon query affected# Return affected including dependency relationships$ moon query affected --upstream deep
```

This will output a map of projects and tasks as JSON. The output has the following structure:

```
{	projects: Record,	tasks: Record,}
```

### Options

- `--downstream` - Include downstream dependents. Supports "none" (default), "direct", "deep".

- `--upstream` - Include upstream dependencies. Supports "none", "direct", "deep" (default).

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
