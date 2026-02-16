----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/clean
- Keywords: moon, moonrepo, docs, monorepo, build, commands, clean
- Summary: [Skip to main content](http://moonrepo.dev/docs/commands/clean#__docusaurus_skipToContent_fallback)
----

Source: https://moonrepo.dev/docs/commands/clean

clean | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/clean#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/clean#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/clean#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/clean#) 
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

*   [Migrations](http://moonrepo.dev/docs/commands/clean#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [clean](http://moonrepo.dev/docs/commands/clean) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

clean
=====

The `moon clean` command will clean the current workspace by deleting stale cache. For the most part, the action pipeline will clean automatically, but this command can be used to reset the workspace entirely.

`$ moon clean# Delete cache with a custom lifetime$ moon clean --lifetime '24 hours'`

### Options[​](http://moonrepo.dev/docs/commands/clean#options "Direct link to Options")

*   `--lifetime` - The maximum lifetime of cached artifacts before being marked as stale. Defaults to "7 days".

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/clean.mdx)

[ci](http://moonrepo.dev/docs/commands/ci)

[completions](http://moonrepo.dev/docs/commands/completions)

*   [Options](http://moonrepo.dev/docs/commands/clean#options)

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
