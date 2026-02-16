----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/run
- Keywords: moon, moonrepo, docs, monorepo, build, commands, run
- Summary: The `moon run` (or `moon r`) command will run one or many [targets](/docs/concepts/target) and all of
----

Source: https://moonrepo.dev/docs/commands/run

# run

The `moon run` (or `moon r`) command will run one or many [targets](/docs/concepts/target) and all of
its dependencies in topological order. Each run will incrementally cache each task, improving speed
and development times... over time. View the official [Run a task](/docs/run-task) and
[Cheat sheet](/docs/cheat-sheet#tasks) articles for more information!

```
# Run `lint` in project `app`$ moon run app:lint# Run `dev` in project `client` and `server`$ moon run client:dev server:dev# Run `test` in all projects$ moon run :test# Run `test` in all projects with tag `frontend`$ moon run '#frontend:test'# Run `format` in default project$ moon run format# Run `build` in projects matching the query$ moon run :build --query "language=javascript && projectLayer=library"
```

info

The default behavior for `moon run` is to "fail fast", meaning that any failed task will immediately
abort execution of the entire action graph. Use `moon exec --on-failure continue` for alternative
behavior.

### Arguments

- `...` - [Targets](/docs/concepts/target) or project relative tasks to run.

- `[-- ]` - Additional arguments to [pass to the underlying command](/docs/run-task#passing-arguments-to-the-underlying-command).

### Options

Inherits all options from [`moon exec`](/docs/commands/exec) except for `--on-failure`.

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

- [`tasks`](/docs/config/tasks#tasks) in `.moon/tasks/all.yml`

- [`tasks`](/docs/config/project#tasks) in `moon.yml`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
