----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/run-task
- Keywords: moon, moonrepo, docs, monorepo, build, run task
- Summary: Even though we've [created a task](/docs/create-task), it's not useful unless we run it, which is done
----

Source: https://moonrepo.dev/docs/run-task

# Run a task

2 min

Even though we've [created a task](/docs/create-task), it's not useful unless we run it, which is done
with the [`moon run `](/docs/commands/run) command. This command requires a single argument, a
[primary target](/docs/concepts/target), which is the pairing of a scope and task name. In the example
below, our project is `app`, the task is `build`, and the target is `app:build`.

```
$ moon run app:build
```

When this command is ran, it will do the following:

- Generate a directed acyclic graph, known as the action (dependency) graph.

- Insert [`deps`](/docs/config/project#deps) as targets into the graph.

- Insert the primary target into the graph.

- Run all tasks in the graph in parallel and in topological order (the dependency chain).

- For each task, calculate [hashes](/docs/concepts/cache) and either: On cache hit, exit early and return the last run.

- On cache miss, run the task and generate a new cache.

## Running dependents

moon will always run upstream dependencies ([`deps`](/docs/config/project#deps)) before running the
primary target, as their outputs may be required for the primary target to function correctly.

However, if you're working on a project that is shared and consumed by other projects, you may want
to verify that downstream dependents have not been indirectly broken by any changes. This can be
achieved by passing the `--dependents` option, which will run dependent targets after the primary
target.

```
$ moon run app:build --dependents
```

## Running based on affected files only

By default [`moon run`](/docs/commands/run) will always run the target, regardless if files have
actually changed. However, this is typically fine because of our
[smart hashing & cache layer](/docs/concepts/cache). With that being said, if you'd like to only run a
target if files have changed, pass the `--affected` flag.

```
$ moon run app:build --affected
```

Under the hood, we extract locally touched (created, modified, staged, etc) files from your
configured [VCS](/docs/config/workspace#vcs), and exit early if no files intersect with the task's
[inputs](/docs/config/project#inputs).

### Using remote changes

If you'd like to determine affected files based on remote changes instead of local changes, pass the
`--remote` flag. This will extract touched files by comparing the current `HEAD` against the
[`vcs.defaultBranch`](/docs/config/workspace#defaultbranch).

```
$ moon run app:build --affected --remote
```

### Filtering based on change status

We can take this a step further by filtering down affected files based on a change status, using the
`--status` option. This option accepts the following values: `added`, `deleted`, `modified`,
`staged`, `unstaged`, `untracked`. If not provided, the option defaults to all.

```
$ moon run app:build --affected --status deleted
```

Multiple status can be provided by passing the `--status` option multiple times.

```
$ moon run app:build --affected --status deleted --status modified
```

## Passing arguments to the underlying command

If you'd like to pass arbitrary arguments to the underlying task command, in addition to the already
defined [`args`](/docs/config/project#args), you can pass them after `--`. These arguments are appended
as-is.

```
$ moon run app:build -- --force
```

The `--` delimiter and any arguments must be defined last on the command line.

## Advanced run targeting

By this point you should have a basic understanding of how to run tasks, but with moon, we want to
provide support for advanced workflows and development scenarios. For example, running a target in
all projects:

```
$ moon run :build
```

Or perhaps running a target based on a query:

```
$ moon run :build --query "language=[javascript, typescript]"
```

Jump to the official [`moon run` documentation](/docs/commands/run) for more examples!

## Next steps

[Migrate to moon](/docs/migrate-to-moon)[Learn about tasks](/docs/concepts/task)[Learn about `moon run`](/docs/commands/run)

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
