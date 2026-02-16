----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/config/tasks
- Keywords: moon, moonrepo, docs, monorepo, build, config, tasks
- Summary: The `.moon/tasks/**/*` files configures file groups and tasks that are inherited by every project
----

Source: https://moonrepo.dev/docs/config/tasks

# .moon/tasks

The `.moon/tasks/**/*` files configures file groups and tasks that are inherited by every project
in the workspace based on inheritance conditions.
[Learn more about task inheritance!](/docs/concepts/task-inheritance)

Projects can override or merge with these settings within their respective [`moon.yml`](/docs/config/project).

## `extends`

Defines one or many external `.moon/tasks/all.yml`'s to extend and inherit settings from. Perfect
for reusability and sharing configuration across repositories and projects. When defined, this
setting must be an HTTPS URL or relative file system path that points to a valid YAML document!

.moon/tasks/all.yml

```
extends: 'https://raw.githubusercontent.com/organization/repository/master/.moon/tasks/all.yml'
```

caution

For map-based settings, `fileGroups` and `tasks`, entries from both the extended configuration and
local configuration are merged into a new map, with the values of the local taking precedence. Map
values are not deep merged!

## `fileGroups`

For more information on file group configuration, refer to the
[`fileGroups`](/docs/config/project#filegroups) section in the [`moon.yml`](/docs/config/project) doc.

Defines [file groups](/docs/concepts/file-group) that will be inherited by projects, and also enables
enforcement of organizational patterns and file locations. For example, encourage projects to place
source files in a `src` folder, and all test files in `tests`.

.moon/tasks/all.yml

```
fileGroups:  configs:    - '*.config.{js,cjs,mjs}'    - '*.json'  sources:    - 'src/**/*'    - 'types/**/*'  tests:    - 'tests/**/*'    - '**/__tests__/**/*'  assets:    - 'assets/**/*'    - 'images/**/*'    - 'static/**/*'    - '**/*.{scss,css}'
```

info

File paths and globs used within a file group are relative from the inherited project's root, and
not the workspace root.

## `implicitDeps`

Defines task [`deps`](/docs/config/project#deps) that are implicitly inserted into all inherited tasks within
a project. This is extremely useful for pre-building projects that are used extensively throughout
the repo, or always building project dependencies. Defaults to an empty list.

.moon/tasks/all.yml

```
implicitDeps:  - '^:build'
```

info

Implicit dependencies are always inherited, regardless of the [`mergeDeps`](/docs/config/project#mergedeps)
option.

## `implicitInputs`

Defines task [`inputs`](/docs/config/project#inputs) that are implicitly inserted into all inherited tasks
within a project. This is extremely useful for the "changes to these files should always trigger a
task" scenario.

Like `inputs`, file paths/globs defined here are relative from the inheriting project.
[Project and workspace relative file patterns](/docs/concepts/file-pattern#project-relative) are
supported and encouraged.

.moon/tasks/node.yml

```
implicitInputs:  - 'package.json'
```

info

Implicit inputs are always inherited, regardless of the [`mergeInputs`](/docs/config/project#mergeinputs)
option.

## `inheritedBy`v2.0.0

A map of conditions that must be met for the configuration within the file to be inherited by a
project. When this field is not defined, or is an empty map, the configuration will be inherited by
all projects.

.moon/tasks/custom.yml

```
inheritedBy:  # Project belongs to either javascript or typescript toolchain, but not the ruby toolchain  toolchains:    or: ['javascript', 'typescript']    not: ['ruby']  # And project is either a frontend or backend stack  stacks: ['frontend', 'backend']  # And project is either a library or tool layer  layers: ['library', 'tool']
```

info

View the [official task inheritance guide](/docs/concepts/task-inheritance) for more information!

## `tasks`

For more information on task configuration, refer to the [`tasks`](/docs/config/project#tasks) section in the
[`moon.yml`](/docs/config/project) doc.

As mentioned in the link above, [tasks](/docs/concepts/task) are actions that are ran within the
context of a project, and commonly wrap a command. For most workspaces, every project should have
linting, typechecking, testing, code formatting, so on and so forth. To reduce the amount of
boilerplate that every project would require, this setting offers the ability to define tasks that
are inherited by many projects within the workspace, but can also be overridden per project.

.moon/tasks/all.yml

```
tasks:  format:    command: 'prettier --check .'  lint:    command: 'eslint --no-error-on-unmatched-pattern .'  test:    command: 'jest --passWithNoTests'  typecheck:    command: 'tsc --build'
```

info

Relative file paths and globs used within a task are relative from the inherited project's root, and
not the workspace root.

## `taskOptions`v1.20.0

For more information on task options, refer to the [`options`](/docs/config/project#options) section in the
[`moon.yml`](/docs/config/project) doc.

Like [tasks](#tasks), this setting allows you to define task options that will be inherited by all
tasks within the configured file, and by all project-level inherited tasks. This setting is the 1st
link in the inheritance chain, and can be overridden within each task.

.moon/tasks/all.yml

```
taskOptions:  # Never cache builds  cache: false  # Always re-run flaky tests  retryCount: 2tasks:  build:    # ...    options:      # Override the default cache setting      cache: true
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
