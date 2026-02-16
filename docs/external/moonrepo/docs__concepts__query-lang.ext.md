----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/concepts/query-lang
- Keywords: moon, moonrepo, docs, monorepo, build, concepts, query lang
- Summary: moon supports an integrated query language, known as MQL, that can be used to filter and select
----

Source: https://moonrepo.dev/docs/concepts/query-lang

# Query language

v1.3.0

moon supports an integrated query language, known as MQL, that can be used to filter and select
projects from the project graph, using an SQL-like syntax. MQL is primarily used by
[`moon run`](/docs/commands/run) with the `--query` option.

## Syntax

### Comparisons

A comparison (also known as an assignment) is an expression that defines a piece of criteria, and is
a building block of a query. This criteria maps a [field](#fields) to a value, with an explicit
comparison operator.

#### Equals, Not equals

The equals (`=`) and not equals (`!=`) comparison operators can be used for exact value matching.

```
projectLayer=library && language!=javascript
```

You can also define a list of values using square bracket syntax, that will match against one of the
values.

```
language=[javascript, typescript]
```

#### Like, Not like

The like (`~`) and not like (`!~`) comparison operators can be used for wildcard value matching,
using [glob syntax](/docs/concepts/file-pattern#globs).

```
projectSource~packages/* && tag!~*-app
```

Like comparisons can only be used on non-enum fields.

### Conditions

The `&&` and `||` logical operators can be used to combine multiple comparisons into a condition.
The `&&` operator is used to combine comparisons into a logical AND, and the `||` operator is used
for logical OR.

```
taskToolchain=system || taskToolchain=node
```

For readability concerns, you can also use `AND` or `OR`.

```
taskToolchain=system OR taskToolchain=node
```

Mixing both operators in the same condition is not supported.

### Grouping

For advanced queries and complex conditions, you can group comparisons using parentheses to create
logical groupings. Groups can also be nested within other groups.

```
language=javascript && (taskType=test || taskType=build)
```

## Fields

The following fields can be used as criteria, and are related to [task tokens](/docs/concepts/token#variables).

### `language`

Programming language the project is written in, as defined in
[`moon.yml`](/docs/config/project#language).

```
language=rust
```

### `project`

Name OR alias of the project.

```
project=server
```

### `projectAlias`

Alias of the project. For example, the `package.json` name.

```
projectAlias~@scope/*
```

### `projectLayer`v1.39.0

The project layer, as defined in [`moon.yml`](/docs/config/project#layer).

```
projectLayer=application
```

### `projectId`

Name of the project, as defined in [`.moon/workspace.yml`](/docs/config/workspace), or `id` in
[`moon.yml`](/docs/config/project#id).

```
projectId=server
```

### `projectSource`

Relative file path from the workspace root to the project root, as defined in
[`.moon/workspace.yml`](/docs/config/workspace).

```
projectSource~packages/*
```

### `projectStack`v1.22.0

The project stack, as defined in [`moon.yml`](/docs/config/project#stack).

```
projectStack=frontend
```

### `tag`

A tag within the project, as defined in [`moon.yml`](/docs/config/project#tags).

```
tag~react-*
```

### `task`

ID/name of a task within the project.

```
task=[build,test]
```

### `taskToolchain`v1.31.0

The toolchain a task will run against, as defined in [`moon.yml`](/docs/config/project).

```
taskToolchain=node
```

### `taskType`

The [type of task](/docs/concepts/task#types), based on its configured settings.

```
taskType=build
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
