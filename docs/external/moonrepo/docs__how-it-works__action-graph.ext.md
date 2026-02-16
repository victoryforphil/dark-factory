----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/how-it-works/action-graph
- Keywords: moon, moonrepo, docs, monorepo, build, how it works, action graph
- Summary: When you run a [task](/docs/config/project#tasks-1) on the command line, we generate an action graph to
----

Source: https://moonrepo.dev/docs/how-it-works/action-graph

# Action graph

When you run a [task](/docs/config/project#tasks-1) on the command line, we generate an action graph to
ensure [dependencies](/docs/config/project#deps) of tasks have ran before running run the primary task.

The action graph is a representation of all [tasks](/docs/concepts/task), derived from the
[project graph](/docs/how-it-works/project-graph) and [task graph](/docs/how-it-works/task-graph), and is also represented internally
as a directed acyclic graph (DAG).

## Actions

Unlike other task runners in the industry that represent each node in the graph as a task to run, we
represent each node in the graph as an action to perform. This allows us to be more flexible and
efficient with how we run tasks, and allows us to provide more functionality and automation than
other runners.

The following actions compose our action graph:

### Sync workspace

This is a common action that always runs and give's moon a chance to perform operations and health
checks across the entire workspace.

info

This action can be skipped by disabling the
[`pipeline.syncWorkspace`](/docs/config/workspace#syncworkspace) setting.

### Setup toolchain

The most important action in the graph is the setup toolchain action, which downloads and installs a
tier 3 language into the toolchain. For other tiers, this is basically a no-operation.

- When the tool has already been installed, this action will be skipped.

- Actions will be scoped by language and version, also known as a runtime. For example, `SetupToolchain(node:18.1.0)` or `SetupToolchain(deno:1.31.0)`.

- Tools that require a global binary (found on `PATH`) will display the version as "global". For example, `SetupToolchain(node:global)`.

info

This action can be skipped by setting the `MOON_SKIP_SETUP_TOOLCHAIN=true` environment variable. The
skip can be scoped per tool by setting the value to the tool name (`node`), and also by version
(`node:20.0.0`). Supports a comma-separated list.

### Setup environmentv1.35.0

This action runs after the toolchain has been setup, but before dependencies are installed, so that
the development environment can be setup and configured. This includes operations such as modifying
a manifest (`package.json`, etc), updating configuration files, initializing venv's (Python), so on
and so forth.

### Setup protov1.39.0

This action runs before all toolchain related actions and ensures that [proto](/proto) has been
installed and is available for use. This is required for toolchains that will be downloaded and
installed.

### Install dependencies

Before we run a task, we ensure that all language dependencies (`node_modules` for example) have
been installed, by automatically installing them if we detect changes since the last run. We achieve
this by comparing lockfile modified timestamps, parsing manifest files, and hashing resolved
dependency versions.

- When dependencies do not need to be installed, this action will be skipped.

- Depending on the language and configuration, we may install dependencies in a project (`InstallProjectDeps`), or in the workspace root for all projects (`InstallWorkspaceDeps`).

- Actions will be scoped by language and version, also known as a runtime. For example, `InstallWorkspaceDeps(node:18.1.0)` or `InstallProjectDeps(node:18.1.0, example)`.

info

This action can be skipped by disabling the
[`pipeline.installDependencies`](/docs/config/workspace#installdependencies) setting.

### Sync project

To ensure a consistently healthy project and repository, we run a process known as syncing
everytime a task is ran. This action will run sync operations for all toolchains associated with
the project.

info

This action can be skipped by disabling the
[`pipeline.syncProject`](/docs/config/workspace#syncproject) setting.

### Run task

The primary action in the graph is the run [task](/docs/concepts/task) action, which runs a project's
task as a child process, derived from a [target](/docs/concepts/target). Tasks can depend on other
tasks, and they'll be effectively orchestrated and executed by running in topological order using a
thread pool.

### Run interactive task

Like the base run task, but runs the [task interactively](/docs/concepts/task#interactive) with stdin
capabilities. All interactive tasks are run in isolation in the graph.

### Run persistent task

Like the base run task, but runs the [task in a persistent process](/docs/concepts/task#persistent)
that never exits. All persistent tasks are run in parallel as the last batch in the graph.

## What is the graph used for?

Without the action graph, tasks would not efficiently run, or possibly at all! The graph helps to
run tasks in parallel, in the correct order, and to ensure a reliable outcome.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
