----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/templates
- Keywords: moon, moonrepo, docs, monorepo, build, commands, templates
- Summary: The `moon templates` command will list all templates available for [code generation](/docs/commands/generate).
----

Source: https://moonrepo.dev/docs/commands/templates

# templates

v1.24.0

The `moon templates` command will list all templates available for [code generation](/docs/commands/generate).
This list will include the template title, description, default destination, where it's source files
are located, and more.

```
$ moon templates
```

### Options

- `--filter` - Filter templates by a search term.

- `--json` - Print templates in JSON format.

### Configuration

- [`generator`](/docs/config/workspace#generator) in `.moon/workspace.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
