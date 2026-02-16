----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/task
- Keywords: moon, moonrepo, docs, monorepo, build, commands, task
- Summary: The `moon task [target]` (or `moon t`) command will display information about a task that has been
----

Source: https://moonrepo.dev/docs/commands/task

# task

v1.1.0

The `moon task [target]` (or `moon t`) command will display information about a task that has been
configured and exists within a project. If a task does not exist, the program will return with a 1
exit code.

```
$ moon task web:build
```

### Arguments

- `[target]` - Fully qualified project + task target.

### Options

- `--json` - Print the task and its configuration as JSON.

## Example output

The following output is an example of what this command prints, using our very own
`@moonrepo/runtime` package.

```
RUNTIME:BUILDTask: buildProject: runtimeToolchain: nodeType: buildPROCESSCommand: packemon build --addFiles --addExports --declarationEnvironment variables:  - NODE_ENV = productionWorking directory: ~/Projects/moon/packages/runtimeRuns dependencies: ConcurrentlyRuns in CI: YesDEPENDS ON  - types:buildINHERITS FROM  - .moon/tasks/node.ymlINPUTS  - .moon/*.yml  - .moon/tasks/node.yml  - packages/runtime/package.json  - packages/runtime/src/**/*  - packages/runtime/tsconfig.*.json  - packages/runtime/tsconfig.json  - packages/runtime/types/**/*  - tsconfig.options.jsonOUTPUTS  - packages/runtime/cjs
```

### Configuration

- [`tasks`](/docs/config/tasks#tasks) in `.moon/tasks/all.yml`

- [`tasks`](/docs/config/project#tasks) in `moon.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
