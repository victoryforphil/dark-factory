----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/action-graph
- Keywords: moon, moonrepo, docs, monorepo, build, commands, action graph
- Summary: [Skip to main content](http://moonrepo.dev/docs/commands/action-graph#__docusaurus_skipToContent_fallback)
----

Source: https://moonrepo.dev/docs/commands/action-graph

action-graph | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/action-graph#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/action-graph#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/action-graph#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/action-graph#) 
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

*   [Migrations](http://moonrepo.dev/docs/commands/action-graph#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [action-graph](http://moonrepo.dev/docs/commands/action-graph) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

action-graph
============

v1.15.0
The `moon action-graph [target]` (or `moon ag`) command will generate and serve a visual graph of all actions and tasks within the workspace, known as the [action graph](http://moonrepo.dev/docs/how-it-works/action-graph). In other tools, this is sometimes referred to as a dependency graph or task graph.

`# Run the visualizer locally$ moon action-graph# Export to DOT format$ moon action-graph --dot > graph.dot`

> A target can be passed to focus the graph, including dependencies _and_ dependents. For example, `moon action-graph app:build`.

### Arguments[​](http://moonrepo.dev/docs/commands/action-graph#arguments "Direct link to Arguments")

*   `[target]` - Optional target to focus.

### Options[​](http://moonrepo.dev/docs/commands/action-graph#options "Direct link to Options")

*   `--dependents` - Include dependents of the focused target.
*   `--dot` - Print the graph in DOT format.
*   `--host` - The host address. Defaults to `127.0.0.1`. v1.36.0
*   `--json` - Print the graph in JSON format.
*   `--port` - The port to bind to. Defaults to a random port. v1.36.0

### Configuration[​](http://moonrepo.dev/docs/commands/action-graph#configuration "Direct link to Configuration")

*   [`runner`](http://moonrepo.dev/docs/config/workspace#runner) in `.moon/workspace.yml`
*   [`tasks`](http://moonrepo.dev/docs/config/tasks#tasks) in `.moon/tasks/all.yml`
*   [`tasks`](http://moonrepo.dev/docs/config/project#tasks) in `moon.yml`

Example output[​](http://moonrepo.dev/docs/commands/action-graph#example-output "Direct link to Example output")
----------------------------------------------------------------------------------------------------------------

The following output is an example of the graph in DOT format.

`digraph {    0 [ label="SetupToolchain(node)" style=filled, shape=oval, fillcolor=black, fontcolor=white]    1 [ label="InstallWorkspaceDeps(node)" style=filled, shape=oval, fillcolor=gray, fontcolor=black]    2 [ label="SyncProject(node, node)" style=filled, shape=oval, fillcolor=gray, fontcolor=black]    3 [ label="RunTask(node:standard)" style=filled, shape=oval, fillcolor=gray, fontcolor=black]    1 -> 0 [ arrowhead=box, arrowtail=box]    2 -> 0 [ arrowhead=box, arrowtail=box]    3 -> 1 [ arrowhead=box, arrowtail=box]    3 -> 2 [ arrowhead=box, arrowtail=box]}`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/action-graph.mdx)

[Overview](http://moonrepo.dev/docs/commands/overview)

[bin](http://moonrepo.dev/docs/commands/bin)

*   [Arguments](http://moonrepo.dev/docs/commands/action-graph#arguments)
*   [Options](http://moonrepo.dev/docs/commands/action-graph#options)
*   [Configuration](http://moonrepo.dev/docs/commands/action-graph#configuration)
*   [Example output](http://moonrepo.dev/docs/commands/action-graph#example-output)

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
