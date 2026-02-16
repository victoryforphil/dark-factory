----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/concepts/file-group
- Keywords: moon, moonrepo, docs, monorepo, build, concepts, file group
- Summary: File groups are a mechanism for grouping similar types of files and environment variables within a
----

Source: https://moonrepo.dev/docs/concepts/file-group

# File groups

File groups are a mechanism for grouping similar types of files and environment variables within a
project using [file glob patterns or literal file paths](/docs/concepts/file-pattern). These groups are then used
by [tasks](/docs/concepts/task) to calculate functionality like cache computation, affected files since last
change, deterministic builds, and more.

## Configuration

File groups can be configured per project through [`moon.yml`](/docs/config/project), or for many
projects through [`.moon/tasks/all.yml`](/docs/config/tasks).

### Token functions

File groups can be referenced in [tasks](/docs/concepts/task) using [token functions](/docs/concepts/token). For example, the
`@group(name)` token will expand to all paths configured in the `sources` file group.

moon.yml

```
tasks:  build:    command: 'vite build'    inputs:      - '@group(sources)'
```

## Inheritance and merging

When a file group of the same name exists in both [configuration files](#configuration), the
project-level group will override the workspace-level group, and all other workspace-level groups
will be inherited as-is.

A primary scenario in which to define file groups at the project-level is when you want to
override file groups defined at the workspace-level. For example, say we want to override the
`sources` file group because our source folder is named "lib" and not "src", we would define our
file groups as followed.

.moon/tasks/all.yml

```
fileGroups:  sources:    - 'src/**/*'    - 'types/**/*'  tests:    - 'tests/**/*.test.*'    - '**/__tests__/**/*'
```

moon.yml

```
fileGroups:  # Overrides global  sources:    - 'lib/**/*'    - 'types/**/*'  # Inherited as-is  tests:    - 'tests/**/*.test.*'    - '**/__tests__/**/*'
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
