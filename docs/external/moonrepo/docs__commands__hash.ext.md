----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/hash
- Keywords: moon, moonrepo, docs, monorepo, build, commands, hash
- Summary: [Skip to main content](http://moonrepo.dev/docs/commands/hash#__docusaurus_skipToContent_fallback)
----

Source: https://moonrepo.dev/docs/commands/hash

hash | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/hash#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/hash#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/hash#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/hash#) 
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

*   [Migrations](http://moonrepo.dev/docs/commands/hash#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [hash](http://moonrepo.dev/docs/commands/hash) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

hash
====

v2.0.0
Use the `moon hash` command to inspect the contents and sources of a generated hash, also known as the hash manifest. This is extremely useful in debugging task inputs.

`$ moon hash 0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fdde# Query hash using short form$ moon hash 0b55b234`

By default, this will output the contents of the hash manifest (which is JSON), and the fully qualified resolved hash.

`Hash: 0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fdde{  "command": "build",  "args": ["./build"]  // ...}`

The command can also be output raw JSON by passing the `--json` flag.

### Comparing hashes[​](http://moonrepo.dev/docs/commands/hash#comparing-hashes "Direct link to Comparing hashes")

The command can also be used to compare two hashes by diffing their contents. Simply pass two hashes as arguments.

`# Diff between 2 hashes$ moon hash 0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fdde 2388552fee5a02062d0ef402bdc7232f0a447458b058c80ce9c3d0d4d7cfe171# Diff between 2 hashes using short form$ moon hash 0b55b234 2388552f`

By default, this will output the contents of a hash file (which is JSON), highlighting the differences between the left and right hashes. Lines that match will be printed in white, while the left differences printed in green, and right differences printed in red. If you use `git diff`, this will feel familiar to you.

`Left:  0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fddeRight: 2388552fee5a02062d0ef402bdc7232f0a447458b058c80ce9c3d0d4d7cfe171{	"command": "build",	"args": [+		"./dist"-		"./build"	],	...}`

The differences can also be output in JSON by passing the `--json` flag. The output has the following structure:

`{	left: string,	left_hash: string,	left_diffs: string[],	right: string,	right_hash: string,	right_diffs: string[],}`

### Options[​](http://moonrepo.dev/docs/commands/hash#options "Direct link to Options")

*   `--json` - Display the diff in JSON format.

### Configuration[​](http://moonrepo.dev/docs/commands/hash#configuration "Direct link to Configuration")

*   [`hasher`](http://moonrepo.dev/docs/config/workspace#hasher) in `.moon/workspace.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/hash.mdx)

[generate](http://moonrepo.dev/docs/commands/generate)

[init](http://moonrepo.dev/docs/commands/init)

*   [Comparing hashes](http://moonrepo.dev/docs/commands/hash#comparing-hashes)
*   [Options](http://moonrepo.dev/docs/commands/hash#options)
*   [Configuration](http://moonrepo.dev/docs/commands/hash#configuration)

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
