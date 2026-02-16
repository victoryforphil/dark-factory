----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/how-it-works/task-graph
- Keywords: moon, moonrepo, docs, monorepo, build, how it works, task graph
- Summary: The task graph is a representation of all configured
----

Source: https://moonrepo.dev/docs/how-it-works/task-graph

# Task graph

The task graph is a representation of all configured
[tasks in the workspace](/docs/config/workspace#projects) and their relationships between each other,
and is represented internally as a directed acyclic graph (DAG). This graph is derived from
information in the [project graph](/docs/how-it-works/project-graph). Below is a visual representation of a task
graph.

info

The [`moon task-graph`](/docs/commands/task-graph) command can be used to view the structure of your
workspace.

## Relationships

A relationship is between a dependent (downstream task) and a dependency/requirement (upstream
task). Relationships are derived explicitly with the task [`deps`](/docs/config/project#deps) setting,
and fall into 1 of 2 categories:

### Required

These are dependencies that are required to run and complete with a success, before the owning task
can run. If a required dependency fails, then the owning task will abort.

### Optional

The opposite of [required](#required), these are dependencies that can either a) not exist during
task inheritance, or b) run and fail without aborting the owning task.

## What is the graph used for?

Great question, the task graph is extremely important for running tasks (duh), and it also:

- Is fed into the [action graph](/docs/how-it-works/action-graph) that can be executed in topological order.

- Determines affected tasks in [continuous integration](/docs/guides/ci) workflows.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
