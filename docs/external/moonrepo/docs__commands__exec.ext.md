----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/exec
- Keywords: moon, moonrepo, docs, monorepo, build, commands, exec
- Summary: [Skip to main content](http://moonrepo.dev/docs/commands/exec#__docusaurus_skipToContent_fallback)
----

Source: https://moonrepo.dev/docs/commands/exec

exec | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/exec#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/exec#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/exec#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/exec#) 
    *   [Setup workspace](http://moonrepo.dev/docs/setup-workspace)
    *   [Create a project](http://moonrepo.dev/docs/create-project)
    *   [Setup toolchain](http://moonrepo.dev/docs/setup-toolchain)
    *   [Create a task](http://moonrepo.dev/docs/create-task)
    *   [Run a task](http://moonrepo.dev/docs/run-task)
    *   [Migrate to moon](http://moonrepo.dev/docs/migrate-to-moon)

*   [Concepts](http://moonrepo.dev/docs/concepts) 
*   [Config files](http://moonrepo.dev/docs/config) 
*   [Editors](http://moonrepo.dev/docs/editors) 
*   [Commands](http://moonrepo.dev/docs/commands) 
    *   [Overview](http://moonrepo.dev/docs/commands/overview)
    *   [action-graph](http://moonrepo.dev/docs/commands/action-graph)
    *   [bin](http://moonrepo.dev/docs/commands/bin)
    *   [check](http://moonrepo.dev/docs/commands/check)
    *   [ci](http://moonrepo.dev/docs/commands/ci)
    *   [clean](http://moonrepo.dev/docs/commands/clean)
    *   [completions](http://moonrepo.dev/docs/commands/completions)
    *   [docker](http://moonrepo.dev/docs/commands/docker) 
    *   [exec](http://moonrepo.dev/docs/commands/exec)
    *   [ext](http://moonrepo.dev/docs/commands/ext)
    *   [extension](http://moonrepo.dev/docs/commands/extension) 
    *   [generate](http://moonrepo.dev/docs/commands/generate)
    *   [hash](http://moonrepo.dev/docs/commands/hash)
    *   [init](http://moonrepo.dev/docs/commands/init)
    *   [mcp](http://moonrepo.dev/docs/commands/mcp)
    *   [project](http://moonrepo.dev/docs/commands/project)
    *   [projects](http://moonrepo.dev/docs/commands/projects)
    *   [project-graph](http://moonrepo.dev/docs/commands/project-graph)
    *   [query](http://moonrepo.dev/docs/commands/query) 
    *   [run](http://moonrepo.dev/docs/commands/run)
    *   [setup](http://moonrepo.dev/docs/commands/setup)
    *   [sync](http://moonrepo.dev/docs/commands/sync) 
    *   [task](http://moonrepo.dev/docs/commands/task)
    *   [tasks](http://moonrepo.dev/docs/commands/tasks)
    *   [task-graph](http://moonrepo.dev/docs/commands/task-graph)
    *   [teardown](http://moonrepo.dev/docs/commands/teardown)
    *   [template](http://moonrepo.dev/docs/commands/template)
    *   [templates](http://moonrepo.dev/docs/commands/templates)
    *   [toolchain](http://moonrepo.dev/docs/commands/toolchain) 
    *   [upgrade](http://moonrepo.dev/docs/commands/upgrade)

*   [Migrations](http://moonrepo.dev/docs/commands/exec#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [exec](http://moonrepo.dev/docs/commands/exec) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

exec
====

v2.0.0
The `moon exec` (or `moon x`, or `moonx`) command is a low-level command for executing tasks in the action pipeline. It provides fine-grained control over how tasks are selected and executed, through command line options, making it ideal for custom workflows and advanced use cases.

The [`moon check`](http://moonrepo.dev/docs/commands/check), [`moon ci`](http://moonrepo.dev/docs/commands/ci), and [`moon run`](http://moonrepo.dev/docs/commands/run) commands are all built on top of `moon exec`, so be sure to check those out for more user-friendly abstractions!

`# Run `lint` in project `app`$ moon exec app:lint$ moonx app:lint# Run `dev` in project `client` and `server`$ moon exec client:dev server:dev$ moonx client:dev server:dev# Run `test` in all projects$ moon exec :test$ moonx :test# Run `test` in all projects with tag `frontend`$ moon exec '#frontend:test'$ moonx '#frontend:test'# Run `format` in the default project$ moon exec format$ moonx format# Run `build` in projects matching the query$ moon exec :build --query "language=javascript && projectLayer=library"`

Arguments[​](http://moonrepo.dev/docs/commands/exec#arguments "Direct link to Arguments")
-----------------------------------------------------------------------------------------

*   `...<target>` - [Targets](http://moonrepo.dev/docs/concepts/target) or project relative tasks to run.
*   `[-- <args>]` - Additional arguments to [pass to the underlying command](http://moonrepo.dev/docs/run-task#passing-arguments-to-the-underlying-command).

Options[​](http://moonrepo.dev/docs/commands/exec#options "Direct link to Options")
-----------------------------------------------------------------------------------

*   `-f`, `--force` - Force run and bypass cache, ignore changed files, and skip affected checks.
*   `-i`, `--interactive` - Run the pipeline and tasks interactively.
*   `-s`, `--summary [LEVEL]` - Print a summary of all actions that were ran in the pipeline.

### Workflow[​](http://moonrepo.dev/docs/commands/exec#workflow "Direct link to Workflow")

*   `--on-failure <ON>` - When a task fails, either bail the pipeline, or continue executing.
*   `--only-ci-tasks` - Filter tasks to those that only run in CI.
*   `--query <QUERY>` - Filter tasks based on the result of a query.
*   `--no-actions` - Run the pipeline without sync and setup related actions.

### Affected[​](http://moonrepo.dev/docs/commands/exec#affected "Direct link to Affected")

*   `--affected [BY]` - Only run tasks if affected by changed files. Optionally accepts "local" or "remote".
*   `--base <BASE>` - Base branch, commit, or revision to compare against.
*   `--head <HEAD>` - Current branch, commit, or revision to compare with.
*   `--status <STATUS>` - Filter changed files based on a changed status.
*   `--stdin` - Accept changed files from stdin for affected checks.

### Graph[​](http://moonrepo.dev/docs/commands/exec#graph "Direct link to Graph")

*   `--downstream <DEPTH>`, `--dependents <DEPTH>` - Control the depth of downstream dependents.
*   `--upstream <DEPTH>`, `--dependencies <DEPTH>` - Control the depth of upstream dependencies.

### Parallelism[​](http://moonrepo.dev/docs/commands/exec#parallelism "Direct link to Parallelism")

*   `--job <INDEX>` - Index of the current job (0 based).
*   `--job-total <TOTAL>` - Total amount of jobs to run.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/exec.mdx)

[setup](http://moonrepo.dev/docs/commands/docker/setup)

[ext](http://moonrepo.dev/docs/commands/ext)

*   [Arguments](http://moonrepo.dev/docs/commands/exec#arguments)
*   [Options](http://moonrepo.dev/docs/commands/exec#options)
    *   [Workflow](http://moonrepo.dev/docs/commands/exec#workflow)
    *   [Affected](http://moonrepo.dev/docs/commands/exec#affected)
    *   [Graph](http://moonrepo.dev/docs/commands/exec#graph)
    *   [Parallelism](http://moonrepo.dev/docs/commands/exec#parallelism)

Footer
------

###### Learn

*   [Docs](http://moonrepo.dev/docs)
*   [Guides](http://moonrepo.dev/docs/guides/ci)
*   [Blog](http://moonrepo.dev/blog)

###### Ecosystem

*   [Releases](https://github.com/moonrepo/moon/releases)
*   [Shared configs](https://github.com/moonrepo/moon-configs)
*   [Developer tools](https://github.com/moonrepo/dev)
*   [Examples repository](https://github.com/moonrepo/examples)

###### Support

*   [GitHub](https://github.com/moonrepo)
*   [Discord](https://discord.gg/qCh9MEynv2)
*   [Twitter](https://twitter.com/tothemoonrepo)

###### Contact us

Want to learn more about moonrepo? Have questions?

Subject 

Next

Backed by

Copyright © 2026, moonrepo, Inc.

[GitHub](https://github.com/moonrepo)[Discord](https://discord.gg/qCh9MEynv2)[Twitter](https://twitter.com/tothemoonrepo)

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
