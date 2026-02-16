----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/concepts/task-inheritance
- Keywords: moon, moonrepo, docs, monorepo, build, concepts, task inheritance
- Summary: Unlike other task runners that require the same tasks to be repeatedly defined for every project,
----

Source: https://moonrepo.dev/docs/concepts/task-inheritance

# Task inheritance

Unlike other task runners that require the same tasks to be repeatedly defined for every project,
moon uses an inheritance model where tasks can be defined once at the workspace-level, and are then
inherited by many or all projects.

Workspace-level tasks (also known as global tasks) are defined in [`.moon/tasks/**/*.yml`](/docs/config/tasks),
and are inherited by based on conditions. However, projects are able to include, exclude, or rename
inherited tasks using the [`workspace.inheritedTasks`](/docs/config/project#inheritedtasks) in
[`moon.yml`](/docs/config/project).

## Conditional inheritance

Task inheritance is powered by the [`inheritedBy`](/docs/config/tasks#inheritedby) setting in global
task configurations (those in [`.moon/tasks/**/*`](/docs/config/tasks)). This setting defines conditions that a
project must meet in order for inheritance to occur. If the setting is not defined, or no conditions
are defined, the configuration is inherited by all projects.

The following conditions are supported:

- `file`, `files` - Inherit for projects that contain specific files.

- `language`, `languages` - Inherit for projects that belong to specific [`language`](/docs/config/project#language)s.

- `layer`, `layers` - Inherit for projects that belong to specific [`layer`](/docs/config/project#layer)s.

- `stack`, `stacks` - Inherit for projects that belong to specific [`stack`](/docs/config/project#stack)s.

- `tag`, `tags` - Inherit for projects that have specific [`tags`](/docs/config/project#tags).

- `toolchain`, `toolchains` - Inherit for projects that belong to specific [`toolchains`](/docs/config/project#toolchains).

One or many conditions can be defined, and all conditions must be met for inheritance to occur. For
example, the following configuration will only be inherited by Node.js frontend libraries.

.moon/tasks/node-frontend-library.yml

```
inheritedBy:  toolchain: 'node'  stack: 'frontend'  layer: 'library'
```

Each condition supports a single value or an array of values. For example, the previous example
could be rewritten to inherit for both Node.js or Deno frontend libraries.

.moon/tasks/js-frontend-library.yml

```
inheritedBy:  toolchains: ['node', 'deno']  stack: 'frontend'  layer: 'library'
```

### Clauses

The `tags` and `toolchains` conditions support special clauses `and`, `or` (the default), and `not`
for matching more complex scenarios.

```
inheritedBy:  toolchains:    or: ['javascript', 'typescript']    not: ['ruby']  layer: 'library'
```

## Merge strategies

When a [global task](/docs/config/tasks#tasks) and [local task](/docs/config/project#tasks) of the same
name exist, they are merged into a single task. To accomplish this, one of many
[merge strategies](/docs/config/project#options) can be used.

Merging is applied to the parameters [`args`](/docs/config/project#args),
[`deps`](/docs/config/project#deps), [`env`](/docs/config/project#env-1),
[`inputs`](/docs/config/project#inputs), [`outputs`](/docs/config/project#outputs), and
[`toolchains`](/docs/config/project#toolchains), using the [`merge`](/docs/config/project#merge),
[`mergeArgs`](/docs/config/project#mergeargs), [`mergeDeps`](/docs/config/project#mergedeps),
[`mergeEnv`](/docs/config/project#mergeenv), [`mergeInputs`](/docs/config/project#mergeinputs),
[`mergeOutputs`](/docs/config/project#mergeoutputs) and
[`mergeToolchains`](/docs/config/project#mergetoolchains) options respectively. Each of these options
support one of the following strategy values.

- `append` (default) - Values found in the local task are merged after the values found in the global task. For example, this strategy is useful for toggling flag arguments.

- `prepend` - Values found in the local task are merged before the values found in the global task. For example, this strategy is useful for applying option arguments that must come before positional arguments.

- `preserve` - Preserve the original global task values. This should rarely be used, but exists for situations where an inheritance chain is super long and complex, but we simply want to the base values. v1.29.0

- `replace` - Values found in the local task entirely replaces the values in the global task. This strategy is useful when you need full control.

All 3 of these strategies are demonstrated below, with a somewhat contrived example, but you get the
point.

```
# Globaltasks:  build:    command:      - 'webpack'      - '--mode'      - 'production'      - '--color'    deps:      - 'designSystem:build'    inputs:      - '/webpack.config.js'    outputs:      - 'build/'# Localtasks:  build:    args: '--no-color --no-stats'    deps:      - 'reactHooks:build'    inputs:      - 'webpack.config.js'    options:      mergeArgs: 'append'      mergeDeps: 'prepend'      mergeInputs: 'replace'# Merged resulttasks:  build:    command:      - 'webpack'      - '--mode'      - 'production'      - '--color'      - '--no-color'      - '--no-stats'    deps:      - 'reactHooks:build'      - 'designSystem:build'    inputs:      - 'webpack.config.js'    outputs:      - 'build/'    options:      mergeArgs: 'append'      mergeDeps: 'prepend'      mergeInputs: 'replace'
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
