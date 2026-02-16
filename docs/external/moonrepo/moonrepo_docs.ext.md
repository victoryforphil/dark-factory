----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:38:46.200Z
- Source root: https://moonrepo.dev/docs
- Scope: 171 pages under /docs (excluding /docs/tags)
- Keywords: moon, monorepo, task graph, hashing, cache, toolchain, proto, workspace, codegen, query, migration, CI
- Summary: moonrepo documentation centers on deterministic task orchestration, project/dependency graphs, and reproducible toolchains with proto-backed version pinning.
----

## /docs

Source: https://moonrepo.dev/docs

moonrepo is a productivity platform that aims to eliminate pain points for both developers and companies, by automating tiresome and complex workflows, and improving the overall developer experience.

We currently achieve this through the following tools and services:

moon[​](http://moonrepo.dev/docs#moon "Direct link to moon")
------------------------------------------------------------

[moon](https://moonrepo.dev/moon) is a repository _m_ anagement, _o_ rganization, _o_ rchestration, and _n_ otification tool for the web ecosystem, written in Rust. Many of the concepts within moon are heavily inspired from Bazel and other popular build systems, but tailored for our [supported languages](http://moonrepo.dev/docs#supported-languages).

You can think of a moon as a tool that sits firmly in the middle between Bazel (high complexity, full structure), and make/just/etc scripts (low complexity, no structure).

### Why use moon?[​](http://moonrepo.dev/docs#why-use-moon "Direct link to Why use moon?")

Working in a language's ecosystem can be very involved, especially when it comes to managing a repository effectively. Which language version to use? Which dependency manager to use? How to use packages? Or how to build packages? So on and so forth. moon aims to streamline this entire process and provide a first-class developer experience.

*   **Increased productivity** - With [Rust](https://www.rust-lang.org/) as our foundation, we can ensure robust speeds, high performance, and low memory usage. Instead of long builds blocking you, focus on your work.
*   **Exceptional developer experience** - As veterans of developer tooling, we're well aware of the pain points and frustrations. Our goal is to mitigate and overcome these obstacles.
*   **Incremental adoption** - At its core, moon has been designed to be adopted incrementally and is _not_ an "all at once adoption". Migrate project-by-project, or task-by-task, it's up to you!
*   **Reduced tasks confusion** - Tasks (for example, `package.json` scripts) can become unwieldy, very quickly. No more duplicating the same task into every project, or reverse-engineering which root scripts to use. With moon, all you need to know is the project name, and a task name.
*   **Ensure correct versions** - Whether it's a programming language or dependency manager, ensure the same version of each tool is the same across _every_ developer's environment. No more wasted hours of debugging.
*   **Automation built-in** - When applicable, moon will automatically install dependencies (`node_modules`), or [sync project dependencies](https://moonrepo.dev/docs/config/toolchain#syncprojectworkspacedependencies), or even [sync TypeScript project references](https://moonrepo.dev/docs/config/toolchain#syncprojectreferences).
*   And of course, the amazing list of [features](http://moonrepo.dev/docs#features) below!

### Supported languages[​](http://moonrepo.dev/docs#supported-languages "Direct link to Supported languages")

moon's long-term vision is to robustly support multiple programming languages (and dependency managers) so that a repository composed of projects with differing languages and tools can all work in unison. This is a lofty vision that requires a massive amount of time and resources to achieve, and as such, is not available on initial release, but will gradually be supported over time.

To help achieve this vision, language support is broken down into 4 tiers, allowing us to incrementally integrate and improve them over time. The 4 tiers are as follows:

*   Tier 0**No direct integration** - Tool is not directly supported in moon, but can still be run using the ["system" task toolchain](https://moonrepo.dev/docs/faq#can-we-run-other-languages), which expects the tool to exist in the current environment.
*   Tier 1**Project categorization** - Projects can configure their primary [language in `moon.yml`](https://moonrepo.dev/docs/config/project#language), and have a dedicated Rust crate for metadata.
*   Tier 2**Ecosystem platformization** - moon deeply integrates with the language's ecosystem by parsing manifests, lockfiles, and other semantic files to infer dependencies, tasks, and other necessary information.
*   Tier 3**Toolchain integration** - Language is directly supported in the toolchain, configured in [`.moon/toolchains.yml`](https://moonrepo.dev/docs/config/toolchain), and will automatically be downloaded and installed.

|  | Tier 0 | Tier 1 | Tier 2 | Tier 3 |
| --- | --- | --- | --- | --- |
| Bash/Batch | 🟢 | 🟢 |  |  |
| Bun (JavaScript, TypeScript) | 🟢 | 🟢 | 🟢 | 🟢 |
| Deno (JavaScript, TypeScript) | 🟢 | 🟢 | 🟢 | 🟢 |
| Go | 🟢 | 🟢 | 🟢 | 🟢 |
| Node (JavaScript, TypeScript) | 🟢 | 🟢 | 🟢 | 🟢 |
| └─ npm, pnpm, yarn | 🟢 | ⚪️ | 🟢 | 🟢 |
| PHP | 🟢 | 🟢 |  |  |
| └─ Composer | 🟢 | ⚪️ |  |  |
| Python | 🟢 | 🟢 | 🟣 | 🟣 |
| └─ Pip | 🟢 | ⚪️ | 🟣 |  |
| Ruby | 🟢 | 🟢 |  |  |
| └─ Gems, Bundler | 🟢 | ⚪️ |  |  |
| Rust | 🟢 | 🟢 | 🟢 | 🟢 |
| └─ Cargo | 🟢 | ⚪️ | 🟢 | 🟢 |
| Other (Kotlin, Java, C#, ...) | 🟢 |  |  |  |

*   ⚪️ Not applicable
*   🟣 Partially supported (experimental)
*   🟢 Fully Supported

### Supported targets[​](http://moonrepo.dev/docs#supported-targets "Direct link to Supported targets")

Because moon is written in Rust, we only support targets that are explicitly compiled for, which are currently:

| Operating system | Architecture | Target |
| --- | --- | --- |
| macOS 64-bit | Intel | `x86_64-apple-darwin` |
| macOS 64-bit | ARM | `aarch64-apple-darwin` |
| Linux 64-bit | Intel GNU | `x86_64-unknown-linux-gnu` |
| Linux 64-bit | Intel musl | `x86_64-unknown-linux-musl` |
| Linux 64-bit | ARM GNU | `aarch64-unknown-linux-gnu` |
| Linux 64-bit | ARM musl | `aarch64-unknown-linux-musl` |
| Windows 64-bit | Intel | `x86_64-pc-windows-msvc` |

### Features[​](http://moonrepo.dev/docs#features "Direct link to Features")

#### Management[​](http://moonrepo.dev/docs#management "Direct link to Management")

*   **Smart hashing** - Collects inputs from multiple sources to ensure builds are deterministic and reproducible.
*   **Remote caching** - Persists builds, hashes, and caches between teammates and CI/CD environments.
*   **Integrated toolchain** - Automatically downloads and installs explicit versions of Node.js and other tools for consistency across the entire workspace or per project.
*   **Multi-platform** - Runs on common development platforms: Linux, macOS, and Windows.

#### Organization[​](http://moonrepo.dev/docs#organization "Direct link to Organization")

*   **Project graph** - Generates a project graph for dependency and dependent relationships.
*   **Code generation** - Easily scaffold new applications, libraries, tooling, and more!
*   **Dependency workspaces** - Works alongside package manager workspaces so that projects have distinct dependency trees.
*   **Code ownership** - Declare owners, maintainers, support channels, and more. Generate CODEOWNERS.

#### Orchestration[​](http://moonrepo.dev/docs#orchestration "Direct link to Orchestration")

*   **Dependency graph** - Generates a dependency graph to increase performance and reduce workloads.
*   **Action pipeline** - Executes actions in parallel and in order using a thread pool and our dependency graph.
*   **Action distribution**Coming soon - Distributes actions across multiple machines to increase throughput.
*   **Incremental builds** - With our smart hashing, only rebuild projects that have been touched since the last build.

#### Notification[​](http://moonrepo.dev/docs#notification "Direct link to Notification")

*   **Flakiness detection** - Reduce flaky builds with automatic retries and passthrough settings.
*   **Webhook events**Experimental - Receive a webhook for every event in the pipeline. Useful for metrics gathering and insights.
*   **Terminal notifications**Experimental - Receives notifications in your chosen terminal when builds are successful... or are not.
*   **Git hooks** - Manage Git hooks to enforce workflows and requirements for contributors.

proto[​](http://moonrepo.dev/docs#proto "Direct link to proto")
---------------------------------------------------------------

[proto](https://moonrepo.dev/proto) is a version manager for your favorite programming languages. [View proto documentation](https://moonrepo.dev/docs/proto).

## /docs/cheat-sheet

Source: https://moonrepo.dev/docs/cheat-sheet

Don't have time to read the docs? Here's a quick cheat sheet to get you started.

Tasks[​](http://moonrepo.dev/docs/cheat-sheet#tasks "Direct link to Tasks")
---------------------------------------------------------------------------

Learn more about [tasks](https://moonrepo.dev/docs/concepts/task) and [targets](https://moonrepo.dev/docs/concepts/target).

#### Run all build and test tasks for all projects[​](http://moonrepo.dev/docs/cheat-sheet#run-all-build-and-test-tasks-for-all-projects "Direct link to Run all build and test tasks for all projects")

`moon check --all`

#### Run all build and test tasks in a project[​](http://moonrepo.dev/docs/cheat-sheet#run-all-build-and-test-tasks-in-a-project "Direct link to Run all build and test tasks in a project")

`moon check project`

#### Run all build and test tasks for closest project based on working directory[​](http://moonrepo.dev/docs/cheat-sheet#run-all-build-and-test-tasks-for-closest-project-based-on-working-directory "Direct link to Run all build and test tasks for closest project based on working directory")

`moon check`

#### Run a task in all projects[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-all-projects "Direct link to Run a task in all projects")

`moon run :task`

#### Run a task in all projects with a tag[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-all-projects-with-a-tag "Direct link to Run a task in all projects with a tag")

`moon run '#tag:task'# ORmoon run \#tag:task# ORmoon run :task --query "tag=tag"`

#### Run a task in a project[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-a-project "Direct link to Run a task in a project")

`moon run project:task`

#### Run multiple tasks in all projects[​](http://moonrepo.dev/docs/cheat-sheet#run-multiple-tasks-in-all-projects "Direct link to Run multiple tasks in all projects")

`moon run :task1 :task2`

#### Run multiple tasks in any project[​](http://moonrepo.dev/docs/cheat-sheet#run-multiple-tasks-in-any-project "Direct link to Run multiple tasks in any project")

`moon run projecta:task1 projectb:task2`

#### Run a task in applications, libraries, or tools[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-applications-libraries-or-tools "Direct link to Run a task in applications, libraries, or tools")

`moon run :task --query "projectLayer=application"`

#### Run a task in projects of a specific language[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-projects-of-a-specific-language "Direct link to Run a task in projects of a specific language")

`moon run :task --query "language=typescript"`

#### Run a task in projects matching a keyword[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-projects-matching-a-keyword "Direct link to Run a task in projects matching a keyword")

`moon run :task --query "project~react-*"`

#### Run a task in projects based on file path[​](http://moonrepo.dev/docs/cheat-sheet#run-a-task-in-projects-based-on-file-path "Direct link to Run a task in projects based on file path")

`moon run :task --query "projectSource~packages/*"`

Task configuration[​](http://moonrepo.dev/docs/cheat-sheet#task-configuration "Direct link to Task configuration")
------------------------------------------------------------------------------------------------------------------

Learn more about [available options](https://moonrepo.dev/docs/config/project#tasks).

#### Disable caching[​](http://moonrepo.dev/docs/cheat-sheet#disable-caching "Direct link to Disable caching")

moon.yml

`tasks:  example:    # ...    options:      cache: false`

#### Re-run flaky tasks[​](http://moonrepo.dev/docs/cheat-sheet#re-run-flaky-tasks "Direct link to Re-run flaky tasks")

moon.yml

`tasks:  example:    # ...    options:      retryCount: 3`

#### Depend on tasks from parent project's dependencies[​](http://moonrepo.dev/docs/cheat-sheet#depend-on-tasks-from-parent-projects-dependencies "Direct link to Depend on tasks from parent project's dependencies")

moon.yml

`# Also inferred from the languagedependsOn:  - 'project-a'  - 'project-b'tasks:  example:    # ...    deps:      - '^:build'`

#### Depend on tasks from arbitrary projects[​](http://moonrepo.dev/docs/cheat-sheet#depend-on-tasks-from-arbitrary-projects "Direct link to Depend on tasks from arbitrary projects")

moon.yml

`tasks:  example:    # ...    deps:      - 'other-project:task'`

#### Run dependencies serially[​](http://moonrepo.dev/docs/cheat-sheet#run-dependencies-serially "Direct link to Run dependencies serially")

moon.yml

`tasks:  example:    # ...    deps:      - 'first'      - 'second'      - 'third'    options:      runDepsInParallel: false`

moon.yml

`tasks:  example:    command: 'noop'    deps:      - 'app:watch'      - 'backend:start'      - 'tailwind:watch'    preset: 'server'`

> The `local` or `persistent` settings are required for this to work.

Languages[​](http://moonrepo.dev/docs/cheat-sheet#languages "Direct link to Languages")
---------------------------------------------------------------------------------------

#### Run system binaries available on `PATH`[​](http://moonrepo.dev/docs/cheat-sheet#run-system-binaries-available-on-path "Direct link to run-system-binaries-available-on-path")

moon.yml

`language: 'bash' # batch, etctasks:  example:    command: 'printenv'`

moon.yml

`tasks:  example:    command: 'printenv'    toolchain: 'system'`

#### Run language binaries not supported in moon's toolchain[​](http://moonrepo.dev/docs/cheat-sheet#run-language-binaries-not-supported-in-moons-toolchain "Direct link to Run language binaries not supported in moon's toolchain")

moon.yml

`language: 'ruby'tasks:  example:    command: 'rubocop'    toolchain: 'system'`

#### Run npm binaries (Node.js)[​](http://moonrepo.dev/docs/cheat-sheet#run-npm-binaries-nodejs "Direct link to Run npm binaries (Node.js)")

moon.yml

`language: 'javascript' # typescripttasks:  example:    command: 'eslint'`

moon.yml

`tasks:  example:    command: 'eslint'    toolchain: 'node'`

## /docs/commands

Source: https://moonrepo.dev/docs/commands

[Skip to main content](http://moonrepo.dev/docs/commands#__docusaurus_skipToContent_fallback)

[![Image 1: moon](https://moonrepo.dev/img/logo.svg)](https://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands#)

*   [**moon** Build system for managing codebases](https://moonrepo.dev/moon)
*   [**proto** Multi-language version manager](https://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands#)

*   [**moon**](https://moonrepo.dev/docs)
*   [**proto**](https://moonrepo.dev/docs/proto)

[Guides](https://moonrepo.dev/docs/guides/ci)[Blog](https://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

1.   [Home](https://moonrepo.dev/)
2.   [Commands](https://moonrepo.dev/docs/commands) 

warning

Documentation is currently for [moon v2](https://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

[📄️Overview ----------- The following options are available for all moon commands.](https://moonrepo.dev/docs/commands/overview)[📄️action-graph --------------- The moon action-graph [target] (or moon ag) command will generate and serve a visual graph of](https://moonrepo.dev/docs/commands/action-graph)[📄️bin ------ The moon bin command will return an absolute path to a tool's binary within the](https://moonrepo.dev/docs/commands/bin)[📄️check -------- The moon check [...projects] (or moon c) command will run all](https://moonrepo.dev/docs/commands/check)[📄️ci ----- The moon ci command is a special command that should be ran in a continuous integration (CI)](https://moonrepo.dev/docs/commands/ci)[📄️clean -------- The moon clean command will clean the current workspace by deleting stale cache. For the most](https://moonrepo.dev/docs/commands/clean)[📄️completions -------------- The moon completions command will generate moon command and argument completions for your current](https://moonrepo.dev/docs/commands/completions)[🗃️docker --------- 4 items](https://moonrepo.dev/docs/commands/docker)[📄️exec ------- The moon exec (or moon x, or moonx) command is a low-level command for executing tasks in the](https://moonrepo.dev/docs/commands/exec)[📄️ext ------ The moon ext command will execute an extension (a WASM plugin) that has been configured with](https://moonrepo.dev/docs/commands/ext)[🗃️extension ------------ 2 items](https://moonrepo.dev/docs/commands/extension)[📄️generate ----------- The moon generate (or moon g) command will generate code (files and folders) from a](https://moonrepo.dev/docs/commands/generate)[📄️hash ------- Use the moon hash command to inspect the contents and sources of a generated hash, also known as](https://moonrepo.dev/docs/commands/hash)[📄️init ------- The moon init command will initialize moon into a repository and scaffold necessary config files](https://moonrepo.dev/docs/commands/init)[📄️mcp ------ The moon mcp command will start an MCP server that listens for](https://moonrepo.dev/docs/commands/mcp)[📄️project ---------- The moon project [id] (or moon p) command will display all available information about a project](https://moonrepo.dev/docs/commands/project)[📄️projects ----------- The moon projects command will list all projects configured in the workspace as a table of](https://moonrepo.dev/docs/commands/projects)[📄️project-graph ---------------- The moon project-graph [id] (or moon pg) command will generate and serve a visual graph of all](https://moonrepo.dev/docs/commands/project-graph)[🗃️query -------- 4 items](https://moonrepo.dev/docs/commands/query)[📄️run ------ The moon run (or moon r) command will run one or many targets and all of](https://moonrepo.dev/docs/commands/run)[📄️setup -------- The moon setup command can be used to setup the developer and pipeline environments. It achieves](https://moonrepo.dev/docs/commands/setup)[🗃️sync ------- 4 items](https://moonrepo.dev/docs/commands/sync)[📄️task ------- The moon task [target] (or moon t) command will display information about a task that has been](https://moonrepo.dev/docs/commands/task)[📄️tasks -------- The moon tasks command will list all tasks available in the workspace as a table of information.](https://moonrepo.dev/docs/commands/tasks)[📄️task-graph ------------- The moon task-graph [target] (or moon tg) command will generate and serve a visual graph of all](https://moonrepo.dev/docs/commands/task-graph)[📄️teardown ----------- The moon teardown command, as its name infers, will teardown and clean the current environment,](https://moonrepo.dev/docs/commands/teardown)[📄️template ----------- The moon template [id] command will display information about a template, its files, and](https://moonrepo.dev/docs/commands/template)[📄️templates ------------ The moon templates command will list all templates available for code generation.](https://moonrepo.dev/docs/commands/templates)[🗃️toolchain ------------ 2 items](https://moonrepo.dev/docs/commands/toolchain)[📄️upgrade ---------- The moon upgrade command can be used to upgrade your current moon binary (if installed globally)](https://moonrepo.dev/docs/commands/upgrade)

## /docs/commands/action-graph

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

## /docs/commands/bin

Source: https://moonrepo.dev/docs/commands/bin

bin | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/bin#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/bin#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/bin#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/bin#) 
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

*   [Migrations](http://moonrepo.dev/docs/commands/bin#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [bin](http://moonrepo.dev/docs/commands/bin) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

bin
===

The `moon bin <toolchain>` command will return an absolute path to a tool's binary within the toolchain. If a tool has not been configured or installed, this will return a 1 or 2 exit code with no value respectively.

`$ moon bin node/Users/example/.proto/tools/node/x.x.x/bin/node`

> A tool is considered "not configured" when not in use, for example, querying yarn/pnpm when the package manager is configured for "npm". A tool is considered "not installed", when it has not been downloaded and installed into the tools directory.

### Arguments[​](http://moonrepo.dev/docs/commands/bin#arguments "Direct link to Arguments")

*   `<toolchain>` - Name of the toolchain to query.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/bin.mdx)

[action-graph](http://moonrepo.dev/docs/commands/action-graph)

[check](http://moonrepo.dev/docs/commands/check)

*   [Arguments](http://moonrepo.dev/docs/commands/bin#arguments)

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

## /docs/commands/check

Source: https://moonrepo.dev/docs/commands/check

check | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/check#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/check#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/check#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/check#) 
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

*   [Migrations](http://moonrepo.dev/docs/commands/check#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [check](http://moonrepo.dev/docs/commands/check) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

check
=====

The `moon check [...projects]` (or `moon c`) command will run _all_[build and test tasks](http://moonrepo.dev/docs/concepts/task#types) for one or many projects. This is a convenience command for verifying the current state of a project, instead of running multiple [`moon run`](http://moonrepo.dev/docs/commands/run) commands.

`# Check project by name$ moon check app# Check multiple projects by name$ moon check client server# Check project at current working directory$ moon check --closest# Check ALL projects (may be costly)$ moon check --all`

### Arguments[​](http://moonrepo.dev/docs/commands/check#arguments "Direct link to Arguments")

*   `[...id]` - List of project IDs or aliases to explicitly check, as defined in [`projects`](http://moonrepo.dev/docs/config/workspace#projects).

### Options[​](http://moonrepo.dev/docs/commands/check#options "Direct link to Options")

Inherits all options from [`moon exec`](http://moonrepo.dev/docs/commands/exec) except for `--on-failure`.

*   `--all` - Run check for all projects in the workspace.
*   `--closest` - Run check for the closest project starting from the current working directory.

### Configuration[​](http://moonrepo.dev/docs/commands/check#configuration "Direct link to Configuration")

*   [`projects`](http://moonrepo.dev/docs/config/workspace#projects) in `.moon/workspace.yml`
*   [`tasks`](http://moonrepo.dev/docs/config/tasks#tasks) in `.moon/tasks/all.yml`
*   [`tasks`](http://moonrepo.dev/docs/config/project#tasks) in `moon.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/check.mdx)

[bin](http://moonrepo.dev/docs/commands/bin)

[ci](http://moonrepo.dev/docs/commands/ci)

*   [Arguments](http://moonrepo.dev/docs/commands/check#arguments)
*   [Options](http://moonrepo.dev/docs/commands/check#options)
*   [Configuration](http://moonrepo.dev/docs/commands/check#configuration)

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

## /docs/commands/ci

Source: https://moonrepo.dev/docs/commands/ci

ci | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/ci#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/ci#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/ci#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/ci#) 
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

*   [Migrations](http://moonrepo.dev/docs/commands/ci#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [ci](http://moonrepo.dev/docs/commands/ci) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

ci
==

The `moon ci` command is a special command that should be ran in a continuous integration (CI) environment, as it does all the heavy lifting necessary for effectively running tasks.

By default this will run all tasks that are affected by touched files and have the [`runInCI`](http://moonrepo.dev/docs/config/project#runinci) task option enabled.

`$ moon ci`

However, you can also provide a list of targets to explicitly run, which will still be filtered down by `runInCI`.

`$ moon ci :build :lint`

> View the official [continuous integration guide](http://moonrepo.dev/docs/guides/ci) for a more in-depth example of how to utilize this command.

### Arguments[​](http://moonrepo.dev/docs/commands/ci#arguments "Direct link to Arguments")

*   `...[target]` - [Targets](http://moonrepo.dev/docs/concepts/target) to run.

### Options[​](http://moonrepo.dev/docs/commands/ci#options "Direct link to Options")

Inherits all options from [`moon exec`](http://moonrepo.dev/docs/commands/exec) except for `--affected`, `--on-failure`, and `--only-ci-tasks`.

### Configuration[​](http://moonrepo.dev/docs/commands/ci#configuration "Direct link to Configuration")

*   [`tasks`](http://moonrepo.dev/docs/config/tasks#tasks) in `.moon/tasks/all.yml`
*   [`tasks`](http://moonrepo.dev/docs/config/project#tasks) in `moon.yml`
*   [`tasks.*.options.runInCI`](http://moonrepo.dev/docs/config/project#runinci) in `moon.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/ci.mdx)

[check](http://moonrepo.dev/docs/commands/check)

[clean](http://moonrepo.dev/docs/commands/clean)

*   [Arguments](http://moonrepo.dev/docs/commands/ci#arguments)
*   [Options](http://moonrepo.dev/docs/commands/ci#options)
*   [Configuration](http://moonrepo.dev/docs/commands/ci#configuration)

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

## /docs/commands/clean

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

## /docs/commands/completions

Source: https://moonrepo.dev/docs/commands/completions

completions | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/completions#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/completions#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/completions#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/completions#) 
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

*   [Migrations](http://moonrepo.dev/docs/commands/completions#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [completions](http://moonrepo.dev/docs/commands/completions) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

completions
===========

The `moon completions` command will generate moon command and argument completions for your current shell. This command will write to stdout, which can then be redirected to a file of your choice.

`$ moon completions > ./path/to/write/to`

### Options[​](http://moonrepo.dev/docs/commands/completions#options "Direct link to Options")

*   `--shell` - Shell to explicitly generate for.

### Examples[​](http://moonrepo.dev/docs/commands/completions#examples "Direct link to Examples")

*   Bash
*   Fish
*   Zsh

If using [bash-completion](https://github.com/scop/bash-completion).

`mkdir -p ~/.bash_completion.dmoon completions > ~/.bash_completion.d/moon.sh`

Otherwise write the file to a common location, and source it in your profile.

`mkdir -p ~/.bash_completionsmoon completions > ~/.bash_completions/moon.sh# In your profilesource ~/.bash_completions/moon.sh`

Write the file to Fish's completions directory.

`mkdir -p ~/.config/fish/completionsmoon completions > ~/.config/fish/completions/moon.fish`

If using [oh-my-zsh](https://ohmyz.sh/) (the `_` prefix is required).

`mkdir -p ~/.oh-my-zsh/completionsmoon completions > ~/.oh-my-zsh/completions/_moon# Reload shell (or restart terminal)omz reload`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/completions.mdx)

[clean](http://moonrepo.dev/docs/commands/clean)

[docker](http://moonrepo.dev/docs/commands/docker)

*   [Options](http://moonrepo.dev/docs/commands/completions#options)
*   [Examples](http://moonrepo.dev/docs/commands/completions#examples)

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

## /docs/commands/docker

Source: https://moonrepo.dev/docs/commands/docker

docker | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/docker#__docusaurus_skipToContent_fallback)

[![Image 2: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/docker#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/docker#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/docker#) 
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
        *   [file](http://moonrepo.dev/docs/commands/docker/file)
        *   [prune](http://moonrepo.dev/docs/commands/docker/prune)
        *   [scaffold](http://moonrepo.dev/docs/commands/docker/scaffold)
        *   [setup](http://moonrepo.dev/docs/commands/docker/setup)

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

*   [Migrations](http://moonrepo.dev/docs/commands/docker#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [docker](http://moonrepo.dev/docs/commands/docker) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

docker
======

Operations for integrating with Docker and Dockerfiles.

[📄️file ------- The moon docker file command can be used to generate a multi-staged Dockerfile for a](http://moonrepo.dev/docs/commands/docker/file)[📄️prune -------- The moon docker prune command will reduce the overall filesize of the Docker environment by](http://moonrepo.dev/docs/commands/docker/prune)[📄️scaffold ----------- The moon docker scaffold command creates multiple repository skeletons for use](http://moonrepo.dev/docs/commands/docker/scaffold)[📄️setup -------- The moon docker setup command will efficiently install dependencies for focused projects. This is](http://moonrepo.dev/docs/commands/docker/setup)

[completions](http://moonrepo.dev/docs/commands/completions)

[file](http://moonrepo.dev/docs/commands/docker/file)

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

## /docs/commands/docker/file

Source: https://moonrepo.dev/docs/commands/docker/file

1.   [Home](https://moonrepo.dev/)

warning

Documentation is currently for [moon v2](https://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

v1.27.0

The `moon docker file <project>` command can be used to generate a multi-staged `Dockerfile` for a project, that takes full advantage of Docker's layer caching, and is primarily for production deploys (this should not be used for development).

`$ moon docker file <project>`

As mentioned above, the generated `Dockerfile` uses a multi-stage approach, where each stage is broken up into the following:

*   `base` - The base stage, which simply installs moon for a chosen Docker image. This stage requires Bash.
*   `skeleton` - Scaffolds workspace and sources repository skeletons using [`moon docker scaffold`](https://moonrepo.dev/docs/commands/docker/scaffold).
*   `build` - Copies required sources, installs the toolchain using [`moon docker setup`](https://moonrepo.dev/docs/commands/docker/setup), optionally builds the project, and optionally prunes the image using [`moon docker prune`](https://moonrepo.dev/docs/commands/docker/prune).
*   `start` - Runs the project after it has been built. This is typically starting an HTTP server, or executing a binary.

info

View the official [Docker usage guide](https://moonrepo.dev/docs/guides/docker) for a more in-depth example of how to utilize this command.

### Arguments[​](http://moonrepo.dev/docs/commands/docker/file#arguments "Direct link to Arguments")

*   `<name>` - Name or alias of a project, as defined in [`projects`](https://moonrepo.dev/docs/config/workspace#projects).
*   `[dest]` - Destination to write the file, relative from the project root. Defaults to `Dockerfile`.

### Options[​](http://moonrepo.dev/docs/commands/docker/file#options "Direct link to Options")

*   `--defaults` - Use default options instead of prompting in the terminal.
*   `--build-task` - Name of a task to build the project. Defaults to the [`docker.file.buildTask`](https://moonrepo.dev/docs/config/project#buildtask) setting, or prompts in the terminal.
*   `--image` - Base Docker image to use. Defaults to an image derived from the toolchain, or prompts in the terminal.
*   `--no-prune` - Do not prune the workspace in the build stage.
*   `--no-toolchain` - Do not use the toolchain and instead use system binaries.
*   `--start-task` - Name of a task to start the project. Defaults to the [`docker.file.startTask`](https://moonrepo.dev/docs/config/project#starttask) setting, or prompts in the terminal.

### Configuration[​](http://moonrepo.dev/docs/commands/docker/file#configuration "Direct link to Configuration")

*   [`docker.file`](https://moonrepo.dev/docs/config/project#file) in `moon.yml`

## /docs/commands/docker/prune

Source: https://moonrepo.dev/docs/commands/docker/prune

docker prune | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/docker/prune#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/docker/prune#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/docker/prune#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/docker/prune#) 
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
        *   [file](http://moonrepo.dev/docs/commands/docker/file)
        *   [prune](http://moonrepo.dev/docs/commands/docker/prune)
        *   [scaffold](http://moonrepo.dev/docs/commands/docker/scaffold)
        *   [setup](http://moonrepo.dev/docs/commands/docker/setup)

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

*   [Migrations](http://moonrepo.dev/docs/commands/docker/prune#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [docker](http://moonrepo.dev/docs/commands/docker) 
4.   [prune](http://moonrepo.dev/docs/commands/docker/prune) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

docker prune
============

The `moon docker prune` command will reduce the overall filesize of the Docker environment by installing production only dependencies for projects that were scaffolded, and removing any applicable extraneous files.

`$ moon docker prune`

info

View the official [Docker usage guide](http://moonrepo.dev/docs/guides/docker) for a more in-depth example of how to utilize this command.

caution

This command _must be_ ran after [`moon docker scaffold`](http://moonrepo.dev/docs/commands/docker/scaffold) and is typically ran within a `Dockerfile`! The [`moon docker file`](http://moonrepo.dev/docs/commands/docker/file) command can be used to generate a `Dockerfile`.

### Configuration[​](http://moonrepo.dev/docs/commands/docker/prune#configuration "Direct link to Configuration")

*   [`docker.prune`](http://moonrepo.dev/docs/config/workspace#prune) in `.moon/workspace.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/docker/prune.mdx)

[file](http://moonrepo.dev/docs/commands/docker/file)

[scaffold](http://moonrepo.dev/docs/commands/docker/scaffold)

*   [Configuration](http://moonrepo.dev/docs/commands/docker/prune#configuration)

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

## /docs/commands/docker/scaffold

Source: https://moonrepo.dev/docs/commands/docker/scaffold

The `moon docker scaffold <...projects>` command creates multiple repository skeletons for use within `Dockerfile`s, to effectively take advantage of Docker's layer caching. It utilizes the [project graph](https://moonrepo.dev/docs/config/workspace#projects) to copy only critical files, like manifests, lockfiles, and configuration.

`# Scaffold a skeleton to .moon/docker$ moon docker scaffold <project>`

info

View the official [Docker usage guide](https://moonrepo.dev/docs/guides/docker) for a more in-depth example of how to utilize this command.

### Arguments[​](http://moonrepo.dev/docs/commands/docker/scaffold#arguments "Direct link to Arguments")

*   `<...projects>` - List of project names or aliases to scaffold sources for, as defined in [`projects`](https://moonrepo.dev/docs/config/workspace#projects).

### Configuration[​](http://moonrepo.dev/docs/commands/docker/scaffold#configuration "Direct link to Configuration")

*   [`docker.scaffold`](https://moonrepo.dev/docs/config/workspace#scaffold) in `.moon/workspace.yml` (entire workspace)
*   [`docker.scaffold`](https://moonrepo.dev/docs/config/project#scaffold) in `moon.yml` (per project)

How it works[​](http://moonrepo.dev/docs/commands/docker/scaffold#how-it-works "Direct link to How it works")
-------------------------------------------------------------------------------------------------------------

This command may seem like magic, but it's relative simple thanks to moon's infrastructure and its project graph. When the command is ran, we generate 2 skeleton structures in `.moon/docker` (be sure to gitignore this). One for the workspace, and the other for sources.

warning

Because scaffolding uses the project graph, it requires all projects with a `package.json` to be [configured in moon](https://moonrepo.dev/docs/config/workspace#projects). Otherwise, moon will fail to copy all required files and builds may fail.

### Workspace[​](http://moonrepo.dev/docs/commands/docker/scaffold#workspace "Direct link to Workspace")

The workspace skeleton mirrors the project folder structure of the repository 1:1, and only copies files required for dependencies to install. This is typically manifests (`package.json`), lockfiles (`yarn.lock`, etc), other critical configs, and `.moon` itself. This is necessary for package managers to install dependencies (otherwise they will fail), and for dependencies to be layer cached in Docker.

An example of this skeleton using Yarn may look like the following:

`.moon/docker/workspace/├── .moon/├── .yarn/├── apps/│   ├── client/│   │   └── package.json│   └── server/│       └── package.json├── packages/│   ├── foo/│   │   └── package.json│   ├── bar/│   │   └── package.json│   └── baz/│       └── package.json├── .yarnrc.yml├── package.json└── yarn.lock`

### Sources[​](http://moonrepo.dev/docs/commands/docker/scaffold#sources "Direct link to Sources")

The sources skeleton is not a 1:1 mirror of the repository, and instead is the source files of a project (passed as an argument to the command), and all of its dependencies. This allows [`moon run`](https://moonrepo.dev/docs/commands/run) and other commands to work within the `Dockerfile`, and avoid having to `COPY . .` the entire repository.

Using our example workspace above, our sources skeleton would look like the following, assuming our `client` project is passed as an argument, and this project depends on the `foo` and `baz` projects.

`.moon/docker/sources/├── apps/│   └── client/|       ├── src/|       ├── tests/|       ├── public/|       ├── package.json|       ├── tsconfig.json│       └── (anything else)└── packages/    ├── foo/    │   ├── lib/    │   ├── src/    │   ├── package.json    │   ├── tsconfig.json    │   └── (anything else)    └── baz/        ├── lib/        ├── src/        ├── package.json        ├── tsconfig.json        └── (anything else)`

## /docs/commands/docker/setup

Source: https://moonrepo.dev/docs/commands/docker/setup

docker setup | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/docker/setup#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/docker/setup#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/docker/setup#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/docker/setup#) 
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
        *   [file](http://moonrepo.dev/docs/commands/docker/file)
        *   [prune](http://moonrepo.dev/docs/commands/docker/prune)
        *   [scaffold](http://moonrepo.dev/docs/commands/docker/scaffold)
        *   [setup](http://moonrepo.dev/docs/commands/docker/setup)

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

*   [Migrations](http://moonrepo.dev/docs/commands/docker/setup#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [docker](http://moonrepo.dev/docs/commands/docker) 
4.   [setup](http://moonrepo.dev/docs/commands/docker/setup) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

docker setup
============

The `moon docker setup` command will efficiently install dependencies for focused projects. This is an all-in-one command for tool and dependency installations, and should replace `npm install` and other commands.

`$ moon docker setup`

info

View the official [Docker usage guide](http://moonrepo.dev/docs/guides/docker) for a more in-depth example of how to utilize this command.

caution

This command _must be_ ran after [`moon docker scaffold`](http://moonrepo.dev/docs/commands/docker/scaffold) and is typically ran within a `Dockerfile`! The [`moon docker file`](http://moonrepo.dev/docs/commands/docker/file) command can be used to generate a `Dockerfile`.

### Configuration[​](http://moonrepo.dev/docs/commands/docker/setup#configuration "Direct link to Configuration")

*   [`*`](http://moonrepo.dev/docs/config/toolchain) in `.moon/toolchains.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/docker/setup.mdx)

[scaffold](http://moonrepo.dev/docs/commands/docker/scaffold)

[exec](http://moonrepo.dev/docs/commands/exec)

*   [Configuration](http://moonrepo.dev/docs/commands/docker/setup#configuration)

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

## /docs/commands/exec

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

## /docs/commands/ext

Source: https://moonrepo.dev/docs/commands/ext

ext | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/ext#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/ext#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/ext#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/ext#) 
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

*   [Migrations](http://moonrepo.dev/docs/commands/ext#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [ext](http://moonrepo.dev/docs/commands/ext) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

ext
===

v1.20.0
The `moon ext <id>` command will execute an extension (a WASM plugin) that has been configured with the [`extensions`](http://moonrepo.dev/docs/config/workspace#extensions) setting in [`.moon/workspace.yml`](http://moonrepo.dev/docs/config). View our official [extensions guide](http://moonrepo.dev/docs/guides/extensions) for more information.

`$ moon ext download -- --url https://github.com/moonrepo/moon/archive/refs/tags/v1.19.3.zip`

Extensions typically support command line arguments, which _must_ be passed after a `--` separator (as seen above). Any arguments before the separator will be passed to the `moon ext` command itself.

caution

This command requires an internet connection if the extension's `.wasm` file must be downloaded from a URL, and it hasn't been cached locally.

### Arguments[​](http://moonrepo.dev/docs/commands/ext#arguments "Direct link to Arguments")

*   `<id>` - Name of the extension to execute.
*   `[-- <args>]` - Arguments to pass to the extension.

### Configuration[​](http://moonrepo.dev/docs/commands/ext#configuration "Direct link to Configuration")

*   [`extensions`](http://moonrepo.dev/docs/config/workspace#extensions) in `.moon/workspace.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/ext.mdx)

[exec](http://moonrepo.dev/docs/commands/exec)

[extension](http://moonrepo.dev/docs/commands/extension)

*   [Arguments](http://moonrepo.dev/docs/commands/ext#arguments)
*   [Configuration](http://moonrepo.dev/docs/commands/ext#configuration)

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

## /docs/commands/extension

Source: https://moonrepo.dev/docs/commands/extension

extension | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/extension#__docusaurus_skipToContent_fallback)

[![Image 2: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/extension#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/extension#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/extension#) 
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
        *   [add](http://moonrepo.dev/docs/commands/extension/add)
        *   [info](http://moonrepo.dev/docs/commands/extension/info)

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

*   [Migrations](http://moonrepo.dev/docs/commands/extension#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [extension](http://moonrepo.dev/docs/commands/extension) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

extension
=========

Manage extension plugins.

[📄️add ------ The moon extension add [plugin] command will add a extension to the workspace by injecting a](http://moonrepo.dev/docs/commands/extension/add)[📄️info ------- The moon extension info [plugin] command will display detailed information about a extension.](http://moonrepo.dev/docs/commands/extension/info)

[ext](http://moonrepo.dev/docs/commands/ext)

[add](http://moonrepo.dev/docs/commands/extension/add)

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

## /docs/commands/extension/add

Source: https://moonrepo.dev/docs/commands/extension/add

extension add | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/extension/add#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/extension/add#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/extension/add#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/extension/add#) 
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
        *   [add](http://moonrepo.dev/docs/commands/extension/add)
        *   [info](http://moonrepo.dev/docs/commands/extension/info)

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

*   [Migrations](http://moonrepo.dev/docs/commands/extension/add#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [extension](http://moonrepo.dev/docs/commands/extension) 
4.   [add](http://moonrepo.dev/docs/commands/extension/add) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

extension add
=============

v2.0.0
The `moon extension add <id> [plugin]` command will add a extension to the workspace by injecting a configuration block into `.moon/extensions.yml`. To do this, the command will download the WASM plugin, extract information, and call initialize functions.

For built-in extensions, the [plugin locator](http://moonrepo.dev/docs/guides/wasm-plugins#configuring-plugin-locations) argument is optional, and will be derived from the identifier.

`$ moon extension add download`

For third-party extensions, the [plugin locator](http://moonrepo.dev/docs/guides/wasm-plugins#configuring-plugin-locations) argument is required, and must point to the WASM plugin.

`$ moon extension add custom https://example.com/path/to/plugin.wasm`

### Arguments[​](http://moonrepo.dev/docs/commands/extension/add#arguments "Direct link to Arguments")

*   `<id>` - ID of the extension to use.
*   `[plugin]` - Optional [plugin locator](http://moonrepo.dev/docs/guides/wasm-plugins#configuring-plugin-locations) for third-party extensions.

### Options[​](http://moonrepo.dev/docs/commands/extension/add#options "Direct link to Options")

*   `--minimal` - Generate minimal configurations and sane defaults.
*   `--yes` - Skip all prompts and enables tools based on file detection.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/extension/add.mdx)

[extension](http://moonrepo.dev/docs/commands/extension)

[info](http://moonrepo.dev/docs/commands/extension/info)

*   [Arguments](http://moonrepo.dev/docs/commands/extension/add#arguments)
*   [Options](http://moonrepo.dev/docs/commands/extension/add#options)

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

## /docs/commands/extension/info

Source: https://moonrepo.dev/docs/commands/extension/info

extension info | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/extension/info#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/extension/info#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/extension/info#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/extension/info#) 
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
        *   [add](http://moonrepo.dev/docs/commands/extension/add)
        *   [info](http://moonrepo.dev/docs/commands/extension/info)

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

*   [Migrations](http://moonrepo.dev/docs/commands/extension/info#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [extension](http://moonrepo.dev/docs/commands/extension) 
4.   [info](http://moonrepo.dev/docs/commands/extension/info) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

extension info
==============

v2.0.0
The `moon extension info <id> [plugin]` command will display detailed information about a extension. To do this, the command will download the WASM plugin, extract information, and call specific functions.

For built-in extensions, the [plugin locator][locator] argument is optional, and will be derived from the identifier.

`$ moon extension info download`

For third-party extensions, the [plugin locator][locator] argument is required, and must point to the WASM plugin.

`$ moon extension info custom https://example.com/path/to/plugin.wasm`

### Arguments[​](http://moonrepo.dev/docs/commands/extension/info#arguments "Direct link to Arguments")

*   `<id>` - ID of the extension to view.
*   `[plugin]` - Optional [plugin locator][locator] for third-party extensions.

Example output[​](http://moonrepo.dev/docs/commands/extension/info#example-output "Direct link to Example output")
------------------------------------------------------------------------------------------------------------------

`Extension ─────────────────────────────────────────────────────────────────  Download a file from a URL into the current working directory.  ID: download  Title: Download  Version: 1.0.0APIs ──────────────────────────────────────────────────────────────────────  ⚫️ define_extension_config  🟢 execute_extension  ⚫️ extend_command  ⚫️ extend_project_graph  ⚫️ extend_task_command  ⚫️ extend_task_script  ⚫️ initialize_extension  🟢 register_extension (required)  ⚫️ sync_project  ⚫️ sync_workspace`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/extension/info.mdx)

[add](http://moonrepo.dev/docs/commands/extension/add)

[generate](http://moonrepo.dev/docs/commands/generate)

*   [Arguments](http://moonrepo.dev/docs/commands/extension/info#arguments)
*   [Example output](http://moonrepo.dev/docs/commands/extension/info#example-output)

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

## /docs/commands/generate

Source: https://moonrepo.dev/docs/commands/generate

generate | moonrepo
===============

[Skip to main content](http://moonrepo.dev/docs/commands/generate#__docusaurus_skipToContent_fallback)

[![Image 1: moon](http://moonrepo.dev/img/logo.svg)](http://moonrepo.dev/)

[Products](http://moonrepo.dev/docs/commands/generate#)
*   [**moon**Build system for managing codebases](http://moonrepo.dev/moon)
*   [**proto**Multi-language version manager](http://moonrepo.dev/proto)

[Docs](http://moonrepo.dev/docs/commands/generate#)
*   [**moon**](http://moonrepo.dev/docs)
*   [**proto**](http://moonrepo.dev/docs/proto)

[Guides](http://moonrepo.dev/docs/guides/ci)[Blog](http://moonrepo.dev/blog)[GitHub](https://github.com/moonrepo)

Search

*   [Introduction](http://moonrepo.dev/docs)
*   [Install moon](http://moonrepo.dev/docs/install)
*   [How it works](http://moonrepo.dev/docs/how-it-works) 
*   [Getting started](http://moonrepo.dev/docs/commands/generate#) 
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

*   [Migrations](http://moonrepo.dev/docs/commands/generate#) 
*   [Cheat sheet](http://moonrepo.dev/docs/cheat-sheet)
*   [Feature comparison](http://moonrepo.dev/docs/comparison)
*   [Terminology](http://moonrepo.dev/docs/terminology)
*   [FAQ](http://moonrepo.dev/docs/faq)
*   [Changelog](https://github.com/moonrepo/moon/releases)

1.   [Home](http://moonrepo.dev/)
2.   [Commands](http://moonrepo.dev/docs/commands) 
3.   [generate](http://moonrepo.dev/docs/commands/generate) 

warning

Documentation is currently for [moon v2](http://moonrepo.dev/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be[found here](https://moonrepo.github.io/website-v1/).

On this page

generate
========

The `moon generate <id>` (or `moon g`) command will generate code (files and folders) from a pre-defined template of the same name, using an interactive series of prompts. Templates are located based on the [`generator.templates`](http://moonrepo.dev/docs/config/workspace#templates) setting.

`# Generate code from a template$ moon generate npm-package# Generate code from a template to a target directory$ moon generate npm-package --to ./packages/example# Generate code while declaring custom variable values$ moon generate npm-package --to ./packages/example -- --name "@company/example"# Create a new template$ moon generate react-app --template`

> View the official [code generation guide](http://moonrepo.dev/docs/guides/codegen) for a more in-depth example of how to utilize this command.

### Arguments[​](http://moonrepo.dev/docs/commands/generate#arguments "Direct link to Arguments")

*   `<id>` - ID of the template to generate.
*   `[-- <vars>]` - Additional arguments to override default variable values.

### Options[​](http://moonrepo.dev/docs/commands/generate#options "Direct link to Options")

*   `--defaults` - Use the default value of all variables instead of prompting the user.
*   `--dry-run` - Run entire generator process without writing files.
*   `--force` - Force overwrite any existing files at the destination.
*   `--template` - Create a new template with the provided name.
*   `--to` - Destination to write files to, relative from the current working directory. If not defined, will be prompted during generation.

### Configuration[​](http://moonrepo.dev/docs/commands/generate#configuration "Direct link to Configuration")

*   [`generator`](http://moonrepo.dev/docs/config/workspace#generator) in `.moon/workspace.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/generate.mdx)

[info](http://moonrepo.dev/docs/commands/extension/info)

[hash](http://moonrepo.dev/docs/commands/hash)

*   [Arguments](http://moonrepo.dev/docs/commands/generate#arguments)
*   [Options](http://moonrepo.dev/docs/commands/generate#options)
*   [Configuration](http://moonrepo.dev/docs/commands/generate#configuration)

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

## /docs/commands/hash

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

## /docs/commands/init

Source: https://moonrepo.dev/docs/commands/init

# init

The `moon init` command will initialize moon into a repository and scaffold necessary config files
by creating a `.moon` folder.

```
$ moon init# In another directory$ moon init ./app
```

### Arguments

- `[dest]` - Destination to initialize and scaffold into. Defaults to `.` (current working directory).

### Options

- `--force` - Overwrite existing config files if they exist.

- `--minimal` - Generate minimal configurations and sane defaults.

- `--yes` - Skip all prompts and enables tools based on file detection.

## /docs/commands/mcp

Source: https://moonrepo.dev/docs/commands/mcp

# mcp

v1.37.0

The `moon mcp` command will start an [MCP](https://modelcontextprotocol.io) server that listens for
requests from AI assistants. This allows for agentic workflows in your favorite editor.

```
$ moon mcp
```

info

This command should not be ran manually and instead should be integrated into your editor. View the
[MCP guide](/docs/guides/mcp) for more information.

## /docs/commands/overview

Source: https://moonrepo.dev/docs/commands/overview

# Overview

The following options are available for all moon commands.

- `--cache ` - The mode for [cache operations](#caching).

- `--color` - Force [colored output](#colors) for moon (not tasks).

- `--concurrency`, `-c` - Maximum number of threads to utilize.

- `--dump` - Dump a [trace profile](#profiling) to the working directory.

- `--help` - Display the help menu for the current command.

- `--log ` - The lowest [log level to output](#logging).

- `--log-file ` - Write logs to the defined file.

- `--quiet`, `-q` - Hide all non-important moon specific terminal output.

- `--theme` - Terminal theme to write output in. v1.35.0

- `--version` - Display the version of the CLI.

## Caching

We provide a powerful [caching layer](/docs/concepts/cache), but sometimes you need to debug failing or
broken tasks, and this cache may get in the way. To circumvent this, we support the `--cache` global
option, or the `MOON_CACHE` environment variable, both of which accept one of the following values.

- `off` - Turn off caching entirely. Every task will run fresh, including dependency installs.

- `read` - Read existing items from the cache, but do not write to them.

- `read-write` (default) - Read and write items to the cache.

- `write` - Do not read existing cache items, but write new items to the cache.

```
$ moon run app:build --cache off# Or$ MOON_CACHE=off moon run app:build
```

## Colors

Colored output is a complicated subject, with differing implementations and standards across tooling
and operating systems. moon aims to normalize this as much as possible, by doing the following:

- By default, moon colors are inherited from your terminal settings (`TERM` and `COLORTERM` environment variables).

- Colors can be force enabled by passing the `--color` option (preferred), or `MOON_COLOR` or `FORCE_COLOR` environment variables.

```
$ moon app:build --color run# Or$ MOON_COLOR=2 moon run app:build
```

When forcing colors with `MOON_COLOR` or `FORCE_COLOR`, you may set it to one of the following
numerical values for the desired level of color support. This is automatically inferred if you use
`--color`.

- `0` - No colors

- `1` - 16 colors (standard terminal colors)

- `2` - 256 colors

- `3` - 16 million colors (truecolor)

### Themesv1.35.0

By default, moon assumes a dark themed terminal is being used, and will output colors accordingly.
However, if you use a light theme, these colors are hard to read. To mitigate this, we support
changing the theme with the `--theme` global option, or the `MOON_THEME` environment variable.

```
$ moon run app:build --theme light# Or$ MOON_THEME=light moon run app:build
```

### Piped output

When tasks (child processes) are piped, colors and ANSI escape sequences are lost, since the target
is not a TTY and we do not implement a PTY. This is a common pattern this is quite annoying.
However, many tools and CLIs support a `--color` option to work around this limitation and to always
force colors, even when not a TTY.

To mitigate this problem as a whole, and to avoid requiring `--color` for every task, moon supports
the [`pipeline.inheritColorsForPipedTasks`](/docs/config/workspace#inheritcolorsforpipedtasks)
configuration setting. When enabled, all piped child processes will inherit the color settings of
the currently running terminal.

## Concurrency

The `--concurrency` option or `MOON_CONCURRENCY` environment variable can be used to control the
maximum amount of threads to utilize in our thread pool. If not defined, defaults to the number of
operating system cores.

```
$ moon run app:build --concurrency 1# Or$ MOON_CONCURRENCY=1 moon run app:build
```

## Debugging

At minimum, most debugging can be done by passing [`--log trace`](#logging) on the command line and
sifting through the logs. We also provide the following environment variables to toggle output.

- `MOON_DEBUG_PROCESS_ENV` - By default moon hides the environment variables (except for `MOON_`) passed to processes to avoid leaking sensitive information. However, knowing what environment variables are passed around is helpful in debugging. Declare this variable to reveal the entire environment.

- `MOON_DEBUG_PROCESS_INPUT` - By default moon truncates the stdin passed to processes to avoid thrashing the console with a large input string. However, knowing what input is passed around is helpful in debugging. Declare this variable to reveal the entire input.

- `MOON_DEBUG_PROTO_INSTALL` - Debug the proto installation process.

- `MOON_DEBUG_REMOTE` - Debug our remote caching implementation by including additional logging output, and printing internal connection errors.

- `MOON_DEBUG_WASM` - Debug our WASM plugins by including additional logging output, and optionally dumping memory/core profiles.

## Logging

By default, moon aims to output as little as possible, as we want to preserve the original output of
the command's being ran, excluding warnings and errors. This is managed through log levels, which
can be defined with the `--log` global option, or the `MOON_LOG` environment variable. The following
levels are supported, in priority order.

- `off` - Turn off logging entirely.

- `error` - Only show error logs.

- `warn` - Only show warning logs and above.

- `info` (default) - Only show info logs and above.

- `debug` - Only show debug logs and above.

- `trace` - Show all logs, including network requests and child processes.

- `verbose` - Like `trace` but also includes span information. v1.35.0

```
$ moon run app:build --log trace# Or$ MOON_LOG=trace moon run app:build
```

### Writing logs to a file

moon can dump the logs from a command to a file using the `--logFile` option, or the `MOON_LOG_FILE`
environment variable. The dumped logs will respect the `--log` option and filter the logs piped to
the output file.

```
$ moon run app:build --logFile=output.log# Or$ MOON_LOG_FILE=output.log moon run app:build
```

## Profilingv1.26.0

When the `--dump` option or `MOON_DUMP` environment variable is set, moon will generate a trace
profile and dump it to the current working directory. This profile can be opened with Chrome (via
`chrome://tracing`) or [Perfetto](https://ui.perfetto.dev/).

This profile will display many of the operations within moon as a flame chart, allowing you to
inspect and debug slow operations.

## /docs/commands/project

Source: https://moonrepo.dev/docs/commands/project

# project

The `moon project [id]` (or `moon p`) command will display all available information about a project
that has been configured and exists within the graph. If a project does not exist, the program will
return with a 1 exit code.

```
$ moon project web
```

### Arguments

- `[id]` - ID or alias of a project, as defined in [`projects`](/docs/config/workspace#projects).

### Options

- `--json` - Print the project and its configuration as JSON.

- `--no-tasks` - Do not list tasks for the project.

## Example output

The following output is an example of what this command prints, using our very own
`@moonrepo/runtime` package.

```
RUNTIMEProject: runtimeAlias: @moonrepo/runtimeSource: packages/runtimeRoot: ~/Projects/moon/packages/runtimeToolchain: nodeLanguage: typescriptStack: unknownType: libraryDEPENDS ON  - types (implicit, production)INHERITS FROM  - .moon/tasks/node.ymlTASKSbuild:  › packemon build --addFiles --addExports --declarationformat:  › prettier --check --config ../../prettier.config.js --ignore-path ../../.prettierignore --no-error-on-unmatched-pattern .lint:  › eslint --cache --cache-location ./.eslintcache --color --ext .js,.ts,.tsx --ignore-path ../../.eslintignore --exit-on-fatal-error --no-error-on-unmatched-pattern --report-unused-disable-directives .lint-fix:  › eslint --cache --cache-location ./.eslintcache --color --ext .js,.ts,.tsx --ignore-path ../../.eslintignore --exit-on-fatal-error --no-error-on-unmatched-pattern --report-unused-disable-directives . --fixtest:  › jest --cache --color --preset jest-preset-moon --passWithNoTeststypecheck:  › tsc --buildFILE GROUPSconfigs:  - packages/runtime/*.{js,json}sources:  - packages/runtime/src/**/*  - packages/runtime/types/**/*tests:  - packages/runtime/tests/**/*
```

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

- [`project`](/docs/config/project#project) in `moon.yml`

## /docs/commands/project-graph

Source: https://moonrepo.dev/docs/commands/project-graph

# project-graph

The `moon project-graph [id]` (or `moon pg`) command will generate and serve a visual graph of all
configured projects as nodes, with dependencies between as edges, and can also output the graph in
[Graphviz DOT format](https://graphviz.org/doc/info/lang.html).

```
# Run the visualizer locally$ moon project-graph# Export to DOT format$ moon project-graph --dot > graph.dot
```

A project name can be passed to focus the graph to only that project and its dependencies. For
example, `moon project-graph app`.

### Arguments

- `[id]` - Optional ID or alias of a project to focus, as defined in [`projects`](/docs/config/workspace#projects).

### Options

- `--dependents` - Include direct dependents of the focused project.

- `--dot` - Print the graph in DOT format.

- `--host` - The host address. Defaults to `127.0.0.1`. v1.36.0

- `--json` - Print the graph in JSON format.

- `--port` - The port to bind to. Defaults to a random port. v1.36.0

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

## Example output

The following output is an example of the graph in DOT format.

```
digraph {    0 [ label="(workspace)" style=filled, shape=circle, fillcolor=black, fontcolor=white]    1 [ label="runtime" style=filled, shape=circle, fillcolor=gray, fontcolor=black]    2 [ label="website" style=filled, shape=circle, fillcolor=gray, fontcolor=black]    0 -> 1 [ arrowhead=none]    0 -> 2 [ arrowhead=none]}
```

## /docs/commands/projects

Source: https://moonrepo.dev/docs/commands/projects

# projects

v2.0.0

The `moon projects` command will list all projects configured in the workspace as a table of
information.

```
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮│Project          Source                    Stack             Layer             Toolchains                                Description                                                   ││───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────││types            packages/types            frontend          library           javascript, node, typescript, yarn                                                                      ││website          website                   frontend          application       javascript, node, typescript, yarn        A static website powered by Docusaurus.                       │╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

info

Use [`moon query projects`](/docs/commands/query/projects) for advanced querying and filtering of projects.

### Options

- `--json` - Print the projects as JSON.

## /docs/commands/query

Source: https://moonrepo.dev/docs/commands/query

- [Home](/)
- [Commands](/docs/commands)
- [query](/docs/commands/query)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# query

Query information about moon, its projects, their tasks, the environment, the pipeline, and many other aspects of the workspace.

[📄️ affectedUse the moon query affected sub-command to query for all affected projects and tasks based on the](/docs/commands/query/affected)

[📄️ changed-filesUse the moon query changed-files sub-command to query for a list of changed files (added,](/docs/commands/query/changed-files)

[📄️ projectsUse the moon query projects sub-command to query information about all projects in the project](/docs/commands/query/projects)

[📄️ tasksUse the moon query tasks sub-command to query task information for all projects in the project](/docs/commands/query/tasks)

[project-graph](/docs/commands/project-graph)

[affected](/docs/commands/query/affected)

## /docs/commands/query/affected

Source: https://moonrepo.dev/docs/commands/query/affected

# query affected

v2.0.0

Use the `moon query affected` sub-command to query for all affected projects and tasks based on the
state of the workspace and VCS.

```
# Return affected$ moon query affected# Return affected including dependency relationships$ moon query affected --upstream deep
```

This will output a map of projects and tasks as JSON. The output has the following structure:

```
{	projects: Record,	tasks: Record,}
```

### Options

- `--downstream` - Include downstream dependents. Supports "none" (default), "direct", "deep".

- `--upstream` - Include upstream dependencies. Supports "none", "direct", "deep" (default).

## /docs/commands/query/changed-files

Source: https://moonrepo.dev/docs/commands/query/changed-files

# query changed-files

Use the `moon query changed-files` sub-command to query for a list of changed files (added,
modified, deleted, etc) using the current VCS state. These are the same queries that
[`moon ci`](/docs/commands/ci) and [`moon run`](/docs/commands/run) use under the hood.

Touches files are determined using the following logic:

- If `--defaultBranch` is provided, and the current branch is the [`vcs.defaultBranch`](/docs/config/workspace#defaultbranch), then compare against the previous revision of the default branch (`HEAD~1`). This is what [continuous integration](/docs/guides/ci) uses.

- If `--local` is provided, changed files are based on your local index only (`git status`).

- Otherwise, then compare the defined base (`--base`) against head (`--head`).

```
# Return all files$ moon query changed-files# Return deleted files$ moon query changed-files --status deleted# Return all files between 2 revisions$ moon query changed-files --base  --head
```

This will output a list of workspace relative files as JSON. The output has the following structure:

```
{	files: string[],	options: QueryOptions,}
```

### Options

- `--default-branch` - When on the default branch, compare against the previous revision.

- `--base ` - Base branch, commit, or revision to compare against. Defaults to [`vcs.defaultBranch`](/docs/config/workspace#defaultbranch).

- `--head ` - Current branch, commit, or revision to compare with. Defaults to `HEAD`.

- `--local` - Gather files from the local state instead of remote.

- `--remote` - Gather files from the remote state instead of local.

- `--status ` - Filter files based on a changed status. Can be passed multiple times. Types: `all` (default), `added`, `deleted`, `modified`, `staged`, `unstaged`, `untracked`

### Configuration

- [`vcs`](/docs/config/workspace#vcs) in `.moon/workspace.yml`

## /docs/commands/query/projects

Source: https://moonrepo.dev/docs/commands/query/projects

# query projects

Use the `moon query projects` sub-command to query information about all projects in the project
graph. The project list can be filtered by passing a [query statement](/docs/concepts/query-lang) as
an argument, or by using [options](#options) arguments.

```
# Find all projects$ moon query projects# Find all projects with an id that matches "react"$ moon query projects --id react$ moon query projects "project~react"# Find all projects with a `lint` or `build` task$ moon query projects --tasks "lint|build"$ moon query projects "task=[lint,build]"
```

This will output a list of projects as JSON. The output has the following structure:

```
{	projects: Project[],	options: QueryOptions,}
```

### Affected projects

This command can also be used to query for affected projects, based on the state of the VCS working
tree. For advanced control, you can also pass the results of `moon query changed-files` to stdin.

```
# Find all affected projects$ moon query projects --affected# Find all affected projects using the results of another query$ moon query changed-files | moon query projects --affected
```

### Arguments

- `[query]` - An optional [query statement](/docs/concepts/query-lang) to filter projects with. When provided, all [filter options](#filters) are ignored. v1.4.0

### Options

#### Affected

- `--affected` - Filter projects that have been affected by touched files.

- `--downstream` - Include downstream dependents of queried projects. Supports "none" (default), "direct", "deep". v1.29.0

- `--upstream` - Include upstream dependencies of queried projects. Supports "none", "direct", "deep" (default). v1.29.0

#### Filters

All option values are case-insensitive regex patterns.

- `--alias ` - Filter projects that match this alias.

- `--id ` - Filter projects that match this ID/name.

- `--language ` - Filter projects of this programming language.

- `--layer ` - Filter project of this layer.

- `--source ` - Filter projects that match this source path.

- `--stack ` - Filter projects of the tech stack.

- `--tags ` - Filter projects that have the following tags.

- `--tasks ` - Filter projects that have the following tasks.

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

## /docs/commands/query/tasks

Source: https://moonrepo.dev/docs/commands/query/tasks

# query tasks

Use the `moon query tasks` sub-command to query task information for all projects in the project
graph. The tasks list can be filtered by passing a [query statement](/docs/concepts/query-lang) as
an argument, or by using [options](#options) arguments.

```
# Find all tasks grouped by project$ moon query tasks# Find all tasks from projects with an id that matches "react"$ moon query tasks --id react$ moon query tasks "task~react"
```

This will output a list of projects as JSON. The output has the following structure:

```
{	tasks: Record>,	options: QueryOptions,}
```

### Arguments

- `[query]` - An optional [query statement](/docs/concepts/query-lang) to filter projects with. When provided, all [filter options](#filters) are ignored. v1.4.0

### Options

#### Affected

- `--affected` - Filter tasks that have been affected by touched files.

- `--downstream` - Include downstream dependents of queried tasks. Supports "none" (default), "direct", "deep". v1.30.0

- `--upstream` - Include upstream dependencies of queried tasks. Supports "none", "direct", "deep" (default). v1.30.0

#### Filtersv1.30.0

All option values are case-insensitive regex patterns.

- `--command ` - Filter tasks that match this command.

- `--id ` - Filter tasks that match this ID.

- `--project ` - Filter tasks that belong to this project.

- `--script ` - Filter tasks that match this script.

- `--toolchain ` - Filter tasks of this toolchain. v1.31.0

- `--type ` - Filter tasks of this type.

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

- [`tasks`](/docs/config/project#tasks) in `moon.yml`

## /docs/commands/run

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

## /docs/commands/setup

Source: https://moonrepo.dev/docs/commands/setup

# setup

The `moon setup` command can be used to setup the developer and pipeline environments. It achieves
this by downloading and installing all configured tools into the toolchain.

```
$ moon setup
```

info

This command should rarely be used, as the environment is automatically setup when running other
commands, like detecting affected projects, running a task, or generating a build artifact.

### Configuration

- [`*`](/docs/config/toolchain) in `.moon/toolchains.yml`

## /docs/commands/sync

Source: https://moonrepo.dev/docs/commands/sync

- [Home](/)
- [Commands](/docs/commands)
- [sync](/docs/commands/sync)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# sync

Operations for syncing the workspace to a healthy state.

[📄️ code-ownersThe moon sync code-owners command will manually sync code owners, by aggregating all owners from](/docs/commands/sync/code-owners)

[📄️ config-schemasThe moon sync config-schemas command will manually generate JSON schemas to .moon/cache/schemas](/docs/commands/sync/config-schemas)

[📄️ projectsThe moon sync projects command will force sync all projects in the workspace to help achieve a](/docs/commands/sync/projects)

[📄️ vcs-hooksThe moon sync vcs-hooks command will manually sync hooks for the configured](/docs/commands/sync/vcs-hooks)

[setup](/docs/commands/setup)

[code-owners](/docs/commands/sync/code-owners)

## /docs/commands/sync/code-owners

Source: https://moonrepo.dev/docs/commands/sync/code-owners

# sync code-owners

v1.8.0

The `moon sync code-owners` command will manually sync code owners, by aggregating all owners from
projects, and generating a single `CODEOWNERS` file. Refer to the official
[code owners](/docs/guides/codeowners) guide for more information.

```
$ moon sync code-owners
```

### Options

- `--clean` - Clean and remove previously generated file.

- `--force` - Bypass cache and force create file.

### Configuration

- [`codeowners`](/docs/config/workspace#codeowners) in `.moon/workspace.yml`

- [`owners`](/docs/config/project#owners) in `moon.yml`

## /docs/commands/sync/config-schemas

Source: https://moonrepo.dev/docs/commands/sync/config-schemas

# sync config-schemas

v1.27.0

The `moon sync config-schemas` command will manually generate JSON schemas to `.moon/cache/schemas`
for all our different configuration files.

```
$ moon sync config-schemas
```

### Options

- `--force` - Bypass cache and force create files.

## /docs/commands/sync/projects

Source: https://moonrepo.dev/docs/commands/sync/projects

# sync projects

v1.8.0

The `moon sync projects` command will force sync all projects in the workspace to help achieve a
[healthy repository state](/docs/faq#what-should-be-considered-the-source-of-truth). This applies
the following:

- Ensures cross-project dependencies are linked based on [`dependsOn`](/docs/config/project#dependson).

- Ensures language specific configuration files are present and accurate (`package.json`, `tsconfig.json`, etc).

- Ensures root configuration and project configuration are in sync.

- Any additional language specific semantics that may be required.

```
$ moon sync projects
```

This command should rarely be ran, as [`moon run`](/docs/commands/run) will sync affected projects
automatically! However, when migrating or refactoring, manual syncing may be necessary.

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

## /docs/commands/sync/vcs-hooks

Source: https://moonrepo.dev/docs/commands/sync/vcs-hooks

# sync vcs-hooks

v1.9.0

The `moon sync vcs-hooks` command will manually sync hooks for the configured
[VCS](/docs/config/workspace#vcs), by generating and referencing hook scripts from the
[`vcs.hooks`](/docs/config/workspace#hooks) setting. Refer to the official
[VCS hooks](/docs/guides/vcs-hooks) guide for more information.

```
$ moon sync vcs-hooks
```

### Options

- `--clean` - Clean and remove previously generated hooks.

- `--force` - Bypass cache and force create hooks.

### Configuration

- [`vcs.hooks`](/docs/config/workspace#hooks) in `.moon/workspace.yml`

## /docs/commands/task

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

## /docs/commands/task-graph

Source: https://moonrepo.dev/docs/commands/task-graph

# task-graph

v1.30.0

The `moon task-graph [target]` (or `moon tg`) command will generate and serve a visual graph of all
configured tasks as nodes, with dependencies between as edges, and can also output the graph in
[Graphviz DOT format](https://graphviz.org/doc/info/lang.html).

```
# Run the visualizer locally$ moon task-graph# Export to DOT format$ moon task-graph --dot > graph.dot
```

A task target can be passed to focus the graph to only that task and its dependencies. For
example, `moon task-graph app:build`.

### Arguments

- `[target]` - Optional target of task to focus.

### Options

- `--dependents` - Include direct dependents of the focused task.

- `--dot` - Print the graph in DOT format.

- `--host` - The host address. Defaults to `127.0.0.1`. v1.36.0

- `--json` - Print the graph in JSON format.

- `--port` - The port to bind to. Defaults to a random port. v1.36.0

## Example output

The following output is an example of the graph in DOT format.

```
digraph {    0 [ label="types:build" style=filled, shape=oval, fillcolor=gray, fontcolor=black]    1 [ label="runtime:build" style=filled, shape=oval, fillcolor=gray, fontcolor=black]    2 [ label="website:build" style=filled, shape=oval, fillcolor=gray, fontcolor=black]    1 -> 0 [ label="required" arrowhead=box, arrowtail=box]    2 -> 1 [ label="required" arrowhead=box, arrowtail=box]    2 -> 0 [ label="required" arrowhead=box, arrowtail=box]}
```

## /docs/commands/tasks

Source: https://moonrepo.dev/docs/commands/tasks

# tasks

v2.0.0

The `moon tasks` command will list all tasks available in the workspace as a table of information.

```
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮│Task                          Command          Type        Preset      Toolchains                                Description                                                           ││───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────││website:build                 docusaurus       build                   typescript, javascript, node, yarn        Builds the Docusaurus app.                                            ││website:format                prettier         test                    javascript, node, yarn                                                                                          ││website:format-write          prettier         test                    javascript, node, yarn                                                                                          ││website:lint                  eslint           test                    javascript, node, yarn                                                                                          ││website:lint-fix              eslint           test                    javascript, node, yarn                                                                                          ││website:start                 docusaurus       run         server      typescript, javascript, node, yarn                                                                              ││website:test                  jest             test                    javascript, node, yarn                                                                                          ││website:typecheck             tsc              test                    typescript, javascript, node, yarn                                                                              │╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

info

Use [`moon query tasks`](/docs/commands/query/tasks) for advanced querying and filtering of tasks.

### Arguments

- `[id]` - Filter tasks to a specific project ID.

### Options

- `--json` - Print the projects as JSON.

## /docs/commands/teardown

Source: https://moonrepo.dev/docs/commands/teardown

# teardown

The `moon teardown` command, as its name infers, will teardown and clean the current environment,
opposite the [`setup`](/docs/commands/setup) command. It achieves this by doing the following:

- Uninstalling all configured tools in the toolchain.

- Removing any download or temporary files/folders.

```
$ moon teardown
```

### Configuration

- [`*`](/docs/config/toolchain) in `.moon/toolchains.yml`

## /docs/commands/template

Source: https://moonrepo.dev/docs/commands/template

# template

v2.0.0

The `moon template [id]` command will display information about a template, its files, and
variables.

```
Template title ───────────────────────────────────────────────────────────  Some description of the template and its files.About ────────────────────────────────────────────────────────────────────  Template: package  Location: /templates/package  Destination: packages/[name | kebab_case]  Extends: —  Assets: —  Files:    - package.jsonVariables ────────────────────────────────────────────────────────────────  name: string
```

### Arguments

- `[id]` - The template ID.

### Options

- `--json` - Print the template as JSON.

## /docs/commands/templates

Source: https://moonrepo.dev/docs/commands/templates

# templates

v1.24.0

The `moon templates` command will list all templates available for [code generation](/docs/commands/generate).
This list will include the template title, description, default destination, where it's source files
are located, and more.

```
$ moon templates
```

### Options

- `--filter` - Filter templates by a search term.

- `--json` - Print templates in JSON format.

### Configuration

- [`generator`](/docs/config/workspace#generator) in `.moon/workspace.yml`

## /docs/commands/toolchain

Source: https://moonrepo.dev/docs/commands/toolchain

- [Home](/)
- [Commands](/docs/commands)
- [toolchain](/docs/commands/toolchain)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# toolchain

Manage toolchain plugins.

[📄️ addThe moon toolchain add [plugin] command will add a toolchain to the workspace by injecting a](/docs/commands/toolchain/add)

[📄️ infoThe moon toolchain info [plugin] command will display detailed information about a toolchain,](/docs/commands/toolchain/info)

[templates](/docs/commands/templates)

[add](/docs/commands/toolchain/add)

## /docs/commands/toolchain/add

Source: https://moonrepo.dev/docs/commands/toolchain/add

# toolchain add

v1.38.0

The `moon toolchain add  [plugin]` command will add a toolchain to the workspace by injecting a
configuration block into `.moon/toolchains.yml`. To do this, the command will download the WASM
plugin, extract information, and call initialize functions.

For built-in toolchains, the [plugin locator](/docs/guides/wasm-plugins#configuring-plugin-locations) argument is optional, and will be derived
from the identifier.

```
$ moon toolchain add typescript
```

For third-party toolchains, the [plugin locator](/docs/guides/wasm-plugins#configuring-plugin-locations) argument is required, and must point to
the WASM plugin.

```
$ moon toolchain add custom https://example.com/path/to/plugin.wasm
```

### Arguments

- `` - ID of the toolchain to use.

- `[plugin]` - Optional [plugin locator](/docs/guides/wasm-plugins#configuring-plugin-locations) for third-party toolchains.

### Options

- `--minimal` - Generate minimal configurations and sane defaults.

- `--yes` - Skip all prompts and enables tools based on file detection.

## /docs/commands/toolchain/info

Source: https://moonrepo.dev/docs/commands/toolchain/info

# toolchain info

v1.38.0

The `moon toolchain info  [plugin]` command will display detailed information about a toolchain,
like what files are scanned, what configuration settings are available, and what tier APIs are
supported. To do this, the command will download the WASM plugin, extract information, and call
specific functions.

For built-in toolchains, the [plugin locator][locator] argument is optional, and will be derived
from the identifier.

```
$ moon toolchain info typescript
```

For third-party toolchains, the [plugin locator][locator] argument is required, and must point to
the WASM plugin.

```
$ moon toolchain info custom https://example.com/path/to/plugin.wasm
```

### Arguments

- `` - ID of the toolchain to view.

- `[plugin]` - Optional [plugin locator][locator] for third-party toolchains.

## Example output

```
Toolchain ─────────────────────────────────────────────────────────────────  Provides sync operations that keep tsconfig.json's in a healthy state.  ID: typescript  Name: TypeScript  Version: 0.2.0Configuration ─────────────────────────────────────────────────────────────  createMissingConfig: bool  When `syncProjectReferences` is enabled, will create a `tsconfig.json`  in referenced projects if it does not exist.  includeProjectReferenceSources: bool  Appends sources of project reference to `include` in `tsconfig.json`,  for each project.  includeSharedTypes: bool  Appends shared types to `include` in `tsconfig.json`, for each project.  projectConfigFileName: string  Name of the `tsconfig.json` file within each project.  root: string  The relative root to the TypeScript root. Primarily used for  resolving project references.  rootConfigFileName: string  Name of the `tsconfig.json` file at the workspace root.  rootOptionsConfigFileName: string  Name of the shared compiler options `tsconfig.json` file  at the workspace root.  routeOutDirToCache: bool  Updates and routes `outDir` in `tsconfig.json` to moon's cache,  for each project.  syncProjectReferences: bool  Syncs all project dependencies as `references` in `tsconfig.json`,  for each project.  syncProjectReferencesToPaths: bool  Syncs all project dependencies as `paths` in `tsconfig.json`,  for each project.Tier 1 - Usage detection ──────────────────────────────────────────────────  Config files: tsconfig.json, tsconfig.*.json, *.tsconfig.json, .tsbuildinfo, *.tsbuildinfo  Executable names: tsc, tsserver  APIs:    🟢 register_toolchain (required)    🟢 define_toolchain_config    🟢 initialize_toolchain    ⚫️ detect_version_files    ⚫️ parse_version_file    🟢 define_docker_metadata    ⚫️ scaffold_docker    ⚫️ prune_docker    🟢 sync_project    ⚫️ sync_workspaceTier 2 - Ecosystem integration ─────────────────────────────────────────────  APIs:    ⚫️ extend_project_graph    ⚫️ extend_task_command    ⚫️ extend_task_script    ⚫️ locate_dependencies_root    ⚫️ install_dependencies    🟢 hash_task_contents    ⚫️ parse_lock    ⚫️ parse_manifest    ⚫️ setup_environmentTier 3 - Tool management ──────────────────────────────────────────────────  APIs:    ⚫️ register_tool (required)    ⚫️ load_versions    ⚫️ resolve_version    ⚫️ download_prebuilt (required)    ⚫️ unpack_archive    ⚫️ locate_executables (required)    ⚫️ setup_toolchain    ⚫️ teardown_toolchain
```

## /docs/commands/upgrade

Source: https://moonrepo.dev/docs/commands/upgrade

# upgrade

The `moon upgrade` command can be used to upgrade your current moon binary (if installed globally)
to the latest version.

```
$ moon upgrade
```

caution

This command will only work if moon was installed in the `~/.moon` directory, using our official
[installation script](/docs/install). If installed through Node.js, you'll need to upgrade manually.

## /docs/comparison

Source: https://moonrepo.dev/docs/comparison

# Feature comparison

The following comparisons are not an exhaustive list of features, and may be inaccurate or out of
date, but represent a good starting point for investigation. If something is not correct, please
[create an issue](https://github.com/moonrepo/moon/issues) or
[submit a patch](https://github.com/moonrepo/moon/blob/master/website/src/components/ComparisonTable.tsx).

Before diving into our comparisons below, we highly suggest reading
[monorepo.tools](https://monorepo.tools/) for a deeper insight into monorepos and available tooling.
It's a great resource for learning about the current state of things and the ecosystem.

info

Looking to migrate from Nx or Turborepo to moon? Use our
[`moon ext migrate-nx`](/docs/guides/extensions#migrate-nx) or
[`moon ext migrate-turborepo`](/docs/guides/extensions#migrate-turborepo) commands for a (somewhat)
seamless migration!

## Unique features

Although moon is still in its infancy, we provide an array of powerful features that other frontend
centric task runners do not, such as...

- [Integrated toolchain](/docs/concepts/toolchain) - moon manages its own version of programming languages and dependency managers behind the scenes, so that every task is executed with the exact same version, across all machines.

- [Task inheritance](/docs/concepts/task-inheritance) - Instead of defining the same tasks (lint, test, etc) over and over again for every project in the monorepo, moon supports a task inheritance model where it only needs to be defined once at the top-level. Projects can then merge with, exclude, or override if need be.

- [Continuous integration](/docs/guides/ci) - By default, all moon tasks will run in CI, as we want to encourage every facet of a project or repository to be continually tested and verified. This can be turned off on a per-task basis.

Curious to learn more? Check out the "[Why use moon?](/docs/)" or "[Features](/docs/)" sections for more
information, or these wonderful articles provided by the community:

- [A review of moon + Packemon](https://azu.github.io/slide/2022/moa/moon-packemon.html) by [azu](https://twitter.com/azu_re)

- [Improve repo management with moon](https://blog.logrocket.com/improve-repo-management-moon/) by [James Sinkala](https://jamesinkala.com/)

## Comparison

### Turborepo

At a high-level, Turborepo and moon seem very similar as they both claim to be task runners. They
both support incremental builds, content/smart hashing, local and remote caching1,
parallel execution, and everything else you'd expect from a task runner. But that's where the
similarities stop, because in the end, Turborepo is nothing more than a `package.json` scripts
orchestrator with a caching layer. While moon also supports this, it
[aims to be far more](#unique-features) with a heavy focus on the developer experience.

In the next section, we'll be talking about a few key areas that we deem important to consumers. If
you'd prefer a more granular comparison, jump down to the [comparison tables](#comparison-tables).

#### Configuration

Turborepo only supports the Node.js ecosystem, so implicitly uses a conventions based approach. It provides very little to no configuration for customizing Turborepo to your needs.

moon is language agnostic, with initial support for Node.js and its ecosystem. Because of this, moon provides a ton of configuration for customizing moon to your needs. It prefers a configuration over conventions approach, as every repository is different.

#### Projects

Turborepo infers projects from `package.json` workspaces, and does not support non-JavaScript based projects.

moon requires projects to be defined in `.moon/workspace.yml`, and supports any programming language2.

#### Tasks

Turborepo requires `package.json` scripts to be defined for every project. This results in the same scripts being repeated constantly.

moon avoids this overhead by using [task inheritance](#unique-features). No more repetition.

#### CI

Each pipeline in `turbo.json` must be individually ran as a step in CI. Scripts not configured as pipeline tasks are never ran.

moon runs every task automatically using `moon ci`, which also supports parallelism/sharding.

#### Long-term

Turborepo is in the process of being rewritten in Rust, with its codebase being shared and coupled with the new Turbopack library, a Rust based bundler. Outside of this, there are no publicly available plans for Turborepo's future.

moon plans to be so much more than a task runner, with one such facet being a repository management tool. This includes code ownership, dependency management and auditing, repository linting, in-repo secrets, and anything else we deem viable. We also plan to support additional languages as first-class citizens within our toolchain.

- Turborepo remote caching is powered by Vercel. moon provides its own paid service.

- moon projects may run commands for any language, but not all languages are supported in the toolchain.

### Lerna

Lerna was a fantastic tool that helped the JavaScript ecosystem grow and excelled at package
versioning and publishing (and still does), but it offered a very rudimentary task runner. While
Lerna was able to run scripts in parallel, it wasn't the most efficient, as it did not support
caching, hashing, or performant scheduling.

However, the reason Lerna is not compared in-depth, is that Lerna was unowned and unmaintained for
quite some time, and has recently fallen under the Nx umbrella. Lerna is basically Nx lite now.

## Comparison tables

🟩 Supported 🟨 Partially supported 🟦 Similarly supported 🟥 Not supported

### Workspace

moon (11) nx (11) turborepo (8)

Core/CLI written in Rust Node.js & Rust (for hot paths) Rust / Go

Plugins written in WASM (any compatible language) TypeScript 🟥

Workspace configured with `.moon/workspace.yml` `nx.json` `turbo.json`

Project list configured in `.moon/workspace.yml` `workspace.json` / `package.json` workspaces `package.json` workspaces

Repo / folder structure loose loose loose

Ignore file support 🟩 via `hasher.ignorePatterns` 🟩 .nxignore 🟩 via `--ignore`

Supports dependencies inherited by all tasks 🟩 via `implicitDeps` 🟩 via `targetDefaults` 🟥

Supports inputs inherited by all tasks 🟩 via `implicitInputs` 🟩 via `implicitDependencies` 🟩 via `globalDependencies`

Supports tasks inherited by all projects 🟩 🟩 via `plugins` 🟥

Integrates with a version control system 🟩 git 🟩 git 🟩 git

Supports scaffolding / generators 🟩 🟩 🟩

### Toolchain

moon (6) nx (2) turborepo (2)

Supported languages in task runner All languages available on `PATH` All languages via plugins. OOTB TS/JS, existing plugins for Rust, Go, Dotnet and more JavaScriptTypeScript via `package.json` scripts

Supported dependency managers npm, pnpm, yarn, bun npm, pnpm, yarn npm, pnpm, yarn

Supported toolchain languages (automatic dev envs) Bun, Deno, Node.js, Rust 🟥 🟥

Has a built-in toolchain 🟩 🟥 🟥

Downloads and installs languages (when applicable) 🟩 🟥 🟥

Configures explicit language/dependency manager versions 🟩 🟥 🟥

### Projects

moon (8) nx (5) turborepo (2)

Dependencies on other projects 🟩 implicit from `package.json` or explicit in `moon.yml` 🟩 implicit from `package.json` or explicit in `project.json` and code imports/exports 🟩 implicit from `package.json`

Ownership metadata 🟩 🟥 🟥

Primary programming language 🟩 🟥 🟥

Project type (app, lib, etc) 🟩 app, lib, tool, automation, config, scaffold 🟩 app, lib 🟥

Project tech stack 🟩 frontend, backend, infra, systems 🟥 🟥

Project-level file groups 🟩 🟩 via `namedInputs` 🟥

Project-level tasks 🟩 🟩 🟩

Tags and scopes (boundaries) 🟩 native for all languages 🟩 boundaries via ESLint (TS and JS), tags for filtering for all languages 🟥

### Tasks

moon (24) nx (24) turborepo (17)

Known as tasks targets tasks

Defines tasks in `moon.yml` or `package.json` scripts `nx.json`, `project.json` or `package.json` scripts `package.json` scripts

Run a single task with `moon run project:task` `nx target project` or `nx run project:target` `turbo run task --filter=project`

Run multiple tasks with `moon run :task` or `moon run a:task b:task` or `moon check` `nx run-many -t task1 task2 task3` `turbo run task` or `turbo run a b c`

Run tasks based on a query/filter `moon run :task --query "..."` `nx run-many -t task -p "tag:.." -p "dir/*" -p "name*" -p "!negation"` 🟥

Can define tasks globally 🟩 with `.moon/tasks/all.yml` 🟨 with `targetDefaults` 🟥

Merges or overrides global tasks 🟩 🟩 🟥

Runs a command with args 🟩 🟩 🟨 within the script

Runs commands from project or workspace root current working directory, or wherever desired via config project root

Supports pipes, redirects, etc, in configured tasks 🟨 encapsulated in a file 🟨 within the executor or script 🟨 within the script

Dependencies on other tasks 🟩 via `deps` 🟩 via `dependsOn` 🟩 via `dependsOn`

Can provide extra params for task dependencies 🟩 🟩 🟥

Can mark a task dependency as optional 🟩 via `optional` 🟥 🟥

Can depend on arbitrary or unrelated tasks 🟩 🟩 🟥 dependent projects only

Runs task dependencies in parallel 🟩 🟩 🟩

Can run task dependencies in serial 🟩 🟩 via `parallel=1` 🟩 via `concurrency=1`

File groups 🟩 🟩 via `namedInputs` 🟥

Environment variables 🟩 via `env`, `envFile` 🟩 automatically via `.env` files and/or inherited from shell 🟨 within the script

Inputs 🟩 files, globs, env vars 🟩 files, globs, env vars, runtime 🟩 files, globs

Outputs 🟩 files, globs 🟩 files, globs 🟩 files, globs

Output logging style 🟩 via `outputStyle` 🟩 via `--output-style` 🟩 via `outputMode`

Custom hash inputs 🟥 🟩 via `runtime` inputs 🟩 via `globalDependencies`

Token substitution 🟩 token functions and variable syntax 🟩 `{workspaceRoot}`, `{projectRoot}`, `{projectName}`, arbitrary patterns `namedInputs` 🟥

Configuration presets 🟩 via task `extends` 🟩 via `configurations` 🟥

Configurable options 🟩 🟩 🟩

### Task runner

moon (9) nx (11) turborepo (7)

Known as action pipeline task runner pipeline

Generates a dependency graph 🟩 🟩 🟩

Runs in topological order 🟩 🟩 🟩

Automatically retries failed tasks 🟩 🟩 when flakiness detected on Nx Cloud 🟥

Caches task outputs via a unique hash 🟩 🟩 🟩

Can customize the underlying runner 🟥 🟩 🟥

Can profile running tasks 🟩 cpu, heap 🟩 cpu 🟩 cpu

Can generate run reports 🟩 🟩 free in Nx Cloud & GitHub App Comment 🟩

Continuous integration (CI) support 🟩 🟩 🟨

Continuous deployment (CD) support 🟥 🟨 via `nx release` 🟥

Remote / cloud caching and syncing 🟩 with Bazel REAPI (free / paid) 🟩 with nx.app Nx Cloud (free / paid) 🟩 requires a Vercel account (free)

### Generator

moon (14) nx (14) turborepo (11)

Known as generator generator generator

Templates are configured with a schema 🟩 via `template.yml` 🟩 🟥

Template file extensions (optional) 🟩 .tera, .twig 🟩 fully under user control, built in utility for .ejs templates 🟩 .hbs

Template files support frontmatter 🟩 🟩 fully under user control 🟥

Creates/copies files to destination 🟩 🟩 🟩

Updates/merges with existing files 🟩 JSON/YAML only 🟩 via TypeScript/JavaScript plugins 🟩

Renders with a template engine 🟩 via Tera 🟩 fully under user control, built in utility for .ejs templates 🟩 via Handlebars

Variable interpolation in file content 🟩 🟩 🟩

Variable interpolation in file paths 🟩 🟩 🟩

Can define variable values via interactive prompts 🟩 🟩 using JSON schema 🟩

Can define variable values via command line args 🟩 🟩 using JSON schema 🟩

Supports dry runs 🟩 🟩 🟥

Supports render helpers, filters, and built-ins 🟩 🟩 🟩

Generators can compose other generators 🟩 via `extends` 🟩 fully under user control, author in TypeScript/JavaScript 🟩 using JavaScript

### Other systems

moon (6) nx (0) turborepo (1)

Can send webhooks for critical pipeline events 🟩 🟥 🟥

Generates run reports with granular stats/metrics 🟩 🟥 🟩

Can define and manage code owners 🟩 🟥 🟥

Can generate a `CODEOWNERS` file 🟩 🟥 🟥

Can define and manage VCS (git) hooks 🟩 🟥 🟥

Supports git worktrees 🟩 🟥 🟥

### JavaScript ecosystem

moon (10) nx (1) turborepo (0)

Will automatically install node modules when lockfile changes 🟩 🟥 🟥

Can automatically dedupe when lockfile changes 🟩 🟥 🟥

Can alias `package.json` names for projects 🟩 🟩 🟥

Can add `engines` constraint to root `package.json` 🟩 🟥 🟥

Can sync version manager configs (`.nvmrc`, etc) 🟩 🟥 🟥

Can sync cross-project dependencies to `package.json` 🟩 🟥 🟥

Can sync project references to applicable `tsconfig.json` 🟩 🟥 🟥

Can auto-create missing `tsconfig.json` 🟩 🟥 🟥

Can sync dependencies as `paths` to `tsconfig.json` 🟩 🟥 🟥

Can route `outDir` to a shared cached in `tsconfig.json` 🟩 🟥 🟥

### Docker integration

moon (3) nx (2) turborepo (2)

Efficient scaffolding for Dockerfile layer caching 🟩 🟦 via custom generator 🟩

Automatic production-only dependency installation 🟩 🟨 generated automatically by first party plugin 🟥

Environment pruning to reduce image/container sizes 🟩 🟥 🟩

## /docs/concepts

Source: https://moonrepo.dev/docs/concepts

- [Home](/)
- [Concepts](/docs/concepts)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Concepts

[📄️ Cachemoon's able to achieve high performance and blazing speeds by implementing a cache that's powered by](/docs/concepts/cache)

[📄️ File groupsFile groups are a mechanism for grouping similar types of files and environment variables within a](/docs/concepts/file-group)

[📄️ File patternsGlobs](/docs/concepts/file-pattern)

[📄️ Query languagemoon supports an integrated query language, known as MQL, that can be used to filter and select](/docs/concepts/query-lang)

[📄️ ProjectsA project is a library, application, package, binary, tool, etc, that contains source files, test](/docs/concepts/project)

[📄️ TargetsA target is a compound identifier that pairs a scope to a task,](/docs/concepts/target)

[📄️ TasksTasks are commands that are ran in the context of a project. Underneath the hood, a](/docs/concepts/task)

[📄️ Task inheritanceUnlike other task runners that require the same tasks to be repeatedly defined for every project,](/docs/concepts/task-inheritance)

[📄️ TokensTokens are variables and functions that can be used by command,](/docs/concepts/token)

[📄️ ToolchainThe toolchain is an internal layer for downloading, installing, and managing tools (languages,](/docs/concepts/toolchain)

[📄️ WorkspaceA workspace is a directory that contains projects, manages a toolchain,](/docs/concepts/workspace)

[Migrate to moon](/docs/migrate-to-moon)

[Cache](/docs/concepts/cache)

## /docs/concepts/cache

Source: https://moonrepo.dev/docs/concepts/cache

# Cache

moon's able to achieve high performance and blazing speeds by implementing a cache that's powered by
our own unique smart hashing layer. All cache is stored in `.moon/cache`, relative from the
workspace root (be sure to git ignore this folder).

## Hashing

Incremental builds are possible through a concept known as hashing, where in multiple sources are
aggregated to generate a unique hash. In the context of moon, each time a target is ran we generate
a hash, and if this hash already exists we abort early (cache hit), otherwise we continue the run
(cache miss).

The tiniest change may trigger a different hash, for example, changing a line of code (when an
input), or updating a package version, so don't worry if you see a lot of hashes.

Our smart hashing currently takes the following sources into account:

- Command (`command`) being ran and its arguments (`args`).

- Input sources (`inputs`).

- Output targets (`outputs`).

- Environment variables (`env`).

- Dependencies between projects (`dependsOn`) and tasks (`deps`).

- For Deno tasks: Deno version.

- `deno.json`/`deps.ts` imports, import maps, and scopes.

- `tsconfig.json` compiler options (when applicable).

- For Bun and Node.js tasks: Bun/Node.js version.

- `package.json` dependencies (including development and peer).

- `tsconfig.json` compiler options (when applicable).

caution

Be aware that greedy inputs (`**/*`, the default) will include everything in the target directory
as a source. We do our best to filter out VCS ignored files, and `outputs` for the current task, but
files may slip through that you don't expect. We suggest using explicit `inputs` and routinely
auditing the hash files for accuracy!

## Archiving & hydration

On top of our hashing layer, we have another concept known as archiving, where in we create a
tarball archive of a task's outputs and store it in `.moon/cache/outputs`. These are akin to build
artifacts.

When we encounter a cache hit on a hash, we trigger a mechanism known as hydration, where we
efficiently unpack an existing tarball archive into a task's outputs. This can be understood as a
timeline, where every point in time will have its own hash + archive that moon can play back.

Furthermore, if we receive a cache hit on the hash, and the hash is the same as the last run, and
outputs exist, we exit early without hydrating and assume the project is already hydrated. In the
terminal, you'll see a message for "cached".

## File structure

The following diagram outlines our cache folder structure and why each piece exists.

```
.moon/cache/	# Stores hash manifests of every ran task. Exists purely for debugging purposes.	hashes/		# Contents includes all sources used to generate the hash.		.json	# Stores `tar.gz` archives of a task's outputs based on its generated hash.	outputs/		.tar.gz	# State information about anything and everything within moon. Toolchain,	# dependencies, projects, running targets, etc.	states/		# Files at the root pertain to the entire workspace.		.json		# Files for a project are nested within a folder by the project name.
/			# Informational snapshot of the project, its tasks, and its configs.			# Can be used at runtime by tasks that require this information.			snapshot.json			/				# Contents of the child process, including the exit code and				# unique hash that is referenced above.				lastRun.json				# Outputs of last run target.				stderr.log				stdout.log
```

## /docs/concepts/file-group

Source: https://moonrepo.dev/docs/concepts/file-group

# File groups

File groups are a mechanism for grouping similar types of files and environment variables within a
project using [file glob patterns or literal file paths](/docs/concepts/file-pattern). These groups are then used
by [tasks](/docs/concepts/task) to calculate functionality like cache computation, affected files since last
change, deterministic builds, and more.

## Configuration

File groups can be configured per project through [`moon.yml`](/docs/config/project), or for many
projects through [`.moon/tasks/all.yml`](/docs/config/tasks).

### Token functions

File groups can be referenced in [tasks](/docs/concepts/task) using [token functions](/docs/concepts/token). For example, the
`@group(name)` token will expand to all paths configured in the `sources` file group.

moon.yml

```
tasks:  build:    command: 'vite build'    inputs:      - '@group(sources)'
```

## Inheritance and merging

When a file group of the same name exists in both [configuration files](#configuration), the
project-level group will override the workspace-level group, and all other workspace-level groups
will be inherited as-is.

A primary scenario in which to define file groups at the project-level is when you want to
override file groups defined at the workspace-level. For example, say we want to override the
`sources` file group because our source folder is named "lib" and not "src", we would define our
file groups as followed.

.moon/tasks/all.yml

```
fileGroups:  sources:    - 'src/**/*'    - 'types/**/*'  tests:    - 'tests/**/*.test.*'    - '**/__tests__/**/*'
```

moon.yml

```
fileGroups:  # Overrides global  sources:    - 'lib/**/*'    - 'types/**/*'  # Inherited as-is  tests:    - 'tests/**/*.test.*'    - '**/__tests__/**/*'
```

## /docs/concepts/file-pattern

Source: https://moonrepo.dev/docs/concepts/file-pattern

# File patterns

## Globs

Globs in moon are [Rust-based globs](https://github.com/olson-sean-k/wax), not JavaScript-based.
This may result in different or unexpected results. The following guidelines must be met when using
globs:

- Must use forward slashes (`/`) for path separators, even on Windows.

- Must not start with or use any relative path parts, `.` or `..`.

### Supported syntax

- `*` - Matches zero or more characters, but does not match the `/` character. Will attempt to match the longest possible text (eager).

- `$` - Like `*`, but will attempt to match the shortest possible text (lazy).

- `**` - Matches zero or more directories.

- `?` - Matches exactly one character, but not `/`.

- `[abc]` - Matches one case-sensitive character listed in the brackets.

- `[!xyz]` - Like the above, but will match any character not listed.

- `[a-z]` - Matches one case-sensitive character in range in the brackets.

- `[!x-z]` - Like the above, but will match any character not in range.

- `{glob,glob}` - Matches one or more comma separated list of sub-glob patterns.

- `` - Matches a sub-glob within a defined bounds.

- `!` - At the start of a pattern, will negate previous positive patterns.

### Examples

```
README.{md,mdx,txt}src/**/*tests/**/*.?js!**/__tests__/**/*logs/--.log
```

## Project relative

When configuring [`fileGroups`](/docs/config/project#filegroups), [`inputs`](/docs/config/project#inputs),
and [`outputs`](/docs/config/project#outputs), all listed file paths and globs are relative from the
project root they will be ran in. They must not traverse upwards with `..`.

```
# Validsrc/**/*./src/**/*package.json# Invalid../utils
```

## Workspace relative

When configuring [`fileGroups`](/docs/config/project#filegroups), [`inputs`](/docs/config/project#inputs),
and [`outputs`](/docs/config/project#outputs), a listed file path or glob can be prefixed with `/` to
resolve relative from the workspace root, and not the project root.

```
# In projectpackage.json# In workspace/package.json
```

## /docs/concepts/project

Source: https://moonrepo.dev/docs/concepts/project

# Projects

A project is a library, application, package, binary, tool, etc, that contains source files, test
files, assets, resources, and more. A project must exist and be configured within a
[workspace](/docs/concepts/workspace).

## IDs

A project identifier (or name) is a unique resource for locating a project. The ID is explicitly
configured within [`.moon/workspace.yml`](/docs/config/workspace), as a key within the
[`projects`](/docs/config/workspace#projects) setting, and can be written in camel/kebab/snake case.
IDs support alphabetic unicode characters, `0-9`, `_`, `-`, `/`, `.`, and must start with a
character.

IDs are used heavily by configuration and the command line to link and reference everything. They're
also a much easier concept for remembering projects than file system paths, and they typically can
be written with less key strokes.

Lastly, a project ID can be paired with a task ID to create a [target](/docs/concepts/target).

## Aliases

Aliases are a secondary approach for naming projects, and can be used as a drop-in replacement for
standard names. What this means is that an alias can also be used when configuring dependencies, or
defining [targets](/docs/concepts/target).

However, the difference between aliases and names is that aliases can not be explicit configured
in moon. Instead, they are specific to a project's primary programming language, and are inferred
based on that context (when enabled in settings). For example, a JavaScript or TypeScript project
will use the `name` field from its `package.json` as the alias.

Because of this, a project can either be referenced by its name or alias, or both. Choose the
pattern that makes the most sense for your company or team!

## Dependencies

Projects can depend on other projects within the [workspace](/docs/concepts/workspace) to build a
[project graph](/docs/how-it-works/action-graph), and in turn, an action graph for executing
[tasks](/docs/concepts/task). Project dependencies are divided into 2 categories:

- Explicit dependencies - These are dependencies that are explicitly defined in a project's [`moon.yml`](/docs/config/project) config file, using the [`dependsOn`](/docs/config/project#dependson) setting.

- Implicit dependencies - These are dependencies that are implicitly discovered by moon when scanning the repository. How an implicit dependency is discovered is based on the project's [`language`](/docs/config/project#language) setting, and how that language's ecosystem functions.

## Configuration

Projects can be configured with an optional [`moon.yml`](/docs/config/project) in the project root, or
through the optional workspace-level [`.moon/tasks/all.yml`](/docs/config/tasks).

## /docs/concepts/query-lang

Source: https://moonrepo.dev/docs/concepts/query-lang

# Query language

v1.3.0

moon supports an integrated query language, known as MQL, that can be used to filter and select
projects from the project graph, using an SQL-like syntax. MQL is primarily used by
[`moon run`](/docs/commands/run) with the `--query` option.

## Syntax

### Comparisons

A comparison (also known as an assignment) is an expression that defines a piece of criteria, and is
a building block of a query. This criteria maps a [field](#fields) to a value, with an explicit
comparison operator.

#### Equals, Not equals

The equals (`=`) and not equals (`!=`) comparison operators can be used for exact value matching.

```
projectLayer=library && language!=javascript
```

You can also define a list of values using square bracket syntax, that will match against one of the
values.

```
language=[javascript, typescript]
```

#### Like, Not like

The like (`~`) and not like (`!~`) comparison operators can be used for wildcard value matching,
using [glob syntax](/docs/concepts/file-pattern#globs).

```
projectSource~packages/* && tag!~*-app
```

Like comparisons can only be used on non-enum fields.

### Conditions

The `&&` and `||` logical operators can be used to combine multiple comparisons into a condition.
The `&&` operator is used to combine comparisons into a logical AND, and the `||` operator is used
for logical OR.

```
taskToolchain=system || taskToolchain=node
```

For readability concerns, you can also use `AND` or `OR`.

```
taskToolchain=system OR taskToolchain=node
```

Mixing both operators in the same condition is not supported.

### Grouping

For advanced queries and complex conditions, you can group comparisons using parentheses to create
logical groupings. Groups can also be nested within other groups.

```
language=javascript && (taskType=test || taskType=build)
```

## Fields

The following fields can be used as criteria, and are related to [task tokens](/docs/concepts/token#variables).

### `language`

Programming language the project is written in, as defined in
[`moon.yml`](/docs/config/project#language).

```
language=rust
```

### `project`

Name OR alias of the project.

```
project=server
```

### `projectAlias`

Alias of the project. For example, the `package.json` name.

```
projectAlias~@scope/*
```

### `projectLayer`v1.39.0

The project layer, as defined in [`moon.yml`](/docs/config/project#layer).

```
projectLayer=application
```

### `projectId`

Name of the project, as defined in [`.moon/workspace.yml`](/docs/config/workspace), or `id` in
[`moon.yml`](/docs/config/project#id).

```
projectId=server
```

### `projectSource`

Relative file path from the workspace root to the project root, as defined in
[`.moon/workspace.yml`](/docs/config/workspace).

```
projectSource~packages/*
```

### `projectStack`v1.22.0

The project stack, as defined in [`moon.yml`](/docs/config/project#stack).

```
projectStack=frontend
```

### `tag`

A tag within the project, as defined in [`moon.yml`](/docs/config/project#tags).

```
tag~react-*
```

### `task`

ID/name of a task within the project.

```
task=[build,test]
```

### `taskToolchain`v1.31.0

The toolchain a task will run against, as defined in [`moon.yml`](/docs/config/project).

```
taskToolchain=node
```

### `taskType`

The [type of task](/docs/concepts/task#types), based on its configured settings.

```
taskType=build
```

## /docs/concepts/target

Source: https://moonrepo.dev/docs/concepts/target

# Targets

A target is a compound identifier that pairs a [scope](#common-scopes) to a [task](/docs/concepts/task),
separated by a `:`, in the format of `scope:task`.

Targets are used by terminal commands...

```
$ moon run designSystem:build
```

And configurations for declaring cross-project or cross-task dependencies.

```
tasks:  build:    command: 'webpack'    deps:      - 'designSystem:build'
```

## Common scopes

These scopes are available for both running targets and configuring them.

### By project

The most common scope is the project scope, which requires the name of a project, as defined in
[`.moon/workspace.yml`](/docs/config/workspace). When paired with a task name, it will run a specific
task from that project.

```
# Run `lint` in project `app`$ moon run app:lint
```

### By tagv1.4.0

Another way to target projects is with the tag scope, which requires the name of a tag prefixed with
`#`, and will run a specific task in all projects with that tag.

```
# Run `lint` in projects with the tag `frontend`$ moon run '#frontend:lint'
```

caution

Because `#` is a special character in the terminal (is considered a comment), you'll need to wrap
the target in quotes, or escape it like so `\#`.

## Run scopes

These scopes are only available on the command line when running tasks with `moon run` or `moon ci`.

### All projects

For situations where you want to run a specific target in all projects, for example `lint`ing, you
can utilize the all projects scope by omitting the project name from the target: `:lint`.

```
# Run `lint` in all projects$ moon run :lint
```

### Closest project `~`v1.33.0

If you are within a project folder, or an arbitrarily nested folder, and want to run a task in the
closest project (traversing upwards), the `~` scope can be used.

```
# Run `lint` in the closest project$ moon run ~:lint
```

## Config scopes

These scopes are only available when configuring a task.

### Dependencies `^`

When you want to include a reference for each project [that's depended on](/docs/concepts/project#dependencies),
you can utilize the `^` scope. This will be expanded to all depended on projects. If you do not
want all projects, then you'll need to explicitly define them.

moon.yml

```
dependsOn:  - 'apiClients'  - 'designSystem'# Configured astasks:  build:    command: 'webpack'    deps:      - '^:build'# Resolves totasks:  build:    command: 'webpack'    deps:      - 'apiClients:build'      - 'designSystem:build'
```

### Self `~`

When referring to another task within the current project, you can utilize the `~` scope, or omit
the `~:` prefix altogether, which will be expanded to the current project's name. This is useful for
situations where the name is unknown, for example, when configuring
[`.moon/tasks/all.yml`](/docs/config/tasks), or if you just want a shortcut!

.moon/tasks/all.yml

```
# Configured astasks:  lint:    command: 'eslint'    deps:      - '~:typecheck'      # OR      - 'typecheck'  typecheck:    command: 'tsc'# Resolves to (assuming project is "foo")tasks:  lint:    command: 'eslint'    deps:      - 'foo:typecheck'  typecheck:    command: 'tsc'
```

## /docs/concepts/task

Source: https://moonrepo.dev/docs/concepts/task

# Tasks

Tasks are commands that are ran in the context of a [project](/docs/concepts/project). Underneath the hood, a
task is simply a binary or system command that is ran as a child process.

## IDs

A task identifier (or name) is a unique resource for locating a task within a project. The ID is
explicitly configured as a key within the [`tasks`](/docs/config/project#tasks) setting, and can be
written in camel/kebab/snake case. IDs support alphabetic unicode characters, `0-9`, `_`, `-`, `/`,
`.`, and must start with a character.

A task ID can be paired with a scope to create a [target](/docs/concepts/target).

## Types

Tasks are grouped into 1 of the following types based on their configured parameters.

- Build - Task generates one or many artifacts, and is derived from the [`outputs`](/docs/config/project#outputs) setting.

- Run - Task runs a one-off, long-running, or never-ending process, and is derived from the [`local`](/docs/config/project#local) setting.

- Test - Task asserts code is correct and behaves as expected. This includes linting, typechecking, unit tests, and any other form of testing. Is the default.

## Modes

Alongside types, tasks can also grouped into a special mode that provides unique handling within the
action graph and pipelines.

### Local only

Tasks either run locally, in CI (continuous integration pipelines), or both. For tasks that should
only be ran locally, for example, development servers and watchers, we provide a mechanism for
marking a task as local only. When enabled, caching is turned off, the task will not run in CI,
terminal output is not captured, and the task is marked as [persistent](#persistent).

To mark a task as local only, enable the [`local`](/docs/config/project#local) setting.

moon.yml

```
tasks:  dev:    command: 'start-dev-server'    preset: 'server'
```

### Internal onlyv1.23.0

Internal tasks are tasks that are not meant to be ran explicitly by the user (via
[`moon check`](/docs/commands/check) or [`moon run`](/docs/commands/run)), but are used internally as
dependencies of other tasks. Additionally, internal tasks are not displayed in a project's tasks
list, but can be inspected with [`moon task`](/docs/commands/task).

To mark a task as internal, enable the [`options.internal`](/docs/config/project#internal) setting.

moon.yml

```
tasks:  prepare:    command: 'intermediate-step'    options:      internal: true
```

### Interactivev1.12.0

Tasks that need to interact with the user via terminal prompts are known as interactive tasks.
Because interactive tasks require stdin, and it's not possible to have multiple parallel running
tasks interact with stdin, we isolate interactive tasks from other tasks in the action graph. This
ensures that only 1 interactive task is ran at a time.

To mark a task as interactive, enable the [`options.interactive`](/docs/config/project#interactive)
setting.

moon.yml

```
tasks:  init:    command: 'init-app'    options:      interactive: true
```

### Persistentv1.6.0

Tasks that never complete, like servers and watchers, are known as persistent tasks. Persistent
tasks are typically problematic when it comes to dependency graphs, because if they run in the
middle of the graph, subsequent tasks will never run because the persistent task never completes!

However in moon, this is a non-issue, as we collect all persistent tasks within the action graph and
run them last as a batch. This is perfect for a few reasons:

- All persistent tasks are ran in parallel, so they don't block each other.

- Running both the backend API and frontend webapp in parallel is a breeze.

- Dependencies of persistent tasks are guaranteed to have ran and completed.

To mark a task as persistent, enable the [`local`](/docs/config/project#local) or
[`options.persistent`](/docs/config/project#persistent) settings.

moon.yml

```
tasks:  dev:    command: 'start-dev-server'    preset: 'server'    # OR    options:      persistent: true
```

## Configuration

Tasks can be configured per project through [`moon.yml`](/docs/config/project), or for many projects
through [`.moon/tasks/all.yml`](/docs/config/tasks).

### Commands vs Scripts

A task is either a command or script, but not both. So what's the difference exactly? In the context
of a moon task, a command is a single binary execution with optional arguments, configured with the
[`command`](/docs/config/project#command) and [`args`](/docs/config/project#args) settings (which both
support a string or array). While a script is one or many binary executions, with support for pipes
and redirects, and configured with the [`script`](/docs/config/project#script) setting (which is only a
string).

A command also supports merging during task inheritance, while a script does not and will always
replace values. Refer to the table below for more differences between the 2.

Command Script

Configured as string, array string

Inheritance merging ✅ via `mergeArgs` option ⚠️ always replaces

Additional args ✅ via `args` setting ❌

Passthrough args (from CLI) ✅ ❌

Multiple commands (with `&&` or `;`) ❌ ✅

Pipes, redirects, etc ❌ ✅

Always ran in a shell ❌ ✅

Custom platform/toolchain ✅ ✅

[Token](/docs/concepts/token) functions and variables ✅ ✅

### Inheritance

View the official documentation on [task inheritance](/docs/concepts/task-inheritance).

## /docs/concepts/task-inheritance

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

## /docs/concepts/token

Source: https://moonrepo.dev/docs/concepts/token

# Tokens

Tokens are variables and functions that can be used by [`command`](/docs/config/project#command),
[`args`](/docs/config/project#args), [`env`](/docs/config/project#env) (>= v1.12),
[`inputs`](/docs/config/project#inputs), and [`outputs`](/docs/config/project#outputs) when configuring a
task. They provide a way of accessing file group paths, referencing values from other task fields,
and referencing metadata about the project and task itself.

## Functions

A token function is labeled as such as it takes a single argument, starts with an `@`, and is
formatted as `@name(arg)`. The following token functions are available, grouped by their
functionality.

caution

Token functions must be the only content within a value, as they expand to multiple files. When
used in an `env` value, multiple files are joined with a comma (`,`).

### File groups

These functions reference file groups by name, where the name is passed as the argument.

### `@group`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@group(file_group)` token is a standard token that will be replaced with the file group items
as-is, for both file paths and globs. This token merely exists for reusability purposes.

```
fileGroups:  storybook:    - '.storybook/**/*'    - 'src/**/*'    - '**/*.stories.*'# Configured astasks:  build:    command: 'build-storybook'    inputs:      - '@group(storybook)'  start:    command: 'start-storybook'    inputs:      - '@group(storybook)'# Resolves totasks:  build:    command: 'build-storybook'    inputs:      - '/path/to/project/.storybook/**/*'      - '/path/to/project/src/**/*'      - '/path/to/project/**/*.stories.*'  start:    command: 'start-storybook'    inputs:      - '/path/to/project/.storybook/**/*'      - '/path/to/project/src/**/*'      - '/path/to/project/**/*.stories.*'
```

### `@dirs`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@dirs(file_group)` token will be replaced with an expanded list of directory paths, derived
from the file group of the same name. If a glob pattern is detected within the file group, it will
aggregate all directories found.

warning

This token walks the file system to verify each directory exists, and filters out those that don't.
If using within `outputs`, you're better off using [`@group`](#group) instead.

```
fileGroups:  lintable:    - 'src'    - 'tests'    - 'scripts'    - '*.config.js'# Configured astasks:  lint:    command: 'eslint @dirs(lintable) --color'    inputs:      - '@dirs(lintable)'# Resolves totasks:  lint:    command:      - 'eslint'      - 'src'      - 'tests'      - 'scripts'      - '--color'    inputs:      - '/path/to/project/src'      - '/path/to/project/tests'      - '/path/to/project/scripts'
```

### `@files`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@files(file_group)` token will be replaced with an expanded list of file paths, derived from
the file group of the same name. If a glob pattern is detected within the file group, it will
aggregate all files found.

warning

This token walks the file system to verify each file exists, and filters out those that don't. If
using within `outputs`, you're better off using [`@group`](#group) instead.

```
fileGroups:  config:    - '*.config.js'    - 'package.json'# Configured astasks:  build:    command: 'webpack build @files(config)'    inputs:      - '@files(config)'# Resolves totasks:  build:    command:      - 'webpack'      - 'build'      - 'babel.config.js'      - 'webpack.config.js'      - 'package.json'    inputs:      - '/path/to/project/babel.config.js'      - '/path/to/project/webpack.config.js'      - '/path/to/project/package.json'
```

### `@globs`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@globs(file_group)` token will be replaced with the list of glob patterns as-is, derived from
the file group of the same name. If a non-glob pattern is detected within the file group, it will be
ignored.

```
fileGroups:  tests:    - 'tests/**/*'    - '**/__tests__/**/*'# Configured astasks:  test:    command: 'jest --testMatch @globs(tests)'    inputs:      - '@globs(tests)'# Resolves totasks:  test:    command:      - 'jest'      - '--testMatch'      - 'tests/**/*'      - '**/__tests__/**/*'    inputs:      - '/path/to/project/tests/**/*'      - '/path/to/project/**/__tests__/**/*'
```

### `@root`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@root(file_group)` token will be replaced with the lowest common directory, derived from the
file group of the same name. If a glob pattern is detected within the file group, it will walk the
file system and aggregate all directories found before reducing.

```
fileGroups:  sources:    - 'src/app'    - 'src/packages'    - 'src/scripts'# Configured astasks:  format:    command: 'prettier --write @root(sources)'    inputs:      - '@root(sources)'# Resolves totasks:  format:    command:      - 'prettier'      - '--write'      - 'src'    inputs:      - '/path/to/project/src'
```

When there's no directies, or too many directories, this function will return the project root
using `.`.

### `@envs`v1.21.0

Usable in `inputs`.

The `@envs(file_group)` token will be replaced with all environment variables that have been
configured in the group of the provided name.

```
fileGroups:  sources:    - 'src/**/*'    - '$NODE_ENV'# Configured astasks:  build:    command: 'vite build'    inputs:      - '@envs(sources)'# Resolves totasks:  build:    command: 'vite build'    inputs:      - '$NODE_ENV'
```

### Inputs & outputs

### `@in`

Usable in `script` and `args` only.

The `@in(index)` token will be replaced with a single path, derived from
[`inputs`](/docs/config/project#inputs) by numerical index. If a glob pattern is referenced by index,
the glob will be used as-is, instead of returning the expanded list of files.

```
# Configured astasks:  build:    command:      - 'babel'      - '--copy-files'      - '--config-file'      - '@in(1)'      - '@in(0)'    inputs:      - 'src'      - 'babel.config.js'# Resolves totasks:  build:    command:      - 'babel'      - '--copy-files'      - '--config-file'      - 'babel.config.js'      - 'src'    inputs:      - '/path/to/project/src'      - '/path/to/project/babel.config.js'
```

### `@out`

Usable in `script` and `args` only.

The `@out(index)` token will be replaced with a single path, derived from
[`outputs`](/docs/config/project#outputs) by numerical index.

```
# Configured astasks:  build:    command:      - 'babel'      - '.'      - '--out-dir'      - '@out(0)'    outputs:      - 'lib'# Resolves totasks:  build:    command:      - 'babel'      - '.'      - '--out-dir'      - 'lib'    outputs:      - '/path/to/project/lib'
```

### Miscellaneous

### `@meta`v1.28.0

Usable in `command`, `script`, `args`, `env`, `inputs`, and `outputs` only.

The `@meta(key)` token can be used to access project metadata and will be replaced with a value
derived from [`project`](/docs/config/project#project) in [`moon.yml`](/docs/config/project).

The top-level fields (like `name` and `description`) will be used as-is (no quotes). If the setting
is not defined, it will default to nothing or an empty string. For lists of values, they will be
joined with `,`.

Custom metadata defined in [`project`](/docs/config/project#project) can also be accessed by key, but
will return a JSON stringified value. For example, a custom string value of `example` will be
stringified to `"example"` (with quotes).

```
project:  title: 'example'  index: 123# Configured astasks:  build:    script: 'build --name @meta(title) --index @meta(index)'# Resolves totasks:  build:    script: 'build --name example --index 123'
```

## Variables

A token variable is a value that starts with `$` and is substituted to a value derived from the
current workspace, project, and task. And unlike token functions, token variables can be placed
within content when necessary, and supports multiple variables within the same content.

### Environmentv1.30.0

- `$arch` - The current host architecture, derived from the Rust [`ARCH` constant](https://doc.rust-lang.org/std/env/consts/constant.ARCH.html).

- `$os` - The current operating system, derived from the Rust [`OS` constant](https://doc.rust-lang.org/std/env/consts/constant.OS.html).

- `$osFamily` - The current operating system family, either `unix` or `windows`.

```
# Configured astasks:  build:    command: 'example --arch $arch'# Resolves totasks:  build:    command:      - 'example'      - '--arch'      - 'aarch64'
```

### Workspace

- `$workingDir` - The current working directory.

- `$workspaceRoot` - Absolute file path to the workspace root.

```
# Configured astasks:  build:    command:      - 'example'      - '--cwd'      - '$workspaceRoot'# Resolves totasks:  build:    command:      - 'example'      - '--cwd'      - '/path/to/repo'
```

### Project

Most values are derived from settings in [`moon.yml`](/docs/config/project). When a setting is not
defined, or does not have a config, the variable defaults to "unknown" (for enums) or an empty
string.

- `$language` Programming language the project is written in, as defined with [`language`](/docs/config/project#language).

- `$project` - ID of the project that owns the currently running task, as defined in [`.moon/workspace.yml`](/docs/config/workspace).

- `$projectAlias` - Alias of the project that owns the currently running task.

- `$projectChannel` - The discussion channel for the project, as defined with [`project.channel`](/docs/config/project#channel). v1.28.0

- `$projectLayer` - The project layer, as defined with [`layer`](/docs/config/project#layer). v1.39.0

- `$projectTitle` - The human-readable name of the project, as defined with [`project.title`](/docs/config/project#title). v1.28.0

- `$projectOwner` - The owner of the project, as defined with [`project.owner`](/docs/config/project#name). v1.28.0

- `$projectRoot` - Absolute file path to the project root.

- `$projectSource` - Relative file path from the workspace root to the project root, as defined in [`.moon/workspace.yml`](/docs/config/workspace).

- `$projectStack` - The stack of the project, as defined with [`stack`](/docs/config/project#stack). v1.22.0

```
# Configured astasks:  build:    command: 'example debug $language'# Resolves totasks:  build:    command:      - 'example'      - 'debug'      - 'node'
```

### Task

- `$target` - Fully-qualified target that is currently running.

- `$task` - ID of the task that is currently running. Does not include the project ID.

- `$taskToolchain` - The toolchain that task will run against, as defined in [`moon.yml`](/docs/config/project). v1.31.0

- `$taskType` - The [type of task](/docs/concepts/task#types), based on its configured settings.

```
# Configured astasks:  build:    command: 'example $target'# Resolves totasks:  build:    command:      - 'example'      - 'web:build'
```

### Date/Time

- `$date` - The current date in the format of `YYYY-MM-DD`.

- `$datetime` - The current date and time in the format of `YYYY-MM-DD_HH:MM:SS`.

- `$time` - The current time in the format of `HH:MM:SS`.

- `$timestamp` - The current date and time as a UNIX timestamp in seconds.

```
# Configured astasks:  build:    command: 'example --date $date'# Resolves totasks:  build:    command:      - 'example'      - '--date'      - '2023-03-17'
```

### VCSv1.30.0

- `$vcsBranch` - The current branch.

- `$vcsRepository` - The repository slug, in the format of `owner/repo`.

- `$vcsRevision` - The current revision (commit, etc).

```
# Configured astasks:  build:    command: 'example --branch $vcsBranch'# Resolves totasks:  build:    command:      - 'example'      - '--branch'      - 'master'
```

## /docs/concepts/toolchain

Source: https://moonrepo.dev/docs/concepts/toolchain

# Toolchain

The toolchain is an internal layer for downloading, installing, and managing tools (languages,
dependency managers, libraries, and binaries) that are required at runtime. We embrace this approach
over relying on these tools "existing" in the current environment, as it ensures the following
across any environment or machine:

- The version and enabled features of a tool are identical.

- Tools are isolated and unaffected by external sources.

- Builds are consistent, reproducible, and hopefully deterministic.

Furthermore, this avoids a developer, pipeline, machine, etc, having to pre-install all the
necessary tools, and to keep them in sync as time passes.

## How it works

The toolchain is built around [proto](/proto), our stand-alone multi-language version manager. moon
will piggyback of proto's toolchain found at `~/.proto` and reuse any tools available, or download
and install them if they're missing.

### Force disabling

The `MOON_TOOLCHAIN_FORCE_GLOBALS` environment variable can be set to `true` to force moon to use
tool binaries available on `PATH`, instead of downloading and installing them. This is useful for
pre-configured environments, like CI and Docker.

```
MOON_TOOLCHAIN_FORCE_GLOBALS=true
```

Additionally, the name of one or many tools can be passed to this variable to only force globals for
those tools, and use the toolchain for the remaining tools.

```
MOON_TOOLCHAIN_FORCE_GLOBALS=node,yarn
```

## Configuration

The tools that are managed by the toolchain are configured through the
[`.moon/toolchains.yml`](/docs/config/toolchain) file, but can be overridden in each project with
[`moon.yml`](/docs/config/project#toolchain).

### Version specification

As mentioned above, tools within the toolchain are managed by version for consistency across
machines. These versions are configured on a per-tool basis in
[`.moon/toolchains.yml`](/docs/config/toolchain). So what kinds of versions are allowed?

- Full versions - A full version is a semantic version that is fully specified, such as `1.2.3` or `2.0.0-rc.1`. This is the most common way to specify a version, and is preferred to avoid subtle deviations.

- Partial versions - A partial version is a version that is either missing a patch number, minor number, or both, such as `1.2` or `1`. These can also be represented with requirement syntax, such as `^1.2` or `~1`. If using partials, we suggest having a major and minor number to reduce the deviation of versions across machines.

- Aliases - An alias is a human-readable word that maps to a specific version. For example, `latest` or `stable` maps to the latest version of a tool, or `canary` which maps to applicable canary release, or even a completely custom alias like `berry`. Aliases are language specific, are not managed by moon, and are not suggested for use since they can change at any time (or even daily!).

This sounds great, but how exactly does this work? For full versions and aliases, it's straight
forward, as the resolved version is used as-is (assuming it's a legitimate version), and can be
found at `~/.proto/tools//`.

For partial versions, we first check locally installed versions for a match, by scanning
`~/.proto/tools/`. For example, if the requested version is `1.2` and we have `1.2.10`
installed locally, we'll use that version instead of downloading the latest `1.2.*` version.
Otherwise, we'll download the latest version that matches the partial version, and install it
locally.

## /docs/concepts/workspace

Source: https://moonrepo.dev/docs/concepts/workspace

# Workspace

A workspace is a directory that contains [projects](/docs/concepts/project), manages a [toolchain](/docs/concepts/toolchain),
runs [tasks](/docs/concepts/task), and is coupled with a VCS repository. The root of a workspace is denoted by a
`.moon` folder.

By default moon has been designed for monorepos, but can also be used for polyrepos.

## Configuration

Configuration that's applied to the entire workspace is defined in
[`.moon/workspace.yml`](/docs/config/workspace).

## /docs/config

Source: https://moonrepo.dev/docs/config

- [Home](/)
- [Config files](/docs/config)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Config files

[📄️ OverviewSupported formats](/docs/config/overview)

[📄️ .moon/workspaceThe .moon/workspace.yml file configures projects and services in the workspace. This file is](/docs/config/workspace)

[📄️ .moon/extensionsThe .moon/extensions.yml file configures extensions that can hook into pipeline events, or be](/docs/config/extensions)

[📄️ .moon/toolchainsThe .moon/toolchains.yml file configures the toolchain and the workspace development environment.](/docs/config/toolchain)

[📄️ .moon/tasksThe .moon/tasks//* files configures file groups and tasks that are inherited by every project](/docs/config/tasks)

[📄️ moonThe moon.yml configuration file is not required but can be used to define additional metadata](/docs/config/project)

[📄️ templateThe template.yml file configures metadata and variables for a template,](/docs/config/template)

[Workspace](/docs/concepts/workspace)

[Overview](/docs/config/overview)

## /docs/config/extensions

Source: https://moonrepo.dev/docs/config/extensions

# .moon/extensions

v2.0.0

The `.moon/extensions.yml` file configures extensions that can hook into pipeline events, or be
executed directly. This file is optional.

## `extends`

Defines one or many external `.moon/extensions.yml`'s to extend and inherit settings from. Perfect
for reusability and sharing configuration across repositories and projects. When defined, this
setting must be an HTTPS URL or relative file system path that points to a valid YAML document!

.moon/extensions.yml

```
extends: 'https://raw.githubusercontent.com/organization/repository/master/.moon/extensions.yml'
```

caution

Settings will be merged recursively for blocks, with values defined in the local configuration
taking precedence over those defined in the extended configuration.

## How it works

A mapping of extensions that can be downloaded and executed with the [`moon ext`](/docs/commands/ext)
command. An extension is a WASM plugin, and the location of the WASM file must be defined with the
`plugin` field, which requires a
[plugin locator string](/docs/guides/wasm-plugins#configuring-plugin-locations).

.moon/extensions.yml

```
example:  plugin: 'file://./path/to/example.wasm'  # or  plugin: 'https://example.com/path/to/example.wasm'
```

Additionally, extensions support custom configuration that is passed to the WASM runtime when the
plugin is instantiated. This configuration is defined by inserting additional fields under the
extension name, relative to the `plugin` field. Each extension may have its own settings, so refer
to their documentation for more information.

.moon/extensions.yml

```
example:  plugin: 'file://./path/to/example.wasm'  setting1: true  setting2: 'abc'
```

## Supported extensions

View the [official guide](/docs/guides/extensions) for all built-in extensions.

## /docs/config/overview

Source: https://moonrepo.dev/docs/config/overview

# Overview

## Supported formatsv2.0.0

In moon, you can define configuration files in a variety of formats. We currently support the
following:

- JSON (`.json`)

- JSON with comments (`.jsonc`)

- [HCL](https://github.com/hashicorp/hcl) (`.hcl`)

- [Pkl](https://pkl-lang.org/) (`.pkl`)

- [TOML](https://toml.io/en/) (`.toml`)

- YAML (`.yml`, `.yaml`)

info

In moon v1, only YAML (`.yml`) and Pkl (`.pkl`) configuration files were supported.

## Schema validationv1.33.0

We support schema validation for all configuration files through
[JSON Schema](https://json-schema.org/), even for formats that are not JSON (depends on tool/editor
support). To reference the schema for a specific configuration file, configure the `$schema`
property at the top of the file with the appropriate schema found at `.moon/cache/schemas`.

- .moon/workspace
- .moon/extensions
- .moon/toolchains
- .moon/tasks
- moon
- template

.moon/workspace.yml

```
$schema: './cache/schemas/workspace.json'
```

.moon/extensions.yml

```
$schema: './cache/schemas/extensions.json'
```

.moon/toolchains.yml

```
$schema: './cache/schemas/toolchains.json'
```

.moon/tasks/all.yml

```
$schema: '../cache/schemas/tasks.json'
```

moon.yml

```
$schema: '../path/to/.moon/cache/schemas/project.json'
```

template.yml

```
$schema: '../path/to/.moon/cache/schemas/template.json'
```

info

The schemas are automatically created when running a task. If they do not exist yet, you can run
[`moon sync config-schemas`](/docs/commands/sync/config-schemas) to generate them manually.

danger

In older versions of moon, the schema files were located at `https://moonrepo.dev/schemas`. These
URLs are now deprecated, as they do not support dynamic settings. Please update your `$schema`
references to point to the local schema files in `.moon/cache/schemas`.

## /docs/config/project

Source: https://moonrepo.dev/docs/config/project

# moon

The `moon.yml` configuration file is not required but can be used to define additional metadata
for a project, override inherited tasks, and more at the project-level. When used, this file must
exist in a project's root, as configured in [`projects`](/docs/config/workspace#projects).

## `dependsOn`

Explicitly defines other projects that this project depends on, primarily when generating the
project and task graphs. The most common use case for this is building those projects before
building this one. When defined, this setting requires an array of project names, which are the keys
found in the [`projects`](/docs/config/workspace#projects) map.

moon.yml

```
dependsOn:  - 'apiClients'  - 'designSystem'
```

A dependency object can also be defined, where a specific `scope` can be assigned, which accepts
"production" (default), "development", "build", or "peer".

moon.yml

```
dependsOn:  - id: 'apiClients'    scope: 'production'  - id: 'designSystem'    scope: 'peer'
```

Learn more about [implicit and explicit dependencies](/docs/concepts/project#dependencies).

## Metadata

## `id`v1.18.0

Overrides the name (identifier) of the project, which was configured in or derived from the
[`projects`](/docs/config/workspace#projects) setting in [`.moon/workspace.yml`](/docs/config/workspace). This setting is
useful when using glob based project location, and want to avoid using the folder name as the
project name.

moon.yml

```
id: 'custom-id'
```

info

All references to the project must use the new identifier, including project and task dependencies.

## `language`

The primary programming language the project is written in. This setting is required for
[task inheritance](/docs/config/tasks), editor extensions, and more. Supports the following values:

- `bash` - A [Bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) based project (Unix only).

- `batch` - A [Batch](https://en.wikibooks.org/wiki/Windows_Batch_Scripting)/PowerShell based project (Windows only).

- `go` - A [Go](https://go.dev/) based project.

- `javascript` - A [JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript) based project.

- `php` - A [PHP](https://www.php.net) based project.

- `python` - A [Python](https://www.python.org/) based project.

- `ruby` - A [Ruby](https://www.ruby-lang.org/en/) based project.

- `rust` - A [Rust](https://www.rust-lang.org/) based project.

- `typescript` - A [TypeScript](https://www.typescriptlang.org/) based project.

- `unknown` (default) - When not configured or inferred.

- `*` - A custom language. Values will be converted to kebab-case.

moon.yml

```
language: 'javascript'# Customlanguage: 'kotlin'
```

For convenience, when this setting is not defined, moon will attempt to detect the language based
on configuration files found in the project root. This only applies to non-custom languages!

## `owners`v1.8.0

Defines ownership of source code within the current project, by mapping file system paths to owners.
An owner is either a user, team, or group.

Currently supports
[GitHub](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners),
[GitLab](https://docs.gitlab.com/ee/user/project/codeowners/reference.html), and
[Bitbucket](https://marketplace.atlassian.com/apps/1218598/code-owners-for-bitbucket?tab=overview&hosting=cloud)
(via app).

### `customGroups`Bitbucket

When using the
[Code Owners for Bitbucket](https://marketplace.atlassian.com/apps/1218598/code-owners-for-bitbucket?tab=overview&hosting=cloud)
app, this setting provides a way to define custom groups that will be injected at the top of the
`CODEOWNERS` file. These groups must be unique across all projects.

moon.yml

```
owners:  customGroups:    '@@@backend': ['@"user name"', '@@team']
```

### `defaultOwner`

The default owner for all [`paths`](#paths). This setting is optional in some cases but helps to
avoid unnecessary repetition.

moon.yml

```
owners:  defaultOwner: '@frontend'
```

### `optional`GitLab

For GitLab, marks the project's
[code owners section](https://docs.gitlab.com/ee/user/project/codeowners/reference.html#optional-sections)
as optional. Defaults to `false`.

moon.yml

```
owners:  optional: true
```

### `paths`

The primary setting for defining ownership of source code within the current project. This setting
supports 2 formats, the first being a list of file paths relative from the current project. This
format requires [`defaultOwner`](#defaultowner) to be defined, and only supports 1 owner for every
path (the default owner).

moon.yml

```
owners:  defaultOwner: '@frontend'  paths:    - '**/*.ts'    - '**/*.tsx'    - '*.config.js'
```

The second format provides far more granularity, allowing for multiple owners per path. This format
requires a map, where the key is a file path relative from the current project, and the value is a
list of owners. Paths with an empty list of owners will fallback to [`defaultOwner`](#defaultowner).

moon.yml

```
owners:  defaultOwner: '@frontend'  paths:    '**/*.rs': ['@backend']    '**/*.js': []    '*.config.js': ['@frontend', '@frontend-infra']
```

The syntax for owners is dependent on the provider you are using for version control (GitHub,
GitLab, Bitbucket). moon provides no validation or guarantees that these are correct.

### `requiredApprovals`Bitbucket / GitLab

Requires a specific number of approvals for a pull/merge request to be satisfied. Defaults to `1`.

- For Bitbucket, defines the [`Check()` condition](https://docs.mibexsoftware.com/codeowners/merge-checks#MergeChecks-2.MergeChecks:HowmanyoftheseCodeOwnersneedtoapprovebeforeapullrequestcanbemerged?) when using a [`defaultOwner`](#defaultowner).

- For GitLab, defines a requirement on the [code owners section](https://docs.gitlab.com/ee/user/project/codeowners/reference.html#sections-requiring-multiple-approvals).

moon.yml

```
owners:  requiredApprovals: 2
```

## `layer`

The layer within a [stack](#stack). Supports the following values:

- `application` - An application of any kind.

- `automation` - An automated testing suite, like E2E, integration, or visual tests. v1.16.0

- `configuration` - Configuration files or infrastructure. v1.22.0

- `library` - A self-contained, shareable, and publishable set of code.

- `scaffolding` - Templates or generators for scaffolding. v1.22.0

- `tool` - An internal tool, CLI, one-off script, etc.

- `unknown` (default) - When not configured.

moon.yml

```
layer: 'application'
```

info

The project layer is used in [task inheritance](/docs/concepts/task-inheritance),
[constraints and boundaries](/docs/config/workspace#constraints), editor extensions, and more!

## `project`

The `project` setting defines metadata about the project itself.

moon.yml

```
project:  title: 'moon'  description: 'A monorepo management tool.'  channel: '#moon'  owner: 'infra.platform'  maintainers: ['miles.johnson']
```

The information listed within `project` is purely informational and primarily displayed within the
CLI. However, this setting exists for you, your team, and your company, as a means to identify and
organize all projects. Feel free to build your own tooling around these settings!

### `channel`

The Slack, Discord, Teams, IRC, etc channel name (with leading #) in which to discuss the project.

### `description`Required

A description of what the project does and aims to achieve. Be as descriptive as possible, as this
is the kind of information search engines would index on.

### `maintainers`

A list of people/developers that maintain the project, review code changes, and can provide support.
Can be a name, email, LDAP name, GitHub username, etc, the choice is yours.

### `title`

A human readable name of the project. This is different from the unique project name configured in
[`projects`](/docs/config/workspace#projects).

### `owner`

The team or organization that owns the project. Can be a title, LDAP name, GitHub team, etc. We
suggest not listing people/developers as the owner, use [maintainers](#maintainers) instead.

### Custom fieldsv2.0.0

Additional fields can be configured as custom metadata to associate to this project. Supports all
value types that are valid JSON.

moon.yml

```
project:  # ...  deprecated: true
```

## `stack`v1.22.0

The technology stack this project belongs to, primarily for categorization. Supports the following
values:

- `backend` - Server-side APIs, etc.

- `data` - Data sources, database layers, etc. v2.0.0

- `frontend` - Client-side user interfaces, etc.

- `infrastructure` - Cloud/server infrastructure, Docker, etc.

- `systems` - Low-level systems programming.

- `unknown` (default) - When not configured.

moon.yml

```
stack: 'frontend'
```

info

The project stack is also used in [constraints and boundaries](/docs/config/workspace#constraints)!

## `tags`

Tags are a simple mechanism for categorizing projects. They can be used to group projects together
for [easier querying](/docs/commands/query/projects), enforcing of
[project boundaries and constraints](/docs/config/workspace#constraints),
[task inheritance](/docs/concepts/task-inheritance), and more.

moon.yml

```
tags:  - 'react'  - 'prisma'
```

## Integrations

## `docker`v1.27.0

Configures Docker integration for the current project.

### `file`

Configures the `Dockerfile` generation process when [`moon docker file`](/docs/commands/docker/file) is
executed.

#### `buildTask`

The name of a task within the current project that will be used for building the project before
running it. If not defined, does nothing.

moon.yml

```
docker:  file:    buildTask: 'build'
```

#### `image`

The Docker image to use in the base stage. Defaults to an image based on the project's toolchain, as
outlined below.

- `oven/bun:latest` for Bun

- `denoland/deno:latest` for Deno

- `node:latest` for Node.js

- `python:latest` for Python

- `rust:latest` for Rust

- `scratch` for everything else

moon.yml

```
docker:  file:    image: 'node:latest'
```

#### `startTask`

The name of a task within the current project that will run the project after it has been built (if
required). This task will be used as `CMD` within the `Dockerfile`.

moon.yml

```
docker:  file:    startTask: 'start'
```

### `scaffold`

Configures aspects of the Docker scaffolding process when
[`moon docker scaffold`](/docs/commands/docker/scaffold) is executed. Only applies to the
[sources skeleton](/docs/commands/docker/scaffold#sources).

#### `sourcesPhaseGlobs`

List of globs in which to copy project-relative files into the `.moon/docker/sources` skeleton. When
not defined, defaults to `**/*`.

moon.yml

```
docker:  scaffold:    sourcesPhaseGlobs:      - 'src/**/*'
```

## Tasks

## `env`

The `env` field is map of strings that are passed as environment variables to all tasks within the
current project. Project-level variables will not override task-level variables of the same name.

moon.yml

```
env:  NODE_ENV: 'production'
```

View the task [`env`](#env-1) setting for more usage examples and information.

## `fileGroups`

Defines [file groups](/docs/concepts/file-group) to be used by local tasks. By default, this setting
is not required for the following reasons:

- File groups are an optional feature, and are designed for advanced use cases.

- File groups defined in [`.moon/tasks/all.yml`](/docs/config/tasks) will be inherited by all projects.

When defined this setting requires a map, where the key is the file group name, and the value is a
list of [globs or file paths](/docs/concepts/file-pattern), or environment variables. Globs and paths
are [relative to a project](/docs/concepts/file-pattern#project-relative) (even when defined
[globally](/docs/config/tasks)).

moon.yml

```
# Example groupsfileGroups:  configs:    - '*.config.{js,cjs,mjs}'    - '*.json'  sources:    - 'src/**/*'    - 'types/**/*'  tests:    - 'tests/**/*'    - '**/__tests__/**/*'  assets:    - 'assets/**/*'    - 'images/**/*'    - 'static/**/*'    - '**/*.{scss,css}'
```

Once your groups have been defined, you can reference them within [`args`](#args),
[`inputs`](#inputs), [`outputs`](#outputs), and more, using
[token functions and variables](/docs/concepts/token).

moon.yml

```
tasks:  build:    command: 'vite build'    inputs:      - '@group(configs)'      - '@group(sources)'
```

## `tasks`

Tasks are actions that are ran within the context of a [project](/docs/concepts/project), and commonly
wrap an npm binary or system command. This setting requires a map, where the key is a unique name
for the task, and the value is an object of task parameters.

moon.yml

```
tasks:  format:    command: 'prettier'  lint:    command: 'eslint'  test:    command: 'jest'  typecheck:    command: 'tsc'
```

### `extends`v1.12.0

The `extends` field can be used to extend the settings from a sibling task within the same project,
or [inherited from the global scope](/docs/concepts/task-inheritance). This is useful for composing
similar tasks with different arguments or options.

When extending another task, the same
[merge strategies](/docs/concepts/task-inheritance#merge-strategies) used for inheritance are applied.

moon.yml

```
tasks:  lint:    command: 'eslint .'    inputs:      - 'src/**/*'  lint-fix:    extends: 'lint'    args: '--fix'    preset: 'utility'
```

### `description`v1.22.0

A human-readable description of what the task does. This information is displayed within the
[`moon project`](/docs/commands/project) and [`moon task`](/docs/commands/task) commands.

moon.yml

```
tasks:  build:    description: 'Builds the project using Vite'    command: 'vite build'
```

### `command`

The `command` field is a single command to execute for the task, including the command binary/name
(must be first) and any optional [arguments](#args). This field supports task inheritance and
merging of arguments.

This setting can be defined using a string, or an array of strings. We suggest using arrays when
dealing with many args, or the args string cannot be parsed easily.

moon.yml

```
tasks:  format:    # Using a string    command: 'prettier --check .'    # Using an array    command:      - 'prettier'      - '--check'      - '.'
```

info

If you need to support pipes, redirects, or multiple commands, use [`script`](#script) instead.
Learn more about [commands vs scripts](/docs/concepts/task#commands-vs-scripts).

#### Special commands

For interoperability reasons, the following command names have special handling.

- `noop`, `no-op`, `nop` - Marks the task as a "no operation". Will not execute a command in the action pipeline but can define dependencies.

- When `toolchain` is "bun": `bun`, `bunx` - Uses the binaries from the toolchain.

- When `toolchain` is "deno": Will execute with `deno` binary.

- When `toolchain` is "node": `node`, `npm`, `pnpm`, `yarn` - Uses the binaries from the toolchain.

- When `toolchain` is "rust": Will execute with `cargo` binary.

### `args`

The `args` field is a collection of additional arguments to append to the [`command`](#command)
when executing the task. This field exists purely to provide arguments for
[inherited tasks](/docs/config/tasks#tasks).

This setting can be defined using a string, or an array of strings. We suggest using arrays when
dealing with many args, or the args string cannot be parsed easily.

moon.yml

```
tasks:  test:    command: 'jest'    # Using a string    args: '--color --maxWorkers 3'    # Using an array    args:      - '--color'      - '--maxWorkers'      - '3'
```

However, for the array approach to work correctly, each argument must be its own distinct item,
including argument values. For example:

moon.yml

```
tasks:  test:    command: 'jest'    args:      # Valid      - '--maxWorkers'      - '3'      # Also valid      - '--maxWorkers=3'      # Invalid      - '--maxWorkers 3'
```

### `deps`

The `deps` field is a list of other tasks (known as [targets](/docs/concepts/target)), either within
this project or found in another project, that will be executed before this task. It achieves this
by generating a directed task graph based on the project graph.

moon.yml

```
tasks:  build:    command: 'webpack'    deps:      - 'apiClients:build'      - 'designSystem:build'      # A task within the current project      - 'codegen'
```

#### Args & env

Furthermore, for each dependency target, you can configure additional command line arguments and
environment variables that'll be passed to the dependent task when it is ran. The `args` field
supports a list of strings, while `env` is an object of key-value pairs.

moon.yml

```
tasks:  build:    command: 'webpack'    deps:      - target: 'apiClients:build'        args: ['--env', 'production']        env:          NODE_ENV: 'production'
```

Dependencies of inherited tasks will be excluded and renamed according to the
[`workspace.inheritedTasks`](#inheritedtasks) setting. This process only uses filters from the
current project, and not filters from dependent projects. Furthermore, `args` and `env` are not
deeply merged.

#### Optional

By default, all dependencies are required to exist when tasks are being built and expanded, but this
isn't always true when dealing with composition and inheritance. For dependencies that may not exist
based on what's inherited, you can mark it as `optional`.

moon.yml

```
tasks:  build:    command: 'webpack'    deps:      - target: 'apiClients:build'        optional: true
```

### `env`

The `env` field is map of strings that are passed as environment variables when running the command.
Variables defined here will take precedence over those loaded with [`envFile`](#envfile).

moon.yml

```
tasks:  build:    command: 'webpack'    env:      NODE_ENV: 'production'
```

Variables also support substitution using the syntax `${VAR_NAME}`. When using substitution, only
variables in the current process can be referenced, and not those currently defined in `env`.

moon.yml

```
tasks:  build:    command: 'webpack'    env:      APP_TARGET: '${REGION}-${ENVIRONMENT}'
```

### `inputs`

The `inputs` field is a list of sources that calculate whether to execute this task based on the
environment and files that have been touched since the last time the task has been ran. If not
defined or inherited, then all files within a project are considered an input (`**/*`), excluding
root-level tasks.

Inputs support the following source types:

- Environment variables

- Environment variable wildcards v1.22.0

- Files, folders, and globs

- [Token functions and variables](/docs/concepts/token)

moon.yml

```
tasks:  lint:    command: 'eslint'    inputs:      # Config files anywhere within the project      - '**/.eslintignore'      - '**/.eslintrc.js'      # Config files at the workspace root      - '/.eslintignore'      - '/.eslintrc.js'      # Tokens      - '$projectRoot'      - '@group(sources)'
```

#### Environment variables

Environment variables can be used as inputs and must start with a `$`. Wildcard variables can use
`*` to match any character.

moon.yml

```
tasks:  example:    inputs:      - '$FOO_CACHE'      - '$FOO_*'
```

caution

When using an environment variable, we assume it's not defined by default, and will trigger an
affected state when it is defined. If the environment variable always exists, then the task will
always run and bypass the cache.

#### File paths

File paths support
[project and workspace relative file/folder patterns](/docs/concepts/file-pattern#project-relative).
They can be defined as a literal path, or a `file://` URI v1.39.0,
or as an object with a `file` property v1.39.0. Additionally, the
following parameters are supported as a URI query or as object fields:

- `content`, `match`, `matches` (`string`) - When determining affected state, will match against the file's content using the defined regex pattern, instead of relying on file existence.

- `optional` (`boolean`) - When hashing and set to `true` and the file is missing, will not log a warning. When set to `false` and the file is missing, will fail with an error. Defaults to logging a warning.

moon.yml

```
tasks:  example:    inputs:      # Literal paths      - 'project/relative/file.js'      - '/workspace/relative/file.js'      # Using file protocol      - 'file://project/relative/file.js?optional'      - 'file:///workspace/relative/file.js?content=a|b|c'      # Using an object      - file: 'project/relative/file.js'        optional: true      - file: '/workspace/relative/file.js'        content: 'a|b|c'
```

#### File groupsv1.41.0

A file group input will reference the defined files/globs within from a file group in the current
project. It can be defined with a `group://` URI, or as an object with a `group` property.
Additionally, the following parameters are supported as a URI query or as object fields:

- `format`, `as` (`string`) - The format in which to gather the file group results. Supported values are `static` (default), `files`, `dirs`, `globs`, `envs`, and `root`.

moon.yml

```
fileGroups:  sources:    - 'src/**/*'tasks:  build:    # ...    inputs:      # Using group protocol      - 'group://sources?format=dirs'      # Using an object      - group: 'sources'        format: 'dirs'
```

#### Glob patterns

Glob patterns support
[project and workspace relative file/folder patterns](/docs/concepts/file-pattern#project-relative).
They can be defined as a literal path, or a `glob://` URI v1.39.0,
or as an object with a `glob` property v1.39.0. Additionally, the
following parameters are supported as a URI query or as object fields:

- `cache` (`boolean`) - When gathering inputs for hashing, defines whether the glob results should be cached for the duration of the moon process. Defaults to `true`.

moon.yml

```
tasks:  example:    inputs:      # Literal paths      - 'project/relative/file.*'      - '/workspace/relative/**/*'      # Using glob protocol      - 'glob://project/relative/file.*?cache=false'      - 'glob:///workspace/relative/**/*?cache'      # Using an object      - glob: 'project/relative/file.*'        cache: false      - glob: '/workspace/relative/**/*'
```

Globs can also be negated by prefixing the path with `!`, which will exclude all files that match
the glob.

moon.yml

```
tasks:  example:    inputs:      - '!**/*.md'      - 'glob://!/workspace/relative/**/*'      - glob: '!/workspace/relative/**/*'
```

warning

Glob patterns that contain `?`, for example `*.tsx?`, cannot be used in URI format, as it conflicts
with the query string syntax. Use the path or object format instead.

danger

Be aware that files that match the glob, but are ignored via `.gitignore` (or similar), will not
be considered an input. To work around this, use explicit file inputs.

#### External projectsv1.41.0

Tasks can also depend on files and globs from other projects within the same workspace. This is
useful for handling cross-project relationships without needing to define explicit task
dependencies.

External projects can be defined as a `project://` URI, or as an object with a `project` property,
both of which require a project identifier, or `^` for all dependent projects. Additionally, the
following parameters are supported as a URI query or as object fields:

- `group`, `fileGroup` (`id`) - The name of a file group within the external project in which file and glob patterns will be used for matching. Takes precedence over `filter`.

- `filter` (`string[]`) - A list of [project relative glob patterns](/docs/concepts/file-pattern#project-relative) that will be used for matching.

If neither `group` nor `filter` are defined, all files within the external project are considered a
match (`**/*`).

moon.yml

```
tasks:  example:    inputs:      # Using project protocol      - 'project://foo'      - 'project://bar?group=sources'      - 'project://baz?filter=src/**/*'      # Using an object      - project: 'foo'      - project: 'bar'        group: 'sources'      - project: 'baz'        filter: ['src/**/*']
```

### `outputs`

The `outputs` field is a list of [files and folders](/docs/concepts/file-pattern#project-relative) that
are created as a result of executing this task, typically from a build or compilation related
task. Outputs are necessary for [incremental caching and hydration](/docs/concepts/cache). If you'd
prefer to avoid that functionality, omit this field.

#### File paths

File paths support
[project and workspace relative file/folder patterns](/docs/concepts/file-pattern#project-relative).
They can be defined as a literal path, or a `file://` URI v1.41.0,
or as an object with a `file` property v1.41.0. Additionally, the
following parameters are supported as a URI query or as object fields:

- `optional` (`boolean`) - When archiving and set to `true` and the file is missing, will not fail with a missing output error. Defaults to `false`.

moon.yml

```
tasks:  example:    inputs:      # Literal paths      - 'build/'      # Using file protocol      - 'file://build/'      # Using an object      - file: 'build/'        optional: true
```

#### Glob patterns

Glob patterns support
[project and workspace relative file/folder patterns](/docs/concepts/file-pattern#project-relative).
They can be defined as a literal path, or a `glob://` URI v1.41.0,
or as an object with a `glob` property v1.41.0. Additionally, the
following parameters are supported as a URI query or as object fields:

- `optional` (`boolean`) - When archiving and set to `true` and the glob produced no results, will not fail with a missing output error. Defaults to `false`.

moon.yml

```
tasks:  example:    inputs:      # Literal paths      - 'build/**/*.js'      - '!build/internal.js'      # Using glob protocol      - 'glob://build/**/*.js'      # Using an object      - glob: 'build/**/*.js'
```

warning

Glob patterns that contain `?`, for example `*.tsx?`, cannot be used in URI format, as it conflicts
with the query string syntax. Use the path or object format instead.

danger

When using globs and moon hydrates an output (a cache hit), all files not matching the glob will be
deleted. Ensure that all files critical for the build to function correctly are included.

### `preset`v1.28.0

Applies the chosen preset to the task. A preset defines a collection of task options that will be
inherited as the default, and can then be overridden within the task itself. The following presets
are available:

- `server` [`cache`](#cache) -> Turned off

- [`outputStyle`](#outputstyle) -> Set to "stream"

- [`persistent`](#persistent) -> Turned on

- [`runInCI`](#runinci) -> Turned off

- `utility` v2.0.0 [`cache`](#cache) -> Turned off

- [`interactive`](#interactive) -> Turned on

- [`outputStyle`](#outputstyle) -> Set to "stream"

- [`persistent`](#persistent) -> Turned off

- [`runInCI`](#runinci) -> Skipped

Tasks named "dev", "start", or "serve" are marked as `server` automatically.

moon.yml

```
tasks:  dev:    command: 'webpack server'    preset: 'server'
```

### `script`v1.27.0

The `script` field is one or many commands to execute for the task, with support for pipes,
redirects, and more. This field does not support task inheritance merging, and can only be defined
with a string.

If defined, will supersede [`command`](#command) and [`args`](#args).

moon.yml

```
tasks:  exec:    # Single command    script: 'cp ./in ./out'    # Multiple commands    script: 'rm -rf ./out && cp ./in ./out'    # Pipes    script: 'ps aux | grep 3000'    # Redirects    script: './gen.sh > out.json'
```

info

If you need to support merging during task inheritance, use [`command`](#command) instead. Learn
more about [commands vs scripts](/docs/concepts/task#commands-vs-scripts).

### `toolchains`v1.31.0

The `toolchain` field defines additional [toolchain(s)](/docs/concepts/toolchain) the command runs on,
where to locate its executable, and more. By default, moon will set to a value based on the
project's [`language`](#language), default [`toolchain.default`](#toolchain-1), or via detection.

moon.yml

```
tasks:  env:    command: 'printenv'    toolchains: 'system'
```

This setting also supports multiple values.

moon.yml

```
tasks:  build:    command: 'npm run build'    toolchains: ['javascript', 'node', 'npm']
```

### `options`

The `options` field is an object of configurable options that can be used to modify the task and its
execution. The following fields can be provided, with merge related fields supporting all
[merge strategies](/docs/concepts/task-inheritance#merge-strategies).

moon.yml

```
tasks:  typecheck:    command: 'tsc --noEmit'    options:      mergeArgs: 'replace'      runFromWorkspaceRoot: true
```

#### `affectedFiles`

When enabled and the [`--affected` option](/docs/run-task#running-based-on-affected-files-only) is
provided, all affected files that match this task's [`inputs`](#inputs) will be passed as relative
file paths as command line arguments, and as a `MOON_AFFECTED_FILES` environment variable.

If there are no affected files, `.` (current directory) will be passed instead for arguments, and an
empty value for the environment variable. This functionality can be changed with the
[`affectedPassInputs`](#affectedpassinputs) setting.

moon.yml

```
tasks:  lint:    command: 'eslint'    options:      affectedFiles: true      # Only pass args      affectedFiles: 'args'      # Only set env var      affectedFiles: 'env'
```

caution

When using this option, ensure that explicit files or `.` are not present in the [`args`](#args)
list. Furthermore, this functionality will only work if the task's command supports an arbitrary
list of files being passed as arguments.

This setting also supports an object format with additional parameters. The `pass` field is
required, which accepts a value described above.

moon.yml

```
tasks:  lint:    command: 'eslint'    options:      affectedFiles:        pass: 'args'
```

The following additional parameters are supported:

- `passInputsWhenNoMatch` (`boolean`) - When no affected files are found, and `pass` includes `args`, will pass all configured [`inputs`](#inputs) as relative file paths instead of `.`. Defaults to `false`.

#### `allowFailure`v1.13.0

Allows a task to fail without failing the entire pipeline. When enabled, the following changes
occur:

- Other tasks cannot depend on this task, as we can't ensure it's side-effect free.

- For [`moon run`](/docs/commands/run), the process will not bail early and will run to completion.

- For [`moon ci`](/docs/commands/ci), the process will not exit with a non-zero exit code, if the only failing tasks are allowed to fail.

moon.yml

```
tasks:  lint:    command: 'eslint'    options:      allowFailure: true
```

#### `cache`

Whether to cache the task's execution result using our [smart hashing](/docs/concepts/cache#hashing)
system. If disabled, will not create a cache hash, and will not persist a task's
[outputs](#outputs). Supports the following values:

- `true` (default) - Cache the task's output.

- `false` - Do not cache the task's output.

- `local` - Only cache locally. v1.40.0

- `remote` - Only cache [remotely](/docs/guides/remote-cache). v1.40.0

We suggest disabling caching when defining cleanup tasks, one-off scripts, or file system heavy
operations.

moon.yml

```
tasks:  clean:    command: 'rm -rf ./temp'    options:      cache: false
```

#### `cacheKey`v1.35.0

A custom key to include in the cache and task hashing process. Can be used to invalidate local and
remote caches.

moon.yml

```
tasks:  build:    command: 'some-costly-build'    options:      cacheKey: 'v1'
```

#### `cacheLifetime`v1.29.0

The lifetime in which a [cached task](#cache) will live before being marked as stale and re-running.
This applies to a task even if it does not produce [outputs](#outputs).

The lifetime can be configured in a human-readable string format, for example, `1 day`, `3 hr`,
`1m`, etc. If the lifetime is not defined, the cache will live forever, or until the task inputs are
touched.

moon.yml

```
tasks:  build:    command: 'some-costly-build'    options:      cacheLifetime: '1 day'
```

String formats are powered by the
[humantime](https://docs.rs/humantime/2.1.0/humantime/fn.parse_duration.html) crate.

#### `envFile`

A boolean or path to a `.env` file (also know as dotenv file) that defines a collection of
[environment variables](#env-1) for the current task. Variables will be loaded on project creation,
but will not override those defined in [`env`](#env-1).

Variables defined in the file support value substitution/expansion by wrapping the variable name in
curly brackets, such as `${VAR_NAME}`.

moon.yml

```
tasks:  build:    command: 'webpack'    options:      # Defaults to .env      envFile: true      # Or      envFile: '.env.production'      # Or from the workspace root      envFile: '/.env.shared'
```

When set to `true`, moon will load the following files in order, with later files taking precedence
over earlier ones:

- `/.env`

- `/.env.local`

- `.env`

- `.env.local`

- `.env.`

- `.env..local`

Additionally, a list of file paths can also be provided. When using a list, the order of the files
is important, as environment variables from all files will be aggregated into a single map, with
subsequent files taking precedence over previous ones. Once aggregated, the variables will be passed
to the task, but will not override those defined in [`env`](#env-1).

moon.yml

```
tasks:  build:    command: 'webpack'    options:      envFile:        - '.env'        - '.env.production'
```

#### `inferInputs`v1.31.0

Automatically infer [inputs](#inputs) based on the following parameters configured within the task's
`command`, `script`, `args`, or `env`. Defaults to `false`.

- File/glob paths derived from [file group based token functions](/docs/concepts/token#file-groups).

- Environment variables being substituted within a command or script.

moon.yml

```
tasks:  build:    # ...    options:      inferInputs: false
```

#### `internal`v1.23.0

Marks the task as internal only. [Internal tasks](/docs/concepts/task#internal-only) can not be
explicitly ran on the command line, but can be depended on by other tasks.

moon.yml

```
tasks:  prepare:    # ...    options:      internal: true
```

#### `interactive`v1.12.0

Marks the task as interactive. [Interactive tasks](/docs/concepts/task#interactive) run in isolation so
that they can interact with stdin.

This setting also disables caching, turns of CI, and other functionality, similar to the
[`preset`](#preset) setting.

moon.yml

```
tasks:  init:    # ...    options:      interactive: true
```

#### `merge`v1.29.0

The [strategy](/docs/concepts/task-inheritance#merge-strategies) to use when merging [`args`](#args),
[`deps`](#deps), [`env`](#env-1), [`inputs`](#inputs), and [`outputs`](#outputs) with an inherited
task. This option can be overridden with the field specific merge options below.

#### `mergeArgs`

The [strategy](/docs/concepts/task-inheritance#merge-strategies) to use when merging the
[`args`](#args) list with an inherited task. Defaults to "append".

#### `mergeDeps`

The [strategy](/docs/concepts/task-inheritance#merge-strategies) to use when merging the
[`deps`](#deps) list with an inherited task. Defaults to "append".

#### `mergeEnv`

The [strategy](/docs/concepts/task-inheritance#merge-strategies) to use when merging the
[`env`](#env-1) map with an inherited task. Defaults to "append".

#### `mergeInputs`

The [strategy](/docs/concepts/task-inheritance#merge-strategies) to use when merging the
[`inputs`](#inputs) list with an inherited task. Defaults to "append".

#### `mergeOutputs`

The [strategy](/docs/concepts/task-inheritance#merge-strategies) to use when merging the
[`outputs`](#outputs) list with an inherited task. Defaults to "append".

#### `mergeToolchains`

The [strategy](/docs/concepts/task-inheritance#merge-strategies) to use when merging the
[`toolchains`](#toolchains) list with an inherited task. Defaults to "append".

#### `mutex`v1.24.0

Creates an exclusive lock on a "virtual resource", preventing other tasks using the same "virtual
resource" from running concurrently.

If you have many tasks that require exclusive access to a resource that can't be tracked by moon
(like a database, an ignored file, a file that's not part of the project, or a remote resource) you
can use the `mutex` option to prevent them from running at the same time.

moon.yml

```
tasks:  a:    # ...    options:      mutex: 'virtual_resource_name'  # b doesn't necessarily have to be in the same project  b:    # ...    options:      mutex: 'virtual_resource_name'
```

#### `os`v1.28.0

When defined, the task will only run on the configured operating system. For other operating
systems, the task becomes a no-operation. Supports the values `linux`, `macos`, and `windows`.

Can be defined as a single value, or a list of values.

moon.yml

```
tasks:  build-unix:    # ...    options:      os: ['linux', 'macos']  build-windows:    # ...    options:      os: 'windows'
```

#### `outputStyle`

Controls how stdout/stderr is displayed when the task is ran as a transitive target. By default,
this setting is not defined and defers to the action pipeline, but can be overridden with one of the
following values:

- `buffer` - Buffers output and displays after the task has exited (either success or failure).

- `buffer-only-failure` - Like `buffer`, but only displays on failures.

- `hash` - Ignores output and only displays the generated [hash](/docs/concepts/cache#hashing).

- `none` - Ignores output.

- `stream` - Streams output directly to the terminal. Will prefix each line of output with the target.

moon.yml

```
tasks:  test:    # ...    options:      outputStyle: 'stream'
```

#### `persistent`v1.6.0

Marks the task as persistent (continuously running). [Persistent tasks](/docs/concepts/task#persistent)
are handled differently than non-persistent tasks in the action graph. When running a target, all
persistent tasks are ran last and in parallel, after all their dependencies have completed.

This is extremely useful for running a server (or a watcher) in the background while other tasks are
running.

moon.yml

```
tasks:  dev:    # ...    options:      persistent: true
```

We suggest using the [`preset`](#preset) setting instead, which enables this setting, amongst
other useful settings.

#### `priority`v1.35.0

The priority level determines the position of the task within the action pipeline queue. A task with
a higher priority will run sooner rather than later, while still respecting the topological order.
Supports the following levels:

- `critical`

- `high`

- `normal` (default)

- `low`

moon.yml

```
tasks:  build:    # ...    options:      priority: 'high'
```

#### `retryCount`

The number of attempts the task will retry execution before returning a failure. This is especially
useful for flaky tasks. Defaults to `0`.

moon.yml

```
tasks:  test:    # ...    options:      retryCount: 3
```

#### `runDepsInParallel`

Whether to run the task's direct [`deps`](#deps) in parallel or serial (in order). Defaults to
`true`.

When disabled, this does not run dependencies of dependencies in serial, only direct dependencies.

moon.yml

```
tasks:  start:    # ...    deps:      - '~:clean'      - '~:build'    options:      runDepsInParallel: false
```

#### `runInCI`

Whether to run the task automatically in a CI (continuous integration) environment when affected by
touched files using the [`moon ci`](/docs/commands/ci) command. Supports the following values:

- `always` - Always run in CI, regardless if affected or not. v1.31.0

- `affected`, `true` (default) - Only run in CI if affected by touched files.

- `false` - Never run in CI.

- `only` - Only run in CI, and not locally, if affected by touched files. v1.41.0

- `skip` - Skip running in CI but run locally and allow task relationships to be valid. v1.41.0

moon.yml

```
tasks:  build:    # ...    options:      runInCI: false
```

#### `runFromWorkspaceRoot`

Whether to use the workspace root as the working directory when executing a task. Defaults to
`false` and runs from the task's project root.

moon.yml

```
tasks:  typecheck:    # ...    options:      runFromWorkspaceRoot: true
```

#### `shell`

Whether to run the command within a shell or not. Defaults to `true` for system toolchain or
Windows, and `false` otherwise. The shell to run is determined by the [`unixShell`](#unixshell) and
[`windowsShell`](#windowsshell) options respectively.

moon.yml

```
tasks:  native:    command: 'echo $SHELL'    options:      shell: true
```

However, if you'd like to use a different shell, or customize the shell's arguments, or have
granular control, you can set `shell` to false and configure a fully qualified command.

moon.yml

```
tasks:  native:    command: '/bin/zsh -c "echo $SHELL"'    options:      shell: false
```

#### `timeout`v1.26.0

The maximum time in seconds that the task is allowed to run, before it is force cancelled. If not
defined, will run indefinitely.

moon.yml

```
tasks:  build:    # ...    options:      timeout: 120
```

#### `unixShell`v1.21.0

Customize the shell to run with when on a Unix operating system. Accepts `bash`, `elvish`, `fish`,
`ion`, `murex`, `nu`, `pwsh`, `xonsh`, or `zsh`. If not defined, will derive the shell from the
`SHELL` environment variable, or defaults to `bash`.

moon.yml

```
tasks:  native:    command: 'echo $SHELL'    options:      unixShell: 'fish'
```

#### `windowsShell`v1.21.0

Customize the shell to run with when on a Windows operating system. Accepts `bash` (typically via
Git), `elvish`, `fish`, `murex`, `nu`, `pwsh`, or `xonsh`. If not defined, defaults to `pwsh`.

moon.yml

```
tasks:  native:    command: 'echo $SHELL'    options:      windowsShell: 'bash'
```

## Overrides

Dictates how a project interacts with settings defined at the top-level.

## `toolchains`

### `default`v1.31.0

The default [`toolchain`](#toolchain-1) for all task's within the current project. When a task's
`toolchain` has not been explicitly configured, the toolchain will fallback to this configured
value, otherwise the toolchain will be detected from the project's environment.

moon.yml

```
toolchains:  default: 'node'
```

### `bun`

Configures Bun for this project and overrides the top-level [`bun`](/docs/config/toolchain#bun) setting.

#### `version`

Defines the explicit Bun [version specification](/docs/concepts/toolchain#version-specification) to use
when running tasks for this project.

moon.yml

```
toolchains:  bun:    version: '1.0.0'
```

### `deno`

Configures Deno for this project and overrides the top-level [`deno`](/docs/config/toolchain#deno) setting.

#### `version`

Defines the explicit Deno [version specification](/docs/concepts/toolchain#version-specification) to
use when running tasks for this project.

moon.yml

```
toolchains:  deno:    version: '1.40.0'
```

### `node`

Configures Node.js for this project and overrides the top-level [`node`](/docs/config/toolchain#node) setting.
Currently, only the Node.js version can be overridden per-project, not the package manager.

#### `version`

Defines the explicit Node.js [version specification](/docs/concepts/toolchain#version-specification) to
use when running tasks for this project.

moon.yml

```
toolchains:  node:    version: '12.12.0'
```

### `python`

Configures Python for this project and overrides the top-level [`python`](/docs/config/toolchain#python)
setting.

#### `version`

Defines the explicit Python
[version/channel specification](/docs/concepts/toolchain#version-specification) to use when running
tasks for this project.

moon.yml

```
toolchains:  python:    version: '3.12.0'
```

### `rust`

Configures Rust for this project and overrides the top-level [`rust`](/docs/config/toolchain#rust) setting.

#### `version`

Defines the explicit Rust
[version/channel specification](/docs/concepts/toolchain#version-specification) to use when running
tasks for this project.

moon.yml

```
toolchains:  rust:    version: '1.68.0'
```

### `typescript`

#### `includeProjectReferenceSources`v1.17.0

Overrides the workspace-level
[`includeProjectReferenceSources`](/docs/config/toolchain#includeprojectreferencesources) setting. Defaults to
undefined.

moon.yml

```
toolchains:  typescript:    includeProjectReferenceSources: false
```

#### `includeSharedTypes`v1.17.0

Overrides the workspace-level [`includeSharedTypes`](/docs/config/toolchain#includesharedtypes) setting.
Defaults to undefined.

moon.yml

```
toolchains:  typescript:    includeSharedTypes: false
```

#### `routeOutDirToCache`

Overrides the workspace-level [`routeOutDirToCache`](/docs/config/toolchain#routeoutdirtocache) setting.
Defaults to undefined.

moon.yml

```
toolchains:  typescript:    routeOutDirToCache: false
```

#### `syncProjectReferences`

Overrides the workspace-level [`syncProjectReferences`](/docs/config/toolchain#syncprojectreferences) setting.
Defaults to undefined.

moon.yml

```
toolchains:  typescript:    syncProjectReferences: false
```

#### `syncProjectReferencesToPaths`

Overrides the workspace-level
[`syncProjectReferencesToPaths`](/docs/config/toolchain#syncprojectreferencestopaths) setting. Defaults to
undefined.

moon.yml

```
toolchains:  typescript:    syncProjectReferencesToPaths: false
```

## `workspace`

### `inheritedTasks`

Provides a layer of control when inheriting tasks from [`.moon/tasks/all.yml`](/docs/config/tasks).

#### `exclude`

The optional `exclude` setting permits a project to exclude specific tasks from being inherited. It
accepts a list of strings, where each string is the name of a global task to exclude.

moon.yml

```
workspace:  inheritedTasks:    # Exclude the inherited `test` task for this project    exclude: ['test']
```

Exclusion is applied after inclusion and before renaming.

#### `include`

The optional `include` setting permits a project to only include specific inherited tasks (works
like an allow/white list). It accepts a list of strings, where each string is the name of a global
task to include.

When this field is not defined, the project will inherit all tasks from the global project config.

moon.yml

```
workspace:  inheritedTasks:    # Include *no* tasks (works like a full exclude)    include: []    # Only include the `lint` and `test` tasks for this project    include:      - 'lint'      - 'test'
```

Inclusion is applied before exclusion and renaming.

#### `rename`

The optional `rename` setting permits a project to rename the inherited task within the current
project. It accepts a map of strings, where the key is the original name (found in the global
project config), and the value is the new name to use.

For example, say we have 2 tasks in the global project config called `buildPackage` and
`buildApplication`, but we only need 1, and since we're an application, we should omit and rename.

moon.yml

```
workspace:  inheritedTasks:    exclude: ['buildPackage']    rename:      buildApplication: 'build'
```

Renaming occurs after inclusion and exclusion.

## /docs/config/tasks

Source: https://moonrepo.dev/docs/config/tasks

# .moon/tasks

The `.moon/tasks/**/*` files configures file groups and tasks that are inherited by every project
in the workspace based on inheritance conditions.
[Learn more about task inheritance!](/docs/concepts/task-inheritance)

Projects can override or merge with these settings within their respective [`moon.yml`](/docs/config/project).

## `extends`

Defines one or many external `.moon/tasks/all.yml`'s to extend and inherit settings from. Perfect
for reusability and sharing configuration across repositories and projects. When defined, this
setting must be an HTTPS URL or relative file system path that points to a valid YAML document!

.moon/tasks/all.yml

```
extends: 'https://raw.githubusercontent.com/organization/repository/master/.moon/tasks/all.yml'
```

caution

For map-based settings, `fileGroups` and `tasks`, entries from both the extended configuration and
local configuration are merged into a new map, with the values of the local taking precedence. Map
values are not deep merged!

## `fileGroups`

For more information on file group configuration, refer to the
[`fileGroups`](/docs/config/project#filegroups) section in the [`moon.yml`](/docs/config/project) doc.

Defines [file groups](/docs/concepts/file-group) that will be inherited by projects, and also enables
enforcement of organizational patterns and file locations. For example, encourage projects to place
source files in a `src` folder, and all test files in `tests`.

.moon/tasks/all.yml

```
fileGroups:  configs:    - '*.config.{js,cjs,mjs}'    - '*.json'  sources:    - 'src/**/*'    - 'types/**/*'  tests:    - 'tests/**/*'    - '**/__tests__/**/*'  assets:    - 'assets/**/*'    - 'images/**/*'    - 'static/**/*'    - '**/*.{scss,css}'
```

info

File paths and globs used within a file group are relative from the inherited project's root, and
not the workspace root.

## `implicitDeps`

Defines task [`deps`](/docs/config/project#deps) that are implicitly inserted into all inherited tasks within
a project. This is extremely useful for pre-building projects that are used extensively throughout
the repo, or always building project dependencies. Defaults to an empty list.

.moon/tasks/all.yml

```
implicitDeps:  - '^:build'
```

info

Implicit dependencies are always inherited, regardless of the [`mergeDeps`](/docs/config/project#mergedeps)
option.

## `implicitInputs`

Defines task [`inputs`](/docs/config/project#inputs) that are implicitly inserted into all inherited tasks
within a project. This is extremely useful for the "changes to these files should always trigger a
task" scenario.

Like `inputs`, file paths/globs defined here are relative from the inheriting project.
[Project and workspace relative file patterns](/docs/concepts/file-pattern#project-relative) are
supported and encouraged.

.moon/tasks/node.yml

```
implicitInputs:  - 'package.json'
```

info

Implicit inputs are always inherited, regardless of the [`mergeInputs`](/docs/config/project#mergeinputs)
option.

## `inheritedBy`v2.0.0

A map of conditions that must be met for the configuration within the file to be inherited by a
project. When this field is not defined, or is an empty map, the configuration will be inherited by
all projects.

.moon/tasks/custom.yml

```
inheritedBy:  # Project belongs to either javascript or typescript toolchain, but not the ruby toolchain  toolchains:    or: ['javascript', 'typescript']    not: ['ruby']  # And project is either a frontend or backend stack  stacks: ['frontend', 'backend']  # And project is either a library or tool layer  layers: ['library', 'tool']
```

info

View the [official task inheritance guide](/docs/concepts/task-inheritance) for more information!

## `tasks`

For more information on task configuration, refer to the [`tasks`](/docs/config/project#tasks) section in the
[`moon.yml`](/docs/config/project) doc.

As mentioned in the link above, [tasks](/docs/concepts/task) are actions that are ran within the
context of a project, and commonly wrap a command. For most workspaces, every project should have
linting, typechecking, testing, code formatting, so on and so forth. To reduce the amount of
boilerplate that every project would require, this setting offers the ability to define tasks that
are inherited by many projects within the workspace, but can also be overridden per project.

.moon/tasks/all.yml

```
tasks:  format:    command: 'prettier --check .'  lint:    command: 'eslint --no-error-on-unmatched-pattern .'  test:    command: 'jest --passWithNoTests'  typecheck:    command: 'tsc --build'
```

info

Relative file paths and globs used within a task are relative from the inherited project's root, and
not the workspace root.

## `taskOptions`v1.20.0

For more information on task options, refer to the [`options`](/docs/config/project#options) section in the
[`moon.yml`](/docs/config/project) doc.

Like [tasks](#tasks), this setting allows you to define task options that will be inherited by all
tasks within the configured file, and by all project-level inherited tasks. This setting is the 1st
link in the inheritance chain, and can be overridden within each task.

.moon/tasks/all.yml

```
taskOptions:  # Never cache builds  cache: false  # Always re-run flaky tests  retryCount: 2tasks:  build:    # ...    options:      # Override the default cache setting      cache: true
```

## /docs/config/template

Source: https://moonrepo.dev/docs/config/template

# template

The `template.yml` file configures metadata and variables for a template,
[used by the generator](/docs/guides/codegen), and must exist at the root of a named template folder.

## `id`v1.23.0

Overrides the name (identifier) of the template, instead of inferring the name from the template
folder. Be aware that template names must be unique across the workspace, and across all template
locations that have been configured in [`generator.templates`](/docs/config/workspace#templates).

template.yml

```
id: 'npm-package'
```

## `title`Required

A human readable title that will be displayed during the [`moon generate`](/docs/commands/generate)
process.

template.yml

```
title: 'npm package'
```

## `description`Required

A description of why the template exists, what its purpose is, and any other relevant information.

template.yml

```
description: |  Scaffolds the initial structure for an npm package,  including source and test folders, a package.json, and more.
```

## `destination`v1.19.0

An optional file path in which this template should be generated into. This provides a mechanism for
standardizing a destination location, and avoids having to manually pass a destination to
[`moon generate`](/docs/commands/generate).

If the destination is prefixed with `/`, it will be relative from the workspace root, otherwise it
is relative from the current working directory.

template.yml

```
destination: 'packages/[name]'
```

This setting supports [template variables](#variables) through `[varName]` syntax. Learn more in
the [code generation documentation](/docs/guides/codegen#interpolation).

## `extends`v1.19.0

One or many other templates that this template should extend. Will deeply inherit all template files
and variables.

template.yml

```
extends: ['base', 'configs']
```

## `variables`

A mapping of variables that will be interpolated into all template files and file system paths when
[rendering with Tera](https://tera.netlify.app/docs/#variables). The map key is the variable name
(in camelCase or snake_case), while the value is a configuration object, as described with the
settings below.

template.yml

```
variables:  name:    type: 'string'    default: ''    required: true    prompt: 'Package name?'
```

### `type`Required

The type of value for the variable. Accepts `array`, `boolean`, `string`, `object`, `number`, or
`enum`. Floats are not supported, use strings instead.

For arrays and objects, the value of each member must be a JSON compatible type.

### `internal`v1.23.0

Marks a variable as internal only, which avoids the variable value being overwritten by command line
arguments.

### `order`v1.23.0

The order in which the variable will be prompted to the user. By default, variables are prompted in
the order they are defined in the `template.yml` file.

### Primitives & collections

Your basic primitives: boolean, numbers, strings, and collections: arrays, objects.

- array
- boolean
- number
- object
- string

template.yml

```
variables:  type:    type: 'array'    prompt: 'Type?'    default: ['app', 'lib']
```

template.yml

```
variables:  private:    type: 'boolean'    prompt: 'Private?'    default: false
```

template.yml

```
variables:  age:    type: 'number'    prompt: 'Age?'    default: 0    required: true
```

template.yml

```
variables:  metadata:    type: 'object'    prompt: 'Metadata?'    default:      type: 'lib'      dev: true
```

template.yml

```
variables:  name:    type: 'string'    prompt: 'Name?'    required: true
```

### `default`Required

The default value of the variable. When `--defaults` is passed to
[`moon generate`](/docs/commands/generate) or [`prompt`](#prompt) is not defined, the default value
will be used, otherwise the user will be prompted to enter a custom value.

### `prompt`

When defined, will prompt the user with a message in the terminal to input a custom value, otherwise
[`default`](#default) will be used.

For arrays and objects, a valid JSON string must be provided as the value.

### `required`

Marks the variable as required during prompting only. For arrays, strings, and objects, will error
for empty values (`''`). For numbers, will error for zero's (`0`).

### Enums

An enum is an explicit list of string values that a user can choose from.

template.yml

```
variables:  color:    type: 'enum'    values: ['red', 'green', 'blue', 'purple']    default: 'purple'    prompt: 'Favorite color?'
```

### `default`

The default value of the variable. When `--defaults` is passed to
[`moon generate`](/docs/commands/generate) or [`prompt`](#prompt) is not defined, the default value
will be used, otherwise the user will be prompted to enter a custom value.

For enums, the default value can be a string when [`multiple`](#multiple) is false, or a string or
an array of strings when `multiple` is true. Furthermore, each default value must exist in the
[`values`](#values) list.

template.yml

```
# Singlevariables:  color:    type: 'enum'    values: ['red', 'green', 'blue', 'purple']    default: 'purple'    prompt: 'Favorite color?'# Multiplevariables:  color:    type: 'enum'    values: ['red', 'green', 'blue', 'purple']    default: ['red', 'purple']    multiple: true    prompt: 'Favorite color?'
```

### `prompt`

When defined, will prompt the user with a message in the terminal to input a custom value, otherwise
[`default`](#default) will be used.

### `multiple`

Allows multiple values to be chosen during prompting. In the template, an array or strings will be
rendered, otherwise when not-multiple, a single string will be.

### `values`Required

List of explicit values to choose from. Can either be defined with a string, which acts as a value
and label, or as an object, which defines an explicit value and label.

template.yml

```
variables:  color:    type: 'enum'    values:      - 'red'      # OR      - value: 'red'        label: 'Red 🔴'    # ...
```

## Frontmatter

The following settings are not available in `template.yml`, but can be defined as frontmatter at
the top of a template file. View the [code generation guide](/docs/guides/codegen#frontmatter) for more
information.

### `force`

When enabled, will always overwrite a file of the same name at the destination path, and will bypass
any prompting in the terminal.

```
---force: true---Some template content!
```

### `to`

Defines a custom file path, relative from the destination root, in which to create the file. This
will override the file path within the template folder, and allow for conditional rendering and
engine filters to be used.

```
{% set component_name = name | pascal_case %}---to: components/{{ component_name }}.tsx---export function {{ component_name }}() {  return
;}
```

### `skip`

When enabled, the template file will be skipped while writing to the destination path. This setting
can be used to conditionally render a file.

```
---skip: {{ name == "someCondition" }}---Some template content!
```

## /docs/config/toolchain

Source: https://moonrepo.dev/docs/config/toolchain

# .moon/toolchains

The `.moon/toolchains.yml` file configures the toolchain and the workspace development environment.
This file is optional.

Managing tool version's within the toolchain ensures a deterministic environment across any machine
(whether a developer, CI, or production machine).

## `extends`

Defines one or many external `.moon/toolchains.yml`'s to extend and inherit settings from. Perfect
for reusability and sharing configuration across repositories and projects. When defined, this
setting must be an HTTPS URL or relative file system path that points to a valid YAML document!

.moon/toolchains.yml

```
extends: 'https://raw.githubusercontent.com/organization/repository/master/.moon/toolchains.yml'
```

caution

Settings will be merged recursively for blocks, with values defined in the local configuration
taking precedence over those defined in the extended configuration.

## `moon`v1.29.0

Configures how moon will receive information about latest releases and download locations.

### `manifestUrl`

Defines an HTTPS URL in which to fetch the current version information from.

.moon/toolchains.yml

```
moon:  manifestUrl: 'https://proxy.corp.net/moon/version'
```

### `downloadUrl`

Defines an HTTPS URL in which the moon binary can be downloaded from. The download file name is
hard-coded and will be appended to the provided URL.

Defaults to downloading from GitHub: [https://github.com/moonrepo/moon/releases](https://github.com/moonrepo/moon/releases)

.moon/toolchains.yml

```
moon:  downloadUrl: 'https://github.com/moonrepo/moon/releases/latest/download'
```

## `proto`v1.39.0

Configures how moon integrates with and utilizes [proto](/proto).

### `version`

The version of proto to install and run toolchains with. If proto or this version of proto has not
been installed yet, it will be installed automatically when running a task.

.moon/toolchains.yml

```
proto:  version: '0.51.0'
```

## Go

## `go`v1.38.0

Run `moon toolchain info go` for all available settings.

## JavaScript

## `javascript`v1.40.0

Run `moon toolchain info javascript` for all available settings.

## `bun`v1.40.0

Run `moon toolchain info bun` for all available settings.

info

This toolchain requires the [`javascript`](#javascript) toolchain to also be enabled.

## `deno`v1.41.0

Run `moon toolchain info deno` for all available settings.

info

This toolchain requires the [`javascript`](#javascript) toolchain to also be enabled.

## `node`v1.40.0

Run `moon toolchain info node` for all available settings.

info

This toolchain requires the [`javascript`](#javascript) toolchain to also be enabled.

## `npm`v1.40.0

Run `moon toolchain info npm` for all available settings.

info

This toolchain requires the [`node`](#node) toolchain to also be enabled.

## `pnpm`v1.40.0

Run `moon toolchain info pnpm` for all available settings.

info

This toolchain requires the [`node`](#node) toolchain to also be enabled.

## `yarn`v1.40.0

Run `moon toolchain info yarn` for all available settings.

info

This toolchain requires the [`node`](#node) toolchain to also be enabled.

## `typescript`

Run `moon toolchain info typescript` for all available settings.

## Pythonv1.30.0

caution

Python support is currently a work in progress for v2!

## `python`

Run `moon toolchain info python` for all available settings.

## Rust

## `rust`v1.37.0

Run `moon toolchain info rust` for all available settings.

## /docs/config/workspace

Source: https://moonrepo.dev/docs/config/workspace

# .moon/workspace

The `.moon/workspace.yml` file configures projects and services in the workspace. This file is
required.

## `extends`

Defines one or many external `.moon/workspace.yml`'s to extend and inherit settings from. Perfect
for reusability and sharing configuration across repositories and projects. When defined, this
setting must be an HTTPS URL or relative file system path that points to a valid YAML document!

.moon/workspace.yml

```
extends: 'https://raw.githubusercontent.com/organization/repository/master/.moon/workspace.yml'
```

info

Settings will be merged recursively for blocks, with values defined in the local configuration
taking precedence over those defined in the extended configuration. However, the `projects` setting
does not merge!

## `projects`Required

Defines the location of all [projects](/docs/concepts/project) within the workspace. Supports either a
manual map of projects (default), a list of globs in which to automatically locate projects, or
both.

caution

Projects that depend on each other and form a cycle must be avoided! While we do our best to avoid
an infinite loop and disconnect nodes from each other, there's no guarantee that tasks will run in
the correct order.

### Using a map

When using a map, each project must be manually configured and requires a unique
[name](/docs/concepts/project#names) as the map key, where this name is used heavily on the command
line and within the project graph for uniquely identifying the project amongst all projects. The map
value (known as the project source) is a file system path to the project folder, relative from the
workspace root, and must be contained within the workspace boundary.

.moon/workspace.yml

```
projects:  admin: 'apps/admin'  apiClients: 'packages/api-clients'  designSystem: 'packages/design-system'  web: 'apps/web'
```

### Using globs

If manually mapping projects is too tedious or cumbersome, you may provide a list of
[globs](/docs/concepts/file-pattern#globs) to automatically locate all project folders, relative from
the workspace root.

When using this approach, the project name is derived from the project folder name, and is cleaned
to our [supported characters](/docs/concepts/project#names), but can be customized with the
[`id`](/docs/config/project#id) setting in [`moon.yml`](/docs/config/project). Furthermore, globbing does risk the
chance of collision, and when that happens, we log a warning and skip the conflicting project from
being configured in the project graph.

.moon/workspace.yml

```
projects:  - 'apps/*'  - 'packages/*'  # Only shared folders with a moon configuration  - 'shared/*/moon.yml'
```

### Using a map and globs

For those situations where you want to use both patterns, you can! The list of globs can be
defined under a `globs` field, while the map of projects under a `sources` field.

.moon/workspace.yml

```
projects:  globs:    - 'apps/*'    - 'packages/*'  sources:    www: 'www'
```

Additionally, you can customize the format of project IDs for glob discovered projects. By default
it inherits the fodler name, but this has a high chance of collision. Instead you can configure
`globFormat` to use a different format, for example, using the full workspace relative path as the
project ID.

.moon/workspace.yml

```
projects:  globFormat: 'source-path'  globs:    - 'packages/**/moon.yml'
```

## `defaultProject`v2.0.0

Defines the default project to focus on when no project scope is specified on the command line for
task targets.

.moon/workspace.yml

```
defaultProject: 'web'
```

## `codeowners`v1.8.0

Configures code owners (`CODEOWNERS`) integration across the entire workspace.

### `globalPaths`

This setting defines file patterns and their owners at the workspace-level, and are applied to any
matching path, at any depth, within the entire workspace. This is useful for defining global or
fallback owners when a granular [project-level path](/docs/config/project#paths) does not match or exist.

.moon/workspace.yml

```
codeowners:  globalPaths:    '*': ['@admins']    'config/': ['@infra']    '/.github/': ['@infra']
```

### `orderBy`

The order in which code owners, grouped by project, are listed in the `CODEOWNERS` file. Accepts
"file-source" (default) or "project-id".

.moon/workspace.yml

```
codeowners:  orderBy: 'project-id'
```

### `sync`

Will automatically generate a `CODEOWNERS` file by aggregating and syncing all project
[`owners`](/docs/config/project#owners) in the workspace when a [target is run](/docs/concepts/target). The format
and location of the `CODEOWNERS` file is based on the [`vcs.provider`](#provider) setting. Defaults
to `false`.

.moon/workspace.yml

```
codeowners:  sync: true
```

## `constraints`

Configures constraints between projects that are enforced during project graph generation. This is
also known as project boundaries.

### `enforceLayerRelationships`

Enforces allowed relationships between a project and its dependencies based on the project's
[`layer`](/docs/config/project#layer) and [`stack`](/docs/config/project#stack) settings. When a project depends on
another project of an invalid layer, a layering violation error will be thrown when attempting to
run a task.

Layers are allowed to depend on lower layers in the same stack, but not higher layers. Additionally,
layers may depend on itself, excluding automations and applications. The following layers are
stacked as such:

Layer Description

`automation` An automated testing suite, like E2E, integration, or visual tests.

`application` An application of any kind.

`tool` An internal tool, CLI, one-off script, etc.

`library` A self-contained, shareable, and publishable set of code.

`scaffolding` Templates or generators for scaffolding.

`configuration` Configuration files or infrastructure.

`unknown` When not configured.

When the project `stack` setting is defined, it alters these rules to allow these kinds of
relationships. For example, a frontend application can depend on a backend application, but not
another frontend application.

.moon/workspace.yml

```
constraints:  enforceLayerRelationships: false
```

Projects with an unconfigured or unknown layer are ignored during enforcement.

### `tagRelationships`

Enforces allowed relationships between a project and its dependencies based on the project's
[`tags`](/docs/config/project#tags) setting. This works in a similar fashion to `enforceLayerRelationships`,
but gives you far more control over what these relationships look like.

For example, let's enforce that Next.js projects using the `next` tag can only depend on React
projects using the `react` tag. If a dependency does not have one of the configured required tags,
in this case `react`, an error will occur.

.moon/workspace.yml

```
constraints:  tagRelationships:    next: ['react']
```

On the project side, we would configure [`moon.yml`](/docs/config/project#tags) like so:

app/moon.yml

```
tags: ['next']dependsOn: ['components']
```

packages/components/moon.yml

```
tags: ['react']
```

## `docker`v1.27.0

Configures Docker integration for the entire workspace.

### `prune`

Configures aspects of the Docker pruning process when
[`moon docker prune`](/docs/commands/docker/prune) is executed.

#### `deleteVendorDirectories`

Automatically delete vendor directories (package manager dependencies, build targets, etc) while
pruning. For example, `node_modules` for JavaScript, or `target` for Rust. Defaults to `true`.

.moon/workspace.yml

```
docker:  prune:    deleteVendorDirectories: false
```

This process happens before toolchain dependencies are installed.

#### `installToolchainDependencies`

Automatically install production dependencies for all required toolchain's of the focused projects
within the Docker build. For example, `node_modules` for JavaScript. Defaults to `true`.

.moon/workspace.yml

```
docker:  prune:    installToolchainDependencies: false
```

This process happens after vendor directories are deleted.

### `scaffold`

Configures aspects of the Docker scaffolding process when
[`moon docker scaffold`](/docs/commands/docker/scaffold) is executed. Only applies to the
[workspace skeleton](/docs/commands/docker/scaffold#workspace).

#### `configsPhaseGlobs`

List of globs in which to copy additional workspace-relative files into the `.moon/docker/workspace`
skeleton. When not defined, does nothing.

.moon/workspace.yml

```
docker:  scaffold:    configsPhaseGlobs:      - '**/package.json'
```

## `experiments`v1.11.0

Enable or disable experiments that alter core functionality.

warning

Experiments are a work in progress and may be buggy. Please report any issues you encounter!

### `fasterGlobWalk`v1.34.0

Utilizes a new concurrent glob walking implementation that is on average, 1.5-2x faster than the
current implementation. Additionally, common globs are now cached for the duration of the process.
Defaults to `true`.

.moon/workspace.yml

```
experiments:  fasterGlobWalk: true
```

### `gitV2`v1.34.0

Utilizes a Git implementation, that has better support for submodules, subtrees, and workspaces.
Additionally, processes are parallized when applicable. Defaults to `true`.

.moon/workspace.yml

```
experiments:  gitV2: true
```

## `generator`

Configures aspects of the template generator.

### `templates`

A list of paths in which templates can be located. Supports the following types of paths, and
defaults to `./templates`.

- File system paths, relative from the workspace root.

- Git repositories and a revision, prefixed with `git://`. v1.23.0

- npm packages and a version, prefixed with `npm://`. v1.23.0

.moon/workspace.yml

```
generator:  templates:    - './templates'    - 'file://./other/templates'    - 'git://github.com/moonrepo/templates#master'    - 'npm://@moonrepo/templates#1.2.3'
```

Learn more about this in the official
[code generation guide](/docs/guides/codegen#configuring-template-locations)!

## `hasher`

Configures aspects of the smart hashing layer.

### `ignoreMissingPatterns`v1.10.0

When [`hasher.warnOnMissingInputs`](#warnonmissinginputs) is enabled, moon will log a warning to the
terminal that an input is missing. This is useful for uncovering misconfigurations, but can be quite
noisy when inputs are truly optional.

To ignore warnings for missing inputs, a list of [glob patterns](/docs/concepts/file-pattern#globs) can
be configured to filter and ignore files. Files are matched against workspace relative paths, so
prefixing patterns with `**/` is suggested.

.moon/workspace.yml

```
hasher:  ignoreMissingPatterns:    - '**/.eslintrc.*'    - '**/*.config.*'
```

### `ignorePatterns`v1.10.0

A list of [glob patterns](/docs/concepts/file-pattern#globs) used to filter and ignore files during the
inputs hashing process. Files are matched against workspace relative paths, so prefixing patterns
with `**/` is suggested.

.moon/workspace.yml

```
hasher:  ignorePatterns:    - '**/*.png'
```

### `optimization`

Determines the optimization level to utilize when hashing content before running targets.

- `accuracy` (default) - When hashing dependency versions, utilize the resolved value in the lockfile. This requires parsing the lockfile, which may reduce performance.

- `performance` - When hashing dependency versions, utilize the value defined in the manifest. This is typically a version range or requirement.

.moon/workspace.yml

```
hasher:  optimization: 'performance'
```

### `walkStrategy`

Defines the file system walking strategy to utilize when discovering inputs to hash.

- `glob` - Walks the file system using glob patterns.

- `vcs` (default) - Calls out to the [VCS](#vcs) to extract files from its working tree.

.moon/workspace.yml

```
hasher:  walkStrategy: 'glob'
```

### `warnOnMissingInputs`

When enabled, will log warnings to the console when attempting to hash an input that does not exist.
This is useful in uncovering misconfigured tasks. Defaults to `true`.

.moon/workspace.yml

```
hasher:  warnOnMissingInputs: false
```

## `notifier`

Configures how moon notifies and interacts with a developer or an external system.

### `terminalNotifications`v1.38.0

When defined, will display OS notifications for action pipeline events when running commands from a
terminal. Supports the following values:

- `always` - Display on pipeline success and failure.

- `failure` - Display on pipeline failure only.

- `success` - Display on pipeline success only.

- `task-failure` - Display for each task failure.

.moon/workspace.yml

```
notifier:  terminalNotifications: 'always'
```

### `webhookUrl`

Defines an HTTPS URL that all pipeline events will be posted to. View the
[webhooks guide for more information](/docs/guides/webhooks) on available events.

.moon/workspace.yml

```
notifier:  webhookUrl: 'https://api.company.com/some/endpoint'
```

### `acknowledge`

When enabled, webhook notifier will wait for request result and validates the return code for 2xx.
Defaults to `false`.

warning

Activating this setting will slow down your pipeline, because every webhook request will be
evaluated!

.moon/workspace.yml

```
notifier:  webhookUrl: 'https://api.company.com/some/endpoint'  webhookAcknowledge: true
```

## `pipeline`

Configures aspects of task running and the action pipeline.

### `autoCleanCache`v1.24.0

Automatically cleans cached artifacts older than [`cacheLifetime`](#cachelifetime) from the cache
directory (`.moon/cache`) after every run. This is useful for keeping the cache directory lean.
Defaults to `true`.

.moon/workspace.yml

```
pipeline:  autoCleanCache: false
```

### `cacheLifetime`

The maximum lifetime of cached artifacts before they're marked as stale and automatically removed by
the action pipeline. Defaults to "7 days". This field requires an integer and a timeframe unit that
can be [parsed as a duration](https://docs.rs/humantime/2.1.0/humantime/fn.parse_duration.html).

.moon/workspace.yml

```
pipeline:  cacheLifetime: '24 hours'
```

### `inheritColorsForPipedTasks`

Force colors to be inherited from the current terminal for all tasks that are ran as a child process
and their output is piped to the action pipeline. Defaults to `true`.
[View more about color handling in moon](/docs/commands/overview#colors).

.moon/workspace.yml

```
pipeline:  inheritColorsForPipedTasks: true
```

### `installDependencies`v1.34.0

When enabled, runs the
[`InstallWorkspaceDeps` and `InstallProjectDeps` actions](/docs/how-it-works/action-graph#install-dependencies)
within the pipeline before running an applicable task. Installation is determined based on changed
manifests and lockfiles. Defaults to `true`.

.moon/workspace.yml

```
pipeline:  installDependencies: false
```

Instead of a boolean, a list of toolchain IDs can be provided to only allow those toolchains to
install dependencies.

.moon/workspace.yml

```
pipeline:  installDependencies: ['node']
```

### `killProcessThreshold`v1.32.1

Threshold in milliseconds in which to force kill running child processes after the pipeline receives
an external signal (like `SIGINT` or `SIGTERM`). A value of 0 will not kill the process and let them
run to completion. Defaults to `2000` (2 seconds).

.moon/workspace.yml

```
pipeline:  killProcessThreshold: 5000
```

### `logRunningCommand`

When enabled, will log the task's command, resolved arguments, and working directory when a target
is ran. Defaults to `false`.

.moon/workspace.yml

```
pipeline:  logRunningCommand: true
```

### `syncProjects`v1.34.0

When enabled, runs the [`SyncProject` action](/docs/how-it-works/action-graph#sync-project) within the
pipeline before running an applicable task. Defaults to `true`.

.moon/workspace.yml

```
pipeline:  syncProjects: false
```

Instead of a boolean, a list of project IDs can be provided to only sync those projects.

.moon/workspace.yml

```
pipeline:  syncProjects: ['app']
```

The [`moon sync projects`](/docs/commands/sync/projects) command can be executed to manually sync
projects.

### `syncWorkspace`v1.34.0

When enabled, runs the [`SyncWorkspace` action](/docs/how-it-works/action-graph#sync-workspace) within
the pipeline before all other actions. This syncing includes operations such as codeowners, VCS
hooks, and more. Defaults to `true`.

.moon/workspace.yml

```
pipeline:  syncWorkspace: false
```

The [`moon sync ...`](/docs/commands/sync) sub-commands can be executed to manually sync features.

## `remote`v1.30.0

Configures a remote service, primarily for cloud-based caching of artifacts. Learn more about this
in the [remote caching](/docs/guides/remote-cache) guide.

### `api`v1.32.0

The API format of the remote server. This format dictates which type of client moon uses for
communicating with. Supports the following:

- `grpc` (default) - Uses the gRPC API: [https://github.com/bazelbuild/remote-apis](https://github.com/bazelbuild/remote-apis)

- `http` - Uses the HTTP API: [https://bazel.build/remote/caching#http-caching](https://bazel.build/remote/caching#http-caching)

.moon/workspace.yml

```
remote:  api: 'grpc'
```

### `auth`v1.32.0

Configures authorization and authentication level features of our remote clients.

#### `headers`v1.32.0

A mapping of HTTP headers to include in all requests to the remote server. These headers are applied
to all [API formats and protocols](#api), not just HTTP.

.moon/workspace.yml

```
remote:  auth:    headers:      'X-Custom-Header': 'value'
```

#### `token`v1.32.0

The name of an environment variable in which to extract a token for
[Bearer HTTP authorization](https://swagger.io/docs/specification/v3_0/authentication/bearer-authentication/).
An `Authorization` HTTP header will be included in all requests to the remote server.

If the token does not exist, or is not enabled, remote caching will be disabled.

.moon/workspace.yml

```
remote:  auth:    token: 'ENV_VAR_NAME'
```

### `cache`

Configures aspects of the caching layer, primarily the action cache (AC) and content addressable
cache (CAS).

#### `compression`v1.31.0

The compression format to use when uploading/downloading blobs. Supports `none` and `zstd`, and
defaults to no compression (`identity` format in RE API).

.moon/workspace.yml

```
remote:  cache:    compression: 'zstd'
```

info

Compression is only applied to gRPC based APIs, not HTTP.

#### `instanceName`

A
[unique identifier](https://github.com/bazelbuild/remote-apis/blob/main/build/bazel/remote/execution/v2/remote_execution.proto#L223)
used to distinguish between the various instances on the host. This allows the same remote service
to serve and partition multiple moon repositories. Defaults to `moon-outputs`.

.moon/workspace.yml

```
remote:  cache:    instanceName: 'custom-dir-name'
```

We suggest changing the instance name to the name of your repository!

#### `localReadOnly`v1.40.0

When enabled and developing locally, existing remote blobs will only be downloaded, but new local
blobs will not be uploaded. Blobs will only be uploaded in CI environments.

.moon/workspace.yml

```
remote:  cache:    localReadOnly: true
```

#### `verifyIntegrity`v1.36.0

When downloading blobs, verify the digests/hashes in the response match the associated blob
contents. This will reduce performance but ensure partial or corrupted blobs won't cause failures.
Defaults to `false`.

.moon/workspace.yml

```
remote:  cache:    verifyIntegrity: true
```

### `host`

The host URL to communicate with when uploading and downloading artifacts. Supports both
`grpc(s)://` and `http(s)://` protocols. This field is required!

.moon/workspace.yml

```
remote:  host: 'grpcs://your-host.com:9092'
```

### `mtls`

Connect to the host using server and client authentication with mTLS. This takes precedence over
normal TLS.

.moon/workspace.yml

```
remote:  # ...  mtls:    caCert: 'certs/ca.pem'    clientCert: 'certs/client.pem'    clientKey: 'certs/client.key'    domain: 'your-host.com'
```

#### `assumeHttp2`

If true, assume that the host supports HTTP/2, even if it doesn't provide protocol negotiation via
ALPN.

#### `caCert`

A file path, relative from the workspace root, to the certificate authority PEM encoded X509
certificate (typically `ca.pem`).

#### `clientCert`

A file path, relative from the workspace root, to the client's PEM encoded X509 certificate
(typically `client.pem`).

#### `clientKey`

A file path, relative from the workspace root, to the client's PEM encoded X509 private key
(typically `client.key`).

#### `domain`

The domain name in which to verify the TLS certificate.

### `tls`

Connect to the host using server-only authentication with TLS.

.moon/workspace.yml

```
remote:  # ...  tls:    cert: 'certs/ca.pem'    domain: 'your-host.com'
```

#### `assumeHttp2`

If true, assume that the host supports HTTP/2, even if it doesn't provide protocol negotiation via
ALPN.

#### `cert`

A file path, relative from the workspace root, to the certificate authority PEM encoded X509
certificate (typically `ca.pem`).

#### `domain`

The domain name in which to verify the TLS certificate.

## `telemetry`

When enabled, will check for a newer moon version and send anonymous usage data to the moonrepo
team. This data is used to improve the quality and reliability of the tool. Defaults to `true`.

.moon/workspace.yml

```
telemetry: false
```

## `vcs`

Configures the version control system to utilize within the workspace (and repository). A VCS is
required for determining touched (added, modified, etc) files, calculating file hashes, computing
affected files, and much more.

### `defaultBranch`

Defines the default branch in the repository for comparing differences against. For git, this is
typically "master" (default) or "main".

.moon/workspace.yml

```
vcs:  defaultBranch: 'master'
```

### `hooks`v1.9.0

Defines a mapping of hooks to a list of commands to run when that event is triggered. There are no
restrictions to what commands can be run, but the binaries for each command must exist on each
machine that will be running hooks.

For Git, each [hook name](https://git-scm.com/docs/githooks#_hooks) must be a valid kebab-cased
name. [Learn more about Git hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks).

.moon/workspace.yml

```
vcs:  hooks:    pre-commit:      - 'moon run :lint :format --affected --status=staged --no-bail'      - 'another-command'
```

info

If running `moon` commands directly, the `moon` binary must be installed globally!

### `hookFormat`v1.29.0

The shell and file type in which generated hook files are formatted with. Supports the following:

- `native` (default) - The format native to the current operating system. Bash on Unix, and PowerShell on Windows.

- `bash` - Forces the format to Bash for all operating systems.

.moon/workspace.yml

```
vcs:  hookFormat: 'bash'
```

### `client`

Defines the VCS tool/binary that is being used for managing the repository. Accepts "git" (default).
Expect more version control systems in the future!

.moon/workspace.yml

```
vcs:  client: 'git'
```

### `provider`v1.8.0

Defines the service provider that the repository is hosted on. Accepts "github" (default), "gitlab",
"bitbucket", or "other".

.moon/workspace.yml

```
vcs:  provider: 'github'
```

### `remoteCandidates`

(Git only) Defines a list of remote candidates to query against to determine merge bases. Defaults
to "origin" and "upstream".

.moon/workspace.yml

```
vcs:  remoteCandidates:    - 'origin'    - 'upstream'
```

### `sync`v1.9.0

Will automatically generate [hook scripts](#hooks) to `.moon/hooks` and sync the scripts to the
local VCS checkout. The hooks format and location is based on the [`vcs.client`](#client) setting.
Defaults to `false`.

.moon/workspace.yml

```
vcs:  hooks:    # ...  sync: true
```

caution

When enabled, this will sync hooks for all users of the repository. For personal or small
projects, this may be fine, but for larger projects, this may be undesirable and disruptive!

## `versionConstraint`

Defines a version requirement for the currently running moon binary. This provides a mechanism for
enforcing that the globally installed moon on every developers machine is using an applicable
version.

.moon/workspace.yml

```
versionConstraint: '>=0.20.0'
```

## /docs/create-project

Source: https://moonrepo.dev/docs/create-project

# Create a project

3 min

With a [workspace](/docs/setup-workspace), we can now house one or many [projects](/docs/concepts/project),
with a project being an application, library, or tool. In the end, each project will have its own
build layer, personal tasks, and custom configuration.

## Declaring a project in the workspace

Although a project may exist in your repository, it's not accessible from moon until it's been
mapped in the [`projects`](/docs/config/workspace#projects) setting found in
[`.moon/workspace.yml`](/docs/config/workspace). When mapping a project, we require a unique name for
the project, and a project source location (path relative from the workspace root).

Let's say we have a frontend web application called "client", and a backend application called
"server", our `projects` setting would look like the following.

.moon/workspace.yml

```
projects:  client: 'apps/client'  server: 'apps/server'
```

We can now run [`moon project client`](/docs/commands/project) and
[`moon project server`](/docs/commands/project) to display information about each project. If these
projects were not mapped, or were pointing to an invalid source, the command would throw an error.

success

The [`projects`](/docs/config/workspace#projects) setting also supports a list of globs, if you'd prefer
to not manually curate the projects list!

## Configuring a project

A project can be configured in 1 of 2 ways:

- Through the [`.moon/tasks/all.yml`](/docs/config/tasks) config file, which defines file groups and tasks that are inherited by all projects within the workspace. Perfect for standardizing common tasks like linting, typechecking, and code formatting.

- Through the [`moon.yml`](/docs/config/project) config file, found at the root of each project, which defines files groups, tasks, dependencies, and more that are unique to that project.

Both config files are optional, and can be used separately or together, the choice is yours!

Now let's continue with our client and server example above. If we wanted to configure both
projects, and define config that's also shared between the 2, we could do something like the
following:

- Client
- Server
- Both (inherited)

apps/client/moon.yml

```
tasks:  build:    command: 'vite dev'    inputs:      - 'src/**/*'    outputs:      - 'dist'
```

apps/server/moon.yml

```
tasks:  build:    command: 'babel src --out-dir build'    inputs:      - 'src/**/*'    outputs:      - 'build'
```

.moon/tasks/all.yml

```
tasks:  format:    command: 'prettier --check .'  lint:    command: 'eslint --no-error-on-unmatched-pattern .'  test:    command: 'jest --passWithNoTests .'  typecheck:    command: 'tsc --build'
```

### Adding optional metadata

When utilizing moon in a large monorepo or organization, ownership becomes very important, but also
difficult to maintain. To combat this problem, moon supports the
[`project`](/docs/config/project#project) field within a project's [`moon.yml`](/docs/config/project)
config.

This field is optional by default, but when defined it provides metadata about the project,
specifically around team ownership, which developers maintain the project, where to discuss it, and
more!

Furthermore, we also support the [`layer`](/docs/config/project#layer) and
[`language`](/docs/config/project#language) settings for a more granular breakdown of what exists in the
repository.

/moon.yml

```
layer: 'tool'language: 'typescript'project:  name: 'moon'  description: 'A repo management tool.'  channel: '#moon'  owner: 'infra.platform'  maintainers: ['miles.johnson']
```

## Next steps

[Setup toolchain](/docs/setup-toolchain)[Configure `.moon/workspace.yml` further](/docs/config/workspace)[Configure `.moon/tasks/all.yml` further](/docs/config/tasks)[Configure `moon.yml` further](/docs/config/project)[Learn about projects](/docs/concepts/project)

## /docs/create-task

Source: https://moonrepo.dev/docs/create-task

# Create a task

6 min

The primary focus of moon is a task runner, and for it to operate in any capacity, it requires tasks
to run. In moon, a task is a binary or system command that is ran as a child process within the
context of a project (is the current working directory). Tasks are defined per project with
[`moon.yml`](/docs/config/project), or inherited by many projects with
[`.moon/tasks/all.yml`](/docs/config/tasks), but can also be inferred from a language's ecosystem
([we'll talk about this later](/docs/migrate-to-moon)).

tip

Change the language dropdown at the top right to switch the examples!

## Configuring a task

Most — if not all projects — utilize the same core tasks: linting, testing, code formatting,
typechecking, and building. Because these are so universal, let's implement the build task within
a project using [`moon.yml`](/docs/config/project).

Begin by creating the `moon.yml` file at the root of a project and add `build` to the
[`tasks`](/docs/config/project#tasks) field, with a [`command`](/docs/config/project#command) parameter.

/moon.yml

```
language: 'javascript'tasks:  build:    command: 'webpack build'
```

By itself, this isn't doing much, so let's add some arguments. Arguments can also be defined with
the [`args`](/docs/config/project#args) setting.

/moon.yml

```
language: 'javascript'tasks:  build:    command: 'webpack build --mode production --no-stats'
```

With this, the task can be ran from the command line with
[`moon run :build`](/docs/commands/run)! This is tasks in its most simplest form, but continue
reading on how to take full advantage of our task runner.

### Inputs

Our task above works, but isn't very efficient as it always runs, regardless of what has changed
since the last time it has ran. This becomes problematic in continuous integration environments, not
just locally.

To mitigate this problem, moon provides a system known as inputs, which are file paths, globs, and
environment variables that are used by the task when it's ran. moon will use and compare these
inputs to calculate whether to run, or to return the previous run state from the cache.

If you're a bit confused, let's demonstrate this by expanding the task with the
[`inputs`](/docs/config/project#inputs) setting.

/moon.yml

```
language: 'javascript'tasks:  build:    command: 'webpack build --mode production --no-stats'    inputs:      - 'src/**/*'      - 'webpack.config.js'      - '/webpack-shared.config.js'
```

This list of inputs may look complicated, but they are merely run checks. For example, when moon
detects a change in...

- Any files within the `src` folder, relative from the project's root.

- A config file in the project's root.

- A shared config file in the workspace root (denoted by the leading `/`).

...the task will be ran! If the change occurs outside of the project or outside the list of
inputs, the task will not be ran.

tip

Inputs are a powerful feature that can be fine-tuned to your project's need. Be as granular or open
as you want, the choice is yours!

### Outputs

Outputs are the opposite of [inputs](#inputs), as they are files and folders that are created as a
result of running the task. With that being said, outputs are optional, as not all tasks require
them, and the ones that do are typically build related.

Now why is declaring outputs important? For incremental builds and smart caching! When moon
encounters a build that has already been built, it hydrates all necessary outputs from the cache,
then immediately exits. No more waiting for long builds!

Continuing our example, let's route the built files and expand our task with the
[`outputs`](/docs/config/project#outputs) setting.

/moon.yml

```
language: 'javascript'tasks:  build:    command: 'webpack build --mode production --no-stats --output-path @out(0)'    inputs:      - 'src/**/*'      - 'webpack.config.js'      - '/webpack-shared.config.js'    outputs:      - 'build'
```

## Depending on other tasks

For scenarios where you need run a task before another task, as you're expecting some repository
state or artifact to exist, can be achieved with the [`deps`](/docs/config/project#deps) setting, which
requires a list of [targets](/docs/concepts/target):

- `:` - Full canonical target.

- `~:` or `` - A task within the current project.

- `^:` - A task from all [depended on projects](/docs/concepts/project#dependencies).

/moon.yml

```
dependsOn:  # ...tasks:  build:    # ...    deps:      - '^:build'
```

## Using file groups

Once you're familiar with configuring tasks, you may notice certain inputs being repeated
constantly, like source files, test files, and configuration. To reduce the amount of boilerplate
required, moon provides a feature known as [file groups](/docs/concepts/file-group), which enables
grouping of similar file types within a project using
[file glob patterns or literal file paths](/docs/concepts/file-pattern).

File groups are defined with the [`fileGroups`](/docs/config/project#filegroups) setting, which maps a
list of file paths/globs to a group, like so.

/moon.yml

```
fileGroups:  configs:    - '*.config.js'  sources:    - 'src/**/*'    - 'types/**/*'  tests:    - 'tests/**/*'
```

We can then replace the inputs in our task above with these new file groups using a syntax known as
[tokens](/docs/concepts/token), specifically the [`@globs`](/docs/concepts/token#globs) and
[`@files`](/docs/concepts/token#files) token functions. Tokens are an advanced feature, so please refer
to their documentation for more information!

/moon.yml

```
language: 'javascript'fileGroups:  # ...tasks:  build:    command: 'webpack build --mode production --no-stats --output-path @out(0)'    inputs:      - '@globs(sources)'      - 'webpack.config.js'      - '/webpack-shared.config.js'    outputs:      - 'build'
```

With file groups (and tokens), you're able to reduce the amount of configuration required and
encourage certain file structures for consuming projects!

## Next steps

[Run a task](/docs/run-task)[Configure `.moon/tasks/all.yml` further](/docs/config/tasks)[Configure `moon.yml` further](/docs/config/project)[Learn about tasks](/docs/concepts/task)[Learn about tokens](/docs/concepts/token)

## /docs/editors

Source: https://moonrepo.dev/docs/editors

- [Home](/)
- [Editors](/docs/editors)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Editors

[📄️ VS CodeEnhance your VS Code experience with our integrated moon console! Whether you're a fan of the](/docs/editors/vscode)

[template](/docs/config/template)

[VS Code](/docs/editors/vscode)

## /docs/editors/vscode

Source: https://moonrepo.dev/docs/editors/vscode

# VS Code extension

Enhance your VS Code experience with our integrated moon console! Whether you're a fan of the
command line, or prefer interactive interfaces, our console will be a welcome experience.

This extension is in its early stages. Expect more advanced features in the future, like
autocompletion, config validation, and more!

## Views

All views are available within the moon sidebar. Simply click the moon icon in the left activity
bar!

### Projects

The backbone of moon is the projects view. In this view, all moon configured projects will be
listed, categorized by their [`layer`](/docs/config/project#layer), [`stack`](/docs/config/project#stack),
and designated with their [`language`](/docs/config/project#language).

Each project can then be expanded to view all available tasks. Tasks can be ran by clicking the `▶`
icon, or using the command palette.

This view is available in both the "Explorer" and "moon" sidebars.

### Tags

Similar to the projects view, the tags view displays projects grouped by their
[`tags`](/docs/config/project#tags).

This view is only available in the "moon" sidebar.

### Last run

Information about the last ran task will be displayed in a beautiful table with detailed stats.

This table displays all actions that were ran alongside the primary target(s). They are ordered
topologically via the action graph.

## Features

### YAML validation

To enable accurate validation of our YAML configuration files, you'll need to update the
`yaml.schemas` setting in `.vscode/settings.json` to point to the local schemas at
`.moon/cache/schemas`.

This can be automated by running the "moon: Append YAML schemas configuration to settings" in the
command palette, after the extension has been installed.

## Troubleshooting

View the
[official VS Code marketplace](https://marketplace.visualstudio.com/items?itemName=moonrepo.moon-console)
for more information on the extension, its commands, available settings, and more!

If you encounter a bug, or have a feature request, please submit them to the
[moonrepo/dev](https://github.com/moonrepo/dev/tree/master/packages/vscode-extension) repository!

## /docs/faq

Source: https://moonrepo.dev/docs/faq

# FAQ

## General

### Where did the name "moon" come from?

The first incarnation of the name was a misspelling of monorepo (= moonrepo). This is where the
domain moonrepo.dev came from, and our official company, moonrepo, Inc.

However, moonrepo is quite a long name with many syllables, and as someone who prefers short 1
syllable words, moon was perfect. The word moon also has great symmetry, as you can see in our logo!

But that's not all... moon is also an acronym. It originally stood for monorepo,
organization, orchestration, and notification tool. But since moon can also be used for
polyrepos, we replaced monorepo with management (as shown on the homepage). This is a great
acronym, as it embraces what moon is trying to solve:

- Manage repos, projects, and tasks with ease.

- Organize projects and the repo to scale.

- Orchestrate tasks as efficiently as possible.

- Notify developers and systems about important events.

### Will moon support other languages?

Yes! Although we're focusing right now on the web ecosystem (Node.js, Rust, Go, PHP, Python, etc),
we've designed moon to be language agnostic and easily pluggable in the future. View our
[supported languages for more information](/docs#supported-languages).

### Will moon support continuous deployment?

Yes! We plan to integrate CD with the current build and CI system, but we are focusing on the latter
2 for the time being. Why not start using moon today so that you can easily adopt CD when it's
ready?

### What should be considered the "source of truth"?

If you're a frontend developer, you'll assume that a `package.json` is the source of truth for a
project, as it defines scripts, dependencies, and repo-local relations. While true, this breaks down
with additional tooling, like TypeScript project references, as now you must maintain
`tsconfig.json` as well as `package.json`. The risk of these falling out of sync is high.

This problem is further exacerbated by more tooling, or additional programming languages. What if
your frontend project is dependent on a backend project? This isn't easily modeled in
`package.json`. What if the backend project needs to be built and ran before running the frontend
project? Again, while not impossible, it's quite cumbersome to model in `package.json` scripts. So
on and so forth.

moon aims to solve this with a different approach, by standardizing all projects in the workspace on
[`moon.yml`](/docs/config/project). With this, the `moon.yml` is the source of truth for each project,
and provides us with the following:

- The configuration is language agnostic. All projects are configured in a similar manner.

- Tasks can reference other tasks easily. For example, npm scripts referencing rake tasks, and vice verse, is a non-ideal experience.

- Dependencies defined with [`dependsOn`](/docs/config/project#dependson) use moon project names, and not language specific semantics. This field also easily populates the dependency/project graphs.

- For JavaScript projects: `package.json` dependencies (via `dependsOn`) are kept in sync when [`node.syncProjectWorkspaceDependencies`](/docs/config/toolchain#syncprojectworkspacedependencies) is enabled.

- `tsconfig.json` project references (via `dependsOn`) are kept in sync when [`typescript.syncProjectReferences`](/docs/config/toolchain#syncprojectreferences) is enabled.

By using moon as the source of truth, we can ensure a healthy repository, by accurately keeping
everything in sync, and modifying project/language configuration to operate effectively.

info

With all that being said, moon supports
[implicit dependency scanning](/docs/concepts/project#dependencies), if you'd prefer to continue
utilizing language specific functionality, instead of migrating entirely to moon.

### How to stop moon formatting JSON and YAML files?

To ensure a healthy repository state, moon constantly modifies JSON and YAML files, specifically
`package.json` and `tsconfig.json`. This may result in a different formatting style in regards to
indentation. While there is no way to stop or turn off this functionality, we respect
[EditorConfig](https://editorconfig.org/) during this process.

Create a root `.editorconfig` file to enforce a consistent syntax.

.editorconfig

```
[*.{json,yaml,yml}]indent_style = spaceindent_size = 4
```

## Projects & tasks

### How to pipe or redirect tasks?

Piping (`|`) or redirecting (`>`) the output of one moon task to another moon task, whether via
stdin or through `inputs`, is not possible within our pipeline (task runner) directly.

However, we do support this functionality on the command line, or within a task itself, using the
[`script`](/docs/config/project#script) setting.

moon.yml

```
tasks:  pipe:    script: 'gen-json | jq ...'
```

Alternativaly, you can wrap this script in something like a Bash file, and execute that instead.

scripts/pipe.sh

```
#!/usr/bin/env bashgen-json | jq ...
```

moon.yml

```
tasks:  pipe:    command: 'bash ./scripts/pipe.sh'
```

### How to run multiple commands within a task?

Only [`script`](/docs/config/project#script) based tasks can run multiple commands via `&&` or `;`
syntax. This is possible as we execute the entire script within a shell, and not directly with the
toolchain.

moon.yml

```
tasks:  multiple:    script: 'mkdir test && cd test && do-something'
```

### How to run tasks in a shell?

By default, all tasks run in a shell, based on the task's [`shell`](/docs/config/project#shell) option,
as demonstrated below:

moon.yml

```
tasks:  # Runs in a shell  global:    command: 'some-command-on-path'  # Custom shells  unix:    command: 'bash -c some-command'    options:      shell: false  windows:    command: 'pwsh.exe -c some-command'    options:      shell: false
```

### Can we run other languages?

Yes! Although our toolchain only supports a few languages at this time, you can still run other
languages within tasks by setting their [`toolchain`](/docs/config/project#toolchain) to "system".
System tasks are an escape hatch that will use any command available on the current machine.

moon.yml

```
tasks:  # Ruby  lint:    command: 'rubocop'    toolchain: 'system'  # PHP  test:    command: 'phpunit tests'    toolchain: 'system'
```

However, because these languages are not supported directly within our toolchain, they will not
receive the benefits of the toolchain. Some of which are:

- Automatic installation of the language. System tasks expect the command to already exist in the environment, which requires the user to manually install them.

- Consistent language and dependency manager versions across all machines.

- Built-in cpu and heap profiling (language specific).

- Automatic dependency installs when the lockfile changes.

- And many more.

## JavaScript ecosystem

### Can we use `package.json` scripts?

We encourage everyone to define tasks in a [`moon.yml`](/docs/config/project#tasks) file, as it allows
for additional metadata like `inputs`, `outputs`, `options`, and more. However, if you'd like to
keep using `package.json` scripts, enable the
[`node.inferTasksFromScripts`](/docs/config/toolchain#infertasksfromscripts) setting.

View the [official documentation](/docs/migrate-to-moon) for more information on this approach,
including risks, disadvantages, and caveats.

### Can moon version/publish packages?

At this time, no, as we're focusing on the build and test aspect of development. With that being
said, this is something we'd like to support first-class in the future, but until then, we suggest
the following popular tools:

- [Yarn releases](https://yarnpkg.com/features/release-workflow) (requires >= v2)

- [Changesets](https://github.com/changesets/changesets)

- [Lerna](https://github.com/lerna/lerna)

### Why is npm/pnpm/yarn install running twice when running a task?

moon will automatically install dependencies in a project or in the workspace root (when using
package workspaces) when the lockfile or `package.json` has been modified since the last time the
install ran. If you are running a task and multiple installs are occurring (and it's causing
issues), it can mean 1 of 2 things:

- If you are using package workspaces, then one of the projects triggering the install is not listed within the `workspaces` field in the root `package.json` (for npm and yarn), or in `pnpm-workspace.yml` (for pnpm).

- If the install is triggering in a non-JavaScript related project, then this project is incorrectly listed as a package workspace.

- If you don't want a package included in the workspace, but do want to install its dependencies, then it'll need its own lockfile.

## Troubleshooting

### How to resolve the "version 'GLIBC_X.XX' not found" error?

This is typically caused by running moon in an old environment, like Ubuntu 18, and the minimum
required libc doesn't exist or is too old. Since moon is Rust based, we're unable to support all
environments and versions perpetually, and will only support relatively modern environments.

There's not an easy fix to this problem, but there are a few potential solutions, from easiest to
hardest:

- Run moon in a Docker container/image that has the correct environment and libs. For example, the `node:latest` image.

- Upgrade the environment to a newer one. For example, Ubuntu 18 -> 22.

- Try and install a newer libc ([more information](https://stackoverflow.com/questions/72513993/how-install-glibc-2-29-or-higher-in-ubuntu-18-04)).

For more information on this problem as a whole,
[refer to this in-depth article](https://kobzol.github.io/rust/ci/2021/05/07/building-rust-binaries-in-ci-that-work-with-older-glibc.html).

## /docs/guides/ci

Source: https://moonrepo.dev/docs/guides/ci

# Continuous integration (CI)

All companies and projects rely on continuous integration (CI) to ensure high quality code and to
avoid regressions. Because this is such a critical piece of every developer's workflow, we wanted to
support it as a first-class feature within moon, and we do just that with the
[`moon ci`](/docs/commands/ci) command.

## How it works

The `ci` command does all the heavy lifting necessary for effectively running jobs. It achieves this
by automatically running the following steps:

- Determines touched files by comparing the current HEAD against a base.

- Determines all [targets](/docs/concepts/target) that need to run based on touched files.

- Additionally runs affected [targets](/docs/concepts/target) dependencies and dependents.

- Generates an action and dependency graph.

- Installs the toolchain, Node.js, and npm dependencies.

- Runs all actions within the graph using a thread pool.

- Displays stats about all passing, failed, and invalid actions.

## Configuring tasks

By default, all tasks run in CI, as you should always be building, linting, typechecking, testing,
so on and so forth. However, this isn't always true, so this can be disabled on a per-task basis
through the [`runInCI`](/docs/config/project#runinci) or [`local`](/docs/config/project#local) options.

```
tasks:  dev:    command: 'webpack server'    options:      runInCI: false    # Or    preset: 'server'
```

caution

This option must be set to false for tasks that spawn a long-running or never-ending process, like
HTTP or development servers. To help mitigate this, tasks named `dev`, `start`, or `serve` are false
by default. This can be easily controlled with the [`local`](/docs/config/project#local) setting.

## Integrating

The following examples can be referenced for setting up moon and its CI workflow in popular
providers. For GitHub, we're using our
[`setup-toolchain` action](https://github.com/moonrepo/setup-toolchain) to install moon. For other
providers, we assume moon is an npm dependency and must be installed with Node.js.

- GitHub
- Buildkite
- CircleCI
- TravisCI

.github/workflows/ci.yml

```
name: 'Pipeline'on:  push:    branches:      - 'master'  pull_request:jobs:  ci:    name: 'CI'    runs-on: 'ubuntu-latest'    steps:      - uses: 'actions/checkout@v4'        with:          fetch-depth: 0      - uses: 'moonrepo/setup-toolchain@v0'      - run: 'moon ci'
```

.buildkite/pipeline.yml

```
steps:  - label: 'CI'    commands:      - 'yarn install --immutable'      - 'moon ci'
```

.circleci/config.yml

```
version: 2.1orbs:  node: 'circleci/node@5.0.2'jobs:  ci:    docker:      - image: 'cimg/base:stable'    steps:      - checkout      - node/install:          install-yarn: true          node-version: '16.13'      - node/install-packages:          check-cache: 'always'          pkg-manager: 'yarn-berry'      - run: 'moon ci'workflows:  pipeline:    jobs:      - 'ci'
```

.travis.yml

```
language: node_jsnode_js:  - 16cache: yarnscript: 'moon ci'
```

## Choosing targetsv1.14.0

By default `moon ci` will run all tasks from all projects that are affected by touched files and
have the [`runInCI`](/docs/config/project#runinci) task option enabled. This is a great catch-all
solution, but may not vibe with your workflow or requirements.

If you'd prefer more control, you can pass a list of targets to `moon ci`, instead of moon
attempting to detect them. When providing targets, `moon ci` will still only run them if affected by
touched files, but will still filter with the `runInCI` option.

```
# Run all builds$ moon ci :build# In another job, run tests$ moon ci :test :lint
```

## Comparing revisions

By default the command will attempt to detect the base and head revisions automatically based on the
current CI provider (powered by the [`ci_env`](https://github.com/milesj/rust-cicd-env) Rust crate).
If nothing was detected, this will fallback to the configured
[`vcs.defaultBranch`](/docs/config/workspace#defaultbranch) for the base revision, and `HEAD` for the
head revision.

These values can be customized with the `--base` and `--head` command line options, or the
`MOON_BASE` and `MOON_HEAD` environment variables, which takes highest precedence.

```
$ moon ci --base  --head # Or$ MOON_BASE= MOON_HEAD= moon ci
```

## Parallelizing tasks

If your CI environment supports sharding across multiple jobs, then you can utilize moon's built in
parallelism by passing `--jobTotal` and `--job` options. The `--jobTotal` option is an integer of
the total number of jobs available, and `--job` is the current index (0 based) amongst the total.

When these options are passed, moon will only run affected [targets](/docs/concepts/target) based on
the current job slice.

- GitHub
- Buildkite
- CircleCI
- TravisCI

GitHub Actions do not support native parallelism, but it can be emulated using it's matrix.

.github/workflows/ci.yml

```
# ...jobs:  ci:    # ...    strategy:      matrix:        index: [0, 1]    steps:      # ...      - run: 'moon ci --job ${{ matrix.index }} --jobTotal 2'
```

- [Documentation](https://docs.github.com/en/actions/using-jobs/using-a-matrix-for-your-jobs)

.buildkite/pipeline.yml

```
# ...steps:  - label: 'CI'    parallelism: 10    commands:      # ...      - 'moon ci --job $$BUILDKITE_PARALLEL_JOB --jobTotal $$BUILDKITE_PARALLEL_JOB_COUNT'
```

- [Documentation](https://buildkite.com/docs/tutorials/parallel-builds#parallel-jobs)

.circleci/config.yml

```
# ...jobs:  ci:    # ...    parallelism: 10    steps:      # ...      - run: 'moon ci --job $CIRCLE_NODE_INDEX --jobTotal $CIRCLE_NODE_TOTAL'
```

- [Documentation](https://circleci.com/docs/2.0/parallelism-faster-jobs/)

TravisCI does not support native parallelism, but it can be emulated using it's matrix.

.travis.yml

```
# ...env:  global:    - TRAVIS_JOB_TOTAL=2  jobs:    - TRAVIS_JOB_INDEX=0    - TRAVIS_JOB_INDEX=1script: 'moon ci --job $TRAVIS_JOB_INDEX --jobTotal $TRAVIS_JOB_TOTAL'
```

- [Documentation](https://docs.travis-ci.com/user/speeding-up-the-build/)

Your CI environment may provide environment variables for these 2 values.

## Caching artifacts

When a CI pipeline reaches a certain scale, its run times increase, tasks are unnecessarily ran, and
build artifacts are not shared. To combat this, we support [remote caching](/docs/guides/remote-cache), a
mechanism where we store build artifacts in the cloud, and sync these artifacts to machines on
demand.

### Manual persistence

If you'd prefer to not use remote caching at this time, you can cache artifacts yourself, by
persisting the `.moon/cache/{hashes,outputs}` directories. All other files and folders in
`.moon/cache` should not be persisted, as they are not safe/portable across machines.

However, because tasks can generate a different hash each run, you'll need to manually invalidate
your cache. Blindly storing the `hashes` and `outputs` directories without a mechanism to invalidate
will simply not work, as the contents will drastically change between CI runs. This is the primary
reason why the remote caching service exists.

## Reporting run results

If you're using GitHub Actions as your CI provider, we suggest using our
[`moonrepo/run-report-action`](https://github.com/marketplace/actions/moon-ci-run-reports). This
action will report the results of a [`moon ci`](/docs/commands/ci) run to a pull request as a comment
and workflow summary.

.github/workflows/ci.yml

```
# ...jobs:  ci:    name: 'CI'    runs-on: 'ubuntu-latest'    steps:      # ...      - run: 'moon ci'      - uses: 'moonrepo/run-report-action@v1'        if: success() || failure()        with:          access-token: ${{ secrets.GITHUB_TOKEN }}
```

The report looks something like the following:

### Community offerings

The following GitHub actions are provided by the community:

- [`appthrust/moon-ci-retrospect`](https://github.com/appthrust/moon-ci-retrospect) - Displays the results of a `moon ci` run in a more readable fashion.

- [`kymckay/moon-ci-booster`](https://github.com/kymckay/moon-ci-booster) - Displays failing `moon ci` tasks as comments with error logs directly on your pull request.

## /docs/guides/codegen

Source: https://moonrepo.dev/docs/guides/codegen

# Code generation

Code generation provides an easy mechanism for automating common development workflows and file
structures. Whether it's scaffolding a new library or application, updating configuration, or
standardizing patterns.

To accomplish this, we provide a generator, which is divided into two parts. The first being the
templates and their files to be scaffolded. The second is our rendering engine that writes template
files to a destination.

## Creating a new template

To create a new template, run [`moon generate`](/docs/commands/generate) while passing the `--template` option. This
will create a template directory and [`template.yml`](/docs/config/template) file in the 1st file-based template
location defined in [`generator.templates`](/docs/config/workspace#templates).

```
$ moon generate  --template
```

### Configuring `template.yml`

Every template requires a [`template.yml`](/docs/config/template) file in the template's directory root. This file
acts as a schema and declares metadata and variables required by the generator.

template.yml

```
title: 'npm package'description: |  Scaffolds the initial structure for an npm package,  including source and test folders, a package.json, and more.variables:  name:    type: 'string'    default: ''    required: true    prompt: 'Package name?'
```

### Managing files

Feel free to add any files and folders to the template that you'd like to be generated by consumers!
These files will then be scaffolded 1:1 in structure at the target destination.

An example of the templates folder structure may look something like the following:

```
templates/├── npm-package/│   ├── src/│   ├── tests/│   ├── package.json│   └── template.yml└── react-app/
```

#### Interpolation

Variables can be interpolated into file paths using the form `[varName]`. For example, if you had a
template file `src/[type].ts`, and a variable `type` with a value of "bin", then the destination
file path would be `src/bin.ts`.

This syntax also supports [filters](#filters), such as `[varName | camel_case]`. However, spaces may
cause issues with file path encoding, so this functionality is primarily recommended for the
[`destination`](/docs/config/template#destination) setting.

#### File extensions

To enable syntax highlighting for template engine syntax, you may use the `.tera` (preferred) or
`.twig` file extensions. These extensions are optional, but will be removed when the files are
generated.

Depending on your preferred editor, these extensions may be supported through a plugin, or can be
configured based on file type.

- VS Code [Tera extension](https://marketplace.visualstudio.com/items?itemName=karunamurti.tera)

- [Twig extension](https://marketplace.visualstudio.com/items?itemName=mblode.twig-language-2)

- Atom [Twig package](https://atom.io/packages/atom-twig)

- Webstorm [Twig plugin](https://plugins.jetbrains.com/plugin/7303-twig)

#### Partials

Partials are special template files that are used for
[composition](https://keats.github.io/tera/docs/#include) and
[inheritance](https://keats.github.io/tera/docs/#inheritance). Because of this, these files should
not be generated into the target destination, and do not support frontmatter.

To ensure they are not generated, include the word "partial" anywhere in the file path. For example,
`partials/header.tpl` or `header.partial.tpl`.

#### Rawsv1.11.0

Raw template files are another special type of file that bypass all Tera rendering, and are used
as-is instead. This is useful for files that contain syntax that conflicts with Tera.

To mark a file as raw, add a `.raw` extension, for example: `file.raw.js` or `file.js.raw`. When the
file is generated, the `.raw` extension will be removed.

#### Frontmatter

Frontmatter is a well-known concept for "per-file configuration", and is achieved by inserting YAML
at the top of the file, delimited by wrapping `---`. This is a very powerful feature that provides
more control than the alternatives, and allows for some very cool integrations.

moon's frontmatter supports functionality like file skipping, force overwriting, and destination
path rewriting.
[View the configuration docs for a full list of supported fields](/docs/config/template#frontmatter).

package.json

```
---force: true---{  "name": "{{ name | kebab_case }}",  "version": "0.0.1"}
```

Since frontmatter exists in the file itself, you can take advantage of the rendering engine to
populate the field values dynamically. For example, if you're scaffolding a React component, you can
convert the component name and file name to PascalCase.

```
{% set component_name = name | pascal_case %}---to: components/{{ component_name }}.tsx---export function {{ component_name }}() {  return
;}
```

#### Assets

Assets are binary files that are copied as-is to the destination, without any rendering, and no
support for frontmatter. This applies to all non-text based files, like images, audio, video, etc.

### Template engine & syntax

Rendering templates is powered by [Tera](https://keats.github.io/tera/), a Rust based template
engine with syntax similar to Twig, Liquid, Django, and more. We highly encourage everyone to read
Tera's documentation for an in-depth understanding, but as a quick reference, Tera supports the
following:

- [Variable interpolation](https://keats.github.io/tera/docs/#variables) (defined with the [`variables`](/docs/config/template#variables) setting), with [built-in filters](https://keats.github.io/tera/docs/#built-in-filters).

```
{{ varName }} -> foo{{ varName | upper }} -> FOO
```

- [Conditional blocks](https://keats.github.io/tera/docs/#if) and [loops](https://keats.github.io/tera/docs/#for).

```
{% if price  1000 and not rich %}  That's expensive!{% else %}  N/A{% endif %}
```

```
{% for item in items %}  {{ loop.index }} - {{ item.name }}{% endfor %}
```

- And many more features, like auto-escaping, white space control, and math operators!

#### Filters

Filters are a mechanism for transforming values during interpolation and are written using pipes
(`|`). Tera provides many [built-in filters](https://keats.github.io/tera/docs/#built-in-filters),
but we also provide the following custom filters:

- Strings - `camel_case`, `pascal_case`, `snake_case`, `upper_snake_case`, `kebab_case`, `upper_kebab_case`, `lower_case`, `upper_case`

```
{{ some_value | upper_case }}
```

- Paths - `path_join`, `path_relative`

```
{{ some_path | path_join(part = "another/folder") }}{{ some_path | path_relative(from = other_path) }}{{ some_path | path_relative(to = other_path) }}
```

#### Functions

The following functions are available within a template:

- `variables()` - Returns an object containing all variables within the current template. v1.23.0

#### Variables

The following variables are always available within a template:

- `dest_dir` - Absolute path to the destination folder.

- `dest_rel_dir` - Relative path to the destination folder from the working directory.

- `working_dir` - Current working directory.

- `workspace_root` - The moon workspace root.

## Generating code from a template

Once a template has been created and configured, you can generate files based on it using the
[`moon generate`](/docs/commands/generate) command! This is also know as scaffolding or code generation.

This command requires the name of a template as the 1st argument. The template name is the folder
name on the file system that houses all the template files, or the [`id`](/docs/config/template#id)
setting configured in [`template.yml`](/docs/config/template).

```
$ moon generate npm-package
```

An optional destination path, relative from the current working directory, can be provided as the
2nd argument. If not provided, the [`destination`](/docs/config/template#destination) setting
configured in [`template.yml`](/docs/config/template) will be used, or you'll be prompted during
generation to provide one.

```
$ moon generate npm-package ./packages/example
```

This command is extremely interactive, as we'll prompt you for the destination path, variable
values, whether to overwrite files, and more. If you'd prefer to avoid interactions, pass
`--defaults`, or `--force`, or both.

### Configuring template locations

Templates can be located anywhere, especially when [being shared](#sharing-templates). Because of
this, our generator will loop through all template paths configured in
[`generator.templates`](/docs/config/workspace#templates), in order, until a match is found.

.moon/workspace.yml

```
generator:  templates:    - './templates'    # Or    - 'file://other/templates'
```

When using literal file paths, all paths are relative from the workspace root.

#### Archive URLsv1.36.0

Template locations can reference archives (zip, tar, etc) through https URLs. These archives should
contain templates and will be downloaded and unpacked. The list of
[available archive formats can be found here](https://github.com/moonrepo/starbase/blob/master/crates/archive/src/lib.rs#L76).

.moon/workspace.yml

```
generator:  templates:    - 'https://domain.com/some/path/to/archive.zip'
```

Archives will be unpacked to `~/.moon/templates/archive/`, and will be cached for future
use.

#### Globsv1.31.0

If you'd prefer more control over literal file paths (above), you can instead use glob paths or the
`glob://` protocol. Globs are relative from the workspace root, and will only match directories, or
patterns that end in `template.yml`.

.moon/workspace.yml

```
generator:  templates:    - './templates/*'    # Or    - 'glob://projects/*/templates/*'
```

#### Git repositoriesv1.23.0

Templates locations can also reference templates in an external Git repository using the `git://`
locator protocol. This locator requires the Git host, repository path, and revision (branch, tag,
commit, etc).

.moon/workspace.yml

```
generator:  templates:    - 'git://github.com/moonrepo/templates#master'    - 'git://gitlab.com/org/repo#v1.2.3'
```

Git repositories will be cloned to `~/.moon/templates/git/` using an HTTPS URL (not a Git
URL), and will be cached for future use.

#### npm packagesv1.23.0

Additionally, template locations can also reference npm packages using the `npm://` locator
protocol. This locator requires a package name and published version.

.moon/workspace.yml

```
generator:  templates:    - 'npm://@moonrepo/templates#1.2.3'    - 'npm://other-templates#4.5.6'
```

npm packages will be downloaded and unpacked to `~/.moon/templates/npm` and cached for future use.

### Declaring variables with CLI arguments

During generation, you'll be prompted in the terminal to provide a value for any configured
variables. However, you can pre-fill these variable values by passing arbitrary command line
arguments after `--` to [`moon generate`](/docs/commands/generate). Argument names must exactly match the variable
names.

Using the package template example above, we could pre-fill the `name` variable like so:

```
$ moon generate npm-package ./packages/example -- --name '@company/example' --private
```

info

- Array variables support multiple options of the same name.

- Boolean variables can be negated by prefixing the argument with `--no-`.

- Object variables can not declare values through arguments.

## Sharing templates

Although moon is designed for a monorepo, you may be using multiple repositories and would like to
use the same templates across all of them. So how can we share templates across repositories? Why
not try...

- Git submodules

- Git repositories (using `git://` protocol)

- File archives

- Node.js modules

- npm packages (using `npm://` protocol)

- Another packaging system

Regardless of the choice, simply configure [`generator.templates`](/docs/config/workspace#templates) to point to these
locations:

.moon/workspace.yml

```
generator:  templates:    - './templates'    - 'file://./templates'    # Git    - './path/to/submodule'    - 'git://github.com/org/repo#branch'    # npm    - './node_modules/@company/shared-templates'    - 'npm://@company/shared-templates#1.2.3'
```

### Git and npm layout structure

If you plan to share templates using Git repositories (`git://`) or npm packages (`npm://`), then
the layout of those projects must follow these guidelines:

- A project must support multiple templates

- A template is denoted by a folder in the root of the project

- Each template must have a [`template.yml`](/docs/config/template) file

- Template names are derived from the folder name, or the `id` field in [`template.yml`](/docs/config/template)

An example of this layout structure may look something like the following:

```
├── template-one/│   └── template.yml├── template-two/│   └── template.yml├── template-three/│   └── template.yml└── package.json, etc
```

These templates can then be referenced by name, such as [`moon generate template-one`](/docs/commands/generate).

## /docs/guides/codeowners

Source: https://moonrepo.dev/docs/guides/codeowners

# Code owners

v1.8.0

Code owners enables companies to define individuals, teams, or groups that are responsible for code
in a repository. This is useful in ensuring that pull/merge requests are reviewed and approved by a
specific set of contributors, before the branch is merged into the base branch.

With that being said, moon does not implement a custom code owners solution, and instead builds
upon the popular `CODEOWNERS` integration in VCS providers, like GitHub, GitLab, and Bitbucket.

## Defining owners

With moon, you do not modify a `CODEOWNERS` file directly. Instead you define owners per project
with [`moon.yml`](/docs/config/project), or globally with [`.moon/workspace.yml`](/docs/config/workspace).
These owners are then aggregated and automatically
[synced to a `CODEOWNERS` file](#generating-codeowners).

info

An owner is a user, team, or group unique to your VCS provider. Please refer to your provider's
documentation for the correct format in which to define owners.

### Project-level

For projects, we support an [`owners`](/docs/config/project#owners) setting in
[`moon.yml`](/docs/config/project) that accepts file patterns/paths and their owners (contributors
required to review), as well as operational settings for minimum required approvals, custom groups,
and more.

Paths configured here are relative from the project root, and will be prefixed with the project
source (path from workspace root to project root) when the file is synced.

packages/components/moon.yml

```
owners:  requiredApprovals: 2  paths:    'src/': ['@frontend', '@design-system']    '*.config.js': ['@frontend-infra']    '*.json': ['@frontend-infra']
```

The configuration above would generate the following:

- GitHub
- GitLab
- Bitbucket

.github/CODEOWNERS

```
# components/packages/components/src/ @frontend @design-system/packages/components/*.config.js @frontend-infra/packages/components/*.json @frontend-infra
```

.gitlab/CODEOWNERS

```
# components[components][2]/packages/components/src/ @frontend @design-system/packages/components/*.config.js @frontend-infra/packages/components/*.json @frontend-infra
```

CODEOWNERS

```
# components/packages/components/src/ @frontend @design-system/packages/components/*.config.js @frontend-infra/packages/components/*.json @frontend-infra
```

### Workspace-level

Project scoped owners are great but sometimes you need to define owners for files that span across
all projects, or files at any depth within the repository. With the
[`codeowners.globalPaths`](/docs/config/workspace#globalpaths) setting in
[`.moon/workspace.yml`](/docs/config/workspace), you can do just that.

Paths configured here are used as-is, allowing for full control of what ownership is applied.

.moon/workspace.yml

```
codeowners:  globalPaths:    # All files    '*': ['@admins']    # Config folder at any depth    'config/': ['@app-platform']    # GitHub folder at the root    '/.github/': ['@infra']
```

The configuration above would generate the following at the top of the file (is the same for all
providers):

- GitHub
- GitLab
- Bitbucket

.github/CODEOWNERS

```
# (workspace)* @adminsconfig/ @app-platform/.github/ @infra
```

.gitlab/CODEOWNERS

```
# (workspace)* @adminsconfig/ @app-platform/.github/ @infra
```

CODEOWNERS

```
# (workspace)* @adminsconfig/ @app-platform/.github/ @infra
```

## Generating `CODEOWNERS`

Code owners is an opt-in feature, and as such, the `CODEOWNERS` file can be generated in a few ways.
The first is manually, with the [`moon sync codeowners`](/docs/commands/sync/code-owners) command.

```
$ moon sync codeowners
```

While this works, it is a manual process, and can easily be forgotten, resulting in an out-of-date
file.

An alternative solution is the [`codeowners.syncOnRun`](/docs/config/workspace#synconrun) setting in
[`.moon/workspace.yml`](/docs/config/workspace#codeowners), that when enabled, moon will automatically
generate a `CODEOWNERS` file when a [target](/docs/concepts/target) is ran.

.moon/workspace.yml

```
codeowners:  syncOnRun: true
```

The format and location of the `CODEOWNERS` file is based on the
[`vcs.provider`](/docs/config/workspace#provider) setting.

## FAQ

### What providers or formats are supported?

The following providers are supported, based on the [`vcs.provider`](/docs/config/workspace#provider)
setting.

- [Bitbucket](https://marketplace.atlassian.com/apps/1218598/code-owners-for-bitbucket?tab=overview&hosting=cloud) (via a 3rd-party app)

- [GitHub](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)

- [GitLab](https://docs.gitlab.com/ee/user/project/codeowners/reference.html)

- Other (very basic syntax)

### Where does the `CODEOWNERS` file get created?

The location of the file is dependent on the configured provider.

- GitHub -> `.github/CODEOWNERS`

- GitLab -> `.gitlab/CODEOWNERS`

- Everything else -> `CODEOWNERS`

### Why are owners defined in `moon.yml` and not an alternative like `OWNERS`?

A very popular pattern for defining owners is through an `OWNERS` file, which can appear in any
folder, at any depth, within the repository. All of these files are then aggregated into a single
`CODEOWNERS` file.

While this is useful for viewing ownership of a folder at a glance, it incurs a massive performance
hit as we'd have to constantly glob the entire repository to find all `OWNERS` files. We found it
best to define owners in `moon.yml` instead for the following reasons:

- No performance hit, as we're already loading and parsing these config files.

- Co-locates owners with the rest of moon's configuration.

- Ownership is now a part of the project graph, enabling future features.

## /docs/guides/debug-task

Source: https://moonrepo.dev/docs/guides/debug-task

# Debugging a task

Running [tasks](/docs/concepts/task) is the most common way to interact with moon, so what do you do
when your task isn't working as expected? Diagnose it of course! Diagnosing the root cause of a
broken task can be quite daunting, but do not fret, as the following steps will help guide you in
this endeavor.

## Verify configuration

Before we dive into the internals of moon, we should first verify that the task is actually
configured correctly. Our configuration layer is very strict, but it can't catch everything, so jump
to the [`moon.yml`](/docs/config/project#tasks) documentation for more information.

To start, moon will create a snapshot of the project and its tasks, with all [tokens](/docs/concepts/token)
resolved, and paths expanded. This snapshot is located at
`.moon/cache/states/
/snapshot.json`. With the snapshot open, inspect the root `tasks`
object for any inconsistencies or inaccuracies.

Some issues to look out for:

- Have `command` and `args` been parsed correctly?

- Have [tokens](/docs/concepts/token) resolved correctly? If not, verify syntax or try another token type.

- Have `inputFiles`, `inputGlobs`, and `inputVars` expanded correctly from [`inputs`](/docs/config/project#inputs)?

- Have `outputFiles` and `outputGlobs` expanded correctly from [`outputs`](/docs/config/project#outputs)?

- Is the `toolchain` (formerly `platform`) correct for the command? If incorrect, explicitly set the [`toolchain`](/docs/config/project#toolchain).

- Are `options` and `flags` correct?

info

Resolved information can also be inspected with the [`moon task --json`](/docs/commands/task)
command.

### Verify inherited configuration

If the configuration from the previous step looks correct, you can skip this step, otherwise let's
verify that the inherited configuration is also correct. In the `snapshot.json` file, inspect the
root `inherited` object, which is structured as follows:

- `order` - The order in which configuration files from `.moon` are loaded, from lowest to highest priority, and the order files are merged. The `*` entry is `.moon/tasks/all.yml`, while other entries map to `.moon/tasks/**/*.yml`.

- `layers` - A mapping of configuration files that were loaded, derived from the `order`. Each layer represents a partial object (not expanded or resolved). Only files that exist will be mapped here.

- `config` - A partial configuration object representing the state of all merged layers. This is what is merged with the project's `moon.yml` file.

Some issues to look out for:

- Is the order correct? If not, verify the project's [`language`](/docs/config/project#language) and the task's [`toolchain`](/docs/config/project#toolchain).

- Does `config` correctly represent the merged state of all `layers`? Do note that tasks are shallow merged (by name), not deep merged.

- Have the root `tasks` properly inherited [`implicitDeps`](/docs/config/tasks#implicitdeps), [`implicitInputs`](/docs/config/tasks#implicitinputs), and `fileGroups`?

## Inspect trace logs

If configuration looks good, let's move on to inspecting the trace logs, which can be a non-trivial
amount of effort. Run the task to generate the logs, bypass the cache, and include debug
information:

```
MOON_DEBUG_PROCESS_ENV=true MOON_DEBUG_PROCESS_INPUT=true moon run  --log trace --force
```

Once ran, a large amount of information will be logged to the terminal. However, most of it can be
ignored, as we're only interested in the "is this task affected by changes" logs. This breaks down
as follows:

- First, we gather touched files from the local checkout, which is typically `git status --porcelain --untracked-files` (from the `moon_process::command_inspector` module). The logs do not output the list of files that are touched, but you can run this command locally to verify the output.

- Secondly, we gather all files from the project directory, using the `git ls-files --full-name --cached --modified --others --exclude-standard --deduplicate` command (also from the `moon_process::command_inspector` module). This command can also be ran locally to verify the output.

- Lastly, all files from the previous 2 commands will be hashed using the `git hash-object` command. If you passed the `MOON_DEBUG_PROCESS_INPUT` environment variable, you'll see a massive log entry of all files being hashed. This is what we use to generate moon's specific hash.

If all went well, you should see a log entry that looks like this:

```
moon_task_runner::task_runner  Generated a unique hash  task_target="" hash=""
```

The important piece is the hash, which is a 64-character SHA256 hash, and represents the unique hash
of this task/target. This is what moon uses to determine a cache hit/miss, and whether or not to
skip re-running a task.

Let's copy the hash and move on to the next step.

## Inspect the hash manifest

With the hash in hand, let's dig deeper into moon's internals, by inspecting the hash manifest at
`.moon/cache/hashes/.json`, or running the [`moon hash`](/docs/commands/hash) command:

```
moon hash
```

The manifest is JSON and its contents are all the information used to generate its unique hash. This
information is an array, and breaks down as follows:

- The first item in the array is the task itself. The important fields to diagnose here are `deps` and `inputs`. Dependencies are other tasks (and their hash) that this task depends on.

- Inputs are all the files (and their hash from `git hash-object`) this task requires to run.

- The remaining items are toolchain/language specific, some examples are: Node.js - The current Node.js version and the resolved versions/hashes of all `package.json` dependencies.

- Rust - The current Rust version and the resolved versions/hashes of all `Cargo.toml` dependencies.

- TypeScript - Compiler options for changing compilation output.

Some issues to look out for:

- Do the dependencies match the task's configured [`deps`](/docs/config/project#deps) and [`implicitDeps`](/docs/config/tasks#implicitdeps)?

- Do the inputs match the task's configured [`inputs`](/docs/config/project#inputs) and [`implicitInputs`](/docs/config/tasks#implicitinputs)? If not, try tweaking the config.

- Are the toolchain/language specific items correct?

- Are dependency versions/hashes correctly parsed from the appropriate lockfile?

### Diffing a previous hash

Another avenue for diagnosing a task is to diff the hash against a hash from a previous run. Since
we require multiple hashes, we'll need to run the task multiple times,
[inspect the logs](#inspect-trace-logs), and extract the hash for each. If you receive the same hash
for each run, you'll need to tweak configuration or change files to produce a different hash.

Once you have 2 unique hashes, we can pass them to the [`moon hash`](/docs/commands/hash) command. This
will produce a `git diff` styled output, allowing for simple line-by-line comparison debugging.

```
moon hash
```

```
Left:  0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fddeRight: 2388552fee5a02062d0ef402bdc7232f0a447458b058c80ce9c3d0d4d7cfe171[	{		"command": "build",		"args": [+			"./dist"-			"./build"		],		...	}]
```

This is extremely useful in diagnoising why a task is running differently than before, and is much
easier than inspecting the hash manifest files manually!

## Ask for help

If you've made it this far, and still can't figure out why a task is not working correctly, please
ask for help!

- [Join the Discord community](https://discord.gg/qCh9MEynv2) (if lost)

- [Report an issue](https://github.com/moonrepo/moon/issues/new/choose) (if an actual bug)

## /docs/guides/docker

Source: https://moonrepo.dev/docs/guides/docker

# Docker integration

Using [Docker](https://www.docker.com/) to run your applications? Or build your artifacts? No
worries, moon can be utilized with Docker, and supports a robust integration layer.

success

Looking to speed up your Docker builds? Want to build in the cloud?
[Give Depot a try](https://depot.dev?ref=moonrepo)!

## Requirements

The first requirement, which is very important, is adding `.moon/cache` to the workspace root
`.dockerignore` (moon assumes builds are running from the root). Not all files in `.moon/cache` are
portable across machines/environments, so copying these file into Docker will definitely cause
interoperability issues.

.dockerignore

```
.moon/cache
```

The other requirement depends on how you want to integrate Git with Docker. Since moon executes
`git` commands under the hood, there are some special considerations to be aware of when running
moon within Docker. There's 2 scenarios to choose from:

- (recommended) Add the `.git` folder to `.dockerignore`, so that it's not `COPY`'d. moon will continue to work just fine, albeit with some functionality disabled, like caching.

- Ensure that the `git` library is installed in the container, and copy the `.git` folder with `COPY`. moon will work with full functionality, but it will increase the overall size of the image because of caching.

## Creating a `Dockerfile`

info

Our [`moon docker file`](/docs/commands/docker/file) command can automatically generate a `Dockerfile` based on this
guide! We suggest generating the file then reading the guide below to understand what's going on.

We're very familiar with how tedious `Dockerfile`s are to write and maintain, so in an effort to
reduce this headache, we've built a handful of tools to make this process much easier. With moon,
we'll take advantage of Docker's layer caching and staged builds as much as possible.

With that being said, there's many approaches you can utilize, depending on your workflow (we'll
document them below):

- Running `moon docker` commands before running `docker run|build` commands.

- Running `moon docker` commands within the `Dockerfile`.

- Using multi-staged or non-staged (standard) builds.

- Something else unique to your setup!

warning

This guide and our Docker approach is merely a suggestion and is not a requirement for using moon
with Docker! Feel free to use this as a starting point, or not at all. Choose the approach that
works best for you!

### What we're trying to avoid

Before we dive into writing a perfect `Dockerfile`, we'll briefly talk about the pain points we're
trying to avoid. In the context of Node.js and monorepo's, you may be familiar with having to `COPY`
each individual `package.json` in the monorepo before installing `node_modules`, to effectively use
layer caching. This is very brittle, as each new application or package is created, every
`Dockerfile` in the monorepo will need to be modified to account for this new `package.json`.

Furthermore, we'll have to follow a similar process for only copying source files necessary for
the build or `CMD` to complete. This is very tedious, so most developers simply use `COPY . .` and
forget about it. Copying the entire monorepo is costly, especially as it grows.

As an example, we'll use moon's official repository. The `Dockerfile` would look something like the
following.

```
FROM node:latestWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli# Copy moon filesCOPY ./.moon ./.moon# Copy all package.json's and lockfilesCOPY ./packages/cli/package.json ./packages/cli/package.jsonCOPY ./packages/core-linux-arm64-gnu/package.json ./packages/core-linux-arm64-gnu/package.jsonCOPY ./packages/core-linux-arm64-musl/package.json ./packages/core-linux-arm64-musl/package.jsonCOPY ./packages/core-linux-x64-gnu/package.json ./packages/core-linux-x64-gnu/package.jsonCOPY ./packages/core-linux-x64-musl/package.json ./packages/core-linux-x64-musl/package.jsonCOPY ./packages/core-macos-arm64/package.json ./packages/core-macos-arm64/package.jsonCOPY ./packages/core-macos-x64/package.json ./packages/core-macos-x64/package.jsonCOPY ./packages/core-windows-x64-msvc/package.json ./packages/core-windows-x64-msvc/package.jsonCOPY ./packages/runtime/package.json ./packages/runtime/package.jsonCOPY ./packages/types/package.json ./packages/types/package.jsonCOPY ./package.json ./package.jsonCOPY ./yarn.lock ./yarn.lockCOPY ./.yarn ./.yarnCOPY ./.yarnrc.yml ./yarnrc.yml# Install toolchain and dependencies# In non-moon repos: yarn installRUN moon docker setup# Copy project and required files# Or COPY . .COPY ./packages/types ./packages/typesCOPY ./packages/runtime ./packages/runtime# Build the targetRUN moon run runtime:build
```

For such a small monorepo, this already looks too confusing!!! Let's remedy this by utilizing moon
itself to the fullest!

### Scaffolding the bare minimum

The first step in this process is to only copy the bare minimum of files necessary for installing
dependencies (Node.js modules, etc). This is typically manifests (`package.json`), lockfiles
(`yarn.lock`, etc), and any configuration (`.yarnrc.yml`, etc).

This can all be achieved with the [`moon docker scaffold`](/docs/commands/docker/scaffold) command, which scaffolds a
skeleton of the repository structure, with only necessary files (the above). Let's update our
`Dockerfile` usage.

- Non-staged
- Multi-staged

This assumes [`moon docker scaffold `](/docs/commands/docker/scaffold) is ran outside of the `Dockerfile`.

```
FROM node:latestWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli# Copy workspace skeletonCOPY ./.moon/docker/workspace .# Install toolchain and dependenciesRUN moon docker setup
```

```
#### BASEFROM node:latest AS baseWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli#### SKELETONFROM base AS skeleton# Copy entire repository and scaffoldCOPY . .RUN moon docker scaffold
#### BUILDFROM base AS build# Copy workspace skeletonCOPY --from=skeleton /app/.moon/docker/workspace .# Install toolchain and dependenciesRUN moon docker setup
```

And with this, our dependencies will be layer cached effectively! Let's now move onto copying source
files.

### Copying necessary source files

The next step is to copy all source files necessary for `CMD` or any `RUN` commands to execute
correctly. This typically requires copying all source files for the project and all source files
of the project's dependencies... NOT the entire repository!

Luckily our [`moon docker scaffold `](/docs/commands/docker/scaffold) command has already done this for us! Let's
continue updating our `Dockerfile` to account for this, by appending the following:

- Non-staged
- Multi-staged

```
# Copy source filesCOPY ./.moon/docker/sources .# Build something (optional)RUN moon run
:
```

```
# Copy source filesCOPY --from=skeleton /app/.moon/docker/sources .# Build something (optional)RUN moon run
:
```

info

If you need to copy additional files for your commands to run successfully, you can configure the
`docker.scaffold.include` setting in [`.moon/workspace.yaml`](/docs/config/workspace#scaffold) (entire
workspace) or [`moon.yml`](/docs/config/project#scaffold) (per project).

### Pruning extraneous files

Now that we've ran a command or built an artifact, we should prune the Docker environment to remove
unneeded files and folders. We can do this with the [`moon docker prune`](/docs/commands/docker/prune) command, which
must be ran within the context of a `Dockerfile`!

```
# Prune workspaceRUN moon docker prune
```

When ran, this command will do the following, in order:

- Remove extraneous dependencies (`node_modules`) for unfocused projects.

- Install production only dependencies for the projects that were scaffolded.

info

This process can be customized using the `docker.prune` setting in
[`.moon/workspace.yaml`](/docs/config/workspace#prune).

### Final result

And with this moon integration, we've reduced the original `Dockerfile` of 35 lines to 18 lines, a
reduction of almost 50%. The original file can also be seen as `O(n)`, as each new manifest requires
cascading updates, while the moon approach is `O(1)`!

- Non-staged
- Multi-staged

```
FROM node:latestWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli# Copy workspace skeletonCOPY ./.moon/docker/workspace .# Install toolchain and dependenciesRUN moon docker setup# Copy source filesCOPY ./.moon/docker/sources .# Build something (optional)RUN moon run
:# Prune workspaceRUN moon docker prune# CMD
```

```
#### BASEFROM node:latest AS baseWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli#### SKELETONFROM base AS skeleton# Copy entire repository and scaffoldCOPY . .RUN moon docker scaffold
#### BUILDFROM base AS build# Copy workspace skeletonCOPY --from=skeleton /app/.moon/docker/workspace .# Install toolchain and dependenciesRUN moon docker setup# Copy source filesCOPY --from=skeleton /app/.moon/docker/sources .# Build something (optional)RUN moon run
:# Prune workspaceRUN moon docker prune# CMD
```

## Running `docker` commands

When running `docker` commands, they must be ran from moon's workspace root (typically the
repository root) so that the project graph and all `moon docker` commands resolve correctly.

```
docker build .
```

If you're `Dockerfile`s are located within each applicable project, use the `-f` argument.

```
docker run -f ./apps/client/Dockerfile .
```

## Troubleshooting

### Supporting `node:alpine` images

If you're trying to use the `node:alpine` image with moon's
[integrated toolchain](/docs/concepts/toolchain), you'll need to set the `MOON_TOOLCHAIN_FORCE_GLOBALS`
environment variable in the Docker image to disable moon's toolchain. This is required as Node.js
does not provide pre-built binaries for the Alpine target, so installing the Node.js toolchain will
fail.

```
FROM node:alpineENV MOON_TOOLCHAIN_FORCE_GLOBALS=true
```

## /docs/guides/examples/angular

Source: https://moonrepo.dev/docs/guides/examples/angular

# Angular example

In this guide, you'll learn how to integrate [Angular](https://angular.io/) into moon.

Begin by creating a new Angular project in the root of an existing moon project (this should not be
created in the workspace root, unless a polyrepo).

```
cd apps && npx -p @angular/cli@latest ng new angular-app
```

View the [official Angular docs](https://angular.io/start) for a more in-depth guide to getting
started!

## Setup

Since Angular is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

/moon.yml

```
fileGroups:  app:    - 'src/**/*'    - 'angular.*'tasks:  dev:    command: 'ng serve'    preset: 'server'  build:    command: 'ng build'    inputs:      - '@group(app)'      - '@group(sources)'    outputs:      - 'dist'  # Extends the top-level lint  lint:    args:      - '--ext'      - '.ts'
```

### ESLint integration

Angular does not provide a built-in linting abstraction, but instead there is an
[ESLint package](https://github.com/angular-eslint/angular-eslint), which is great, but complicates
things a bit. Because of this, you have two options for moving forward:

- Use a [global `lint` task](/docs/guides/examples/eslint) and bypass Angular's solution (preferred).

- Use Angular's ESLint package solution only.

Regardless of which option is chosen, the following changes are applicable to all options and should
be made. Begin be installing the dependencies that the
[`@angular-eslint`](https://nextjs.org/docs/basic-features/eslint#eslint-config) package need in the
application's `package.json`.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add --dev @angular-eslint/builder @angular-eslint/eslint-plugin @angular-eslint/eslint-plugin-template @angular-eslint/schematics @angular-eslint/template-parser
```

```
yarn workspace
 add --dev @angular-eslint/builder @angular-eslint/eslint-plugin @angular-eslint/eslint-plugin-template @angular-eslint/schematics @angular-eslint/template-parser
```

```
npm install --save-dev --workspace
 @angular-eslint/builder @angular-eslint/eslint-plugin @angular-eslint/eslint-plugin-template @angular-eslint/schematics @angular-eslint/template-parser
```

```
pnpm add --save-dev --filter
 @angular-eslint/builder @angular-eslint/eslint-plugin @angular-eslint/eslint-plugin-template @angular-eslint/schematics @angular-eslint/template-parser
```

```
bun install --dev @angular-eslint/builder @angular-eslint/eslint-plugin @angular-eslint/eslint-plugin-template @angular-eslint/schematics @angular-eslint/template-parser
```

Since Angular has some specific rules, we'll need to tell the ESLint package to overrides the
default ones. This can be achieved with a project-level `.eslintrc.json` file.

/.eslintrc.json

```
{  "root": true,  "ignorePatterns": ["projects/**/*"],  "overrides": [    {      "files": ["*.ts"],      "extends": [        "eslint:recommended",        "plugin:@typescript-eslint/recommended",        "plugin:@angular-eslint/recommended",        // This is required if you use inline templates in Components        "plugin:@angular-eslint/template/process-inline-templates"      ],      "rules": {        /**         * Any TypeScript source code (NOT TEMPLATE) related rules you wish to use/reconfigure over and above the         * recommended set provided by the @angular-eslint project would go here.         */        "@angular-eslint/directive-selector": [          "error",          { "type": "attribute", "prefix": "app", "style": "camelCase" }        ],        "@angular-eslint/component-selector": [          "error",          { "type": "element", "prefix": "app", "style": "kebab-case" }        ]      }    },    {      "files": ["*.html"],      "extends": [        "plugin:@angular-eslint/template/recommended",        "plugin:@angular-eslint/template/accessibility"      ],      "rules": {        /**         * Any template/HTML related rules you wish to use/reconfigure over and above the         * recommended set provided by the @angular-eslint project would go here.         */      }    }  ]}
```

With the basics now setup, choose the option that works best for you.

- Global lint
- Angular lint

We encourage using the global `lint` task for consistency across all projects within the repository.
With this approach, the `eslint` command itself will be ran and the `ng lint` command will be
ignored, but the `@angular-eslint` rules will still be used.

If you'd prefer to use the `ng lint` command, add it as a task to the project's
[`moon.yml`](/docs/config/project).

/moon.yml

```
tasks:  lint:    command: 'ng lint'    inputs:      - '@group(angular)'
```

Furthermore, if a global `lint` task exists, be sure to exclude it from being inherited.

/moon.yml

```
workspace:  inheritedTasks:    exclude: ['lint']
```

In addition to configuring `moon.yml`, you also need to add a lint target in the `angular.json` file
for linting to work properly. The lint target specifies which builder to use for linting, as well as
the file patterns that should be linted.

/angular.json

```
{  "projects": {    "angular-app": {      "architect": {        "lint": {          "builder": "@angular-eslint/builder:lint",          "options": {            "lintFilePatterns": ["src/**/*.ts", "src/**/*.html"]          }        }      }    }  }}
```

Adding this lint target is crucial for ensuring that the linting process is properly configured and
integrated with Angular's build system.

### TypeScript integration

Angular has [built-in support for TypeScript](https://angular.io/guide/typescript-configuration), so
there is no need for additional configuration to enable TypeScript support.

At this point we'll assume that a `tsconfig.json` has been created in the application, and
typechecking works. From here we suggest utilizing a [global `typecheck` task](/docs/guides/examples/typescript) for
consistency across all projects within the repository.

## Configuration

### Root-level

We suggest against root-level configuration, as Angular should be installed per-project, and the
`ng` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Angular project, a [`angular.json`](https://angular.io/guide/workspace-config)
is created, and must exist in the project root. This allows each project to configure Angular for
their needs.

/angular.json

```
{  "$schema": "./node_modules/@angular/cli/lib/config/schema.json",  "version": 1,  "projects": {    "angular-app": {      "projectType": "application",      ...    }  },  ...}
```

## /docs/guides/examples/astro

Source: https://moonrepo.dev/docs/guides/examples/astro

# Astro example

In this guide, you'll learn how to integrate [Astro](https://docs.astro.build).

Begin by creating a new Astro project in the root of an existing moon project (this should not be
created in the workspace root, unless a polyrepo).

```
cd apps && npm create astro@latest
```

## Setup

Since Astro is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Astro tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/astro).

/moon.yml

```
# Inherit tasks from the `astro` preset# https://github.com/moonrepo/moon-configstags: ['astro']# Disable project referencestoolchain:  typescript:    syncProjectReferences: false
```

### ESLint integration

When using a [`lint`](/docs/guides/examples/eslint) task, the
[`eslint-plugin-astro`](https://ota-meshi.github.io/eslint-plugin-astro/user-guide/) package must be
installed to lint `.astro` files.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace  add --dev eslint-plugin-astro
```

```
yarn workspace  add --dev eslint-plugin-astro
```

```
npm install --save-dev --workspace  eslint-plugin-astro
```

```
pnpm add --save-dev --filter  eslint-plugin-astro
```

```
bun install --dev eslint-plugin-astro
```

Once the dependency has been installed in the application's `package.json`. We can then enable this
configuration by creating an `.eslintrc.js` file in the project root. Be sure this file is listed in
your lint task's inputs!

/.eslintrc.js

```
module.exports = {  extends: ['plugin:astro/recommended'],  overrides: [    {      files: ['*.astro'],      parser: 'astro-eslint-parser',      // If using TypeScript      parserOptions: {        parser: '@typescript-eslint/parser',        extraFileExtensions: ['.astro'],        project: 'tsconfig.json',        tsconfigRootDir: __dirname,      },    },  ],};
```

And lastly, when linting through moon's command line, you'll need to include the `.astro` extension
within the `lint` task. This can be done by extending the top-level task within the project (below),
or by adding it to the top-level entirely.

/moon.yml

```
tasks:  lint:    args:      - '--ext'      - '.ts,.tsx,.astro'
```

### Prettier integration

When using a [`format`](/docs/guides/examples/prettier) task, the `prettier-plugin-astro` package must be installed to
format `.astro` files. View the official
[Astro docs](https://docs.astro.build/en/editor-setup/#prettier) for more information.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace  add --dev prettier-plugin-astro
```

```
yarn workspace  add --dev prettier-plugin-astro
```

```
npm install --save-dev --workspace  prettier-plugin-astro
```

```
pnpm add --save-dev --filter  prettier-plugin-astro
```

```
bun install --dev prettier-plugin-astro
```

### TypeScript integration

Since Astro utilizes custom `.astro` files, it requires a specialized TypeScript integration, and
luckily Astro provides an [in-depth guide](https://docs.astro.build/en/guides/typescript/). With
that being said, we do have a few requirements and pointers!

- Use the official [Astro `tsconfig.json`](https://docs.astro.build/en/guides/typescript/#setup) as a basis.

- From our internal testing, the `astro check` command (that typechecks `.astro` files) does not support project references. If the `composite` compiler option is enabled, the checker will fail to find `.astro` files. To work around this, we disable `workspace.typescript` in our moon config above.

- Since typechecking requires 2 commands, one for `.astro` files, and the other for `.ts`, `.tsx` files, we've added the [`typecheck`](/docs/guides/examples/typescript) task as a dependency for the `check` task. This will run both commands through a single task!

## Configuration

### Root-level

We suggest against root-level configuration, as Astro should be installed per-project, and the
`astro` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Astro project, a
[`astro.config.mjs`](https://docs.astro.build/en/reference/configuration-reference/) is created, and
must exist in the project root. This allows each project to configure Astro for their needs.

/astro.config.mjs

```
import { defineConfig } from 'astro/config';// https://astro.build/configexport default defineConfig({});
```

## /docs/guides/examples/eslint

Source: https://moonrepo.dev/docs/guides/examples/eslint

# ESLint example

In this guide, you'll learn how to integrate [ESLint](https://eslint.org/) into moon.

Begin by installing `eslint` and any plugins in your root. We suggest using the same version across
the entire repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev eslint eslint-config-moon
```

```
yarn add --dev eslint eslint-config-moon# If using workspacesyarn add --dev -W eslint eslint-config-moon
```

```
npm install --save-dev eslint eslint-config-moon
```

```
pnpm add --save-dev eslint eslint-config-moon# If using workspacespnpm add --save-dev -w eslint eslint-config-moon
```

```
bun install --dev eslint eslint-config-moon
```

## Setup

Since linting is a universal workflow, add a `lint` task to
[`.moon/tasks/node.yml`](/docs/config/tasks) with the following parameters.

.moon/tasks/node.yml

```
tasks:  lint:    command:      - 'eslint'      # Support other extensions      - '--ext'      - '.js,.jsx,.ts,.tsx'      # Always fix and run extra checks      - '--fix'      - '--report-unused-disable-directives'      # Dont fail if a project has nothing to lint      - '--no-error-on-unmatched-pattern'      # Do fail if we encounter a fatal error      - '--exit-on-fatal-error'      # Only 1 ignore file is supported, so use the root      - '--ignore-path'      - '@in(4)'      # Run in current dir      - '.'    inputs:      # Source and test files      - 'src/**/*'      - 'tests/**/*'      # Other config files      - '*.config.*'      # Project configs, any format, any depth      - '**/.eslintrc.*'      # Root configs, any format      - '/.eslintignore'      - '/.eslintrc.*'
```

Projects can extend this task and provide additional parameters if need be, for example.

/moon.yml

```
tasks:  lint:    args:      # Enable caching for this project      - '--cache'
```

### TypeScript integration

If you're using the [`@typescript-eslint`](https://typescript-eslint.io) packages, and want to
enable type-safety based lint rules, we suggest something similar to the official
[monorepo configuration](https://typescript-eslint.io/docs/linting/monorepo).

Create a `tsconfig.eslint.json` in your repository root, extend your shared compiler options (we use
[`tsconfig.options.json`](/docs/guides/examples/typescript)), and include all your project files.

tsconfig.eslint.json

```
{  "extends": "./tsconfig.options.json",  "compilerOptions": {    "emitDeclarationOnly": false,    "noEmit": true  },  "include": ["apps/**/*", "packages/**/*"]}
```

Append the following inputs to your `lint` task.

.moon/tasks/node.yml

```
tasks:  lint:    # ...    inputs:      # TypeScript support      - 'types/**/*'      - 'tsconfig.json'      - '/tsconfig.eslint.json'      - '/tsconfig.options.json'
```

And lastly, add `parserOptions` to your [root-level config](#root-level).

## Configuration

### Root-level

The root-level ESLint config is required, as ESLint traverses upwards from each file to find
configurations, and this denotes the stopping point. It's also used to define rules for the entire
repository.

.eslintrc.js

```
module.exports = {  root: true, // Required!  extends: ['moon'],  rules: {    'no-console': 'error',  },  // TypeScript support  parser: '@typescript-eslint/parser',  parserOptions: {    project: 'tsconfig.eslint.json',    tsconfigRootDir: __dirname,  },};
```

The `.eslintignore` file must also be defined at the root, as
[only 1 ignore file](https://eslint.org/docs/user-guide/configuring/ignoring-code#the-eslintignore-file)
can exist in a repository. We ensure this ignore file is used by passing `--ignore-path` above.

.eslintignore

```
node_modules/*.min.js*.map*.snap
```

### Project-level

A project-level ESLint config can be utilized by creating a `.eslintrc.` in the
project root. This is optional, but necessary when defining rules and ignore patterns unique to the
project.

/.eslintrc.js

```
module.exports = {  // Patterns to ignore (alongside the root .eslintignore)  ignorePatterns: ['build', 'lib'],  // Project specific rules  rules: {    'no-console': 'off',  },};
```

The
[`extends`](https://eslint.org/docs/user-guide/configuring/configuration-files#extending-configuration-files)
setting should not extend the root-level config, as ESLint will automatically merge configs
while traversing upwards!

### Sharing

To share configuration across projects, you have 3 options:

- Define settings in the [root-level config](#root-level). This only applies to the parent repository.

- Create and publish an [`eslint-config`](https://eslint.org/docs/developer-guide/shareable-configs#using-a-shareable-config) or [`eslint-plugin`](https://eslint.org/docs/developer-guide/working-with-plugins) npm package. This can be used in any repository.

- A combination of 1 and 2.

For options 2 and 3, if you're utilizing package workspaces, create a local package with the
following content.

packages/eslint-config-company/index.js

```
module.exports = {  extends: ['airbnb'],};
```

Within your root-level ESLint config, you can extend this package to inherit the settings.

.eslintrc.js

```
module.exports = {  extends: 'eslint-config-company',};
```

When using this approach, the package must be built and symlinked into `node_modules` before the
linter will run correctly. Take this into account when going down this path!

## FAQ

### How to lint a single file or folder?

Unfortunately, this isn't currently possible, as the `eslint` binary itself requires a file or
folder path to operate on, and in the task above we pass `.` (current directory). If this was not
passed, then nothing would be linted.

This has the unintended side-effect of not being able to filter down lintable targets by passing
arbitrary file paths. This is something we hope to resolve in the future.

To work around this limitation, you can create another lint task.

### Should we use `overrides`?

Projects should define their own rules using an ESLint config in their project root. However, if you
want to avoid touching many ESLint configs (think migrations), then
[overrides in the root](https://eslint.org/docs/user-guide/configuring/configuration-files#configuration-based-on-glob-patterns)
are a viable option. Otherwise, we highly encourage project-level configs.

.eslintrc.js

```
module.exports = {  // ...  overrides: [    // Only apply to apps "foo" and "bar", but not others    {      files: ['apps/foo/**/*', 'apps/bar/**/*'],      rules: {        'no-magic-numbers': 'off',      },    },  ],};
```

## /docs/guides/examples/jest

Source: https://moonrepo.dev/docs/guides/examples/jest

# Jest example

In this guide, you'll learn how to integrate [Jest](https://jestjs.io/) into moon.

Begin by installing `jest` in your root. We suggest using the same version across the entire
repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev jest
```

```
yarn add --dev jest# If using workspacesyarn add --dev -W jest
```

```
npm install --save-dev jest
```

```
pnpm add --save-dev jest# If using workspacespnpm add --save-dev -w jest
```

```
bun install --dev jest
```

## Setup

Since testing is a universal workflow, add a `test` task to
[`.moon/tasks/node.yml`](/docs/config/tasks) with the following parameters.

.moon/tasks/node.yml

```
tasks:  test:    command:      - 'jest'      # Always run code coverage      - '--coverage'      # Dont fail if a project has no tests      - '--passWithNoTests'    inputs:      # Source and test files      - 'src/**/*'      - 'tests/**/*'      # Project configs, any format      - 'jest.config.*'
```

Projects can extend this task and provide additional parameters if need be, for example.

/moon.yml

```
tasks:  test:    args:      # Disable caching for this project      - '--no-cache'
```

## Configuration

### Root-level

A root-level Jest config is not required and should be avoided, instead, use a [preset](#sharing) to
share configuration.

### Project-level

A project-level Jest config can be utilized by creating a `jest.config.` in the
project root. This is optional, but necessary when defining project specific settings.

/jest.config.js

```
module.exports = {  // Project specific settings  testEnvironment: 'node',};
```

### Sharing

To share configuration across projects, you can utilize Jest's built-in
[`preset`](https://jestjs.io/docs/configuration#preset-string) functionality. If you're utilizing
package workspaces, create a local package with the following content, otherwise publish the npm
package for consumption.

packages/company-jest-preset/jest-preset.js

```
module.exports = {  testEnvironment: 'jsdom',  watchman: true,};
```

Within your project-level Jest config, you can extend the preset to inherit the settings.

/jest.config.js

```
module.exports = {  preset: 'company-jest-preset',};
```

You can take this a step further by passing the `--preset` option in the [task above](#setup), so
that all projects inherit the preset by default.

## FAQ

### How to test a single file or folder?

You can filter tests by passing a file name, folder name, glob, or regex pattern after `--`. Any
passed files are relative from the project's root, regardless of where the `moon` command is being
ran.

```
$ moon run
:test -- filename
```

### How to use `projects`?

With moon, there's no reason to use
[`projects`](https://jestjs.io/docs/configuration#projects-arraystring--projectconfig) as the `test`
task is ran per project. If you'd like to test multiple projects, use
[`moon run :test`](/docs/commands/run).

## /docs/guides/examples/nest

Source: https://moonrepo.dev/docs/guides/examples/nest

# Nest example

In this guide, you'll learn how to integrate [NestJS](https://nestjs.com/) into moon.

Begin by creating a new NestJS project in the root of an existing moon project (this should not be
created in the workspace root, unless a polyrepo).

```
npx @nestjs/cli@latest new nestjs-app --skip-git
```

View the [official NestJS docs](https://docs.nestjs.com/first-steps) for a more in-depth guide to
getting started!

## Setup

Since NestJS is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

/moon.yml

```
layer: 'application'fileGroups:  app:    - 'nest-cli.*'tasks:  dev:    command: 'nest start --watch'    preset: 'server'  build:    command: 'nest build'    inputs:      - '@group(app)'      - '@group(sources)'
```

### TypeScript integration

NestJS has [built-in support for TypeScript](https://NestJS.io/guide/typescript-configuration), so
there is no need for additional configuration to enable TypeScript support.

At this point we'll assume that a `tsconfig.json` has been created in the application, and
typechecking works. From here we suggest utilizing a [global `typecheck` task](/docs/guides/examples/typescript) for
consistency across all projects within the repository.

## Configuration

### Root-level

We suggest against root-level configuration, as NestJS should be installed per-project, and the
`nest` command expects the configuration to live relative to the project root.

### Project-level

When creating a new NestJS project, a [`nest-cli.json`](https://docs.nestjs.com/cli/monorepo) is
created, and must exist in the project root. This allows each project to configure NestJS for
their needs.

/nest-cli.json

```
{  "$schema": "https://json.schemastore.org/nest-cli",  "collection": "@nestjs/schematics",  "type": "application",  "root": "./",  "sourceRoot": "src",  "compilerOptions": {    "tsConfigPath": "tsconfig.build.json"  }}
```

## /docs/guides/examples/next

Source: https://moonrepo.dev/docs/guides/examples/next

# Next example

In this guide, you'll learn how to integrate [Next.js](https://nextjs.org) into moon.

Begin by creating a new Next.js project at a specified folder path (this should not be created in
the workspace root, unless a polyrepo).

```
cd apps && npx create-next-app
 --typescript
```

View the [official Next.js docs](https://nextjs.org/learn/basics/create-nextjs-app/setup) for a
more in-depth guide to getting started!

## Setup

Since Next.js is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Next.js tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/next).

/moon.yml

```
# Inherit tasks from the `next` preset# https://github.com/moonrepo/moon-configstags: ['next']
```

### ESLint integration

Next.js has [built-in support for ESLint](https://nextjs.org/docs/basic-features/eslint), which is
great, but complicates things a bit. Because of this, you have two options for moving forward:

- Use a [global `lint` task](/docs/guides/examples/eslint) and bypass Next.js's solution (preferred).

- Use Next.js's solution only.

Regardless of which option is chosen, the following changes are applicable to all options and should
be made. Begin be installing the
[`eslint-config-next`](https://nextjs.org/docs/basic-features/eslint#eslint-config) dependency in
the application's `package.json`.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add --dev eslint-config-next
```

```
yarn workspace
 add --dev eslint-config-next
```

```
npm install --save-dev --workspace
 eslint-config-next
```

```
pnpm add --save-dev --filter
 eslint-config-next
```

```
bun install --dev eslint-config-next
```

Since the Next.js app is located within a subfolder, we'll need to tell the ESLint plugin where to
locate it. This can be achieved with a project-level `.eslintrc.js` file.

/.eslintrc.js

```
module.exports = {  extends: 'next', // or 'next/core-web-vitals'  settings: {    next: {      rootDir: __dirname,    },  },};
```

With the basics now setup, choose the option that works best for you.

- Global lint
- Next.js lint

We encourage using the global `lint` task for consistency across all projects within the repository.
With this approach, the `eslint` command itself will be ran and the `next lint` command will be
ignored, but the `eslint-config-next` rules will still be used.

Additionally, we suggest disabling the linter during the build process, but is not a requirement. As
a potential alternative, add the `lint` task as a dependency for the `build` task.

/next.config.js

```
module.exports = {  eslint: {    ignoreDuringBuilds: true,  },};
```

If you'd prefer to use the `next lint` command, add it as a task to the project's
[`moon.yml`](/docs/config/project).

/moon.yml

```
tasks:  lint:    command: 'next lint'    inputs:      - '@group(next)'
```

Furthermore, if a global `lint` task exists, be sure to exclude it from being inherited.

/moon.yml

```
workspace:  inheritedTasks:    exclude: ['lint']
```

### TypeScript integration

Next.js also has
[built-in support for TypeScript](https://nextjs.org/docs/basic-features/typescript), but has
similar caveats to the [ESLint integration](#eslint-integration). TypeScript itself is a bit
involved, so we suggest reading the official Next.js documentation before continuing.

At this point we'll assume that a `tsconfig.json` has been created in the application, and
typechecking works. From here we suggest utilizing a [global `typecheck` task](/docs/guides/examples/typescript) for
consistency across all projects within the repository.

Additionally, we suggest disabling the typechecker during the build process, but is not a
requirement. As a potential alternative, add the `typecheck` task as a dependency for the `build`
task.

/next.config.js

```
module.exports = {  typescript: {    ignoreBuildErrors: true,  },};
```

## Configuration

### Root-level

We suggest against root-level configuration, as Next.js should be installed per-project, and the
`next` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Next.js project, a
[`next.config.`](https://nextjs.org/docs/api-reference/next.config.js/introduction) is
created, and must exist in the project root. This allows each project to configure Next.js for
their needs.

/next.config.js

```
module.exports = {  compress: true,};
```

## /docs/guides/examples/nuxt

Source: https://moonrepo.dev/docs/guides/examples/nuxt

# Nuxt example

In this guide, you'll learn how to integrate [Nuxt v3](https://nuxt.com), a [Vue](/docs/guides/examples/vue) framework,
into moon.

Begin by creating a new Nuxt project at a specified folder path (this should not be created in the
workspace root, unless a polyrepo).

```
cd apps && npx nuxi init

```

View the [official Nuxt docs](https://nuxt.com/docs/getting-started/installation) for a more
in-depth guide to getting started!

## Setup

Since Nuxt is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

/moon.yml

```
fileGroups:  nuxt:    - 'assets/**/*'    - 'components/**/*'    - 'composables/**/*'    - 'content/**/*'    - 'layouts/**/*'    - 'middleware/**/*'    - 'pages/**/*'    - 'plugins/**/*'    - 'public/**/*'    - 'server/**/*'    - 'utils/**/*'    - '.nuxtignore'    - 'app.config.*'    - 'app.vue'    - 'nuxt.config.*'tasks:  nuxt:    command: 'nuxt'    preset: 'server'  # Production build  build:    command: 'nuxt build'    inputs:      - '@group(nuxt)'    outputs:      - '.nuxt'      - '.output'  # Development server  dev:    command: 'nuxt dev'    preset: 'server'  # Preview production build locally  preview:    command: 'nuxt preview'    deps:      - '~:build'    preset: 'server'
```

Be sure to keep the `postinstall` script in your project's `package.json`.

/package.json

```
{  // ...  "scripts": {    "postinstall": "nuxt prepare"  }}
```

### ESLint integration

Refer to our [Vue documentation](/docs/guides/examples/vue#eslint-integration) for more information on linting.

### TypeScript integration

Nuxt requires `vue-tsc` for typechecking, so refer to our
[Vue documentation](/docs/guides/examples/vue#typescript-integration) for more information.

## Configuration

### Root-level

We suggest against root-level configuration, as Nuxt should be installed per-project, and the
`nuxt` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Nuxt project, a
[`nuxt.config.ts`](https://v3.nuxtjs.org/api/configuration/nuxt-config) is created, and must exist
in the project root. This allows each project to configure Next.js for their needs.

/nuxt.config.ts

```
export default defineNuxtConfig({});
```

## Testing

Nuxt supports testing through [Jest](https://jestjs.io/) or [Vitest](https://vitest.dev/). Refer to
our [Jest documentation](/docs/guides/examples/jest) or [Vitest documentation](/docs/guides/examples/vite) for more information on testing.

## /docs/guides/examples/packemon

Source: https://moonrepo.dev/docs/guides/examples/packemon

# Packemon example

In this guide, you'll learn how to integrate [Packemon](https://packemon.dev/) into moon. Packemon
is a tool for properly building npm packages for distribution, it does this by providing the
following functionality:

- Compiles source code to popular formats: CJS, MJS, ESM, UMD, etc.

- Validates the `package.json` for incorrect fields or values.

- Generates `exports` mappings for `package.json` based on the define configuration.

- And many more [optimizations and features](https://packemon.dev/docs/features)!

Begin by installing `packemon` in your root. We suggest using the same version across the entire
repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev packemon
```

```
yarn add --dev packemon# If using workspacesyarn add --dev -W packemon
```

```
npm install --save-dev packemon
```

```
pnpm add --save-dev packemon# If using workspacespnpm add --save-dev -w packemon
```

```
bun install --dev packemon
```

## Setup

Since Packemon is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Packemon tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/packemon).

/moon.yml

```
# Inherit tasks from the `packemon` preset# https://github.com/moonrepo/moon-configstags: ['packemon']# Set the output formatstasks:  build:    outputs:      - 'cjs'
```

### TypeScript integration

Packemon has built-in support for TypeScript, but to not conflict with a
[typecheck task](/docs/guides/examples/typescript), a separate `tsconfig.json` file is required, which is named
`tsconfig..json`.

This config is necessary to only compile source files, and to not include unwanted files in the
declaration output directory.

tsconfig.esm.json

```
{  "extends": "../../tsconfig.options.json",  "compilerOptions": {    "outDir": "esm",    "rootDir": "src"  },  "include": ["src/**/*"],  "references": []}
```

### Build targets

To configure the target platform(s) and format(s), you must define a
[`packemon` block](https://packemon.dev/docs/config) in the project's `package.json`. The chosen
formats must also be listed as `outputs` in the task.

package.json

```
{  "name": "package",  // ...  "packemon": {    "format": "esm",    "platform": "browser"  }}
```

## /docs/guides/examples/prettier

Source: https://moonrepo.dev/docs/guides/examples/prettier

# Prettier example

In this guide, you'll learn how to integrate [Prettier](https://prettier.io/) into moon.

Begin by installing `prettier` in your root. We suggest using the same version across the entire
repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev prettier
```

```
yarn add --dev prettier# If using workspacesyarn add --dev -W prettier
```

```
npm install --save-dev prettier
```

```
pnpm add --save-dev prettier# If using workspacespnpm add --save-dev -w prettier
```

```
bun install --dev prettier
```

## Setup

Since code formatting is a universal workflow, add a `format` task to
[`.moon/tasks/node.yml`](/docs/config/tasks) with the following parameters.

.moon/tasks/node.yml

```
tasks:  format:    command:      - 'prettier'      # Use the same config for the entire repo      - '--config'      - '@in(4)'      # Use the same ignore patterns as well      - '--ignore-path'      - '@in(3)'      # Fail for unformatted code      - '--check'      # Run in current dir      - '.'    inputs:      # Source and test files      - 'src/**/*'      - 'tests/**/*'      # Config and other files      - '**/*.{md,mdx,yml,yaml,json}'      # Root configs, any format      - '/.prettierignore'      - '/.prettierrc.*'
```

## Configuration

### Root-level

The root-level Prettier config is required, as it defines conventions and standards to apply to
the entire repository.

.prettierrc.js

```
module.exports = {  arrowParens: 'always',  semi: true,  singleQuote: true,  tabWidth: 2,  trailingComma: 'all',  useTabs: true,};
```

The `.prettierignore` file must also be defined at the root, as
[only 1 ignore file](https://prettier.io/docs/en/ignore.html#ignoring-files-prettierignore) can
exist in a repository. We ensure this ignore file is used by passing `--ignore-path` above.

.prettierignore

```
node_modules/*.min.js*.map*.snap
```

### Project-level

We suggest against project-level configurations, as the entire repository should be formatted
using the same standards. However, if you're migrating code and need an escape hatch,
[overrides in the root](https://prettier.io/docs/en/configuration.html#configuration-overrides) will
work.

## FAQ

### How to use `--write`?

Unfortunately, this isn't currently possible, as the `prettier` binary itself requires either the
`--check` or `--write` options, and since we're configuring `--check` in the task above, that takes
precedence. This is also the preferred pattern as checks will run (and fail) in CI.

To work around this limitation, we suggest the following alternatives:

- Configure your editor to run Prettier on save.

- Define another task to write the formatted code, like `format-write`.

## /docs/guides/examples/react

Source: https://moonrepo.dev/docs/guides/examples/react

# React example

React is an application or library concern, and not a build system one, since the bundling of React
is abstracted away through another tool like webpack. Because of this, moon has no guidelines around
utilizing React directly. You can use React however you wish!

However, with that being said, we do suggest the following:

- Add `react` and related dependencies to each project, not the root. This includes `@types/react` as well. This will ensure accurate [hashing](/docs/concepts/cache#hashing).

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add react
```

```
yarn workspace
 add react
```

```
npm install --workspace
 react
```

```
pnpm add --filter
 react
```

```
bun install react
```

- Configure Babel with the `@babel/preset-react` preset.

- Configure [TypeScript](/docs/guides/examples/typescript) compiler options with `"jsx": "react-jsx"`.

## /docs/guides/examples/remix

Source: https://moonrepo.dev/docs/guides/examples/remix

# Remix example

In this guide, you'll learn how to integrate [Remix](https://remix.run) into moon.

Begin by creating a new Remix project at a specified folder path (this should not be created in the
workspace root, unless a polyrepo).

```
cd apps && npx create-remix
```

During this installation, Remix will ask a handful of questions, but be sure to answer "No" for the
"Do you want me to run `npm install`?" question. We suggest installing dependencies at the workspace
root via package workspaces!

View the [official Remix docs](https://remix.run/docs/en/v1) for a more in-depth guide to getting
started!

## Setup

Since Remix is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Remix tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/remix).

/moon.yml

```
# Inherit tasks from the `remix` preset# https://github.com/moonrepo/moon-configstags: ['remix']
```

### ESLint integration

Remix does not provide a built-in linting abstraction, and instead provides a simple ESLint
configuration package,
[`@remix-run/eslint-config`](https://www.npmjs.com/package/@remix-run/eslint-config). For the rest
of this section, we're going to assume that a [global `lint` task](/docs/guides/examples/eslint) has been configured.

Begin be installing the `@remix-run/eslint-config` dependency in the application's `package.json`.
We can then enable this configuration by creating an `.eslintrc.js` file in the project root. Be
sure this file is listed in your `lint` task's inputs!

/.eslintrc.js

```
module.exports = {  extends: ['@remix-run/eslint-config', '@remix-run/eslint-config/node'],  // If using TypeScript  parser: '@typescript-eslint/parser',  parserOptions: {    project: 'tsconfig.json',    tsconfigRootDir: __dirname,  },};
```

### TypeScript integration

Remix ships with TypeScript support (when enabled during installation), but the `tsconfig.json` it
generates is not setup for TypeScript project references, which we suggest using with a
[global `typecheck` task](/docs/guides/examples/typescript).

When using project references, we suggest the following `tsconfig.json`, which is a mix of Remix and
moon. Other compiler options, like `isolatedModules` and `esModuleInterop`, should be declared in a
shared configuration found in the workspace root (`tsconfig.projectOptions.json` in the example).

/tsconfig.json

```
{  "extends": "../../tsconfig.projectOptions.json",  "compilerOptions": {    "baseUrl": ".",    "emitDeclarationOnly": false,    "jsx": "react-jsx",    "resolveJsonModule": true,    "moduleResolution": "node",    "noEmit": true,    "paths": {      "~/*": ["./app/*"]    }  },  "include": [".eslintrc.js", "remix.env.d.ts", "**/*"],  "exclude": [".cache", "build", "public"]}
```

## Configuration

### Root-level

We suggest against root-level configuration, as Remix should be installed per-project, and the
`remix` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Remix project, a
[`remix.config.js`](https://remix.run/docs/en/v1/api/conventions) is created, and must exist in
the project root. This allows each project to configure Remix for their needs.

/remix.config.js

```
module.exports = {  appDirectory: 'app',};
```

## /docs/guides/examples/solid

Source: https://moonrepo.dev/docs/guides/examples/solid

# Solid example

[Solid](https://www.solidjs.com) (also known as SolidJS) is a JavaScript framework for building
interactive web applications. Because of this, Solid is an application or library concern, and not a
build system one, since the bundling of Solid is abstracted away through the application or a
bundler.

With that being said, we do have some suggestions on utilizing Solid effectively in a monorepo. To
begin, install Solid to a project.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add solid-js
```

```
yarn workspace
 add solid-js
```

```
npm install --workspace
 solid-js
```

```
pnpm add --filter
 solid-js
```

```
bun install solid-js
```

## Setup

Solid utilizes JSX for rendering markup, which requires
[`babel-preset-solid`](https://www.npmjs.com/package/babel-preset-solid) for parsing and
transforming. To enable the preset for the entire monorepo, add the preset to a root
`babel.config.js`, otherwise add it to a `.babelrc.js` in each project that requires it.

```
module.exports = {  presets: ['solid'],};
```

### TypeScript integration

For each project using Solid, add the following compiler options to the `tsconfig.json` found in the
project root.

/tsconfig.json

```
{  "compilerOptions": {    "jsx": "preserve",    "jsxImportSource": "solid-js"  }}
```

### Vite integration

If you're using a [Vite](/docs/guides/examples/vite) powered application (Solid Start or starter templates), you should
enable [`vite-plugin-solid`](https://www.npmjs.com/package/vite-plugin-solid) instead of configuring
Babel. Be sure to read our [guide on Vite](/docs/guides/examples/vite) as well!

/vite.config.js

```
import { defineConfig } from 'vite';import solidPlugin from 'vite-plugin-solid';export default defineConfig({  // ...  plugins: [solidPlugin()],});
```

## /docs/guides/examples/storybook

Source: https://moonrepo.dev/docs/guides/examples/storybook

# Storybook example

Storybook is a frontend workshop for building UI components and pages in isolation. Thousands of
teams use it for UI development, testing, and documentation. It’s open source and free.

[Storybook v7](https://storybook.js.org/docs/7.0) is typically coupled with
[Vite](https://vitejs.dev/). To scaffold a new Storybook project with Vite, run the following
command in a project root. This guide assumes you are using React, however it is possible to use
almost any (meta) framework with Storybook.

```
cd
 && npx storybook init
```

We highly suggest reading our documentation on [using Vite (and Vitest) with moon](/docs/guides/examples/vite) and
[using Jest with moon](/docs/guides/examples/jest) for a more holistic view.

## Setup

This section assumes Storybook is being used with Vite, and is integrated on a per-project basis.

After setting up Storybook, ensure [`moon.yml`](/docs/config/project) has the following tasks:

/moon.yml

```
fileGroups:  storybook:    - 'src/**/*'    - 'stories/**/*'    - 'tests/**/*'    - '.storybook/**/*'tasks:  buildStorybook:    command: 'build-storybook --output-dir @out(0)'    inputs:      - '@group(storybook)'    outputs:      - 'build'  storybook:    preset: 'server'    command: 'start-storybook'    inputs:      - '@group(storybook)'
```

To run the Storybook development server:

```
moon run
:storybook
```

### Vite integration

Storybook 7 uses Vite out of the box, and as such, no configuration is required, but should you
choose to extend the Vite config, you can do so by passing in `viteFinal`:

.storybook/main.ts

```
import { mergeConfig } from 'vite';export default {  stories: ['../stories/**/*.stories.mdx', '../stories/**/*.stories.@(js|jsx|ts|tsx)'],  addons: ['@storybook/addon-links', '@storybook/addon-essentials'],  core: {    builder: '@storybook/builder-vite',  },  async viteFinal(config) {    // Merge custom configuration into the default config    return mergeConfig(config, {      // Use the same "resolve" configuration as your app      resolve: (await import('../vite.config.js')).default.resolve,      // Add dependencies to pre-optimization      optimizeDeps: {        include: ['storybook-dark-mode'],      },    });  },};
```

For more information on how to integrate Vite with Storybook see the
[relevant documentation](https://storybook.js.org/docs/7.0/react/builders/vite#configuration).

### Webpack integration

If you want to use Webpack with your Storybook project, you can do so by installing the relevant
package and updating configuration.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add --dev @storybook/builder-webpack5
```

```
yarn workspace
 add --dev @storybook/builder-webpack5
```

```
npm install --save-dev --workspace
 @storybook/builder-webpack5
```

```
pnpm add --save-dev --filter
 @storybook/builder-webpack5
```

```
bun install --dev @storybook/builder-webpack5
```

.storybook/main.ts

```
export default {  core: {    builder: '@storybook/builder-webpack5',  },};
```

For more information on how to integrate Webpack with Storybook, see the
[relevant documentation](https://storybook.js.org/docs/7.0/react/builders/webpack).

### Jest integration

You can use Jest to test your stories, but isn't a requirement. Storybook ships with first-party
plugins for improved developer experience.

Install the test runner and any relevant packages:

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add --dev @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

```
yarn workspace
 add --dev @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

```
npm install --save-dev --workspace
 @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

```
pnpm add --save-dev --filter
 @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

```
bun install --dev @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

Add the test task to your project:

/moon.yml

```
tasks:  testStorybook:    command: 'test-storybook'    inputs:      - '@group(storybook)'
```

Then enable plugins and interactions in your Storybook project:

.storybook/main.ts

```
export default {  stories: ['../src/**/*.stories.mdx', '../src/**/*.stories.@(js|jsx|ts|tsx)'],  addons: [    // Other Storybook addons    '@storybook/addon-interactions', // Addon is registered here    '@storybook/addon-coverage',  ],  features: {    interactionsDebugger: true, // Enable playback controls  },};
```

You can now start writing your tests. For an extended guide on how to write tests within your
stories, see
[writing an interaction test](https://storybook.js.org/docs/react/writing-tests/interaction-testing#write-an-interaction-test)
on the Storybook docs.

## Configuration

Storybook requires a `.storybook` folder relative to the project root. Because of this, Storybook
should be scaffolded in each project individually. Configuration may be shared through package
imports.

## /docs/guides/examples/sveltekit

Source: https://moonrepo.dev/docs/guides/examples/sveltekit

# SvelteKit example

[SvelteKit](https://kit.svelte.dev) is built on [Svelte](https://svelte.dev), a UI framework that
uses a compiler to let you write breathtakingly concise components that do minimal work in the
browser, using languages you already know — HTML, CSS and JavaScript. It's a love letter to web
development.

```
cd apps && npm create svelte@latest

```

You will be prompted to choose between select templates, TypeScript, ESLint, Prettier, Playwright
and Vitest among other options. moon supports and has guides for many of these tools.

We highly suggest reading our documentation on [using Vite (and Vitest) with moon](/docs/guides/examples/vite),
[using ESLint with moon](/docs/guides/examples/eslint) and [using Prettier with moon](/docs/guides/examples/prettier) for a more holistic
view.

## Setup

Since SvelteKit is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting SvelteKit tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/sveltekit).

/moon.yml

```
# Inherit tasks from the `sveltekit` preset# https://github.com/moonrepo/moon-configstags: ['sveltekit']
```

### ESLint integration

SvelteKit provides an option to setup ESLint along with your project, with moon you can use a
[global `lint` task](/docs/guides/examples/eslint). We encourage using the global `lint` task for consistency across all
projects within the repository. With this approach, the `eslint` command itself will be ran and the
`svelte3` rules will still be used.

/moon.yml

```
tasks:  # Extends the top-level lint  lint:    args:      - '--ext'      - '.ts,.svelte'
```

Be sure to enable the Svelte parser and plugin in a project local ESLint configuration file.

.eslintrc.cjs

```
module.exports = {  plugins: ['svelte3'],  ignorePatterns: ['*.cjs'],  settings: {    'svelte3/typescript': () => require('typescript'),  },  overrides: [{ files: ['*.svelte'], processor: 'svelte3/svelte3' }],};
```

### TypeScript integration

SvelteKit also has built-in support for TypeScript, but has similar caveats to the
[ESLint integration](#eslint-integration). TypeScript itself is a bit involved, so we suggest
reading the official [SvelteKit documentation](https://kit.svelte.dev/docs/introduction) before
continuing.

At this point we'll assume that a `tsconfig.json` has been created in the application, and
typechecking works. From here we suggest utilizing a [global `typecheck` task](/docs/guides/examples/typescript) for
consistency across all projects within the repository. However, because Svelte isn't standard
JavaScript, it requires the use of the `svelte-check` command for type-checking.

info

The
[moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/sveltekit)
provides the `check` task below.

/moon.yml

```
workspace:  inheritedTasks:    exclude: ['typecheck']tasks:  check:    command: 'svelte-check --tsconfig ./tsconfig.json'    deps:      - 'typecheck-sync'    inputs:      - '@group(svelte)'      - 'tsconfig.json'
```

In case Svelte doesn't automatically create a `tsconfig.json`, you can use the following:

/tsconfig.json

```
{  "extends": "./.svelte-kit/tsconfig.json",  "compilerOptions": {    "allowJs": true,    "checkJs": true,    "esModuleInterop": true,    "forceConsistentCasingInFileNames": true,    "resolveJsonModule": true,    "skipLibCheck": true,    "sourceMap": true,    "strict": true  }}
```

## Configuration

### Root-level

We suggest against root-level configuration, as SvelteKit should be installed per-project, and the
`vite` command expects the configuration to live relative to the project root.

### Project-level

When creating a new SvelteKit project, a
[`svelte.config.js`](https://kit.svelte.dev/docs/configuration) is created, and must exist in the
project root. This allows each project to configure SvelteKit for their needs.

/svelte.config.js

```
import adapter from '@sveltejs/adapter-auto';import { vitePreprocess } from '@sveltejs/kit/vite';/** @type {import('@sveltejs/kit').Config} */const config = {  // Consult https://kit.svelte.dev/docs/integrations#preprocessors  // for more information about preprocessors  preprocess: vitePreprocess(),  kit: {    adapter: adapter(),  },};export default config;
```

## /docs/guides/examples/typescript

Source: https://moonrepo.dev/docs/guides/examples/typescript

# TypeScript example

In this guide, you'll learn how to integrate [TypeScript](https://www.typescriptlang.org/) into
moon. We'll be using [project references](/docs/guides/javascript/typescript-project-refs), as it ensures that
only affected projects are built, and not the entire repository.

Begin by installing `typescript` and any pre-configured tsconfig packages in your root. We suggest
using the same version across the entire repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev typescript tsconfig-moon
```

```
yarn add --dev typescript tsconfig-moon# If using workspacesyarn add --dev -W typescript tsconfig-moon
```

```
npm install --save-dev typescript tsconfig-moon
```

```
pnpm add --save-dev typescript tsconfig-moon# If using workspacespnpm add --save-dev -w typescript tsconfig-moon
```

```
bun install --dev typescript tsconfig-moon
```

## Setup

Since typechecking is a universal workflow, add a `typecheck` task to
[`.moon/tasks/node.yml`](/docs/config/tasks) with the following parameters.

.moon/tasks/node.yml

```
tasks:  typecheck:    command:      - 'tsc'      # Use incremental builds with project references      - '--build'      # Always use pretty output      - '--pretty'      # Use verbose logging to see affected projects      - '--verbose'    inputs:      # Source and test files      - 'src/**/*'      - 'tests/**/*'      # Type declarations      - 'types/**/*'      # Project configs      - 'tsconfig.json'      - 'tsconfig.*.json'      # Root configs (extended from only)      - '/tsconfig.options.json'    outputs:      # Matches `compilerOptions.outDir`      - 'lib'
```

Projects can extend this task and provide additional parameters if need be, for example.

/moon.yml

```
tasks:  typecheck:    args:      # Force build every time      - '--force'
```

## Configuration

### Root-level

Multiple root-level TypeScript configs are required, as we need to define compiler options that
are shared across the repository, and we need to house a list of all project references.

To start, let's create a `tsconfig.options.json` that will contain our compiler options. In our
example, we'll extend [tsconfig-moon](https://www.npmjs.com/package/tsconfig-moon) for convenience.
Specifically, the `tsconfig.workspaces.json` config, which enables ECMAScript modules, composite
mode, declaration emitting, and incremental builds.

tsconfig.options.json

```
{  "extends": "tsconfig-moon/tsconfig.projects.json",  "compilerOptions": {    // Your custom options    "moduleResolution": "nodenext",    "target": "es2022"  }}
```

We'll also need the standard `tsconfig.json` to house our project references. This is used by
editors and tooling for deep integrations.

tsconfig.json

```
{  "extends": "./tsconfig.options.json",  "files": [],  // All project references in the repo  "references": []}
```

The [`typescript.rootConfigFileName`](/docs/config/toolchain#rootconfigfilename) setting can be
used to change the root-level config name and the
[`typescript.syncProjectReferences`](/docs/config/toolchain#syncprojectreferences) setting will
automatically keep project references in sync!

### Project-level

Every project will require a `tsconfig.json`, as TypeScript itself requires it. The following
`tsconfig.json` will typecheck the entire project, including source and test files.

/tsconfig.json

```
{  // Extend the root compiler options  "extends": "../../tsconfig.options.json",  "compilerOptions": {    // Declarations are written here    "outDir": "lib"  },  // Include files in the project  "include": ["src/**/*", "tests/**/*"],  // Depends on other projects  "references": []}
```

The [`typescript.projectConfigFileName`](/docs/config/toolchain#projectconfigfilename) setting can
be used to change the project-level config name.

### Sharing

To share configuration across projects, you have 3 options:

- Define settings in a [root-level config](#root-level). This only applies to the parent repository.

- Create and publish an [`tsconfig base`](https://www.typescriptlang.org/docs/handbook/tsconfig-json.html#tsconfig-bases) npm package. This can be used in any repository.

- A combination of 1 and 2.

For options 2 and 3, if you're utilizing package workspaces, create a local package with the
following content.

packages/tsconfig-company/tsconfig.json

```
{  "compilerOptions": {    // ...    "lib": ["esnext"]  }}
```

Within another `tsconfig.json`, you can extend this package to inherit the settings.

tsconfig.json

```
{  "extends": "tsconfig-company/tsconfig.json"}
```

## FAQ

### How to preserve pretty output?

TypeScript supports a pretty format where it includes codeframes and color highlighting for
failures. However, when `tsc` is piped or the terminal is not a TTY, the pretty format is lost. To
preserve and always display the pretty format, be sure to pass the `--pretty` argument!

## /docs/guides/examples/vite

Source: https://moonrepo.dev/docs/guides/examples/vite

# Vite & Vitest example

In this guide, you'll learn how to integrate [Vite](https://vitejs.dev/) and
[Vitest](https://vitest.dev/) into moon.

Begin by creating a new Vite project in the root of an existing moon project (this should not be
created in the workspace root, unless a polyrepo).

- Yarn
- Yarn (classic)
- npm
- pnpm

```
yarn create vite
```

```
yarn create vite
```

```
npm create vite
```

```
pnpm create vite
```

If you plan on using Vitest, run the following command to add the `vitest` dependency to a project,
otherwise skip to the setup section.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add --dev vitest
```

```
yarn workspace
 add --dev vitest
```

```
npm install --save-dev --workspace
 vitest
```

```
pnpm add --save-dev --filter
 vitest
```

```
bun install --dev vitest
```

## Setup

Since Vite is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Vite tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/vite).

/moon.yml

```
# Inherit tasks from the `vite` and `vitest` presets# https://github.com/moonrepo/moon-configstags: ['vite', 'vitest']
```

## Configuration

### Root-level

We suggest against root-level configuration, as Vite should be installed per-project, and the
`vite` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Vite project, a [`vite.config.`](https://vitejs.dev/config) is created,
and must exist in the project root.

/vite.config.js

```
import { defineConfig } from 'vite';export default defineConfig({  // ...  build: {    // These must be `outputs` in the `build` task    outDir: 'dist',  },  test: {    // Vitest settings  },});
```

If you'd prefer to configure Vitest in a
[separate configuration file](https://vitest.dev/guide/#configuring-vitest), create a
`vitest.config.` file.

## /docs/guides/examples/vue

Source: https://moonrepo.dev/docs/guides/examples/vue

# Vue example

Vue is an application or library concern, and not a build system one, since the bundling of Vue is
abstracted away through other tools. Because of this, moon has no guidelines around utilizing Vue
directly. You can use Vue however you wish!

However, with that being said, Vue is typically coupled with [Vite](https://vitejs.dev/). To
scaffold a new Vue project with Vite, run the following command in a project root.

```
npm init vue@latest
```

We highly suggest reading our documentation on [using Vite (and Vitest) with moon](/docs/guides/examples/vite) for a
more holistic view.

## Setup

This section assumes Vue is being used with Vite.

### ESLint integration

When linting with [ESLint](/docs/guides/examples/eslint) and the
[`eslint-plugin-vue`](https://eslint.vuejs.org/user-guide/#installation) library, you'll need to
include the `.vue` extension within the `lint` task. This can be done by extending the top-level
task within the project (below), or by adding it to the top-level entirely.

/moon.yml

```
tasks:  lint:    args:      - '--ext'      - '.js,.ts,.vue'
```

Furthermore, when using TypeScript within ESLint, we need to make a few additional changes to the
`.eslintrc.js` config found in the root (if the entire repo is Vue), or within the project (if only
the project is Vue).

```
module.exports = {  parser: 'vue-eslint-parser',  parserOptions: {    extraFileExtensions: ['.vue'],    parser: '@typescript-eslint/parser',    project: 'tsconfig.json', // Or another config    tsconfigRootDir: __dirname,  },};
```

### TypeScript integration

Vue does not use [TypeScript](/docs/guides/examples/typescript)'s `tsc` binary directly, but instead uses
[`vue-tsc`](https://vuejs.org/guide/typescript/overview.html), which is a thin wrapper around `tsc`
to support Vue components. Because of this, we should update the `typecheck` task in the project to
utilize this command instead.

/moon.yml

```
workspace:  inheritedTasks:    exclude: ['typecheck']tasks:  typecheck:    command:      - 'vue-tsc'      - '--noEmit'      # Always use pretty output      - '--pretty'    inputs:      - 'env.d.ts'      # Source and test files      - 'src/**/*'      - 'tests/**/*'      # Project configs      - 'tsconfig.json'      - 'tsconfig.*.json'      # Root configs (extended from only)      - '/tsconfig.options.json'
```

Be sure `tsconfig.json` compiler options are based on
[`@vue/tsconfig`](https://vuejs.org/guide/typescript/overview.html#configuring-tsconfig-json).

## /docs/guides/extensions

Source: https://moonrepo.dev/docs/guides/extensions

# Extensions

v1.20.0

An extension is a WASM plugin that allows you to extend moon with additional functionality, have
whitelisted access to the file system, and receive partial information about the current workspace.
Extensions are extremely useful in offering new and unique functionality that doesn't need to be
built into moon's core. It also enables the community to build and share their own extensions!

## Using extensions

Before an extension can be executed with the [`moon ext`](/docs/commands/ext) command, it must be
configured with the [`extensions`](/docs/config/workspace#extensions) setting in
[`.moon/workspace.yml`](/docs/config/workspace) (excluding [built-in's](#built-in-extensions)).

.moon/workspace.yml

```
extensions:  example:    plugin: 'https://example.com/path/to/example.wasm'
```

Once configured, it can be executed with [`moon ext`](/docs/commands/ext) by name. Arguments unique to
the extension must be passed after a `--` separator.

```
$ moon ext example -- --arg1 --arg2
```

## Built-in extensions

moon is shipped with a few built-in extensions that are configured and enabled by default. Official
moon extensions are built and published in our [moonrepo/moon-extensions](https://github.com/moonrepo/moon-extensions) repository.

### `download`

The `download` extension can be used to download a file from a URL into the current workspace, as
defined by the `--url` argument. For example, say we want to download the latest [proto](/proto)
binary:

```
$ moon ext download --\  --url https://github.com/moonrepo/proto/releases/latest/download/proto_cli-aarch64-apple-darwin.tar.xz
```

By default this will download `proto_cli-aarch64-apple-darwin.tar.xz` into the current working
directory. To customize the location, use the `--dest` argument. However, do note that the
destination must be within the current moon workspace, as only certain directories are whitelisted
for WASM.

```
$ moon ext download --\  --url https://github.com/moonrepo/proto/releases/latest/download/proto_cli-aarch64-apple-darwin.tar.xz\  --dest ./temp
```

#### Arguments

- `--url` (required) - URL of a file to download.

- `--dest` - Destination folder to save the file. Defaults to the current working directory.

- `--name` - Override the file name. Defaults to the file name in the URL.

### `migrate-nx`v1.22.0

This extension is currently experimental and will be improved over time.

The `migrate-nx` extension can be used to migrate an Nx powered repository to moon. This process
will convert the root `nx.json` and `workspace.json` files, and any `project.json` and
`package.json` files found within the repository. The following changes are made:

- Migrates `targetDefaults` as global tasks to [`.moon/tasks/node.yml`](/docs/config/tasks#tasks) (or `bun.yml`), `namedInputs` as file groups, `workspaceLayout` as projects, and more.

- Migrates all `project.json` settings to [`moon.yml`](/docs/config/project#tasks) equivalent settings. Target to task conversion assumes the following: Target `executor` will be removed, and we'll attempt to extract the appropriate npm package command. For example, `@nx/webpack:build` -> `webpack build`.

- Target `options` will be converted to task `args`.

- The `{projectRoot}` and `{workspaceRoot}` interpolations will be replaced with moon tokens.

```
$ moon ext migrate-nx
```

caution

Nx and moon are quite different, so many settings are either ignored when converting, or are not a
1:1 conversion. We do our best to convert as much as possible, but some manual patching will most
likely be required! We suggest testing each converted task 1-by-1 to ensure it works as expected.

#### Arguments

- `--bun` - Migrate to Bun based commands instead of Node.js.

- `--cleanup` - Remove Nx configs/files after migrating.

#### Unsupported

The following features are not supported in moon, and are ignored when converting.

- Most settings in `nx.json`.

- Named input variants: external dependencies, dependent task output files, dependent project inputs, or runtime commands.

- Target `configurations` and `defaultConfiguration`. Another task will be created instead that uses `extends`.

- Project `root` and `sourceRoot`.

### `migrate-turborepo`v1.21.0

The `migrate-turborepo` extension can be used to migrate a Turborepo powered repository to moon.
This process will convert the root `turbo.json` file, and any `turbo.json` files found within the
repository. The following changes are made:

- Migrates `pipeline` (v1) and `tasks` (v2) global tasks to [`.moon/tasks/node.yml`](/docs/config/tasks#tasks) (or `bun.yml`) and project scoped tasks to [`moon.yml`](/docs/config/project#tasks). Task commands will execute `package.json` scripts through a package manager.

- Migrates root `global*` settings to [`.moon/tasks/node.yml`](/docs/config/tasks#implicitinputs) (or `bun.yml`) as `implicitInputs`.

```
$ moon ext migrate-turborepo
```

#### Arguments

- `--bun` - Migrate to Bun based commands instead of Node.js.

- `--cleanup` - Remove Turborepo configs/files after migrating.

### `unpack`v2.0.0

The `unpack` extension can be used to unpack an archive (zip/tar) from a file path or URL into a
destination folder.

```
$ moon ext unpack -- --src ./path/to/archive.zip --dest ./output --prefix path/to/strip
```

#### Arguments

- `--src` (required) - Path or URL of a file to unpack.

- `--dest` - Destination folder to unpack into. Defaults to the current working directory.

- `--prefix` - A prefix path to strip from unpacked files.

## Creating an extension

Refer to our [official WASM guide](/docs/guides/wasm-plugins) for more information on how our WASM plugins
work, critical concepts to know, how to create a plugin, and more. Once you have a good
understanding, you may continue this specific guide.

note

Refer to our [moonrepo/moon-extensions](https://github.com/moonrepo/moon-extensions) repository for in-depth examples.

### Registering metadata

Before we begin, we must implement the `register_extension` function, which simply provides some
metadata that we can bubble up to users, or to use for deeper integrations.

```
use extism_pdk::*;use moon_pdk::*;#[plugin_fn]pub fn register_extension(Json(input): Json) -> FnResult> {   Ok(Json(ExtensionMetadataOutput {        name: "Extension name".into(),        description: Some("A description about what the extension does.".into()),        plugin_version: env!("CARGO_PKG_VERSION").into(),        ..ExtensionMetadataOutput::default()    }))}
```

#### Configuration schema

If you are using [configuration](#supporting-configuration), you can register the shape of the
configuration using the [`schematic`](https://crates.io/crates/schematic) crate. This shape will be
used to generate outputs such as JSON schemas, or TypeScript types.

```
#[plugin_fn]pub fn register_extension(_: ()) -> FnResult> {    Ok(Json(ExtensionMetadataOutput {        // ...        config_schema: Some(schematic::SchemaBuilder::generate::()),    }))}
```

Schematic is a heavy library, so we suggest adding the dependency like so:

```
[dependencies]schematic = { version = "*", default-features = false, features = ["schema"] }
```

### Implementing execution

Extensions support a single plugin function, `execute_extension`, which is called by the
[`moon ext`](/docs/commands/ext) command to execute the extension. This is where all your business
logic will reside.

```
#[host_fn]extern "ExtismHost" {    fn host_log(input: Json);}#[plugin_fn]pub fn execute_extension(Json(input): Json) -> FnResult {  host_log!(stdout, "Executing extension!");  Ok(())}
```

### Supporting arguments

Most extensions will require arguments, as it provides a mechanism for users to pass information
into the WASM runtime. To parse arguments, we provide the
[`Args`](https://docs.rs/clap/latest/clap/trait.Args.html) trait/macro from the
[clap](https://crates.io/crates/clap) crate. Refer to their
[official documentation on usage](https://docs.rs/clap/latest/clap/_derive/index.html) (we don't
support everything).

```
use moon_pdk::*;#[derive(Args)]pub struct ExampleExtensionArgs {  // --url, -u  #[arg(long, short = 'u', required = true)]  pub url: String,}
```

Once your struct has been defined, you can parse the provided input arguments using the
[`parse_args`](https://docs.rs/moon_pdk/latest/moon_pdk/args/fn.parse_args.html) function.

```
#[plugin_fn]pub fn execute_extension(Json(input): Json) -> FnResult {  let args = parse_args::(&input.args)?;  args.url; // --url  Ok(())}
```

### Supporting configuration

Users can configure [extensions](/docs/config/workspace#extensions) with additional settings in
[`.moon/workspace.yml`](/docs/config/workspace). Do note that settings should be in camelCase for them
to be parsed correctly!

.moon/workspace.yml

```
extensions:  example:    plugin: 'file://./path/to/example.wasm'    someSetting: 'abc'    anotherSetting: 123
```

In the plugin, we can map these settings (excluding `plugin`) into a struct. The `Default` trait
must be implemented to handle situations where settings were not configured, or some are missing.

```
config_struct!(  #[derive(Default)]  pub struct ExampleExtensionConfig {    pub some_setting: String,    pub another_setting: u32,  });
```

Once your struct has been defined, you can access the configuration using the
[`get_extension_config`](https://docs.rs/moon_pdk/latest/moon_pdk/extension/fn.get_extension_config.html)
function.

```
#[plugin_fn]pub fn execute_extension(Json(input): Json) -> FnResult {  let config = get_extension_config::()?;  config.another_setting; // 123  Ok(())}
```

## /docs/guides/javascript/bun-handbook

Source: https://moonrepo.dev/docs/guides/javascript/bun-handbook

# Bun handbook

Utilizing JavaScript (and TypeScript) in a monorepo can be a daunting task, especially when using
Bun (or Node.js), as there are many ways to structure your code and to configure your tools. With
this handbook, we'll help guide you through this process.

info

This guide is a living document and will continue to be updated over time!

## moon setup

For this part of the handbook, we'll be focusing on [moon](/moon), our task runner. To start,
languages in moon act like plugins, where their functionality and support is not enabled unless
explicitly configured. We follow this approach to avoid unnecessary overhead.

### Enabling the language

To enable JavaScript support via Bun, define the [`bun`](/docs/config/toolchain#bun) setting in
[`.moon/toolchains.yml`](/docs/config/toolchain), even if an empty object.

.moon/toolchains.yml

```
# Enable Bunbun: {}
```

info

In moon v1.40+, use `javascript` and `bun` instead of `bun` to enable the new WASM powered Bun
toolchain, which is far more accurate and efficient. The non-WASM toolchain will be deprecated in
the future.

Or by pinning a `bun` version in [`.prototools`](/docs/proto/config) in the workspace root.

.prototools

```
bun = "1.0.0"
```

This will enable the Bun toolchain and provide the following automations around its ecosystem:

- Node modules will automatically be installed if dependencies in `package.json` have changed, or the lockfile has changed, since the last time a task has ran. We'll also take `package.json` workspaces into account and install modules in the correct location; either the workspace root, in a project, or both.

- Relationships between projects will automatically be discovered based on `dependencies`, `devDependencies`, and `peerDependencies` in `package.json`.

### Utilizing the toolchain

When a language is enabled, moon by default will assume that the language's binary is available
within the current environment (typically on `PATH`). This has the downside of requiring all
developers and machines to manually install the correct version of the language, and to stay in
sync.

Instead, you can utilize [moon's toolchain](/docs/concepts/toolchain), which will download and
install the language in the background, and ensure every task is executed using the exact version
across all machines.

Enabling the toolchain is as simple as defining the [`bun.version`](/docs/config/toolchain#version)
setting.

.moon/toolchains.yml

```
# Enable Bun toolchain with an explicit versionbun:  version: '1.0.0'
```

Versions can also be defined with [`.prototools`](/docs/proto/config).

### Configuring the toolchain

Since the JavaScript ecosystem supports multiple runtimes, moon is unable to automatically detect
the correct runtime for all scenarios. Does the existence of a `package.json` mean Node.js or Bun?
We don't know, and default to Node.js because of its popularity.

To work around this, you can set `toolchain` to "bun" at the task-level or project-level.

moon.yml

```
# For all tasks in the projecttoolchain:  default: 'bun'tasks:  build:    command: 'webpack'    # For this specific task    toolchain: 'bun'
```

The task-level `toolchain.default` only needs to be set if executing a `node_modules` binary! The
`bun` binary automatically sets the toolchain to Bun.

### Using `package.json` scripts

If you're looking to prototype moon, or reduce the migration effort to moon tasks, you can configure
moon to inherit `package.json` scripts, and internally convert them to moon tasks. This can be
achieved with the [`bun.inferTasksFromScripts`](/docs/config/toolchain#infertasksfromscripts)
setting.

.moon/toolchains.yml

```
bun:  inferTasksFromScripts: true
```

Or you can run scripts through `bun run` calls.

moon.yml

```
tasks:  build:    command: 'bun run build'
```

## Handbook

info

Refer to the [Node.js handbook](/docs/guides/javascript/node-handbook) for more information on repository structure,
dependency management, and more. Since both runtimes are extremely similar, the information in that
handbook also applies to Bun!

## /docs/guides/javascript/deno-handbook

Source: https://moonrepo.dev/docs/guides/javascript/deno-handbook

# Deno handbook

Utilizing Deno in a TypeScript based monorepo can be a non-trivial task. With this handbook, we'll
help guide you through this process.

info

This guide is a living document and will continue to be updated over time!

## moon setup

For this part of the handbook, we'll be focusing on [moon](/moon), our task runner. To start,
languages in moon act like plugins, where their functionality and support is not enabled unless
explicitly configured. We follow this approach to avoid unnecessary overhead.

### Enabling the language

To enable TypeScript support via Deno, define the [`deno`](/docs/config/toolchain#deno) setting in
[`.moon/toolchains.yml`](/docs/config/toolchain), even if an empty object.

.moon/toolchains.yml

```
# Enable Denodeno: {}# Enable Deno and override default settingsdeno:  lockfile: true
```

Or by pinning a `deno` version in [`.prototools`](/docs/proto/config) in the workspace root.

.prototools

```
deno = "1.31.0"
```

This will enable the Deno toolchain and provide the following automations around its ecosystem:

- Automatic handling and caching of lockfiles (when the setting is enabled).

- Relationships between projects will automatically be discovered based on `imports`, `importMap`, and `deps.ts` (currently experimental).

- And more to come!

### Work in progress

caution

Deno support is currently experimental while we finalize the implementation.

The following features are not supported:

- `deno.jsonc` files (use `deno.json` instead).

- `files.exclude` are currently considered an input. These will be filtered in a future release.

## Coming soon!

The handbook is currently being written while we finalize our Deno integration support!

## /docs/guides/javascript/node-handbook

Source: https://moonrepo.dev/docs/guides/javascript/node-handbook

# Node.js handbook

Utilizing JavaScript (and TypeScript) in a monorepo can be a daunting task, especially when using
Node.js, as there are many ways to structure your code and to configure your tools. With this
handbook, we'll help guide you through this process.

info

This guide is a living document and will continue to be updated over time!

## moon setup

For this part of the handbook, we'll be focusing on [moon](/moon), our task runner. To start,
languages in moon act like plugins, where their functionality and support is not enabled unless
explicitly configured. We follow this approach to avoid unnecessary overhead.

### Enabling the language

To enable JavaScript support via Node.js, define the [`node`](/docs/config/toolchain#node) setting
in [`.moon/toolchains.yml`](/docs/config/toolchain), even if an empty object.

.moon/toolchains.yml

```
# Enable Node.jsnode: {}# Enable Node.js and override default settingsnode:  packageManager: 'pnpm'
```

info

In moon v1.40+, use `javascript` and `node` instead of `node` to enable the new WASM powered Node.js
toolchain, which is far more accurate and efficient. The non-WASM toolchain will be deprecated in
the future.

Or by pinning a `node` version in [`.prototools`](/docs/proto/config) in the workspace root.

.prototools

```
node = "18.0.0"pnpm = "7.29.0"
```

This will enable the Node.js toolchain and provide the following automations around its ecosystem:

- Node modules will automatically be installed if dependencies in `package.json` have changed, or the lockfile has changed, since the last time a task has ran. We'll also take `package.json` workspaces into account and install modules in the correct location; either the workspace root, in a project, or both.

- Relationships between projects will automatically be discovered based on `dependencies`, `devDependencies`, and `peerDependencies` in `package.json`. The versions of these packages will also be automatically synced when changed.

- Tasks can be [automatically inferred](/docs/config/toolchain#infertasksfromscripts) from `package.json` scripts.

- And much more!

### Utilizing the toolchain

When a language is enabled, moon by default will assume that the language's binary is available
within the current environment (typically on `PATH`). This has the downside of requiring all
developers and machines to manually install the correct version of the language, and to stay in
sync.

Instead, you can utilize [moon's toolchain](/docs/concepts/toolchain), which will download and
install the language in the background, and ensure every task is executed using the exact version
across all machines.

Enabling the toolchain is as simple as defining the [`node.version`](/docs/config/toolchain#version)
setting.

.moon/toolchains.yml

```
# Enable Node.js toolchain with an explicit versionnode:  version: '18.0.0'
```

Versions can also be defined with [`.prototools`](/docs/proto/config).

### Using `package.json` scripts

If you're looking to prototype moon, or reduce the migration effort to moon tasks, you can configure
moon to inherit `package.json` scripts, and internally convert them to moon tasks. This can be
achieved with the [`node.inferTasksFromScripts`](/docs/config/toolchain#infertasksfromscripts)
setting.

.moon/toolchains.yml

```
node:  inferTasksFromScripts: true
```

Or you can run scripts through `npm run` (or `pnpm`, `yarn`) calls.

moon.yml

```
tasks:  build:    command: 'npm run build'
```

## Repository structure

JavaScript monorepo's work best when projects are split into applications and packages, with each
project containing its own `package.json` and dependencies. A root `package.json` must also exist
that pieces all projects together through workspaces.

For small repositories, the following structure typically works well:

```
/├── .moon/├── package.json├── apps/│   ├── client/|   |   ├── ...│   |   └── package.json│   └── server/|       ├── ...│       └── package.json└── packages/    ├── components/    |   ├── ...    │   └── package.json    ├── theme/    |   ├── ...    │   └── package.json    └── utils/        ├── ...        └── package.json
```

For large repositories, grouping projects by team or department helps with ownership and
organization. With this structure, applications and libraries can be nested at any depth.

```
/├── .moon/├── package.json├── infra/│   └── ...├── internal/│   └── ...├── payments/│   └── ...└── shared/    └── ...
```

### Applications

Applications are runnable or executable, like an HTTP server, and are pieced together with packages
and its own encapsulated code. They represent the whole, while packages are the pieces. Applications
can import and depend on packages, but they must not import and depend on other applications.

In moon, you can denote a project as an application using the [`layer`](/docs/config/project#layer)
setting in [`moon.yml`](/docs/config/project).

moon.yml

```
layer: 'application'
```

### Packages

Packages (also known as a libraries) are self-contained reusable pieces of code, and are the
suggested pattern for [code sharing](#code-sharing). Packages can import and depend on other
packages, but they must not import and depend on applications!

In moon, you can denote a project as a library using the [`layer`](/docs/config/project#layer)
setting in [`moon.yml`](/docs/config/project).

moon.yml

```
layer: 'library'
```

### Configuration

Every tool that you'll utilize in a repository will have its own configuration file. This will be a
lot of config files, but regardless of what tool it is, where the config file should go will fall
into 1 of these categories:

- Settings are inherited by all projects. These are known as universal tools, and enforce code consistency and quality across the entire repository. Their config file must exist in the repository root, but may support overrides in each project. Examples: Babel, [ESLint](/docs/guides/examples/eslint), [Prettier](/docs/guides/examples/prettier), [TypeScript](/docs/guides/examples/typescript)

- Settings are unique per project. These are developers tools that must be configured separately for each project, as they'll have different concerns. Their config file must exist in each project, but a shared configuration may exist as a base (for example, Jest presets). Examples: [Jest](/docs/guides/examples/jest), [TypeScript](/docs/guides/examples/typescript) (with project references)

- Settings are one-offs. These are typically for applications or tools that require their own config, but aren't prevalent throughout the entire repository. Examples: [Astro](/docs/guides/examples/astro), [Next](/docs/guides/examples/next), [Nuxt](/docs/guides/examples/nuxt), [Remix](/docs/guides/examples/remix), Tailwind

## Dependency management

Dependencies, also known as node modules, are required by all projects, and are installed through a
package manager like npm, pnpm, or yarn. It doesn't matter which package manager you choose, but we
highly suggest choosing one that has proper workspaces support. If you're unfamiliar with
workspaces, they will:

- Resolve all `package.json`'s in a repository using glob patterns.

- Install dependencies from all `package.json`'s at once, in the required locations.

- Create symlinks of local packages in `node_modules` (to emulate an installed package).

- Deduplicate and hoist `node_modules` when applicable.

All of this functionality enables robust monorepo support, and can be enabled with the following:

- npm
- pnpm
- Yarn
- Yarn (classic)

package.json

```
{  // ...  "workspaces": ["apps/*", "packages/*"]}
```

.yarnrc.yml

```
# ...nodeLinker: 'node-modules'
```

- [Documentation](https://yarnpkg.com/features/workspaces)

package.json

```
{  // ...  "workspaces": ["apps/*", "packages/*"]}
```

- [Documentation](https://classic.yarnpkg.com/en/docs/workspaces)

package.json

```
{  // ...  "workspaces": ["apps/*", "packages/*"]}
```

- [Documentation](https://docs.npmjs.com/cli/v8/using-npm/workspaces)

pnpm-workspace.yaml

```
packages:  - 'apps/*'  - 'packages/*'
```

- [Documentation](https://pnpm.io/workspaces)

caution

Package workspaces are not a requirement for monorepos, but they do solve an array of problems
around module resolution, avoiding duplicate packages in bundles, and general interoperability.
Proceed with caution for non-workspaces setups!

### Workspace commands

The following common commands can be used for adding, removing, or managing dependencies in a
workspace. View the package manager's official documentation for a thorough list of commands.

- npm
- pnpm
- Yarn
- Yarn (classic)

Install dependencies:

```
npm install
```

Add a package:

```
# At the rootnpm install # In a projectnpm install  --workspace

```

Remove a package:

```
# At the rootnpm install # In a projectnpm install  --workspace

```

Update packages:

```
npx npm-check-updates --interactive
```

Install dependencies:

```
pnpm install
```

Add a package:

```
# At the rootpnpm add # In a projectpnpm add  --filter

```

Remove a package:

```
# At the rootpnpm remove # In a projectpnpm remove  --filter

```

Update packages:

```
pnpm update -i -r --latest
```

Install dependencies:

```
yarn install
```

Add a package:

```
# At the rootyarn add # In a projectyarn workspace
 add
```

Remove a package:

```
# At the rootyarn remove # In a projectyarn workspace
 remove
```

Update packages:

```
yarn upgrade-interactive
```

Install dependencies:

```
yarn install
```

Add a package:

```
# At the rootyarn add  -w# In a projectyarn workspace
 add
```

Remove a package:

```
# At the rootyarn remove  -w# In a projectyarn workspace
 remove
```

Update packages:

```
yarn upgrade-interactive --latest
```

### Developer tools at the root

While not a strict guideline to follow, we've found that installing universal developer tool related
dependencies (Babel, ESLint, Jest, TypeScript, etc) in the root `package.json` as `devDependencies`
to be a good pattern for consistency, quality, and the health of the repository. It provides the
following benefits:

- It ensures all projects are utilizing the same version (and sometimes configuration) of a tool.

- It allows the tool to easily be upgraded. Upgrade once, applied everywhere.

- It avoids conflicting or outdated versions of the same package.

With that being said, this does not include development dependencies that are unique to a project!

### Product libraries in a project

Product, application, and or framework specific packages should be installed as production
`dependencies` in a project's `package.json`. We've found this pattern to work well for the
following reasons:

- Application dependencies are pinned per project, avoiding accidental regressions.

- Applications can upgrade their dependencies and avoid breaking neighbor applications.

## Code sharing

One of the primary reasons to use a monorepo is to easily share code between projects. When code is
co-located within the same repository, it avoids the overhead of the "build -> version -> publish to
registry -> upgrade in consumer" workflow (when the code is located in an external repository).

Co-locating code also provides the benefit of fast iteration, fast adoption, and easier migration
(when making breaking changes for example).

With [package workspaces](#dependency-management), code sharing is a breeze. As mentioned above,
every project that contains a `package.json` that is part of the workspace, will be symlinked into
`node_modules`. Because of this, these packages can easily be imported using their `package.json`
name.

```
// Imports from /packages/utils/package.jsonimport utils from '@company/utils';
```

### Depending on packages

Because packages are symlinked into `node_modules`, we can depend on them as if they were normal npm
packages, but with 1 key difference. Since these packages aren't published, they do not have a
version to reference, and instead, we can use the special `workspace:^` version (yarn and pnpm only,
use `*` for npm).

```
{  "name": "@company/consumer",  "dependencies": {    "@company/provider": "workspace:^"  }}
```

The `workspace:` version basically means "use the package found in the current workspace". The `:^`
determines the version range to substitute with when publishing. For example, the `workspace:^`
above would be replaced with version of `@company/provider` as `^` when the
`@company/consumer` package is published.

There's also `workspace:~` and `workspace:*` which substitutes to `~` and ``
respectively. We suggest using `:^` so that version ranges can be deduped.

### Types of packages

When sharing packages in a monorepo, there's typically 3 different kinds of packages:

#### Local only

A local only package is just that, it's only available locally to the repository and is not
published to a registry, and is not available to external repositories. For teams and companies
that utilize a single repository, this will be the most common type of package.

A benefit of local packages is that they do not require a build step, as source files can be
imported directly ([when configured correctly](#bundler-integration)). This avoids a lot of
`package.json` overhead, especially in regards to `exports`, `imports`, and other import patterns.

#### Internally published

An internal package is published to a private registry, and is not available to the public.
Published packages are far more strict than local packages, as the `package.json` structure plays a
much larger role for downstream consumers, as it dictates how files are imported, where they can be
found, what type of formats are supported (CJS, ESM), so on and so forth.

Published packages require a build step, for both source code and TypeScript types (when
applicable). We suggest using [esbuild](https://esbuild.github.io/) or
[Packemon](/docs/guides/examples/packemon) to handle this entire flow. With that being said, local projects
can still [import their source files](#bundler-integration).

#### Externally published

An external package is structured similarly to an internal package, but instead of publishing to a
private registry, it's published to the npm public registry.

External packages are primarily for open source projects, and require the repository to also be
public.

### Bundler integration

Co-locating packages is great, but how do you import and use them effectively? The easiest solution
is to configure resolver aliases within your bundler (Webpack, Vite, etc). By doing so, you enable
the following functionality:

- Avoids having to build (and rebuild) the package everytime its code changes.

- Enables file system watching of the package, not just the application.

- Allows for hot module reloading (HMR) to work.

- Package code is transpiled and bundled alongside application code.

- Vite
- Webpack

vite.config.ts

```
import path from 'path';import { defineConfig } from 'vite';export default defineConfig({  // ...  resolve: {    alias: {      '@company/utils': path.join(__dirname, '../packages/utils/src'),    },  },});
```

webpack.config.js

```
const path = require('path');module.exports = {  // ...  resolve: {    alias: {      '@company/utils': path.join(__dirname, '../packages/utils/src'),    },  },};
```

info

When configuring aliases, we suggest using the `package.json` name as the alias! This ensures that
on the consuming side, you're using the package as if it's a normal node module, and avoids
deviating from the ecosystem.

### TypeScript integration

We suggest using TypeScript project references. Luckily, we have an
[in-depth guide on how to properly and efficiently integrate them](/docs/guides/javascript/typescript-project-refs)!

## /docs/guides/javascript/typescript-eslint

Source: https://moonrepo.dev/docs/guides/javascript/typescript-eslint

# typescript-eslint

## ESLint integration

### Disabling problematic rules

A handful of ESLint rules are not compatible with the TypeScript plugin, or they cause serious
performance degradation, and should be disabled entirely. According to the
[official typescript-eslint.io documentation](https://typescript-eslint.io/docs/linting/troubleshooting#eslint-plugin-import),
most of these rules come from the `eslint-plugin-import` plugin.

.eslintrc.js

```
module.exports = {  // ...  rules: {    'import/default': 'off',    'import/named': 'off',    'import/namespace': 'off',    'import/no-cycle': 'off',    'import/no-deprecated': 'off',    'import/no-named-as-default': 'off',    'import/no-named-as-default-member': 'off',    'import/no-unused-modules': 'off',  },};
```

### Running from the command line

### Running within editors

#### ESLint

Use the
[dbaeumer.vscode-eslint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)
extension. Too avoid poor performance, do not use ESLint for formatting code (via the
`eslint-plugin-prettier` plugin or something similar), and only use it for linting. The difference
in speed is comparable to 100ms vs 2000ms.

.vscode/settings.json

```
{  // Automatically run all linting fixes on save as a concurrent code action,  // and avoid formatting with ESLint. Use another formatter, like Prettier.  "editor.codeActionsOnSave": ["source.fixAll.eslint"],  "eslint.format.enable": false,  // If linting is *too slow* while typing, uncomment the following line to  // only run the linter on save only.  // "editor.run": "onSave",  // Your package manager of choice.  "eslint.packageManager": "yarn",  // Use the newer and more performant `ESLint` class implementation.  "eslint.useESLintClass": true,  // List of directories that that linter should operate on.  "eslint.workingDirectories": [{ "pattern": "apps/*" }, { "pattern": "packages/*" }]}
```

#### Prettier

Use the
[esbenp.prettier-vscode](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)
extension.

.vscode/settings.json

```
{  // Use Prettier as the default formatter for all file types. Types not  // supported by Prettier can be overridden using bracket syntax, or ignore files.  "editor.defaultFormatter": "esbenp.prettier-vscode",  "editor.formatOnSave": true}
```

## /docs/guides/javascript/typescript-project-refs

Source: https://moonrepo.dev/docs/guides/javascript/typescript-project-refs

# TypeScript project references

The ultimate in-depth guide for using TypeScript in a monorepo effectively!

How to use TypeScript in a monorepo? What are project references? Why use project references? What
is the best way to use project references? These are just a handful of questions that are
constantly asked on Twitter, forums, Stack Overflow, and even your workplace.

Based on years of experience managing large-scale frontend repositories, we firmly believe that
TypeScript project references are the proper solution for effectively scaling TypeScript in a
monorepo. The official
[TypeScript documentation on project references](https://www.typescriptlang.org/docs/handbook/project-references.html)
answers many of these questions, but it basically boils down to the following:

- Project references enforce project boundaries, disallowing imports to arbitrary projects unless they have been referenced explicitly in configuration. This avoids circular references / cycles.

- It enables TypeScript to process individual units, instead of the entire repository as a whole. Perfect for reducing CI and local development times.

- It supports incremental compilation, so only out-of-date or affected projects are processed. The more TypeScript's cache is warmed, the faster it will be.

- It simulates how types work in the Node.js package ecosystem.

This all sounds amazing but there's got to be some downsides right? Unfortunately, there is:

- Project references require generating declarations to resolve type information correctly. This results in a lot of compilation artifacts littered throughout the repository. There [are ways](#gitignore) [around this](/docs/config/toolchain#routeoutdirtocache).

- This approach is a bit involved and may require some cognitive overhead based on your current level of TypeScript tooling knowledge.

success

If you'd like a real-world repository to reference, our
[moonrepo/moon](https://github.com/moonrepo/moon), [moonrepo/dev](https://github.com/moonrepo/dev),
and [moonrepo/examples](https://github.com/moonrepo/examples) repositories utilizes this
architecture!

## Preface

Before you dive into this questionably long guide, we'd like to preface with:

- This guide is a living document and will continually be updated with best practices and frequently asked questions. Keep returning to learn more!

- This guide assumes a basic level knowledge of TypeScript and how it works.

- The architecture outlined in this guide assumes that TypeScript is only used for typechecking and not compiling. However, supporting compilation should be as easy as modifying a handful of compiler options.

- Although this guide exists within moon's documentation, it does not require moon. We've kept all implementation details generic enough for it be used in any repository, but have also included many notes on how moon would improve this experience.

## Configuration

The most complicated part of integrating TypeScript in a monorepo is a proper configuration setup.
Based on our extensive experience, we suggest the following architecture as a base! This is not
perfect and can most definitely be expanded upon or modified to fit your needs.

### Root-level

In a polyrepo, the root `tsconfig.json` is typically the only configuration file, as it defines
common compiler options, and includes files to typecheck. In a monorepo, these responsibilities are
now split across multiple configuration files.

#### `tsconfig.json`

To start, the root `tsconfig.json` file is nothing more than a list of all projects in the
monorepo, with each project being an individual entry in the `references` field. Each entry must
contain a `path` field with a relative file system path to the project root (that contains their
config).

We also do not define compiler options in this file, as project-level configuration files would
not be able to extend this file, as it would trigger a circular reference. Instead, we define
common compiler options in a root [`tsconfig.options.json`](#tsconfigoptionsjson) file, that this
file also `extends` from.

In the end, this file should only contain 3 fields: `extends`, `files` (an empty list), and
`references`. This abides the
[official guidance around structure](https://www.typescriptlang.org/docs/handbook/project-references.html#overall-structure).

```
{  "extends": "./tsconfig.options.json",  "files": [],  "references": [    {      "path": "apps/foo"    },    {      "path": "packages/bar"    }    // ... more  ]}
```

When using moon, the
[`typescript.syncProjectReferences`](/docs/config/toolchain#syncprojectreferences) setting will
keep this `references` list automatically in sync, and the name of the file can be customized with
[`typescript.rootConfigFileName`](/docs/config/toolchain#rootconfigfilename).

#### `tsconfig.options.json`

This file will contain common compiler options that will be inherited by all projects in the
monorepo. For project references to work correctly, the following settings must be enabled at the
root, and typically should not be disabled in each project.

- `composite` - Enables project references and informs the TypeScript program where to find referenced outputs.

- `declaration` - Project references rely on the compiled declarations (`.d.ts`) of external projects. If declarations do not exist, TypeScript will generate them on demand.

- `declarationMap` - Generate sourcemaps for declarations, so that language server integrations in editors like "Go to" resolve correctly.

- `incremental` - Enables incremental compilation, greatly improving performance.

- `noEmitOnError` - If the typechecker fails, avoid generating invalid or partial declarations.

- `skipLibCheck` - Avoids eager loading and analyzing all declarations, greatly improving performance.

Furthermore, we have 2 settings that should be enabled per project, depending on the project type.

- `emitDeclarationOnly` - For packages: Emit declarations, as they're required for references, but avoid compiling to JavaScript.

- `noEmit` - For applications: Don't emit declarations, as others should not be depending on the project.

For convenience, we provide the
[`tsconfig-moon`](https://github.com/moonrepo/dev/tree/master/packages/tsconfig) package, which
defines common compiler options and may be used here.

```
{  "compilerOptions": {    "composite": true,    "declaration": true,    "declarationMap": true,    "emitDeclarationOnly": true,    "incremental": true,    "noEmitOnError": true,    "skipLibCheck": true    // ... others  }}
```

When using moon, the name of the file can be customized with
[`typescript.rootOptionsConfigFileName`](/docs/config/toolchain#rootoptionsconfigfilename).

##### ECMAScript interoperability

ECMAScript modules (ESM) have been around for quite a while now, but the default TypeScript settings
are not configured for them. We suggest the following compiler options if you want proper ESM
support with interoperability with the ecosystem.

```
{  "compilerOptions": {    "allowSyntheticDefaultImports": true,    "esModuleInterop": true,    "isolatedModules": true,    "module": "esnext",    "moduleResolution": "bundler",    "strict": true,    "target": "esnext"    // ... others  }}
```

#### `.gitignore`

Project references unfortunately generate a ton of artifacts that typically shouldn't be committed
to the repository (but could be if you so choose). We suggest ignoring the following:

.gitignore

```
# The `outDir` for declarationslib/# Build cache manifests*.tsbuildinfo
```

### Project-level

Each project that contains TypeScript files and will utilize the typechecker must contain a
`tsconfig.json` in the project root, typically as a sibling to `package.json`.

#### `tsconfig.json`

A `tsconfig.json` in the root of a project (application or package) is required, as it informs
TypeScript that this is a project, and that it can be referenced by other projects. In its simplest
form, this file should extend the root [`tsconfig.options.json`](#tsconfigoptionsjson) to inherit
common compiler options, define its own compiler options (below), define includes/excludes, and any
necessary references.

When using moon, the name of the file can be customized with
[`typescript.projectConfigFileName`](/docs/config/toolchain#projectconfigfilename).

- Applications
- Packages

For applications, declaration emitting can be disabled, since external projects should not be
importing files from an application. If this use case ever arises, move those files into a package.

apps/foo/tsconfig.json

```
{  "extends": "../../../../tsconfig.options.json",  "compilerOptions": {    "noEmit": true  },  "include": [],  "references": []}
```

For packages, we must define the location in which to generate declarations. These are the
declarations that external projects would reference. This location is typically
[gitignored](#gitignore)!

packages/bar/tsconfig.json

```
{  "extends": "../../../../tsconfig.options.json",  "compilerOptions": {    "emitDeclarationOnly": true,    "outDir": "./lib"  },  "include": [],  "references": []}
```

When using moon, the `outDir` can automatically be re-routed to a shared cache using
[`typescript.routeOutDirToCache`](/docs/config/toolchain#routeoutdirtocache), to avoid littering
the repository with compilation artifacts.

##### Includes and excludes

Based on experience, we suggest defining `include` instead of `exclude`, as managing a whitelist of
typecheckable files is much easier. When dealing with excludes, there are far too many
possibilities. To start, you have `node_modules`, and for applications maybe `dist`, `build`,
`.next`, or another application specific folder, and then for packages you may have `lib`, `cjs`,
`esm`, etc. It becomes very... tedious.

The other benefit of using `include` is that it forces TypeScript to only load what's necessary,
instead of eager loading everything into memory, and for typechecking files that aren't part of
source, like configuration.

/tsconfig.json

```
{  // ...  "include": ["src/**/*", "tests/**/*", "*.js", "*.ts"]}
```

##### Depending on other projects

When a project depends on another project (by importing code from it), either using relative paths,
[path aliases](#using-paths-aliases), or its `package.json` name, it must be declared as a
reference. If not declared, TypeScript will error with a message about importing outside the project
boundary.

/tsconfig.json

```
{  // ...  "references": [    {      "path": "../../foo"    },    {      "path": "../../bar"    },    {      "path": "../../../../baz"    }  ]}
```

To make use of editor intellisense and auto-imports of deeply nested files, you'll most likely need
to add includes for referenced projects as well.

/tsconfig.json

```
{  // ...  "include": [    // ...    "src/**/*",    "../../foo/src/**/*",    "../../bar/src/**/*",    "../../../../baz/src/**/*"  ]}
```

When using moon, the
[`typescript.syncProjectReferences`](/docs/config/toolchain#syncprojectreferences) setting will
keep this `references` list automatically in sync, and
[`typescript.includeProjectReferenceSources`](/docs/config/toolchain#syncprojectreferences) for
`include`.

#### `tsconfig.*.json`

Additional configurations may exist in a project that serve a role outside of typechecking, with one
such role being npm package publishing. These configs are sometimes named `tsconfig.build.json`,
`tsconfig.types.json`, or `tsconfig.lib.json`. Regardless of what they're called, these configs are
optional, so unless you have a business need for them, you may skip this section.

##### Package publishing

As mentioned previously, these configs may be used for npm packages, primarily for generating
TypeScript declarations that are mapped through the `package.json`
[`types` (or `typings`) field](https://www.typescriptlang.org/docs/handbook/declaration-files/publishing.html).

Given this `package.json`...

/package.json

```
{  // ...  "types": "./lib/index.d.ts"}
```

Our `tsconfig.build.json` may look like...

/tsconfig.build.json

```
{  "extends": "../../../../tsconfig.options.json",  "compilerOptions": {    "outDir": "lib",    "rootDir": "src"  },  "include": ["src/**/*"]}
```

Simple right? But why do we need an additional configuration? Why not use the other `tsconfig.json`?
Great questions! The major reason is that we only want to publish declarations for source files,
and the declarations file structure should match 1:1 with the sources structure. The `tsconfig.json`
does not guarantee this, as it may include test, config, or arbitrary files, all of which may not
exist in the sources directory (`src`), and will alter the output to an incorrect directory
structure. Our `tsconfig.build.json` solves this problem by only including source files, and by
forcing the source root to `src` using the `rootDir` compiler option.

However, there is a giant caveat with this approach! Because TypeScript utilizes Node.js's module
resolution, it will reference the declarations defined by the `package.json` `types` or
[`exports`](#supporting-packagejson-exports) fields, instead of the `outDir` compiler option, and
the other `tsconfig.json` does not guarantee these files will exist. This results in TypeScript
failing to find the appropriate types! To solve this, add the `tsconfig.build.json` as a project
reference to `tsconfig.json`.

/tsconfig.json

```
{  // ...  "references": [    {      "path": "./tsconfig.build.json"    }    // ... others  ]}
```

##### Vendor specific

Some vendors, like [Vite](/docs/guides/examples/vite), [Vitest](/docs/guides/examples/vite), and
[Astro](/docs/guides/examples/astro) may include additional `tsconfig.*.json` files unique to their ecosystem.
We suggest following their guidelines and implementation when applicable.

## Running the typechecker

Now that our configuration is place, we can run the typechecker, or attempt to at least! This can be
done with the `tsc --build` command, which acts as a
[build orchestrator](https://www.typescriptlang.org/docs/handbook/project-references.html#build-mode-for-typescript).
We also suggest passing `--verbose` for insights into what projects are compiling, and which are
out-of-date.

### On all projects

From the root of the repository, run `tsc --build --verbose` to typecheck all projects, as defined
in [tsconfig.json](#tsconfigjson). TypeScript will generate a directed acyclic graph (DAG) and
compile projects in order so that dependencies and references are resolved correctly.

info

Why run TypeScript in the root? Typically you would only want to run against projects, but for
situations where you need to verify that all projects still work, running in the root is the best
approach. Some such situations are upgrading TypeScript itself, upgrading global `@types` packages,
updating shared types, reworking build processes, and more.

### On an individual project

To only typecheck a single project (and its dependencies), there are 2 approaches. The first is to
run from the root, and pass a relative path to the project, such as
`tsc --build --verbose packages/foo`. The second is to change the working directory to the project,
and run from there, such as `cd packages/foo && tsc --build --verbose`.

Both approaches are viable, and either may be used based on your tooling, build system, task runner,
so on and so forth. This is the approach moon suggests with its
[`typecheck` task](/docs/guides/examples/typescript).

### On affected projects

In CI environments, it's nice to only run the typechecker on affected projects — projects that
have changed files. While this isn't entirely possible with `tsc`, it is possible with moon! Head
over to the
[official docs for more information](/docs/run-task#running-based-on-affected-files-only).

## Using `paths` aliases

Path aliases, also known as path mapping or magic imports, is the concept of defining an import
alias that re-maps its underlying location on the file system. In TypeScript, this is achieved with
the
[`paths` compiler option](https://www.typescriptlang.org/docs/handbook/module-resolution.html#path-mapping).

In a monorepo world, we suggest using path aliases on a per-project basis, instead of defining them
"globally" in the root. This gives projects full control of what's available and what they want to
import, and also plays nice with the mandatory `baseUrl` compiler option.

/tsconfig.json

```
{  // ...  "compilerOptions": {    // ...    "baseUrl": ".",    "paths": {      // Within the project      ":components/*": ["./src/components/*"],      // To a referenced project      ":shared/*": ["../../shared/code/*"]    }  },  "references": [    {      "path": "../../shared/code"    }  ]}
```

The above aliases would be imported like the following:

```
// Beforeimport { Button } from '../../../../components/Button';import utils from '../../shared/code/utils';// Afterimport { Button } from ':components/Button';import utils from ':shared/utils';
```

info

When using path aliases, we suggest prefixing or suffixing the alias with `:` so that it's apparent
that it's an alias (this also matches the new `node:` import syntax). Using no special character or
`@` is problematic as it risks a chance of collision with a public npm package and may accidentally
open your repository to a
[supply chain attack](https://snyk.io/blog/npm-security-preventing-supply-chain-attacks/). Other
characters like `~` and `$` have an existing meaning in the ecosystem, so it's best to avoid them
aswell.

### Importing source files from local packages

If you are importing from a project reference using a `package.json` name, then TypeScript will
abide by Node.js module resolution logic, and will import using the
[`main`/`types` or `exports` entry points](https://nodejs.org/api/packages.html#package-entry-points).
This means that you're importing compiled code instead of source code, and will require the
package to be constantly rebuilt if changes are made to it.

However, why not simply import source files instead? With path aliases, you can do just that, by
defining a `paths` alias that maps the `package.json` name to its source files, like so.

/tsconfig.json

```
{  // ...  "compilerOptions": {    // ...    "paths": {      // Index import      "@scope/name": ["../../shared/package/src/index.ts"],      // Deep imports      "@scope/name/*": ["../../shared/package/src/*"]    }  },  "references": [    {      "path": "../../shared/package"    }  ]}
```

When using moon, the
[`typescript.syncProjectReferencesToPaths`](/docs/config/toolchain#syncprojectreferencestopaths)
setting will automatically create `paths` based on the local references.

## Sharing and augmenting types

Declaring global types, augmenting node modules, and sharing reusable types is a common practice.
There are many ways to achieve this, so choose what works best for your repository. We use the
following pattern with great success.

At the root of the repository, create a `types` folder as a sibling to `tsconfig.json`. This folder
must only contain declarations (`.d.ts`) files for the following reasons:

- Declarations can be `include`ed in a project without having to be a project reference.

- Hard-coded declarations do not need to be compiled from TypeScript files.

Based on the above, update your project's `tsconfig.json` to include all of these types, or just
some of these types.

/tsconfig.json

```
{  // ...  "include": ["src/**/*", "../../../../types/**/*"]}
```

In the future, moon will provide a setting to automate this workflow!

## Supporting `package.json` exports

In Node.js v12, they introduced a new field to `package.json` called `exports` that aims to solve
the shortcomings of the `main` field. The `exports` field is very complicated, and instead of
repeating all of its implementation details, we suggest reading
[the official Node.js docs on this topic](https://nodejs.org/api/packages.html#package-entry-points).

With that being said, TypeScript completely ignored the `exports` field until
[v4.7](https://devblogs.microsoft.com/typescript/announcing-typescript-4-7/#esm-nodejs), and
respecting `exports` is still ignored unless the `moduleResolution` compiler option is set to
"nodenext", "node16", or "bundler". If `moduleResolution` is set to "node", then your integration is
resolving based on the `main` and `types` field, which are basically "legacy".

warning

Enabling `package.json` imports/exports resolution is very complicated, and may be very tedious,
especially considering the state of the npm ecosystem. Proceed with caution!

### State of the npm ecosystem

As mentioned above, the npm ecosystem (as of November 2022) is in a very fragile state in regards to
imports/exports. Based on our experience attempting to utilize them in a monorepo, we ran into an
array of problems, some of which are:

- Published packages are simply utilizing imports/exports incorrectly. The semantics around CJS/ESM are very strict, and they may be configured wrong. This is exacerbated by the new `type` field.

- The `exports` field overrides the `main` and `types` fields. If `exports` exists without type conditions, but the `types` field exists, the `types` entry point is completely ignored, resulting in TypeScript failures.

With that being said, there are [ways around this](#resolving-issues) and moving forward is
possible, if you dare!

### Enabling imports/exports resolution

To start, set the `moduleResolution` compiler option to "nodenext" (for packages) or "bundler" (for
apps) in the [`tsconfig.options.json`](#tsconfigoptionsjson) file.

```
{  "compilerOptions": {    // ...    "moduleResolution": "nodenext"  }}
```

Next, [run the typechecker from the root](#on-all-projects) against all projects. This will help
uncover all potential issues with the dependencies you're using or the current configuration
architecture. If no errors are found, well congratulations, otherwise jump to the next section for
more information on [resolving them](#resolving-issues).

If you're trying to use `exports` in your own packages, ensure that the `types` condition is set,
and it's the first condition in the mapping! We also suggest including `main` and the top-level
`types` for tooling that do not support `exports` yet.

package.json

```
{  // ...  "main": "./lib/index.js",  "types": "./lib/index.d.ts",  "exports": {    "./package.json": "./package.json",    ".": {      "types": "./lib/index.d.ts",      "node": "./lib/index.js"    }  }}
```

info

Managing `exports` is non-trivial. If you'd prefer them to be automatically generated based on a set
of inputs, we suggest using [Packemon](https://packemon.dev/)!

### Resolving issues

There's only one way to resolve issues around incorrectly published `exports`, and that is package
patching, either with [Yarn's patching feature](https://yarnpkg.com/features/protocols/#patch),
[pnpm's patching feature](https://pnpm.io/cli/patch), or the
[`patch-package` package](https://www.npmjs.com/package/patch-package). With patching, you can:

- Inject the `types` condition/field if it's missing.

- Re-structure the `exports` mapping if it's incorrect.

- Fix incorrect entry point paths.

- And even fix invalid TypeScript declarations or JavaScript code!

package.json

```
{  "main": "./lib/index.js",  "types": "./lib/index.d.ts",  "exports": {    "./package.json": "./package.json",-    ".": "./lib/index.js"+    ".": {+      "types": "./lib/index.d.ts",+      "node": "./lib/index.js"+    }  }}
```

info

More often than not, the owners of these packages may be unaware that their `exports` mapping is
incorrect. Why not be a good member of the community and report an issue or even submit a pull
request?

## Editor integration

Unfortunately, we only have experience with VS Code. If you prefer another editor and have guidance
you'd like to share with the community, feel free to submit a pull request and we'll include it
below!

### VS Code

[VS Code](https://code.visualstudio.com/) has first-class support for TypeScript and project
references, and should "just work" without any configuration. You can verify this by restarting the
TypeScript server in VS Code (with the cmd + shift + p command palette) and navigating to
each project. Pay attention to the status bar at the bottom, as you'll see this:

When this status appears, it means that VS Code is compiling a project. It will re-appear multiple
times, basically for each project, instead of once for the entire repository.

Furthermore, ensure that VS Code is using the version of TypeScript from the `typescript` package in
`node_modules`. Relying on the version that ships with VS Code may result in unexpected TypeScript
failures.

.vscode/settings.json

```
{  "typescript.tsdk": "node_modules/typescript/lib"  // Or "Select TypeScript version" from the command palette}
```

## FAQ

### I still have questions, where can I ask them?

We'd love to answer your questions and help anyway that we can. Feel free to...

- Join the [moonrepo discord](https://discord.gg/qCh9MEynv2) and post your question in the `#typescript` channel.

- Ping me, [Miles Johnson](https://twitter.com/mileswjohnson), on Twitter. I'll try my best to respond to every tweet.

### Do I have to use project references?

Short answer, no. If you have less than say 10 projects, references may be overkill. If your
repository is primarily an application, but then has a handful of shared npm packages, references
may also be unnecessary here. In the end, it really depends on how many projects exist in the
monorepo, and what your team/company is comfortable with.

However, we do suggest using project references for very large monorepos (think 100s of projects),
or repositories with a large number of contributors, or if you merely want to reduce CI typechecking
times.

### What about not using project references and only using source files?

A popular alternative to project references is to simply use the source files as-is, by updating the
`main` and `types` entry fields within each `package.json` to point to the original TypeScript
files. This approach is also known as "internal packages".

package.json

```
{  // ...  "main": "./src/index.tsx",  "types": "./src/index.tsx"}
```

While this works, there are some downsides to this approach.

- Loading declaration files are much faster than source files.

- You'll lose all the benefits of TypeScript's incremental caching and compilation. TypeScript will consistently load, parse, and evaluate these source files every time. This is especially true for CI environments.

- When using `package.json` workspaces, bundlers and other tools may consider these source files "external" as they're found in `node_modules`. This will require custom configuration to allow it.

- It breaks consistency. Consistency with the npm ecosystem, and consistency with how packaging and TypeScript was designed to work. If all packages are internal, then great, but if you have some packages that are published, you now have 2 distinct patterns for "using packages" instead of 1.

With that being said, theres a 3rd alternative that may be the best of both worlds, using project
references and source files,
[by using `paths` aliases](#importing-source-files-from-local-packages).

All in all, this is a viable approach if you're comfortable with the downsides listed above. Use the
pattern that works best for your repository, team, or company!

### How to integrate with ESLint?

We initially included ESLint integration in this guide, but it was very complex and in-depth on its
own, so we've opted to push it to another guide. Unfortunately, that guide is not yet available, so
please come back soon! We'll announce when it's ready.

### How to handle circular references?

Project references do not support
[circular references](https://github.com/microsoft/TypeScript/issues/33685) (cycles), which is
great, as they are a code smell! If you find yourself arbitrarily importing code from random
sources, or between 2 projects that depend on each other, then this highlights a problem with your
architecture. Projects should be encapsulated and isolated from outside sources, unless explicitly
allowed through a dependency. Dependencies are "upstream", so having them depend on the current
project (the "downstream"), makes little to no sense.

If you're trying to adopt project references and are unfortunately hitting the circular reference
problem, don't fret, untangling is possible, although non-trivial depending on the size of your
repository. It basically boils down to creating an additional project to move coupled code to.

For example, if project A was importing from project B, and B from A, then the solution would be to
create another project, C (typically a shared npm package), and move both pieces of code into C. A
and B would then import from C, instead of from each other. We're not aware of any tools that would
automate this, or detect cycles, so you'll need to do it manually.

## /docs/guides/mcp

Source: https://moonrepo.dev/docs/guides/mcp

# MCP integration

v1.37.0

[Model Context Protocol](https://modelcontextprotocol.io) (MCP) is an open standard that enables AI
models to interact with external tools and services through a unified interface. The moon CLI
contains an MCP server that you can register with your code editor to allow LLMs to use moon
directly.

## Setup

### Claude Code

To use [MCP servers in Claude Code](https://docs.anthropic.com/en/docs/claude-code/mcp), run the
following command in your terminal:

```
claude mcp add moon -s project -e MOON_WORKSPACE_ROOT=/absolute/path/to/your/moon/workspace -- moon mcp
```

Or create an `.mcp.json` file in your project directory.

```
{  "mcpServers": {    "moon": {      "command": "moon",      "args": ["mcp"],      "env": {        "MOON_WORKSPACE_ROOT": "/absolute/path/to/your/moon/workspace"      }    }  }}
```

### Cursor

To use [MCP servers in Cursor](https://docs.cursor.com/context/model-context-protocol), create a
`.cursor/mcp.json` file in your project directory, or `~/.cursor/mcp.json` globally, with the
following content:

.cursor/mcp.json

```
{  "mcpServers": {    "moon": {      "command": "moon",      "args": ["mcp"],      "env": {        "MOON_WORKSPACE_ROOT": "/absolute/path/to/your/moon/workspace"      }    }  }}
```

Once configured, the moon MCP server should appear in the "Available Tools" section on the MCP
settings page in Cursor.

### VS Code

To use MCP servers in VS Code, you must have the
[Copilot Chat](https://code.visualstudio.com/docs/copilot/chat/copilot-chat) extension installed.
Once installed, create a `.vscode/mcp.json` file with the following content:

.vscode/mcp.json

```
{  "servers": {    "moon": {      "type": "stdio",      "command": "moon",      "args": ["mcp"],      // >= 1.102 (June 2025)      "cwd": "${workspaceFolder}",      // Older versions      "env": {        "MOON_WORKSPACE_ROOT": "${workspaceFolder}"      }    }  }}
```

Once your MCP server is configured, you can use it with
[GitHub Copilot’s agent mode](https://code.visualstudio.com/docs/copilot/chat/chat-agent-mode):

- Open the Copilot Chat view in VS Code

- Enable agent mode using the mode select dropdown

- Toggle on moon's MCP tools using the "Tools" button

### Zed

To use [MCP servers in Zed](https://zed.dev/docs/ai/mcp), create a `.zed/settings.json` file in your
project directory, or `~/.config/zed/settings.json` globally, with the following content:

.zed/settings.json

```
{  "context_servers": {    "moon": {      "command": {        "path": "moon",        "args": ["mcp"],        "env": {          "MOON_WORKSPACE_ROOT": "/absolute/path/to/your/moon/workspace"        }      }    }  }}
```

Once your MCP server is configured, you'll need to enable the tools using the following steps:

- Open the Agent panel in Zed

- Click the Write/Ask toggle button and go to "Configure Profiles"

- Click "Customize" in the Ask section

- Click "Configure MCP Tools"

- Enable each tool under the "moon" section

## Available tools

The following tools are available in the moon MCP server and can be executed by LLMs using agent
mode.

- `get_project` - Get a project and its tasks by `id`.

- `get_projects` - Get all projects.

- `get_task` - Get a task by `target`.

- `get_tasks` - Get all tasks.

- `get_touched_files` - Gets touched files between base and head revisions. v1.38.0

- `sync_projects` - Runs the `SyncProject` action for one or many projects by `id`. v1.38.0

- `sync_workspace` - Runs the `SyncWorkspace` action. v1.38.0

info

The
[request and response shapes](https://github.com/moonrepo/moon/blob/master/packages/types/src/mcp.ts)
for these tools are defined as TypeScript types in the
[`@moonrepo/types`](https://www.npmjs.com/package/@moonrepo/types) package.

## /docs/guides/node/examples

Source: https://moonrepo.dev/docs/guides/node/examples

- [Home](/)
- JavaScript
- [Examples](/docs/guides/node/examples)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Node.js examples

[📄️ AngularIn this guide, you'll learn how to integrate Angular into moon.](/docs/guides/examples/angular)

[📄️ AstroIn this guide, you'll learn how to integrate Astro.](/docs/guides/examples/astro)

[📄️ ESLintIn this guide, you'll learn how to integrate ESLint into moon.](/docs/guides/examples/eslint)

[📄️ JestIn this guide, you'll learn how to integrate Jest into moon.](/docs/guides/examples/jest)

[📄️ NestIn this guide, you'll learn how to integrate NestJS into moon.](/docs/guides/examples/nest)

[📄️ NextIn this guide, you'll learn how to integrate Next.js into moon.](/docs/guides/examples/next)

[📄️ NuxtIn this guide, you'll learn how to integrate Nuxt v3, a Vue framework,](/docs/guides/examples/nuxt)

[📄️ PackemonIn this guide, you'll learn how to integrate Packemon into moon. Packemon](/docs/guides/examples/packemon)

[📄️ PrettierIn this guide, you'll learn how to integrate Prettier into moon.](/docs/guides/examples/prettier)

[📄️ ReactReact is an application or library concern, and not a build system one, since the bundling of React](/docs/guides/examples/react)

[📄️ RemixIn this guide, you'll learn how to integrate Remix into moon.](/docs/guides/examples/remix)

[📄️ SolidSolid (also known as SolidJS) is a JavaScript framework for building](/docs/guides/examples/solid)

[📄️ StorybookStorybook is a frontend workshop for building UI components and pages in isolation. Thousands of](/docs/guides/examples/storybook)

[📄️ SvelteKitSvelteKit is built on Svelte, a UI framework that](/docs/guides/examples/sveltekit)

[📄️ TypeScriptIn this guide, you'll learn how to integrate TypeScript into](/docs/guides/examples/typescript)

[📄️ Vite & VitestIn this guide, you'll learn how to integrate Vite and](/docs/guides/examples/vite)

[📄️ VueVue is an application or library concern, and not a build system one, since the bundling of Vue is](/docs/guides/examples/vue)

[TypeScript project references](/docs/guides/javascript/typescript-project-refs)

[Angular](/docs/guides/examples/angular)

## /docs/guides/notifications

Source: https://moonrepo.dev/docs/guides/notifications

# Terminal notifications

v1.38.0

moon is able to send operating system desktop notifications for specific events in the action
pipeline, on behalf of your terminal application. This is useful for continuous feedback loops and
reacting to long-running commands while multi-tasking.

Notifications are opt-in and must be enabled with the
[`notify.terminalNotifications`](/docs/config/workspace#terminalnotifications) setting.

.moon/workspace.yml

```
notifier:  terminalNotifications: 'always'
```

## Setup

Notifications must be enabled at the operating system level.

### Linux

Linux support is based on the [XDG specification](https://en.wikipedia.org/wiki/XDG) and utilizes
D-BUS APIs, primarily the
[`org.freedesktop.Notifications.Notify`](https://www.galago-project.org/specs/notification/0.9/x408.html#command-notify)
method. Refer to your desktop distribution for more information.

Notifications will be sent using the `moon` application name (the current executable).

### macOS

- Open "System Settings" or "System Preferences"

- Select "Notifications" in the left sidebar

- Select your terminal application from the list (e.g., "Terminal", "iTerm", etc)

- Ensure "Allow notifications" is enabled

- Customize the other settings as desired

Notifications will be sent from your currently running terminal application, derived from the
`TERM_PROGRAM` environment variable. If we fail to detect the terminal, it will default to "Finder".

### Windows

Requires Windows 10 or later.

- Open "Settings"

- Go to the "System" panel

- Select "Notifications & Actions" in the left sidebar

- Ensure notifications are enabled

Notifications will be sent from the "Windows Terminal" app if it's currently in use, otherwise from
"Microsoft PowerShell".

## /docs/guides/offline-mode

Source: https://moonrepo.dev/docs/guides/offline-mode

# Offline mode

moon assumes that an internet connection is always available, as we download and install tools into
the toolchain, resolve versions against upstream manifests, and automatically install dependencies.
While this is useful, having a constant internet connection isn't always viable.

To support workflows where internet isn't available or is spotty, moon will automatically check for
an active internet connection, and drop into offline mode if necessary.

## What's disabled when offline

When offline, moon will skip or disable the following:

- Automatic dependency installation will be skipped.

- Toolchain will skip resolving, downloading, and installing tools, and instead use the local cache. If no local cache available, will fallback to binaries found on `PATH`.

- If not available on `PATH`, will fail to run.

- Upgrade and version checks will be skipped.

## Toggling modes

While we automatically check for an internet connection, both online and offline modes can be forced
with the `PROTO_OFFLINE` environment variable. Setting the variable to `1` or `true` will force
offline mode, while `0` and `false` will force online mode.

## Environment variables

Some additional variables to interact with offline checks.

- `PROTO_OFFLINE_TIMEOUT` - Customize the timeout for offline checks (in milliseconds). Defaults to `750`.

- `PROTO_OFFLINE_HOSTS` - Customize additional hosts/IPs to check for offline status. Separate multiple hosts with a `,`.

- `PROTO_OFFLINE_IP_VERSION` - Customize which IP version to support, `4` or `6`. If not defined, supports both.

## /docs/guides/open-source

Source: https://moonrepo.dev/docs/guides/open-source

# Open source usage

Although moon was designed for large monorepos, it can also be used for open source projects,
especially when coupled with our [built-in continuous integration support](/docs/guides/ci).

However, a pain point with moon is that it has an explicitly configured version for each tool in the
[toolchain](/docs/concepts/toolchain), but open source projects typically need to run checks against
multiple versions! To mitigate this problem, you can set the matrix value as an environment
variable, in the format of `MOON__VERSION`.

.github/workflows/ci.yml

```
name: 'Pipeline'on:  push:    branches:      - 'master'  pull_request:jobs:  ci:    name: 'CI'    runs-on: ${{ matrix.os }}    strategy:      matrix:        os: ['ubuntu-latest', 'windows-latest']        node-version: [16, 18, 20]    steps:      # Checkout repository      - uses: 'actions/checkout@v4'        with:          fetch-depth: 0      # Install Node.js      - uses: 'actions/setup-node@v6'      # Install dependencies      - run: 'yarn install --immutable'      # Run moon and affected tasks      - run: 'yarn moon ci'        env:          MOON_NODE_VERSION: ${{ matrix.node-version }}
```

info

This example is only for GitHub actions, but the same mechanism can be applied to other CI
environments.

## Reporting run results

We also suggest using our
[`moonrepo/run-report-action`](https://github.com/marketplace/actions/moon-ci-run-reports) GitHub
action. This action will report the results of a [`moon ci`](/docs/commands/ci) run to a pull request
as a comment and workflow summary.

.github/workflows/ci.yml

```
# ...jobs:  ci:    name: 'CI'    runs-on: 'ubuntu-latest'    steps:      # ...      - run: 'yarn moon ci'      - uses: 'moonrepo/run-report-action@v1'        if: success() || failure()        with:          access-token: ${{ secrets.GITHUB_TOKEN }}
```

The report looks something like the following:

## /docs/guides/pkl-config

Source: https://moonrepo.dev/docs/guides/pkl-config

# Pkl configuration

v1.32.0

While YAML is our official configuration format, we want to support dynamic formats, and as such,
have added support for Pkl. What is Pkl? If you haven't heard of Pkl yet,
[Pkl is a programmable configuration format by Apple](https://pkl-lang.org/). We like Pkl, as it
meets the following requirements:

- Is easy to read and write.

- Is dynamic and programmable (loops, variables, etc).

- Has type-safety / built-in schema support.

- Has Rust serde integration.

The primary requirement that we are hoping to achieve is supporting a configuration format that is
programmable. We want something that has native support for variables, loops, conditions, and
more, so that you could curate and compose your configuration very easily. Hacking this
functionality into YAML is a terrible user experience in our opinion!

## Installing Pkl

Pkl utilizes a client-server architecture, which means that the `pkl` binary must exist in the
environment for parsing and evaluating `.pkl` files. Jump over to the
[official documentation for instructions on how to install Pkl](https://pkl-lang.org/main/current/pkl-cli/index.html#installation).

If you are using [proto](/proto), you can install Pkl with the following commands.

```
proto plugin add pkl https://raw.githubusercontent.com/milesj/proto-plugins/refs/heads/master/pkl.tomlproto install pkl --pin
```

## Using Pkl

To start using Pkl in moon, simply:

- Install [Pkl](#installing-pkl) and the [VS Code extension](https://pkl-lang.org/vscode/current/index.html)

- Create configs with the `.pkl` extension instead of `.yml`

info

We highly suggest reading the Pkl
[language reference](https://pkl-lang.org/main/current/language-reference/index.html), the
[standard library](https://pkl-lang.org/main/current/standard-library.html), or looking at our
[example configurations](#example-configs) when using Pkl.

### Caveats and restrictions

Since this is an entirely new configuration format that is quite dynamic compared to YAML, there are
some key differences to be aware of!

- Only files are supported. Cannot use or extend from URLs.

- Each `.pkl` file is evaluated in isolation (loops are processed, variables assigned, etc). This means that task inheritance and file merging cannot extend or infer this native functionality.

- `default` is a [special feature](https://pkl-lang.org/main/current/language-reference/index.html#default-element) in Pkl and cannot be used as a setting name. This only applies to [`template.pkl`](/docs/config/template#default), but can be worked around by using `defaultValue` instead.

template.pkl

```
variables {  ["age"] {    type = "number"    prompt = "Age?"    defaultValue = 0}
```

- `local` is also a reserved word in Pkl. It can be worked around by escaping it with backticks, or you can simply use the [`preset` setting](/docs/config/project#preset) instead.

```
tasks {  ["example"] {    `local` = true    # Or    preset = "server"  }}
```

## Example configs

### `.moon/workspace.pkl`

```
projects {  globs = List("apps/*", "packages/*")  sources {    ["root"] = "."  }}vcs {  defaultBranch = "master"}
```

### `.moon/toolchain.pkl`

```
node {  version = "20.15.0"  packageManager = "yarn"  yarn {    version = "4.3.1"  }  addEnginesConstraint = false  inferTasksFromScripts = false}
```

### `moon.pkl`

```
type = "application"language = "typescript"dependsOn = List("client", "ui")tasks {  ["build"] {    command = "docusaurus build"    deps = List("^:build")    outputs = List("build")    options {      interactive = true      retryCount = 3    }  }  ["typecheck"] {    command = "tsc --build"    inputs = new Listing {      "@globs(sources)"      "@globs(tests)"      "tsconfig.json"      "/tsconfig.options.json"    }  }}
```

## Example functionality

### Loops and conditionals

```
tasks {  for (_os in List("linux", "macos", "windows")) {    ["build-\(_os)"] {      command = "cargo"      args = List(        "--target",        if (_os == "linux") "x86_64-unknown-linux-gnu"          else if (_os == "macos") "x86_64-apple-darwin"          else "i686-pc-windows-msvc",        "--verbose"      )      options {        os = _os      }    }  }}
```

### Local variables

```
local _sharedInputs = List("src/**/*")tasks {  ["test"] {    // ...    inputs = List("tests/**/*") + _sharedInputs  }  ["lint"] {    // ...    inputs = List("**/*.graphql") + _sharedInputs  }}
```

## /docs/guides/profile

Source: https://moonrepo.dev/docs/guides/profile

# Task profiling

Troubleshooting slow or unperformant tasks? Profile and diagnose them with ease!

caution

Profiling is only supported by `node` based tasks, and is not supported by tasks that are created
through `package.json` inference, or for packages that ship non-JavaScript code (like Rust or Go).

## CPU snapshots

CPU profiling helps you get a better understanding of which parts of your code require the most CPU
time, and how your code is executed and optimized by Node.js. The profiler will measure code
execution and activities performed by the engine itself, such as compilation, calls of system
libraries, optimization, and garbage collection.

### Record a profile

To record a CPU profile, pass `--profile cpu` to the [`moon run`](/docs/commands/run) command. When
successful, the profile will be written to
`.moon/cache/states/
//snapshot.cpuprofile`.

```
$ moon run --profile cpu app:lint
```

### Analyze in Chrome

CPU profiles can be reviewed and analyzed with
[Chrome developer tools](https://developer.chrome.com/docs/devtools/) using the following steps.

- Open Chrome and navigate to `chrome://inspect`.

- Under "Devices", navigate to "Open dedicated DevTools for Node".

- The following window will popup. Ensure the "Profiler" tab is selected.

- Click "Load" and select the `snapshot.cpuprofile` that was [previously recorded](#record-a-profile). If successful, the snapshot will appear in the left column.

On macOS, press `command` + `shift` + `.` to display hidden files and folders, to locate the
`.moon` folder.

- Select the snapshot in the left column. From here, the snapshot can be analyzed and represented with [Bottom up](#bottom-up), [Top down](#top-down), or [Flame chart](#flame-chart) views.

## Heap snapshots

Heap profiling lets you detect memory leaks, dynamic memory problems, and locate the fragments of
code that caused them.

### Record a profile

To record a heap profile, pass `--profile heap` to the [`moon run`](/docs/commands/run) command. When
successful, the profile will be written to
`.moon/cache/states/
//snapshot.heapprofile`.

```
$ moon run --profile heap app:lint
```

### Analyze in Chrome

Heap profiles can be reviewed and analyzed with
[Chrome developer tools](https://developer.chrome.com/docs/devtools/) using the following steps.

- Open Chrome and navigate to `chrome://inspect`.

- Under "Devices", navigate to "Open dedicated DevTools for Node".

- The following window will popup. Ensure the "Memory" tab is selected.

- Click "Load" and select the `snapshot.heapprofile` that was [previously recorded](#record-a-profile-1). If successful, the snapshot will appear in the left column.

On macOS, press `command` + `shift` + `.` to display hidden files and folders, to locate the
`.moon` folder.

- Select the snapshot in the left column. From here, the snapshot can be analyzed and represented with [Bottom up](#bottom-up), [Top down](#top-down), or [Flame chart](#flame-chart) views.

## Views

Chrome DevTools provide 3 views for analyzing activities within a snapshot. Each view gives you a
different perspective on these activities.

### Bottom up

The Bottom up view is helpful if you encounter a heavy function and want to find out where it was
called from.

- The "Self Time" column represents the aggregated time spent directly in that activity, across all of its occurrences.

- The "Total Time" column represents aggregated time spent in that activity or any of its children.

- The "Function" column is the function that was executed, including source location, and any children.

### Top down

The Top down view works in a similar fashion to [Bottom up](#bottom-up), but displays functions
starting from the top-level entry points. These are also known as root activities.

### Flame chart

DevTools represents main thread activity with a flame chart. The x-axis represents the recording
over time. The y-axis represents the call stack. The events on top cause the events below it.

## /docs/guides/remote-cache

Source: https://moonrepo.dev/docs/guides/remote-cache

# Remote caching

Is your CI pipeline running slower than usual? Are you tired of running the same build over and over
although nothing has changed? Do you wish to reuse the same local cache across other machines and
environments? These are just a few scenarios that remote caching aims to solve.

Remote caching is a system that shares artifacts to improve performance, reduce unnecessary
computation time, and alleviate resources. It achieves this by uploading hashed artifacts to a cloud
storage provider, like AWS S3 or Google Cloud, and downloading them on demand when a build matches a
derived hash.

To make use of remote caching, we provide 2 solutions.

## Self-hosted v1.30.0

This solution allows you to host any remote caching service that is compatible with the
[Bazel Remote Execution v2 API](https://github.com/bazelbuild/remote-apis/tree/main/build/bazel/remote/execution/v2),
such as [`bazel-remote`](https://github.com/buchgr/bazel-remote). When using this solution, the
following RE API features must be enabled:

- Action result caching

- Content addressable storage caching

- SHA256 digest hashing

- gRPC requests

warning

This feature and its implementation is currently unstable, and its documentation is incomplete.
Please report any issues on GitHub or through Discord!

### Host your service

When you have chosen (or built) a compatible service, host it and make it available through gRPC (we
do not support HTTP at this time). For example, if you plan to use `bazel-remote`, you can do
something like the following:

```
bazel-remote --dir /path/to/moon-cache --max_size 10 --storage_mode uncompressed --grpc_address 0.0.0.0:9092
```

If you've configured the [`remote.cache.compression`](/docs/config/workspace#compression) setting to
"zstd", you'll need to run the binary with that storage mode as well.

```
bazel-remote --dir /path/to/moon-cache --max_size 10 --storage_mode zstd --grpc_address 0.0.0.0:9092
```

info

View the official [`bazel-remote`](https://github.com/buchgr/bazel-remote#usage) documentation for
all the available options, like storing artifacts in S3, configuring authentication (TLS/mTLS),
proxies, and more.

### Configure remote caching

Once your service is running, you can enable remote caching by configuring the
[`remote`](/docs/config/workspace#remote) settings in [`.moon/workspace.yml`](/docs/config/workspace). At
minimum, the only setting that is required is `host`.

.moon/workspace.yml

```
remote:  host: 'grpc://your-host.com:9092'
```

#### TLS and mTLS

We have rudimentary support for TLS and mTLS, but it's very unstable, and has not been thoroughly
tested. There's also [many](https://github.com/hyperium/tonic/issues/1652)
[many](https://github.com/hyperium/tonic/issues/1989)
[issues](https://github.com/hyperium/tonic/issues/1033) around authentication in Tonic.

.moon/workspace.yml

```
# TLSremote:  host: 'grpcs://your-host.com:9092'  tls:    cert: 'certs/ca.pem'    domain: 'your-host.com'# mTLSremote:  host: 'grpcs://your-host.com:9092'  mtls:    caCert: 'certs/ca.pem'    clientCert: 'certs/client.pem'    clientKey: 'certs/client.key'    domain: 'your-host.com'
```

## Cloud-hosted: Depotv1.32.0

If you'd prefer not to host your own solution, you could use
[Depot Cache](https://depot.dev/products/cache), a cloud-based caching solution. To make use of
Depot, follow these steps:

- Create an account on [depot.dev](https://depot.dev)

- Create an organization

- Go to organization settings -> API tokens

- Create a new API token

- Add the token as a `DEPOT_TOKEN` environment variable to your moon pipelines

Once these steps have been completed, you can enable remote caching in moon with the following
configuration. If your Depot account has more than 1 organization, you'll need to set the
`X-Depot-Org` header.

.moon/workspace.yml

```
remote:  host: 'grpcs://cache.depot.dev'  auth:    token: 'DEPOT_TOKEN'    headers:      'X-Depot-Org': ''
```

## FAQ

#### What is an artifact?

In the context of moon and remote caching, an artifact is the
[outputs of a task](/docs/config/project#outputs), as well as the stdout and stderr of the task that
generated the outputs. Artifacts are uniquely identified by the
[moon generated hash](/docs/concepts/cache#hashing).

#### Do I have to use remote caching?

No, remote caching is optional. It's intended purpose is to store long lived build artifacts to
speed up CI pipelines, and optionally local development. For the most part,
[`moon ci`](/docs/commands/ci) does a great job of only running what's affected in pull requests, and
is a great starting point.

#### Does remote caching store source code?

No, remote caching does not store source code. It stores the
[outputs of a task](/docs/config/project#outputs), which is typically built and compiled code. To
verify this, you can inspect the tar archives in `.moon/cache/outputs`.

#### Does moon collect any personally identifiable information?

No, moon does not collect any PII as part of the remote caching process.

#### Are artifacts encrypted?

We do not encrypt on moon's side, as encryption is provided by your cloud storage provider.

## /docs/guides/root-project

Source: https://moonrepo.dev/docs/guides/root-project

# Root-level project

Coming from other repositories or task runner, you may be familiar with tasks available at the
repository root, in which one-off, organization, maintenance, or process oriented tasks can be ran.
moon supports this through a concept known as a root-level project.

Begin by adding the root to [`projects`](/docs/config/workspace#projects) with a source value of `.`
(current directory relative from the workspace).

.moon/workspace.yml

```
# As a mapprojects:  root: '.'# As a list of globsprojects:  - '.'
```

When using globs, the root project's name will be inferred from the repository folder name. Be
wary of this as it can change based on what a developer has checked out as.

Once added, create a [`moon.yml`](/docs/config/project) in the root of the repository. From here you
can define tasks that can be ran using this new root-level project name, for example,
`moon run root:`.

moon.yml

```
tasks:  versionCheck:    command: 'yarn version check'    inputs: []    options:      cache: false
```

And that's it, but there are a few caveats to be aware of...

## Caveats

### Greedy inputs

warning

In moon v1.24, root-level tasks default to no inputs. In previous versions, inputs defaulted to
`**/*`. This section is only applicable for older moon versions!

Task [`inputs`](/docs/config/project#inputs) default to `**/*`, which would result in root-level tasks
scanning all files in the repository. This will be a very expensive operation! We suggest
restricting inputs to a very succinct whitelist, or disabling inputs entirely.

moon.yml

```
tasks:  oneOff:    # ...    inputs: []
```

### Inherited tasks

Because a root project is still a project in the workspace, it will inherit all tasks defined in
[`.moon/tasks/all.yml`](/docs/config/tasks), which may be unexpected. To mitigate this, you can exclude
some or all of these tasks in the root config with
[`workspace.inheritedTasks`](/docs/config/project#inheritedtasks).

moon.yml

```
workspace:  inheritedTasks:    include: []
```

## /docs/guides/rust/handbook

Source: https://moonrepo.dev/docs/guides/rust/handbook

# Rust handbook

Utilizing Rust in a monorepo is a trivial task, thanks to Cargo, and also moon. With this handbook,
we'll help guide you through this process.

info

moon is not a build system and does not replace Cargo. Instead, moon runs `cargo` commands, and
efficiently orchestrates those tasks within the workspace.

## moon setup

For this part of the handbook, we'll be focusing on [moon](/moon), our task runner. To start,
languages in moon act like plugins, where their functionality and support is not enabled unless
explicitly configured. We follow this approach to avoid unnecessary overhead.

### Enabling the language

To enable Rust, define the [`rust`](/docs/config/toolchain#rust) setting in
[`.moon/toolchains.yml`](/docs/config/toolchain), even if an empty object.

.moon/toolchains.yml

```
# Enable Rustrust: {}# Enable Rust and override default settingsrust:  syncToolchainConfig: true
```

Or by pinning a `rust` version in [`.prototools`](/docs/proto/config) in the workspace root.

.prototools

```
rust = "1.69.0"
```

This will enable the Rust toolchain and provide the following automations around its ecosystem:

- Manifests and lockfiles are parsed for accurate dependency versions for hashing purposes.

- Cargo binaries (in `~/.cargo/bin`) are properly located and executed.

- Automatically sync `rust-toolchain.toml` configuration files.

- For non-workspaces, will inherit `package.name` from `Cargo.toml` as a project alias.

- And more to come!

### Utilizing the toolchain

When a language is enabled, moon by default will assume that the language's binary is available
within the current environment (typically on `PATH`). This has the downside of requiring all
developers and machines to manually install the correct version of the language, and to stay in
sync.

Instead, you can utilize [moon's toolchain](/docs/concepts/toolchain), which will download and
install the language in the background, and ensure every task is executed using the exact version
across all machines.

Enabling the toolchain is as simple as defining the
[`rust.version`](/docs/config/toolchain#version-2) setting.

.moon/toolchains.yml

```
# Enable Rust toolchain with an explicit versionrust:  version: '1.69.0'
```

Versions can also be defined with [`.prototools`](/docs/proto/config).

caution

moon requires `rustup` to exist in the environment, and will use this to install the necessary Rust
toolchains. This requires Rust to be manually installed on the machine, as moon does not
auto-install the language, just the toolchains.

## Repository structure

Rust/Cargo repositories come in two flavors: a single crate with one `Cargo.toml`, or multiple
crates with many `Cargo.toml`s using
[Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). The latter is
highly preferred as it enables Cargo incremental caching.

Regardless of which flavor your repository uses, in moon, both flavors are a single
[moon project](/docs/concepts/project). This means that all Rust crates are grouped together into a
single moon project, and the [`moon.yml`](/docs/config/project) file is located at the root relative
to `Cargo.lock` and the `target` folder.

An example of this layout is demonstrated below:

- Workspaces
- Non-workspaces

```
/├── .moon/├── crates/│   ├── client/|   │   ├── ...│   │   └── Cargo.toml│   ├── server/|   │   ├── ...│   │   └── Cargo.toml│   └── utils/|       ├── ...│       └── Cargo.toml├── target/├── Cargo.lock├── Cargo.toml└── moon.yml
```

```
/├── .moon/├── src/│   └── lib.rs├── tests/│   └── ...├── target/├── Cargo.lock├── Cargo.toml└── moon.yml
```

### Example `moon.yml`

The following configuration represents a base that covers most Rust projects.

- Workspaces
- Non-workspaces

/moon.yml

```
language: 'rust'layer: 'application'env:  CARGO_TERM_COLOR: 'always'fileGroups:  sources:    - 'crates/*/src/**/*'    - 'crates/*/Cargo.toml'    - 'Cargo.toml'  tests:    - 'crates/*/benches/**/*'    - 'crates/*/tests/**/*'tasks:  build:    command: 'cargo build'    inputs:      - '@globs(sources)'  check:    command: 'cargo check --workspace'    inputs:      - '@globs(sources)'  format:    command: 'cargo fmt --all --check'    inputs:      - '@globs(sources)'      - '@globs(tests)'  lint:    command: 'cargo clippy --workspace'    inputs:      - '@globs(sources)'      - '@globs(tests)'  test:    command: 'cargo test --workspace'    inputs:      - '@globs(sources)'      - '@globs(tests)'
```

/moon.yml

```
language: 'rust'layer: 'application'env:  CARGO_TERM_COLOR: 'always'fileGroups:  sources:    - 'src/**/*'    - 'Cargo.toml'  tests:    - 'benches/**/*'    - 'tests/**/*'tasks:  build:    command: 'cargo build'    inputs:      - '@globs(sources)'  check:    command: 'cargo check'    inputs:      - '@globs(sources)'  format:    command: 'cargo fmt --check'    inputs:      - '@globs(sources)'      - '@globs(tests)'  lint:    command: 'cargo clippy'    inputs:      - '@globs(sources)'      - '@globs(tests)'  test:    command: 'cargo test'    inputs:      - '@globs(sources)'      - '@globs(tests)'
```

## Cargo integration

You can't use Rust without Cargo -- well you could but why would you do that? With moon, we're doing
our best to integrate with Cargo as much as possible. Here's a few of the benefits we currently
provide.

### Global binaries

Cargo supports global binaries through the
[`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html) command, which
installs a crate to `~/.cargo/bin`, or makes it available through the `cargo ` command. These
are extremely beneficial for development, but they do require every developer to manually install
the crate (and appropriate version) to their machine.

With moon, this is no longer an issue with the [`rust.bins`](/docs/config/toolchain#bins) setting.
This setting requires a list of crates (with optional versions) to install, and moon will install
them as part of the task runner install dependencies action. Furthermore, binaries will be installed
with [`cargo-binstall`](https://crates.io/crates/cargo-binstall) in an effort to reduce build and
compilation times.

.moon/toolchains.yml

```
rust:  bins:    - 'cargo-make@0.35.0'    - 'cargo-nextest'
```

At this point, tasks can be configured to run this binary as a command. The `cargo` prefix is
optional, as we'll inject it when necessary.

/moon.yml

```
tasks:  test:    command: 'nextest run --workspace'    toolchain: 'rust'
```

tip

The `cargo-binstall` crate may require a `GITHUB_TOKEN` environment variable to make GitHub Releases
API requests, especially in CI. If you're being rate limited, or fail to find a download, try
creating a token with necessary permissions.

### Lockfile handling

To expand our integration even further, we also take `Cargo.lock` into account, and apply the
following automations when a target is being ran:

- If the lockfile does not exist, we generate one with [`cargo generate-lockfile`](https://doc.rust-lang.org/cargo/commands/cargo-generate-lockfile.html).

- We parse and extract the resolved checksums and versions for more accurate hashing.

## FAQ

### Should we cache the `target` directory as an output?

No, we don't believe so. Both moon and Cargo support incremental caching, but they're not entirely
compatible, and will most likely cause problems when used together.

The biggest factor is that moon's caching and hydration uses a tarball strategy, where each task
would unpack a tarball on cache hit, and archive a tarball on cache miss. The Cargo target directory
is extremely large (moon's is around 50gb), and coupling this with our tarball strategy is not
viable. This would cause massive performance degradation.

However, at maximum, you could cache the compiled binary itself as an output, instead of the
entire target directory. Example:

moon.yml

```
tasks:  build:    command: 'cargo build --release'    outputs: ['target/release/moon']
```

### How can we improve CI times?

Rust is known for slow build times and CI is no exception. With that being said, there are a few
patterns to help alleviate this, both on the moon side and outside of it.

To start, you can cache Rust builds in CI. This is a non-moon solution to the `target` directory
problem above.

- If you use GitHub Actions, feel free to use our [moonrepo/setup-rust](https://github.com/moonrepo/setup-rust) action, which has built-in caching.

- A more integrated solution is [sccache](https://crates.io/crates/sccache), which stores build artifacts in a cloud storage provider.

## /docs/guides/sharing-config

Source: https://moonrepo.dev/docs/guides/sharing-config

# Sharing workspace configuration

For large companies, open source maintainers, and those that love reusability, more often than not
you'll want to use the same configuration across all repositories for consistency. This helps reduce
the maintenance burden while ensuring a similar developer experience.

To help streamline this process, moon provides an `extends` setting in both
[`.moon/workspace.yml`](/docs/config/workspace#extends),
[`.moon/toolchains.yml`](/docs/config/toolchain#extends), and
[`.moon/tasks/all.yml`](/docs/config/tasks#extends). This setting requires a HTTPS URL or relative
file system path that points to a valid YAML document for the configuration in question.

A great way to share configuration is by using GitHub's "raw file view", as demonstrated below using
our very own [examples repository](https://github.com/moonrepo/examples).

.moon/tasks/all.yml

```
extends: 'https://raw.githubusercontent.com/moonrepo/examples/master/.moon/tasks/all.yml'
```

## Versioning

Inheriting an upstream configuration can be dangerous, as the settings may change at any point,
resulting in broken builds. To mitigate this, you can used a "versioned" upstream configuration,
which is ideally a fixed point in time. How this is implemented is up to you or your company, but we
suggest the following patterns:

### Using versioned filenames

A rudimentary solution is to append a version to the upstream filename. When the file is modified, a
new version should be created, while the previous version remains untouched.

```
-extends: '../shared/project.yml'+extends: '../shared/project-v1.yml'
```

### Using branches, tags, or commits

When using a version control platform, like GitHub above, you can reference the upstream
configuration through a branch, tag, commit, or sha. Since these are a reference point in time, they
are relatively safe.

```
-extends: 'https://raw.githubusercontent.com/moonrepo/examples/master/.moon/tasks/all.yml'+extends: 'https://raw.githubusercontent.com/moonrepo/examples/c3f10160bcd16b48b8d4d21b208bb50f6b09bd96/.moon/tasks/all.yml'
```

## /docs/guides/vcs-hooks

Source: https://moonrepo.dev/docs/guides/vcs-hooks

# VCS hooks

v1.9.0

VCS hooks (most popular with [Git](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)) are a
mechanism for running scripts at pre-defined phases in the VCS's lifecycle, most commonly
pre-commit, pre-push, or pre-merge. With moon, we provide a built-in solution for managing hooks,
and syncing them across developers and machines.

- [Learn more about Git hooks](https://git-scm.com/docs/githooks)

## Defining hooks

Hooks can be configured with the [`vcs.hooks`](/docs/config/workspace#hooks) setting in
[`.moon/workspace.yml`](/docs/config/workspace). This setting requires a map of hook names (in the
format required by your VCS), to a list of arbitrary commands to run within the hook script.
Commands are used as-is and are not formatted or interpolated in any way.

To demonstrate this, let's configure a `pre-commit` hook that runs a moon `lint` task for affected
projects, and also verifies that the commit message abides by a specified format (using
[pre-commit](https://pre-commit.com/) and the
[commitlint hook](https://github.com/alessandrojcm/commitlint-pre-commit-hook), for example).

.moon/workspace.yml

```
vcs:  hooks:    pre-commit:      - 'pre-commit run'      - 'moon run :lint --affected'    commit-msg:      - 'pre-commit run --hook-stage commit-msg --commit-msg-filename $ARG1'
```

info

All commands are executed from the repository root (not moon's workspace root) and must exist on
`PATH`. If `moon` is installed locally, you can execute it using a repository relative path, like
`./node_modules/@moonrepo/cli/moon`.

### Accessing argumentsv1.40.3

To ease interoperability between operating systems and terminal shells, we set passed arguments as
environment variables.

In your hook commands, you can access these arguments using the `$ARG` format, where `` is the
1-indexed position of the argument. For example, to access the first argument, you would use
`$ARG1`, the second argument would be `$ARG2`, and so on. `$ARG0` exists and points to the current
script.

## Enabling hooks

Hooks are a divisive subject, as some developers love them, and others hate them. Finding a viable
solution for everyone can be difficult, so with moon, we opted to support 2 distinct options, but
only 1 can be used at a time. Choose the option that works best for your project, team, or company!

caution

If you have existing VCS hooks, back them up as moon's implementation will overwrite them! To
migrate your existing hooks, [configure them as commands to run](#defining-hooks).

### Automatically for everyone

If you'd like hooks to be enforced for every contributor of the repository, then simply enable the
[`vcs.syncHooks`](/docs/config/workspace#synchooks) setting in
[`.moon/workspace.yml`](/docs/config/workspace). This will automatically generate hook scripts and link
them with the local VCS checkout, everytime a [target](/docs/concepts/target) is ran.

.moon/workspace.yml

```
vcs:  hooks: [...]  syncHooks: true
```

caution

Automatically activating hooks on everyone's computer is considered a sensitive action, because it
enables the execution of arbitrary code on the computers of the team members. Be careful about the
hook commands you define in the [`.moon/workspace.yml`](/docs/config/workspace) file.

### Manually by each developer

If you'd prefer contributors to have a choice in whether or not they want to use hooks, then simply
do nothing, and guide them to run the [`moon sync hooks`](/docs/commands/sync/vcs-hooks) command. This
command will generate hook scripts and link them with the local VCS checkout.

```
$ moon sync hooks
```

## Disabling hooks

If you choose to stop using hooks, you'll need to cleanup the previously generated hook scripts, and
reset the VCS checkout. To start, disable the `vcs.syncHooks` setting.

.moon/workspace.yml

```
vcs:  syncHooks: false
```

And then run the following command, which will delete files from your local filesystem. Every
developer that is using hooks will need to run this command.

```
$ moon sync hooks --clean
```

## How it works

When hooks are [enabled](#enabling-hooks), the following processes will take place.

- The configured [hooks](#defining-hooks) will be generated as individual script files in the `.moon/hooks` directory. Whether or not you commit or ignore these script files is your choice. They are written to the `.moon` directory so that they can be reviewed, audited, and easily tested, but are required.

- We then sync these generated hook scripts with the current VCS. For Git, we create `.git/hooks` files that execute our generated scripts, using repository relative commands. Any existing VCS hooks will be overwritten.

info

The `.moon/hooks` scripts are generated as Bash scripts (use a `.sh` file extension) on Unix, and
PowerShell scripts (use a `.ps1` file extension) on Windows.

### Git

On Unix based operating systems (Linux, macOS, etc), the `.moon/hooks` scripts are executed from
`.git/hooks` Bash files. Because of this, `bash` should be available on the system (which is
typically the case).

On Windows, things get tricky. Since Git has a requirement that `.git/hooks` files must be
extensionless, and older versions of PowerShell require an extension, we have to use a workaround.
To handle this, the `.git/hooks` files are Bash-like scripts (that should work on most machines)
that execute `.moon/hooks` using the `powershell.exe` (or `pwsh.exe`) executables. Because of this,
PowerShell must be available on the system.

## Examples

### Pre-commit

A perfect use case for the `pre-commit` hook is to check linting and formatting of the files being
committed. If either of these tasks fail, the commit will abort until they are fixed. Be sure to use
the [`--affected`](/docs/run-task#running-based-on-affected-files-only) option so that we only run on
changed projects!

.moon/workspace.yml

```
vcs:  hooks:    pre-commit:      - 'moon run :lint :format --affected --status=staged'
```

By default this will run on the entire project (all files). If you want to filter it to only the
changed files, enable the [`affectedFiles`](/docs/config/project#affectedfiles) task option.

## /docs/guides/wasm-plugins

Source: https://moonrepo.dev/docs/guides/wasm-plugins

# WASM plugins

[moon](/moon) and [proto](/proto) plugins can be written in
[WebAssembly (WASM)](https://webassembly.org/), a portable binary format. This means that plugins
can be written in any language that compiles to WASM, like Rust, C, C++, Go, TypeScript, and more.
Because WASM based plugins are powered by a programming language, they implicitly support complex
business logic and behavior, have access to a sandboxed file system (via WASI), can execute child
processes, and much more.

danger

Since our WASM plugin implementations are still experimental, expect breaking changes to occur in
non-major releases.

## Powered by Extism

Our WASM plugin system is powered by [Extism](https://extism.org/), a Rust-based cross-language
framework for building WASM plugins under a unified guest and host API. Under the hood, Extism uses
[wasmtime](https://wasmtime.dev/) as its WASM runtime.

For the most part, you do not need to know about Extism's host SDK, as we have implemented the
bulk of it within moon and proto directly. However, you should be familiar with the guest PDKs, as
this is what you'll be using to implement Rust-based plugins. We suggest reading the following
material:

- [Plugin development kits](https://extism.org/docs/concepts/pdk) (PDKs)

- The [extism-pdk](https://github.com/extism/rust-pdk) Rust crate

- [Host functions](https://extism.org/docs/concepts/host-functions) (how they work)

## Concepts

Before we begin, let's talk about a few concepts that are critical to WASM and our plugin systems.

### Plugin identifier

When implementing plugin functions, you'll need to access information about the current plugin. To
get the current plugin identifier (the key the plugin was configured with), use the
[`get_plugin_id`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.get_plugin_id.html) function.

```
let id = get_plugin_id();
```

### Virtual paths

WASM by default does not have access to the host file system, but through [WASI](https://wasi.dev/),
we can provide sandboxed access to a pre-defined list of allowed directories. We call these
[virtual paths](https://docs.rs/warpgate_api/latest/warpgate_api/enum.VirtualPath.html), and all
paths provided via function input or context use them.

Virtual paths are implemented by mapping a real path (host machine) to a virtual path (guest
runtime) using file path prefixes. The following prefixes are currently supported:

Real path Virtual path Only for

`~` `/userhome` ~

`~/.proto` `/proto` ~

`~/.moon` `/moon` moon

moon workspace `/workspace` moon

For example, from the context of WASM, you may have a virtual path of `/proto/tools/node/1.2.3`,
which simply maps back to `~/.proto/tools/node/1.2.3` on the host machine. However, this should
almost always be transparent to you, the developer, and to end users.

However, there may be a few cases where you need access to the real path from WASM, for example,
logging or executing commands. For this, the real path can be accessed with the
[`real_path`](https://docs.rs/warpgate_api/latest/warpgate_api/enum.VirtualPath.html#method.real_path)
function on the `VirtualPath` enum (this is a Rust only feature).

```
virtual_path.real_path();
```

#### File system caveats

When working with the file system from the context of WASM, there are a few caveats to be aware of.

- All `fs` calls must use the virtual path. Real paths will error.

- Paths not white listed (using prefixes above) will error.

- Changing file permissions is not supported (on Unix and Windows). This is because WASI does not support this.

- This also means operations like unpacking archives is not possible.

### Host environment

Since WASM executes in its own runtime, it does not have access to the current host operating
system, architecture, so on and so forth. To bridge this gap, we provide the
[`get_host_environment`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.get_host_environment.html)
function.
[Learn more about this type](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/struct.HostEnvironment.html).

```
let env = get_host_environment()?;
```

The host operating system and architecture can be accessed with `os` and `arch` fields respectively.
Both fields are an enum in Rust, or a string in other languages.

```
if env.os == HostOS::Windows {    // Windows only}if env.arch == HostArch::Arm64 {    // aarch64 only}
```

Furthermore, the user's home directory (`~`) can be accessed with the `home_dir` field, which is a
[virtual path](#virtual-paths).

```
if env.home_dir.join(some_path).exists() {    // Do something}
```

### Host functions & macros

WASM is pretty powerful but it can't do everything since it's sandboxed. To work around this, we
provide a mechanism known as host functions, which are functions that are implemented on the host
(in Rust), and can be executed from WASM. The following host functions are currently available:

- [`exec_command`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.exec_command.html) - Execute a system command on the host machine, with a provided list of arguments or environment variables.

- [`from_virtual_path`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.real_path.html) - Converts a virtual path into a real path.

- [`get_env_var`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.host_env.html) - Get an environment variable value from the host environment.

- [`host_log`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.host_log.html) - Log an stdout, stderr, or tracing message to the host's terminal.

- [`send_request`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.send_request.html) - Requests a URL on the host machine using a Rust-based HTTP client (not WASM).

- [`set_env_var`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.host_env.html) - Set an environment variable to the host environment.

- [`to_virtual_path`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.virtual_path.html) - Converts a real path into a virtual path.

To use host functions, you'll need to make them available by registering them at the top of your
Rust file (only add the functions you want to use) using the
[extism-pdk](https://crates.io/crates/extism-pdk) crate.

```
use extism_pdk::*;#[host_fn]extern "ExtismHost" {    fn exec_command(input: Json) -> Json;    fn from_virtual_path(path: String) -> String;    fn get_env_var(key: String) -> String;    fn host_log(input: Json);    fn send_request(input: Json) -> Json;    fn set_env_var(key: String, value: String);    fn to_virtual_path(path: String) -> Json;}
```

info

To simplify development, we provide built-in functions and macros for the host functions above.
Continue reading for more information on these macros.

#### Converting paths

When working with virtual paths, you may need to convert them to real paths, and vice versa. The
[`into_virtual_path`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.into_virtual_path.html)
and [`into_real_path`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.into_real_path.html)
functions can be used for such situations, which use the `to_virtual_path` and `from_virtual_path`
host functions respectively.

```
// Supports strings or pathslet virt = into_virtual_path("/some/real/path")?;let real = into_real_path(PathBuf::from("/some/virtual/path"))?;
```

#### Environment variables

The [`get_host_env_var`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.get_host_env_var.html)
and [`set_host_env_var`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.set_host_env_var.html)
functions can be used to read and write environment variables on the host, using the `set_env_var`
and `get_env_var` host functions respectively.

```
// Set a valueset_host_env_var("ENV_VAR", "value")?;// Get a value (returns an `Option`)let value = get_host_env_var("ENV_VAR")?;
```

Additionally, the
[`add_host_paths`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.add_host_paths.html) function
can be used to append paths to the `PATH` environment variable.

```
// Append to pathadd_host_paths(["/userhome/some/virtual/path"])?;
```

#### Executing commands

The [`exec_command!`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.exec_command.html)
macro can be used to execute a command on the host, using the `exec_command` host function. If the
command does not exist on `PATH`, an error is thrown. This macros supports three modes: pipe,
inherit, and raw (returns `Result`).

```
let result = exec_command!(raw, "which", ["node"]);
```

If you want a simpler API, the
[`exec`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.exec.html),
[`exec_captured`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.exec_captured.html) (pipe),
and [`exec_streamed`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.exec_streamed.html)
(inherit) functions can be used.

```
// Pipe stdout/stderrlet output = exec_captured("which", ["node"])?;// Inherit stdout/stderrexec_streamed("npm", ["install"])?;// Full controlexec(ExecCommandInput {    command: "npm".into(),    args: vec!["install".into()],    ..ExecCommandInput::default()})?;
```

#### Sending requests

The [`send_request`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.send_request.html) macro
can be used to request a URL on the host, instead of from WASM, allowing it to use the same HTTP
client as the host CLI. This macro returns a response object, with the raw body in bytes, and the
status code.

```
let response = send_request!("https://some.com/url/to/fetch");if response.status == 200 {  let json = response.json::()?;  let text = response.text()?;} else {  // Error!}
```

To simplify the handling of requests -> responses, we also provide the
[`fetch_bytes`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.fetch_bytes.html),
[`fetch_json`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.fetch_json.html), and
[`fetch_text`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.fetch_text.html) functions.

```
let json: T = fetch_json("https://some.com/url/to/fetch.json")?;
```

Only GET requests are supported.

#### Logging

The [`host_log!`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.host_log.html) macro can be
used to write stdout or stderr messages to the host's terminal, using the `host_log` host function.
It supports the same argument patterns as `format!`.

If you want full control, like providing data/fields, use the input mode and provide
[`HostLogInput`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/struct.HostLogInput.html).

```
host_log!(stdout, "Some message");host_log!(stderr, "Some message with {}", "args");// With datahost_log!(input, HostLogInput {    message: "Some message with data".into(),    data: HashMap::from_iter([        ("data".into(), serde_json::to_value(data)?),    ]),    target: HostLogTarget::Stderr,});
```

Furthermore, the [extism-pdk](https://crates.io/crates/extism-pdk) crate provides a handful of
macros for writing level-based messages that'll appear in the host's terminal when `--log` is
enabled in the CLI. These also support arguments.

```
debug!("This is a debug message");info!("Something informational happened");warn!("Proceed with caution");error!("Oh no, something went wrong");
```

## Configuring plugin locations

To use a WASM plugin, it'll need to be configured in both moon and proto. Luckily both tools use a
similar approach for configuring plugins called the
[plugin locator](https://docs.rs/warpgate/latest/warpgate/enum.PluginLocator.html). A locator string
is composed of 2 parts separated by `://`, the former is the protocol, and the latter is the
location.

```
"
://"
```

The following locator patterns are supported:

### `file`

The `file://` protocol represents a file path, either absolute or relative (from the current
configuration file).

```
# Relative"file://./path/to/example.wasm"# Absolute"file:///root/path/to/example.wasm"
```

### `github`

The `github://` protocol can be used to target and download an asset from a specific GitHub release.
The location must be an organization + repository slug (owner/repo), and the release must have a
`.wasm` asset available to download.

```
"github://moonrepo/example-repo"
```

If you are targeting releases in a monorepo, you can append the project name after the repository.
The project name will be used as a prefix for tags, and will match `@v?` or
`-v?` based tags.

```
"github://moonrepo/example-repo/project-name"
```

By default, the latest release will be used and cached for 7 days. If you'd prefer to target a
specific release (preferred), append the release tag to the end of the location.

```
"github://moonrepo/example-repo@v1.2.3"
```

This strategy is powered by the [GitHub API](https://api.github.com/) and is subject to rate
limiting. If running in a CI environment, we suggesting setting a `GITHUB_TOKEN` environment
variable to authorize API requests with. If using GitHub Actions, it's as simple as:

```
# In some job or step...env:  GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}'
```

### `https`

The `https://` protocol is your standard URL, and must point to an absolute file path. Files will be
downloaded to `~/.moon/plugins` or `~/.proto/plugins`. Non-secure URLs are not supported!

```
"https://domain.com/path/to/plugins/example.wasm"
```

## Creating a plugin

info

Although plugins can be written in any language that compiles to WASM, we've only tested Rust. The
rest of this article assume you're using Rust and Cargo! Refer to [Extism](https://extism.org/)'s
documentation for other examples.

To start, create a new crate with Cargo:

```
cargo new plugin --libcd plugin
```

Set the lib type to `cdylib`, and provide other required settings.

Cargo.toml

```
[package]name = "example_plugin"version = "0.0.1"edition = "2024"publish = false[lib]crate-type = ['cdylib'][profile.release]codegen-units = 1debug = falselto = trueopt-level = "s"panic = "abort"
```

Our Rust plugins are powered by [Extism](https://extism.org/), so lets add their PDK and ours as a
dependency.

```
cargo add extism-pdk# For protocargo add proto_pdk# For mooncargo add moon_pdk
```

In all Rust files, we can import all the PDKs with the following:

src/lib.rs

```
use extism_pdk::*;
```

We can then build the WASM binary. The file will be available at
`target/wasm32-wasip1/debug/.wasm`.

```
cargo build --target wasm32-wasip1
```

## Building and publishing

At this point, you should have a fully working WASM plugin, but to make it available to the
community, you'll still need to build and make the `.wasm` file available. The easiest solution is
to publish a GitHub release and include the `.wasm` file as an asset.

### Building, optimizing, and stripping

WASM files are pretty fat, even when compiling in release mode. To reduce the size of these files,
we can use `wasm-opt` and `wasm-strip`, both of which are provided by the
[WebAssembly](https://github.com/WebAssembly) group. The following script is what we use to build
our own plugins.

info

This functionality is natively supported in our
[moonrepo/build-wasm-plugin](https://github.com/moonrepo/build-wasm-plugin) GitHub Action!

build-wasm

```
#!/usr/bin/env bashtarget="${CARGO_TARGET_DIR:-target}"input="$target/wasm32-wasip1/release/$1.wasm"output="$target/wasm32-wasip1/$1.wasm"echo "Building"cargo build --target wasm32-wasip1 --releaseecho "Optimizing"# https://github.com/WebAssembly/binaryen~/binaryen/bin/wasm-opt -Os "$input" --output "$output"echo "Stripping"# https://github.com/WebAssembly/wabt~/wabt/bin/wasm-strip "$output"
```

### Manually create releases

When your plugin is ready to be published, you can create a release on GitHub using the following
steps.

- Tag the release and push to GitHub.

```
git tag v0.0.1git push --tags
```

- Build a release version of the plugin using the `build-wasm` script above. The file will be available at `target/wasm32-wasip1/.wasm`.

```
build-wasm
```

- In GitHub, navigate to the tags page, find the new tag, create a new release, and attach the built file as an asset.

### Automate releases

If you're using GitHub Actions, you can automate the release process with our official
[moonrepo/build-wasm-plugin](https://github.com/moonrepo/build-wasm-plugin) action.

- Create a new workflow file at `.github/workflows/release.yml`. Refer to the link above for a working example.

- Tag the release and push to GitHub.

```
# In a polyrepogit tag v0.0.1# In a monorepogit tag example_plugin-v0.0.1# Push the tagsgit push --tags
```

- The action will automatically build the plugin, create a release, and attach the built file as an asset.

## /docs/guides/webhooks

Source: https://moonrepo.dev/docs/guides/webhooks

# Webhooks (experimental)

Looking to gather metrics for your pipelines? Gain insight into run durations and failures? Maybe
you want to send Slack or Discord notifications? With our webhooks, all of these are possible!

When the [`notifier.webhookUrl`](/docs/config/workspace#webhookurl) setting is configured with an HTTPS
URL, and moon is running in a CI environment, moon will POST a payload to this endpoint for every
event in our pipeline.

## Payload structure

Every webhook event is posted with the following request body, known as a payload.

- `type` (`string`) - The type of [event](#events).

- `environment` (`object | null`) - Information about the current CI/CD pipeline environment.

- `event` (`object`) - The event specific payload. View each event for an example of their structure.

- `createdAt` (`string`) - When the event was created, as a UTC timestamp in ISO 8601 (RFC 3339) format.

- `uuid` (`string`) - A unique identifier for all webhooks in the current run batch.

- `trace` (`string`) - A unique identifier for all webhooks in the overall run batch. Can be defined via `MOON_TRACE_ID` environment variable.

```
{  "type": "...",  "environment": "...",  "event": {    // ...  },  "createdAt": "...",  "uuid": "...",  "trace": "..."}
```

The `uuid` field can be used to differentiate concurrently running pipelines!

### Pipeline environment

When webhooks are sent from a CI/CD pipeline, we attempt to include information about the
environment under the `environment` field. If information could not be detected, this field is null,
otherwise it contains these fields.

- `baseBranch` (`string | null`) - When a merge/pull request, the target (base) branch, otherwise null.

- `branch` (`string`) - When a merge/pull request, the source (head) branch, otherwise the triggering branch.

- `id` (`string`) - ID of the current pipeline instance.

- `provider` (`string`) - Name of your CI/CD provider. GitHub Actions, GitLab, CircleCI, etc.

- `requestId` (`string | null`) - The ID of the merge/pull request.

- `requestUrl` (`string | null`) - Link to the merge/pull request.

- `revision` (`string`) - The HEAD commit, revision, tag, ref, etc, that triggered the pipeline.

- `url` (`string | null`) - Link to the current pipeline, when available.

## Events

### Pipeline

Runs actions within moon using a robust dependency graph. Is triggered when using
[`moon run`](/docs/commands/run).

### `pipeline.started`

Triggered when the pipeline has been created but before actions have started to run.

This event includes the number of actions registered within the pipeline, but does not provide
detailed information about the actions. Use the [`action.*`](#actionstarted) events for this.

```
{  "type": "pipeline.started",  "createdAt": "...",  "environment": "...",  "event": {    "actionsCount": 15  },  "uuid": "..."}
```

### `pipeline.finished`

Triggered when the pipeline has finished running all actions, with aggregated counts based on final
status.

This event is not triggered if the pipeline crashes (this does not include actions that have
failed, as those are legitimate runs). Use the [`pipeline.aborted`](#pipelineaborted) event if you
want to also catch crashes.

```
{  "type": "pipeline.finished",  "createdAt": "...",  "environment": "...",  "event": {    "cachedCount": 10,    "baselineDuration": {      "secs": 60,      "nanos": 3591693    },    "duration": {      "secs": 120,      "nanos": 3591693    },    "estimatedSavings": {      "secs": 60,      "nanos": 0    },    "failedCount": 1,    "passedCount": 4  },  "uuid": "..."}
```

### `pipeline.aborted`

Triggered when the pipeline has crashed for unknown reasons, or had to abort as a result of a
critical action failing.

```
{  "type": "pipeline.aborted",  "createdAt": "...",  "environment": "...",  "event": {    "error": "..."  },  "uuid": "..."}
```

### Actions

Actions are "jobs" within the pipeline that are executed topologically.

### `action.started`

Triggered when an action within the pipeline has started to run.

```
{  "type": "action.started",  "createdAt": "...",  "environment": "...",  "event": {    "action": {      "attempts": null,      "createdAt": "...",      "duration": {        "secs": 0,        "nanos": 3591693      },      "error": null,      "label": "InstallWorkspaceDeps(node:18.0.0)",      "nodeIndex": 5,      "status": "passed"    },    "node": {      "action": "InstallDeps",      "params": [        {          "toolchain": "Node",          "version": "18.0.0"        }      ]    }  },  "uuid": "..."}
```

### `action.finished`

Triggered when an action within the pipeline has finished running, either with a success or failure.
If the action failed, the `error` field will be set with the error message.

```
{  "type": "action.finished",  "createdAt": "...",  "environment": "...",  "event": {    "action": {      "attempts": null,      "createdAt": "...",      "duration": {        "secs": 0,        "nanos": 3591693      },      "error": null,      "label": "InstallWorkspaceDeps(node:18.0.0)",      "nodeIndex": 5,      "status": "passed"    },    "error": null,    "node": {      "action": "InstallDeps",      "params": {        "toolchain": "Node",        "version": "18.0.0"      }    }  },  "uuid": "..."}
```

### `dependencies.installing`

Triggered when dependencies for a workspace or project have started to install. When targeting a
project, the `project` field will be set, otherwise `null` for the entire workspace.

```
{  "type": "dependencies.installing",  "createdAt": "...",  "environment": "...",  "event": {    "project": {      "id": "server"      // ...    },    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    },    "root": ".",    "toolchain": "node"  },  "uuid": "..."}
```

### `dependencies.installed`

Triggered when dependencies for a workspace or project have finished installing. When targeting a
project, the `project` field will be set, otherwise `null` for the entire workspace. If the install
failed, the `error` field will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Installed deps can be scoped with the `InstallDeps(...)` labels.

```
{  "type": "dependencies.installed",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "project": null,    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    },    "root": ".",    "toolchain": "node"  },  "uuid": "..."}
```

### `environment.initializing`v1.37.0

Triggered when an environment is being setup for a toolchain. When targeting a project, the
`project` field will be set, otherwise `null` for the entire workspace.

```
{  "type": "environment.initializing",  "createdAt": "...",  "environment": "...",  "event": {    "project": {      "id": "server"      // ...    },    "root": ".",    "toolchain": "node"  },  "uuid": "..."}
```

### `environment.initialized`v1.37.0

Triggered when an environment has been setup for a toolchain. When targeting a project, the
`project` field will be set, otherwise `null` for the entire workspace. If setup failed, the `error`
field will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Installed deps can be scoped with the `SetupEnvironment(...)` labels.

```
{  "type": "environment.initialized",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "project": null,    "root": ".",    "toolchain": "node"  },  "uuid": "..."}
```

### `project.syncing`

Triggered when an affected project has started syncing its workspace state. This occurs
automatically before a project's task is ran.

```
{  "type": "project.syncing",  "createdAt": "...",  "environment": "...",  "event": {    "project": {      "id": "client"      // ...    },    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    }  },  "uuid": "..."}
```

### `project.synced`

Triggered when an affected project has finished syncing. If the sync failed, the `error` field will
be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Synced projects can be scoped with the `SyncProject(...)` labels.

```
{  "type": "project.synced",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "project": {      "id": "client"      // ...    },    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    }  },  "uuid": "..."}
```

### `tool.installing`

Triggered when a tool within the toolchain has started downloading and installing.

This event is always triggered, regardless of whether the tool has already been installed or not.
For an accurate state, use the [`action.finished`](#actionfinished) event. If the `status` is
"skipped", then the tool was already installed.

```
{  "type": "tool.installing",  "createdAt": "...",  "environment": "...",  "event": {    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    }  },  "uuid": "..."}
```

### `tool.installed`

Triggered when a tool within the toolchain has finished installing. If the install failed, the
`error` field will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Tools can be scoped with the `SetupToolchain(...)` labels.

```
{  "type": "tool.installed",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "runtime": {      "toolchain": "Node",      "version": "18.0.0"    }  },  "uuid": "..."}
```

### `toolchain.installing`

Triggered when a toolchain plugin has started downloading and installing.

This event is always triggered, regardless of whether the tool has already been installed or not.
For an accurate state, use the [`action.finished`](#actionfinished) event. If the `status` is
"skipped", then the tool was already installed.

```
{  "type": "toolchain.installing",  "createdAt": "...",  "environment": "...",  "event": {    "spec": {      "id": "node",      "req": "18.0.0"    }  },  "uuid": "..."}
```

### `toolchain.installed`

Triggered when a toolchain plugin has finished installing. If the install failed, the `error` field
will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event.
Tools can be scoped with the `SetupToolchain(...)` labels.

```
{  "type": "toolchain.installed",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "spec": {      "id": "node",      "req": "18.0.0"    }  },  "uuid": "..."}
```

### `task.running`

Triggered when a [task](/docs/concepts/task) has started to run (via [`moon run`](/docs/commands/run) or
similar command).

```
{  "type": "task.running",  "createdAt": "...",  "environment": "...",  "event": {    "target": "app:build"  },  "uuid": "..."}
```

### `task.ran`

Triggered when a [task](/docs/concepts/task) has finished running. If the run failed, the `error` field
will be set with the error message.

For more information about the action, refer to the [`action.finished`](#actionfinished) event. Ran
tasks can be scoped with the `RunTask(...)`, `RunInteractiveTask(...)`, and `RunPersistentTask(...)`
labels.

```
{  "type": "task.ran",  "createdAt": "...",  "environment": "...",  "event": {    "error": null,    "target": "app:build"  },  "uuid": "..."}
```

### `workspace.syncing`

Triggered when the workspace is being synced.

```
{  "type": "workspace.syncing",  "createdAt": "...",  "environment": "...",  "event": {    "target": "app:build"  },  "uuid": "..."}
```

### `workspace.synced`

Triggered when the workspace has finished syncing. If the action failed, the `error` field will be
set with the error message.

```
{  "type": "workspace.synced",  "createdAt": "...",  "environment": "...",  "event": {    "error": null  },  "uuid": "..."}
```

## /docs/how-it-works

Source: https://moonrepo.dev/docs/how-it-works

- [Home](/)
- [How it works](/docs/how-it-works)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# How it works

[📄️ LanguagesAlthough moon is currently focusing on the JavaScript ecosystem, our long-term vision is to be a](/docs/how-it-works/languages)

[📄️ Project graphThe project graph is a representation of all configured](/docs/how-it-works/project-graph)

[📄️ Task graphThe task graph is a representation of all configured](/docs/how-it-works/task-graph)

[📄️ Action graphWhen you run a task on the command line, we generate an action graph to](/docs/how-it-works/action-graph)

[Install moon](/docs/install)

[Languages](/docs/how-it-works/languages)

## /docs/how-it-works/action-graph

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

## /docs/how-it-works/languages

Source: https://moonrepo.dev/docs/how-it-works/languages

# Languages

Although moon is currently focusing on the JavaScript ecosystem, our long-term vision is to be a
multi-language task runner and monorepo management tool. To that end, we've designed our languages
to work like plugins, where their functionality is implemented in isolation, and is opt-in.

info

We do not support third-party language plugins at this time, but are working towards it!

## Enabling a language

moon [supported languages](/docs/#supported-languages) are opt-in, and are not enabled by default. We
chose this pattern to avoid unnecessary overhead, especially for the future when we have 10 or more
built-in languages.

To enable a supported language, simply define a configuration block with the language's name in
[`.moon/toolchains.yml`](/docs/config/toolchain). Even an empty block will enable the language.

.moon/toolchains.yml

```
# Enable Node.jsnode: {}# Enable Node.js with custom settingsnode:  packageManager: 'pnpm'# Enable Denodeno: {}
```

For unsupported languages, use the system toolchain. Continue reading to learn more!

## System language and toolchain

When working with moon, you'll most likely have tasks that run built-in system commands that do not
belong to any of the supported languages. For example, you may have a task that runs `git` or
`docker` commands, or common commands like `rm`, `cp`, `mv`, etc.

For these cases, moon provides a special language/toolchain called `system`, that is always enabled.
This toolchain is a catch-all, an escape-hatch, a fallback, and provides the following:

- Runs a system command or a binary found on `PATH`.

- Wraps the execution in a shell.

To run system commands, set a task's [`toolchain`](/docs/config/project#toolchain) setting to "system".

moon.yml

```
tasks:  example:    command: 'git status'    toolchain: 'system'
```

## Tier structure and responsibilities

As mentioned in our introduction,
[language support is divided up into tiers](/docs/#supported-languages), where each tier introduces
more internal integrations and automations, but requires more work to properly implement.

Internally each tier maps to a Rust crate, as demonstrated by the graph at the top of the article.

### Tier 0 = Unsupported

The zero tier represents all languages not directly supported by moon. This tier merely exists as
a mechanism for running non-supported language binaries via the
[system toolchain](#system-language-and-toolchain).

moon.yml

```
tasks:  example:    command: 'ruby'    toolchain: 'system'
```

### Tier 1 = Language

The first tier is the language itself. This is the most basic level of support, and is the only tier
that is required to be implemented for a language to be considered minimally supported. This tier is
in charge of:

- Declaring metadata about the language. For example, the name of the binary, supported file extensions, available dependency/package/version managers, names of config/manifest/lock files, etc.

- Helpers for parsing lockfiles and manifest files, and interacting with the language's ecosystem (for example, Node.js module resolution).

- Mechanisms for detecting the language of a project based on config files and other criteria.

- Maps to a project's [`language`](/docs/config/project#language) setting.

moon.yml

```
language: 'javascript'
```

### Tier 2 = Platform

The second tier requires the language functionality from tier 1, and eventually the toolchain
functionality from tier 3, and provides interoperability with moon's internals. This is the most
complex of all tiers, and the tier is in charge of:

- Determining when, where, and how to install dependencies for a project or the workspace.

- Loading project aliases and inferring implicit relationships between projects.

- Syncing a project and ensuring a healthy project state.

- Hashing efficiently for dependency installs and target runs.

- Prepending `PATH` with appropriate lookups to execute a task.

- Running a target's command with proper arguments, environment variables, and flags.

- Maps to a project's [`toolchain.default`](/docs/config/project#toolchain-1) or task's [`toolchain`](/docs/config/project#toolchain) setting.

- Supports a configuration block by name in [`.moon/toolchains.yml`](/docs/config/toolchain).

moon.yml

```
tasks:  example:    command: 'webpack'    toolchain: 'node'
```

.moon/toolchains.yml

```
node: {}
```

### Tier 3 = Toolchain

The third tier is toolchain support via [proto](/proto). This is the final tier, as the toolchain is
unusable unless the platform has been entirely integrated, and as such, the platform depends on this
tier. This tier handles:

- Downloading and installing a language into the toolchain.

- Installing and deduping project dependencies.

- Detecting appropriate versions of tools to use.

- Determining which binary to use and execute targets with.

- Supports a `version` field in the named configuration block in [`.moon/toolchains.yml`](/docs/config/toolchain).

.moon/toolchains.yml

```
node:  version: '18.0.0'
```

## /docs/how-it-works/project-graph

Source: https://moonrepo.dev/docs/how-it-works/project-graph

# Project graph

The project graph is a representation of all configured
[projects in the workspace](/docs/config/workspace#projects) and their relationships between each
other, and is represented internally as a directed acyclic graph (DAG). Below is a visual
representation of a project graph, composed of multiple applications and libraries, where both
project types depend on libraries.

info

The [`moon project-graph`](/docs/commands/project-graph) command can be used to view the structure of
your workspace.

## Relationships

A relationship is between a dependent (downstream project) and a dependency/requirement (upstream
project). Relationships are derived from source code and configuration files within the repository,
and fall into 1 of 2 categories:

### Explicit

These are dependencies that are explicitly defined in a project's [`moon.yml`](/docs/config/project)
config file, using the [`dependsOn`](/docs/config/project#dependson) setting.

moon.yml

```
dependsOn:  - 'components'  - id: 'utils'    scope: 'peer'
```

### Implicit

These are dependencies that are implicitly discovered by moon when scanning the repository. How an
implicit dependency is discovered is based on a
[language's platform integration](/docs/how-it-works/languages#tier-2--platform), and how that language's ecosystem
functions.

package.json

```
{  // ...  "dependencies": {    "@company/components": "workspace:*"  },  "peerDependencies": {    "@company/utils": "workspace:*"  }}
```

caution

If a language is not officially supported by moon, then implicit dependencies will not be
resolved. For unsupported languages, you must explicitly configure dependencies.

### Scopes

Every relationship is categorized into a scope that describes the type of relationship between the
parent and child. Scopes are currently used for [project syncing](/docs/commands/sync) and deep Docker
integration.

- Production - Dependency is required in production, will not be pruned in production environments, and will sync as a production dependency.

- Development - Dependency is required in development and production, will be pruned from production environments, and will sync as a development-only dependency.

- Build - Dependency is required for building only, and will sync as a build dependency.

- Peer - Dependency is a peer requirement, with language specific semantics. Will sync as a peer dependency when applicable.

## What is the graph used for?

Great question, the project graph is used throughout the codebase to accomplish a variety of
functions, but mainly:

- Is fed into the [task graph](/docs/how-it-works/task-graph) to determine relationships of tasks between other tasks, and across projects.

- Powers our [Docker](/docs/guides/docker) layer caching and scaffolding implementations.

- Utilized for [project syncing](/docs/commands/sync) to ensure a healthy repository state.

- Determines affected projects in [continuous integration](/docs/guides/ci) workflows.

## /docs/how-it-works/task-graph

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

## /docs/install

Source: https://moonrepo.dev/docs/install

# Install moon

2 min

The following guide can be used to install moon and integrate it into an existing repository (with
or without incremental adoption), or to a fresh repository.

## Installing

The entirety of moon is packaged and shipped as a single binary. It works on all major operating
systems, and does not require any external dependencies. For convenience, we provide the following
scripts to download and install moon.

### proto

moon can be installed and managed in [proto's toolchain](/proto). This will install moon to
`~/.proto/tools/moon` and make the binary available at `~/.proto/bin`.

```
proto install moon
```

Furthermore, the version of moon can be pinned on a per-project basis using the
[`.prototools` config file](/docs/proto/config).

.prototools

```
moon = "1.31.0"
```

info

We suggest using proto to manage moon (and other tools), as it allows for multiple versions to be
installed and used. The other installation options only allow for a single version (typically the
last installed).

### Linux, macOS, WSL

In a terminal that supports Bash, run:

```
bash
info

If you are using Git Bash on Windows, you can run the [Unix commands](#linux-macos-wsl) above.

### npm

moon is also packaged and shipped as a single binary through the
[`@moonrepo/cli`](https://www.npmjs.com/package/@moonrepo/cli) npm package. Begin by installing this
package at the root of the repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev @moonrepo/cli
```

```
yarn add --dev @moonrepo/cli# If using workspacesyarn add --dev -W @moonrepo/cli
```

```
npm install --save-dev @moonrepo/cli
```

```
pnpm add --save-dev @moonrepo/cli# If using workspacespnpm add --save-dev -w @moonrepo/cli
```

```
bun install --dev @moonrepo/cli
```

If you are installing with Bun, you'll need to add `@moonrepo/cli` as a
[trusted dependency](https://bun.sh/docs/install/lifecycle#trusteddependencies).

info

When a global `moon` binary is executed, and the `@moonrepo/cli` binary exists within the
repository, the npm package version will be executed instead. We do this because the npm package
denotes the exact version the repository is pinned it.

### Other

moon can also be downloaded and installed manually, by downloading an asset from
[https://github.com/moonrepo/moon/releases](https://github.com/moonrepo/moon/releases). Be sure to
rename the file after downloading, and apply the executable bit (`chmod +x`) on macOS and Linux.

## Upgrading

If using proto, moon can be upgraded using the following command:

```
proto install moon --pin
```

Otherwise, moon can be upgraded with the [`moon upgrade`](/docs/commands/upgrade) command. However, this
will only upgrade moon if it was installed in `~/.moon/bin`.

```
moon upgrade
```

Otherwise, you can re-run the installers above and it will download, install, and overwrite with the
latest version.

## Canary releases

moon supports canary releases, which are built and published for every commit to our development
branches. These releases will include features and functionality that have not yet landed on master.
When using a canary release, you'll need to download and execute the binaries manually:

- Using our npm package [`@moonrepo/cli`](https://www.npmjs.com/package/@moonrepo/cli?activeTab=versions) under the `canary` tag. Releases are versioned by date.

- From a [GitHub prerelease](https://github.com/moonrepo/moon/releases/tag/canary) using the `canary` tag. This tag always represents the latest development release.

## Nightly releases

moon supports nightly releases, which are built and published once a day from the latest commit on
master. When using a nightly release, you'll need to download and execute the binaries manually.

- Using our npm package [`@moonrepo/cli`](https://www.npmjs.com/package/@moonrepo/cli?activeTab=versions) under the `nightly` tag. Releases are versioned by date.

- From a [GitHub prerelease](https://github.com/moonrepo/moon/releases/tag/nightly) using the `nightly` tag. This tag always represents the latest stable release.

## Next steps

[Setup workspace](/docs/setup-workspace)

## /docs/migrate-to-moon

Source: https://moonrepo.dev/docs/migrate-to-moon

# Migrate to moon

Now that we've talked about the workspace, projects, tasks, and more, we must talk about something
important... Should you embrace moon tasks? Or keep using language/ecosystem specific scripts? Or
both (incremental adoption)?

## Migrate to moon tasks

We suggest using moon tasks (of course), as they provide far more granular control and configurable
options than scripts, and a `moon.yml` is a better
[source of truth](/docs/faq#what-should-be-considered-the-source-of-truth). Scripts aren't powerful
enough to scale for large codebases.

An example of what this may look like can be found below. This may look like a lot, but it pays
dividends in the long run.

/moon.yml

```
language: 'javascript'fileGroups:  sources:    - 'src/**/*'  tests:    - 'tests/**/*'tasks:  build:    command: 'webpack build --output-path @out(0)'    inputs:      - '@globs(sources)'      - 'webpack.config.js'    outputs:      - 'build'  dev:    command: 'webpack server'    inputs:      - '@globs(sources)'      - 'webpack.config.js'    preset: 'server'  format:    command: 'prettier --check .'    inputs:      - '@globs(sources)'      - '@globs(tests)'      - '/prettier.config.js'  lint:    command: 'eslint .'    inputs:      - '@globs(sources)'      - '@globs(tests)'      - '.eslintignore'      - '.eslintrc.js'      - '/.eslintrc.js'  test:    command: 'jest .'    inputs:      - '@globs(sources)'      - '@globs(tests)'      - 'jest.config.js'  typecheck:    command: 'tsc --build'    inputs:      - '@globs(sources)'      - '@globs(tests)'      - 'tsconfig.json'      - '/tsconfig.json'
```

## Continue using scripts

As a frontend developer you're already familiar with the Node.js ecosystem, specifically around
defining and using `package.json` scripts, and you may not want to deviate from this. Don't worry,
simply enable the [`node.inferTasksFromScripts`](/docs/config/toolchain#infertasksfromscripts) setting
to automatically create moon tasks from a project's scripts! These can then be ran with
[`moon run`](/docs/commands/run).

This implementation is a simple abstraction that runs `npm run ` (or pnpm/yarn) in the
project directory as a child process. While this works, relying on `package.json` scripts incurs the
following risks and disadvantages:

- [Inputs](/docs/config/project#inputs) default to `**/*`: A change to every project relative file will mark the task as affected, even those not necessary for the task. Granular input control is lost.

- A change to workspace relative files will not mark the task as affected. For example, a change to `/prettier.config.js` would not be detected for a `npm run format` script.

- [Outputs](/docs/config/project#outputs) default to an empty list unless: moon will attempt to extract outputs from arguments, by looking for variations of `--out`, `--outFile`, `--dist-dir`, etc.

- If no output could be determined, builds will not be cached and hydrated.

- Tasks will always [run in CI](/docs/config/project#runinci) unless: moon will attempt to determine invalid CI tasks by looking for popular command usage, for example: `webpack serve`, `next dev`, `--watch` usage, and more. This is not an exhaustive check.

- The script name contains variations of `dev`, `start`, or `serve`.

## Next steps

By this point, you should have a better understanding behind moon's fundamentals! Why not adopt
incrementally next? Jump into [guides](/docs/guides/ci) for advanced use cases or [concepts](/docs/concepts)
for a deeper understanding.

[Community help & support](https://discord.gg/qCh9MEynv2)[Releases & updates](https://twitter.com/tothemoonrepo)

## /docs/migrate/2.0

Source: https://moonrepo.dev/docs/migrate/2.0

# Migrate to moon v2.0

To ease the migration process from moon v1 to v2, we've compiled a list of all breaking changes and
important changes that you should be aware of. Please read through these carefully before upgrading
your workspace.

To automate some of the migration process, we've created the `moon migrate v2` command that will
migrate all applicable settings in configuration files.

```
$ moon migrate v2
```

## CLI

- Removed `x86_64-apple-darwin` (Apple Intel) as a supported operating system. Only `aarch64-apple-darwin` (Apple Silicon) is now supported.

### Commands

- We've done a large polish pass for all commands, based on the [CLI guidelines](https://clig.dev).

- Updated the `moonx` binary to use `moon exec` instead of `moon run` under the hood.

- Renamed all options and flags to kebab-case instead of camelCase. Example: `--logLevel` -> `--log-level`

- Renamed options for all commands: `--platform` -> `--toolchain`

- Removed commands: `moon node`

- `moon migrate from-package-json` (use the `migrate-turborepo` extension instead)

- `moon query hash` (use `moon hash` instead)

- `moon query hash-diff` (use `moon hash` instead)

#### `moon action-graph`

- Changed the output of `--json` to a new JSON structure (now matches the project and task graphs).

#### `moon check`

- Now runs `moon exec` under the hood, with some arguments/options pre-filled.

- If the project ID is not specifed, it will no longer find the closest project. Instead you must pass `--closest`.

- Renamed options: `--update-cache, -u` -> `--force, -f`

#### `moon ci`

- Now runs `moon exec` under the hood, with some arguments/options pre-filled.

- Renamed options: `--update-cache, -u` -> `--force, -f`

#### `moon generate`

- Changed the destination from a positional argument, to the `--to` option. Example: `moon generate ./dist` -> `moon generate --to ./dist`

#### `moon init`

- Removed "scaffolding a toolchain" functionality from the command. Use the `moon toolchain add` command instead.

- Removed options: `--to` (use a positional argument instead)

#### `moon mcp`

- Updated protocol version to 2025-11-25.

- Updated the `get_projects` tool to no longer have an `includeTasks` option.

- Updated the `get_projects` tool to return a list of project fragments, instead of the whole project object. This was required as the response was too large for MCP.

- Updated the `get_tasks` tool to return a list of task fragments, instead of the whole task object. This was required as the response was too large for MCP.

#### `moon query projects`

- Removed options: `--dependents`

#### `moon run`

- Now runs `moon exec` under the hood, with some arguments/options pre-filled.

- Updated options: `--dependents` now requires a value, either `deep` or `direct`

- Renamed options: `--update-cache, -u` -> `--force, -f`

- Removed options: `--no-bail` (use `moon exec` instead)

- `--profile`

- `--remote` (use `--affected remote` instead)

#### `moon templates`

- Changed the output to render a table instead of a list.

#### `moon run`

- Running a target without a scope no longer locates the closest project and instead uses the new default project feature. To run a target in the closets project, use `~:` scope instead. Example: `moon run build` -> `moon run ~:build`

## Workspace

### Configuration: `.moon/workspace.*`

- Renamed setting values: `codeowners.orderBy` value `project-name` -> `project-id`

- Renamed settings: `codeowners.syncOnRun` -> `codeowners.sync`

- `constraints.enforceProjectTypeRelationships` -> `constraints.enforceLayerRelationships`

- `docker.prune.installToolchainDeps` -> `docker.prune.installToolchainDependencies`

- `docker.scaffold.include` -> `docker.scaffold.configsPhaseGlobs`

- `runner` -> `pipeline`

- `unstable_remote` -> `remote`

- `vcs.manager` -> `vcs.client`

- `vcs.syncHooks` -> `vcs.sync`

- Removed settings: `docker.scaffold.copyToolchainFiles`

- `experiments.*`

- `hasher.batchSize`

- `pipeline.archivableTargets`

## Toolchains

### Configuration: `.moon/toolchain.*`

- This file was renamed to `.moon/toolchains.*` (plural) to reflect that multiple toolchains are configured. This also aligns with the new `.moon/extensions.*` file.

- All toolchains have been stabilized, so the `unstable_` prefix must be removed from identifiers.

#### JavaScript

The `bun`, `deno`, and `node` toolchains now require the `javascript` toolchain to be defined as
well. All shared settings have been moved to the `javascript` toolchain.

In addition, all `node` package managers are no longer nested under the `node` toolchain, but are
now top-level settings. These are only required when the `javascript.packageManager` setting is
defined.

- Moved settings: `bun.dependencyVersionFormat` -> `javascript.dependencyVersionFormat`

- `bun.inferTasksFromScripts` -> `javascript.inferTasksFromScripts`

- `bun.rootPackageOnly` -> `javascript.rootPackageDependenciesOnly`

- `bun.syncProjectWorkspaceDependencies` -> `javascript.syncProjectWorkspaceDependencies`

- `node.dependencyVersionFormat` -> `javascript.dependencyVersionFormat`

- `node.dedupeOnLockfileChange` -> `javascript.dedupeOnLockfileChange`

- `node.inferTasksFromScripts` -> `javascript.inferTasksFromScripts`

- `node.packageManager` -> `javascript.packageManager`

- `node.rootPackageOnly` -> `javascript.rootPackageDependenciesOnly`

- `node.syncPackageManagerField` -> `javascript.syncPackageManagerField`

- `node.syncProjectWorkspaceDependencies` -> `javascript.syncProjectWorkspaceDependencies`

- `node.bun` -> `bun`

- `node.npm` -> `npm`

- `node.pnpm` -> `pnpm`

- `node.yarn` -> `yarn`

- Renamed settings: `node.binExecArgs` -> `node.executeArgs`

- Removed settings: `bun.packagesRoot`

- `deno.depsFile`

- `deno.lockfile`

- `node.addEnginesConstraint`

- `node.packagesRoot`

.moon/toolchains.yml

```
# Beforenode:  version: '22.14.0'  packageManager: 'yarn'  inferTasksFromScripts: false  syncPackageManagerField: true  syncProjectWorkspaceDependencies: true  yarn:    version: '4.8.0'# Afterjavascript:  packageManager: 'yarn'  inferTasksFromScripts: false  syncPackageManagerField: true  syncProjectWorkspaceDependencies: truenode:  version: '22.14.0'yarn:  version: '4.8.0'
```

## Extensions

### Configuration: `.moon/extensions.*`

- The `extensions` setting from `.moon/workspace.*` has been moved (and flattened) to its own file, `.moon/extensions.*`.

- The built-in extensions `download`, `migrate-nx`, and `migrate-turborepo` must now be enabled in the configuration file before they can be used. Simply set an empty object. This change was made to reduce the number of extensions that are loaded by default, improving performance.

.moon/extensions.yml

```
download: {}migrate-nx: {}
```

## Project

### Configuration: `moon.*`

- Renamed settings: `docker.scaffold.include` -> `docker.scaffold.sourcesPhaseGlobs`

- `project.name` -> `project.title`

- `type` -> `layer`

- `toolchain` -> `toolchains`

- `platform` -> `toolchains.default`

- Removed settings: [`project.metadata`](#custom-metadata)

- [`toolchain.*.disabled`](#toolchain-disabling)

### Language detection

The primary `language` is now detected from toolchains, instead of being a hardcoded implementation.
The result may now differ, as the first toolchain in the list will be used. Additionally, languages
that don't have a toolchain yet, like PHP or Ruby, will not be detected and must be explicitly
configured.

moon.yml

```
# Afterlanguage: 'ruby'
```

### Custom metadata

The `project.metadata` setting has been removed, but all custom metadata fields can now be defined
at the root of the `project` setting.

moon.yml

```
# Beforeproject:  metadata:    customField: 'value'# Afterproject:  customField: 'value'
```

### Toolchain disabling

The `toolchain.*.disabled` setting was removed. Instead set the toolchain itself to null/false.

moon.yml

```
# Beforetoolchain:  typescript:    disabled: true# Aftertoolchains:  typescript: null
```

## Tasks

### Configuration: `moon.*`, `.moon/tasks`

- The file `.moon/tasks.yml` has been removed. If you want to support tasks that are inherited by all projects, then move this to `.moon/tasks/all.yml` and do not configure the new `inheritedBy` setting.

- Renamed tokens: `$projectName` -> `$projectTitle`

- `$projectType` -> `$projectLayer`

- `$taskPlatform` -> `$taskToolchain`

- Renamed settings: [`tasks.*.local`](#local-mode) -> `tasks.*.preset` using `server` value

- `tasks.*.platform` -> `tasks.*.toolchains`

- `tasks.*.options.affectedPassInputs` -> `tasks.*.options.affectedFiles.passInputsWhenNoMatch`

- Removed setting values: `tasks.*.preset` value `watcher`

- Changed settings: `tasks.deps.*.args` no longer supports a string. Use a list of strings instead.

- `tasks.*.options.affectedFiles` now supports an object for more granular control.

- Changed option defaults: `tasks.*.options.envFile` now defaults to a list of files, instead of a single file, when `true`. Refer to the blog post for more information.

- `tasks.*.options.inferInputs` now defaults to `false` instead of `true`.

- `tasks.*.options.shell` now defaults to `true` instead of `false`.

- `tasks.*.options.unixShell` now defaults to `bash` instead of nothing.

- `tasks.*.options.windowsShell` now defaults to `pwsh` instead of nothing.

### Simple commands only

We've reworked `command` (and `args`) to only support simple commands. A simple command is an
executable (binary, file, etc) followed by zero or many arguments.

Complex commands that involve shell features like piping (`|`), redirection (`>`, `

Shell features like expansion, globbing, and substitution is still supported in `command`.

### Local mode

The `local` task setting has been removed as the name was confusing. Users assumed it meant "only
run locally", but it actually meant "this is a persistent server that should only run locally".
Instead, use the `preset` setting with a value of `server`.

moon.yml

```
# Beforetasks:  dev:    command: 'start-dev'    local: true# Beforetasks:  dev:    command: 'start-dev'    preset: 'server'
```

If you want a task that is simply "local only" without other options changes, use `options.runInCI`
directly.

moon.yml

```
tasks:  dev:    command: 'start-dev'    options:      runInCI: false
```

### Shells by default

Tasks now run in a shell by default, and will use Bash on Unix (`options.unixShell`), and pwsh on
Windows (`options.windowsShell`). You can disable this behavior by setting `options.shell` to
`false`.

moon.yml

```
tasks:  dev:    command: 'start-dev'    options:      shell: false
```

### Env var substitution behavior

The syntax and behavior for substituting (expanding/interpolation) environment variables has
changed, to better align with the standard of `.env` files. The biggest change is that flagless
tokens (`$VAR`) and `?` flag tokens (`$VAR?`) swapped functionality. Refer to the following table:

Syntax v1 v2

`$VAR` Substitute with variable syntax (`$VAR`) if variable empty 💥 Substitute with empty string if variable empty

`$VAR!` Don't substitute and keep variable syntax (`$VAR`) 💥 Removed syntax

`$VAR?` Substitute with empty string if variable empty 💥 Removed syntax

`${VAR}` Substitute with variable syntax (`$VAR`) if variable empty 💥 Substitute with empty string if variable empty

`${VAR!}` Don't substitute and keep variable syntax (`$VAR`) ~

`${VAR?}` Substitute with empty string if variable empty 💥 Substitute with variable syntax (`$VAR`) if variable empty

`${VAR:default}` Use default value if variable empty ~

`${VAR:-default}`, `${VAR-default}` ⛔️ Not supported Use default value if variable empty

`${VAR:+alternate}`, `${VAR+alternate}` ⛔️ Not supported Use alternate value if variable non-empty

Legend:

- `~` indicates the syntax/functionality is the same as v1

- `💥` indicates breaking change

- `⛔️` indicates not supported

### Env var precedence

The order of precedence for environment variables has slightly changed when running tasks, as it was
confusing to users. The new order of precedence is as follows, from lowest to highest tier, with the
latter overwriting the former:

- Task `.env` files via `options.envFile`

- Task env variables via `env` or `deps.*.env`

- System variables via profile scripts (`.bashrc`, `.zshrc`, etc)

- via command line: `KEY=value moon ...`

In regards to variable substitution, each tier can reference variables within the same tier, or the
higher tier(s), but not from lower tier(s). This is because variables are processed in reverse
order. Refer to the following table:

Tier Can reference Evaluated during

Dotenv Dotenv, Task, System Before task execution

Task Task, System After task creation

System System CLI startup

If you don't want to inherit a system variable, you can override it in the task with a null value.

```
tasks:  dev:    env:      EXAMPLE: null
```

### Deferred `.env` files

In v1, when `options.envFile` was enabled, the `.env` file(s) were loaded at the time of task
creation, during the building of project/task graphs. This meant that if the `.env` file changed
between the time of graph creation and task execution, the changes would not be reflected.

In v2 and later, `.env` files are loaded just before task execution, ensuring that any changes to
the file are picked up.

caution

Because of this change, task `env` variables will continue to override `.env` file variables, BUT
can no longer reference them for substitution. This is because the `.env` files are loaded later in
the process.

### Other changes

- When `options.affectedFiles` is enabled, the list of files will be joined with the OS path separator (`:` on Unix, `;` on Windows) instead of a comma (`,`) when passed as the `MOON_AFFECTED_FILES` environment variable.

## Task inheritance

### Deep merged instead of shallow merged

In v1, when inheriting tasks, all global configs (those in `.moon`) were shallow merged into a
single config ignoring merge task options, and then merged with the local config (`moon.*`) using
merge task options. This was not intuitive, as users expected all configs to be merged in sequence.
To demonstrate this, take the following example configs, in order of inherited:

```
# .moon/tasks.ymltasks:  build:    command: 'build --cache'    options:      mutex: 'build'# .moon/tasks/tag-a.ymltasks:  build:    args: '--force'# .moon/tasks/tag-b.ymltasks:  build:    args: '--clean'
```

Users would expect the final `build` task to be `build --cache --force --clean` with the mutex
option set. However, since the `tasks` setting was shallow merged, only the last config (`tag-b`)
would be used, resulting in the task being `noop --clean` without the mutex. The `noop` pops up
because the `command` setting was not configured in the last config.

To remedy this, and to improve task composition overall, global configs are no longer shallow merged
into a single config before merging with the local config. Instead, all configs are merged in
sequence, while respecting the task merge options. This is the order of operations for the new
system:

- Load all global configs (`.moon`) into a list, in order, based on the `inheritedBy` setting.

- Load the local config (`moon.*`).

- Create an inheritance chain by resolving global configs first, then the local config last, while respecting `extend` and other composition settings.

- Merge all task options in order, to create the final task options.

- Merge all tasks in order, using the final task options to guide the merging behavior.

### File groups are merged

Because of the new deep merging behavior, file groups defined in inherited configs are now merged
together, instead of being replaced by the following config in the sequence. For example, given the
configs:

```
# .moon/tasks/tag-a.ymlfileGroups:  sources:    - 'src/**'# .moon/tasks/tag-b.ymlfileGroups:  sources:    - 'docs/**'
```

In v1, the final `sources` file group would only include `docs/**`, as the second config would
replace the first. In v2, the final `sources` file group includes both `src/**` and `docs/**`.

## VCS

### New hooks system

We've rewritten our Git hooks from the ground up to be based around the `core.hooksPath` setting.
The following changes have been made:

- We no longer write hooks to the `.git/hooks` directory.

- Instead, all hooks are written to `.moon/hooks` and Git is configured to use this directory.

- Bash scripts no longer end in `.sh`.

caution

We currently don't have an easy way to clean the previous implementation of hooks. You may need to
manually remove the old scripts from `.git/hooks` and `.moon/hooks` if they are causing issues.

## Other changes

### Changed (touched) files

We renamed the terminology "touched files" to "changed files" throughout the codebase and
documentation. This better aligns with common VCS terminology and reduces confusion. Because of
this, the following changes were made:

- Renamed the `moon query touched-files` CLI command to `moon query changed-files`.

- Renamed the `get_touched_files` MCP tool to `get_changed_files`.

- Renamed the `touchedFiles` run report field to `changedFiles`.

```
# Before$ moon query touched-files# After$ moon query changed-files
```

### Docker

- The scaffolded `.moon/docker/workspace` directory was renamed to `.moon/docker/configs`.

- The `moon docker file` command will now loop through all toolchains and use the first image found, otherwise it defaults to "scratch". If you want to be explicit, set the `docker.file.image` setting.

### Query language (MQL)

- Renamed fields: `projectName` -> `projectId`

- `projectType` -> `projectLayer`

- `taskPlatform` -> `taskToolchain`

```
# BeforeprojectType=application && taskPlatform=node# AfterprojectLayer=application && taskToolchain=node
```

### Webhooks

- Removed the `tool.*` events. Use `toolchain.*` events instead.

- Removed the `runtime` field from `dependencies.*` events. Use the `toolchain` field instead.

## /docs/proto

Source: https://moonrepo.dev/docs/proto

# What is proto?

3 min

proto is a pluggable version manager, a unified toolchain.

If you're unfamiliar with the concept of a toolchain, a toolchain is a collection of tools that are
downloaded, installed, and managed by version through a single interface. In the context of proto's
toolchain, a tool is either a programming language, a dependency/package manager for a language, or
a custom implementation provided by a plugin. It's the next step in the version manager evolution.

## Features

- Lightspeed! With Rust and WASM, we can guarantee exceptional performance.

- Multi-language. A single CLI for managing versions for all of your languages.

- Cross-platform, for a consistent experience across machines and teams.

- [Contextual version detection](/docs/proto/detection), ensuring the correct version of a tool is always used.

- Checksum verification, ensuring a tool came from a trusted source.

- Detects and infers from a language's ecosystem for maximum compatibility.

- [Pluggable architecture](/docs/proto/plugins), allowing for custom tooling.

## Why proto?

proto was designed to be a modern and holistic version manager for all of your favorite programming
languages. We believe a single tool that works the same across every language is better than
multiple ad-hoc tools. While we only support a handful of languages today, we aim to support many
more in the future!

success

proto powers [moon](/moon)'s toolchain, enabling a single source of truth for both tools!

## How does it work?

The toolchain is a `.proto` directory within the current user's home directory, e.g., `~/.proto`.

The first step in a tool's life-cycle is being downloaded to `~/.proto/temp`. Downloads are
typically an archive that can be unpacked into a target directory. Once downloaded, we verify the
downloaded file by running a checksum. If this check fails for any reason, the tool is unusable,
and the process is aborted.

After a successful verification, the last step in the tool's life-cycle can begin, installation.
Depending on the type of download, the installation process may differ. For archives, we unpack the
tool to `~/.proto/tools//`. In the future, we'll support building from source.

From here, we make these tools globally available by prepending `~/.proto/shims` and `~/.proto/bin`
to `PATH` (typically as part of your shell profile). Continue reading for more about these folders.

## Supported tools

The following tools are [officially supported](/docs/proto/tools) in proto via moonrepo. Additional
tools can be supported through [third-party plugins](/docs/proto/plugins).

+ npm, pnpm, yarn

+ pip, poetry, uv

... with [0 more proto plugins](/docs/proto/tools#third-party), and over [800 asdf plugins](/docs/proto/tool-spec#asdf)...

## Supported targets

Because proto is written in Rust, we only support targets that are explicitly compiled for, which
are currently:

Operating system Architecture Target

macOS 64-bit Intel `x86_64-apple-darwin`

macOS 64-bit ARM `aarch64-apple-darwin`

Linux 64-bit Intel GNU `x86_64-unknown-linux-gnu`

Linux 64-bit Intel musl `x86_64-unknown-linux-musl`

Linux 64-bit ARM GNU `aarch64-unknown-linux-gnu`

Linux 64-bit ARM musl `aarch64-unknown-linux-musl`

Windows 64-bit Intel `x86_64-pc-windows-msvc`

## /docs/proto/commands/activate

Source: https://moonrepo.dev/docs/proto/commands/activate

# activate

v0.38.0

The `proto activate ` command will activate proto for the current shell session, by exporting
environment variables and prepending `PATH` for each tool configured in the current directory.
Activation is ran each time the current directory changes using a shell hook.

info

Learn more about
[shell activation in the official workflow documentation](/docs/proto/workflows#shell-activation)!

### Arguments

- `` - The shell to activate for.

### Options

- `--export` - Print the activate instructions in shell-specific syntax.

- `--json` - Print the activate instructions in JSON format.

- `--no-bin` - Do not include `~/.proto/bin` when appending `PATH`.

- `--no-shim` - Do not include `~/.proto/shims` when prepending `PATH`.

- `--no-init` - Do not trigger activation when initialized in the shell, and instead wait for a cd/prompt change. v0.50.0

### Caveats

- Only tools that have a [version configured in `.prototools`](/docs/proto/config#pinning-versions) will be activated.

- Tool versions configured in the global `~/.proto/.prototools` are not included by default. Pass `--config-mode all` during activation to include them. Do note that this will worsen performance depending on the number of tools.

### Setup

The following activation steps should be added after all environment variable and `PATH`
modifications have happened in your shell, typically at the end of your shell profile.

#### Bash

Add the following line to the end of your `~/.bashrc` or `~/.bash_profile`.

```
eval "$(proto activate bash)"
```

#### Elvish

Generate the hook:

```
proto activate elvish > ~/.elvish/lib/proto-hook.elv
```

Then add the following line to your `~/.elvish/rc.elv` file.

```
use proto-hook
```

#### Fish

Add the following line to the end of your `~/.config/fish/config.fish`.

```
proto activate fish | source
```

#### Murex

Add the following line to the end of your `~/.murex_profile`.

```
proto activate murex -> source
```

#### Nu

Generate the hook:

```
(proto activate nu) | save ~/.config/nushell/proto-hook.nu
```

Then add the following line to your `~/.config/nushell/config.nu` file.

```
use proto-hook.nu
```

#### Pwsh

Add the following line to the end of your profile (`$PROFILE`).

```
proto activate pwsh | Out-String | Invoke-Expression
```

#### Zsh

Add the following line to the end of your `~/.zshrc`.

```
eval "$(proto activate zsh)"
```

## /docs/proto/commands/alias

Source: https://moonrepo.dev/docs/proto/commands/alias

# alias

The `proto alias   ` (or `proto a`) command will define a custom alias that
maps to a specific version for the provided tool. Aliases can be used anywhere a version is
accepted.

```
$ proto alias node work 16.16
```

By default this will update the local [`./.prototools`](/docs/proto/config) file. Pass `--to` to customize
the location.

### Arguments

- `` - Type of tool.

- `` - Name of the alias. Supports alphanumeric chars.

- `` - Version to map to the alias.

## Options

- `--to` - [Location of `.prototools`](/docs/proto/config#locations) to update. Supports `global`, `local`, and `user`. v0.41.0

## /docs/proto/commands/bin

Source: https://moonrepo.dev/docs/proto/commands/bin

# bin

The `proto bin  [version]` command will return an absolute path to a tool's binary within the
toolchain. When a tool has not been installed, or a version cannot be resolved, the command will
exit with a failure.

```
$ proto bin node 16.10.0/Users/example/.proto/tools/node/16.10.0/bin/node
```

This command can also return directories using the `--dir` option.

```
$ proto bin node 16.10.0 --dir exes/Users/example/.proto/tools/node/16.10.0/bin$ proto bin node 16.10.0 --dir globals/Users/example/.proto/tools/node/globals/bin
```

### Arguments

- `` - Type of tool.

- `[version]` - Version of tool. If not provided, will attempt to [detect the version](/docs/proto/detection).

### Options

- `--all` - Return multiple paths, separated by newlines, instead of the first path. v0.50.0

- `--dir ` - Return a directory instead of of the main file. v0.50.0 `exes` - Returns the executable's directory.

- `globals` - Returns the globals/packages directory.

- `--bin` - When applicable, return the `~/.proto/bin` path.

- `--shim` - When applicable, return the `~/.proto/shims` path.

## /docs/proto/commands/clean

Source: https://moonrepo.dev/docs/proto/commands/clean

# clean

The `proto clean` command can be used to uninstall stale and unused tools, plugins, and more. By
default, it will remove items that haven't been used in the last 30 days.

```
$ proto clean
```

Furthermore, the command can be used to target a specific artifact type.

```
$ proto clean plugins
```

### Arguments

- `[target]` - Type of target. Accepts `cache`, `plugins`, `temp`, or `tools`. v0.44.0

### Options

- `--days` - Number of days before a tool is considered stale.

- `--json` - Print the clean result in JSON format. v0.44.0

- `--yes` - Avoid and confirm all prompts.

## /docs/proto/commands/completions

Source: https://moonrepo.dev/docs/proto/commands/completions

# completions

The `proto completions` command will generate proto command and argument completions for your
current shell. This command will write to stdout, which can then be redirected to a file of your
choice.

```
$ proto completions > ./path/to/write/to
```

### Options

- `--shell` - Shell to explicitly generate for.

### Examples

- Bash
- Fish
- Zsh

If using [bash-completion](https://github.com/scop/bash-completion).

```
mkdir -p ~/.bash_completion.dproto completions > ~/.bash_completion.d/proto.sh
```

Otherwise write the file to a common location, and source it in your profile.

```
mkdir -p ~/.bash_completionsproto completions > ~/.bash_completions/proto.sh# In your profilesource ~/.bash_completions/proto.sh
```

Write the file to Fish's completions directory.

```
mkdir -p ~/.config/fish/completionsproto completions > ~/.config/fish/completions/proto.fish
```

If using [oh-my-zsh](https://ohmyz.sh/) (the `_` prefix is required).

```
mkdir -p ~/.oh-my-zsh/completionsproto completions > ~/.oh-my-zsh/completions/_proto# Reload shell (or restart terminal)omz reload
```

## /docs/proto/commands/debug

Source: https://moonrepo.dev/docs/proto/commands/debug

- [Home](/)
- Commands
- [debug](/docs/proto/commands/debug)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# debug

Operations for debugging the current proto environment.

[📄️ configThe proto debug config command will list all .prototools configuration files (in TOML format)](/docs/proto/commands/debug/config)

[📄️ envThe proto debug env command will print information about your current proto environment. Primarily](/docs/proto/commands/debug/env)

[completions](/docs/proto/commands/completions)

[config](/docs/proto/commands/debug/config)

## /docs/proto/commands/debug/config

Source: https://moonrepo.dev/docs/proto/commands/debug/config

# debug config

v0.25.0

The `proto debug config` command will list all `.prototools` configuration files (in TOML format)
that have been loaded, in order of precedence, with the final merged configuration printed at the
end.

```
$ proto debug config/Users/name/.proto/.prototools ───────────────────────────────────────────  node = "20.0.0"  npm = "bundled"  [tools.node.aliases]  stable = "~20"  [settings]  auto-clean = falseFinal configuration ───────────────────────────────────────────────────────  node = "20.0.0"  npm = "bundled"  [tools.node.aliases]  stable = "~20"  [plugins.tools]  node = "https://github.com/moonrepo/node-plugin/releases/download/v0.6.1/node_plugin.wasm"  [settings]  auto-clean = false  auto-install = false  detect-strategy = "first-available"  [settings.http]  allow-invalid-certs = false  proxies = []
```

### Options

- `--json` - Print the list in JSON format.

## /docs/proto/commands/debug/env

Source: https://moonrepo.dev/docs/proto/commands/debug/env

# debug env

v0.26.0

The `proto debug env` command will print information about your current proto environment. Primarily
the store location, relevant file paths, and environment variables.

```
$ proto debug envStore ─────────────────────────────────────────────────────────────────────  Root: /Users/name/.proto  Bins: /Users/name/.proto/bin  Shims: /Users/name/.proto/shims  Plugins: /Users/name/.proto/plugins  Tools: /Users/name/.proto/tools  Temp: /Users/name/.proto/tempEnvironment ───────────────────────────────────────────────────────────────  Proto version: 0.44.0  Operating system: macos  Architecture: arm64  Config sources:    - /Users/name/Projects/example/.prototools    - /Users/name/.proto/.prototools  Virtual paths:    /userhome = /Users/name    /proto = /Users/name/.proto  Environment variables:    PROTO_APP_LOG = proto=info,schematic=info,starbase=info,warpgate=info,extism::pdk=info    PROTO_HOME = /Users/name/.proto    PROTO_OFFLINE_TIMEOUT = 750    PROTO_VERSION = 0.44.0
```

### Options

- `--json` - Print the list in JSON format.

## /docs/proto/commands/diagnose

Source: https://moonrepo.dev/docs/proto/commands/diagnose

# diagnose

v0.37.0

The `proto diagnose` command will diagnose your proto installation for any potential issues. Issues
are categorized into errors and warnings, with the former being a must fix, and the latter being a
maybe fix (depending on your usage of proto).

```
$ proto diagnoseShell: zshShell profile: /Users/name/.zshrcErrors ────────────────────────────────────────────────────────────────────  - Issue: Bin directory /Users/name/.proto/bin was found BEFORE the shims directory /Users/name/.proto/shims on PATH    Resolution: Ensure the shims path comes before the bin path in your shell    Comment: Runtime version detection will not work correctly unless shims are used
```

### Options

- `--shell` - The shell to diagnose (will detect automatically).

- `--json` - Print the diagnosis in JSON format.

## /docs/proto/commands/exec

Source: https://moonrepo.dev/docs/proto/commands/exec

# exec

v0.53.0

The `proto exec  -- ` (or `proto x`) command will activate a temporary
environment by loading and initializing any number of tools, and then execute an arbitrary command
within that environment.

```
$ proto exec node pnpm -- pnpm run dev
```

Tools will automatically detect a version to execute with based on loaded `.prototools`, but the
version can be provided inline by suffixing the tool with `@`.

```
$ proto exec node@24.2 pnpm@10 -- pnpm run dev
```

### Shell support

By default, the command will not be executed in a shell, and will be excuted in the context of the
parent process. If you want to execute the command within a shell (using `-c`), you can use the
`--shell` option.

```
$ proto exec node pnpm --shell bash --
```

If your command contains special characters, complex expressions, or shell specific syntax, you may
need to pass `--raw` to avoid quoting/escaping issues.

```
$ proto exec node pnpm --shell bash --raw --
```

Furthermore, if you want to launch an interactive shell session with the activated environment, you
can pass the shell command itself as the exec command.

```
$ proto exec node pnpm -- bash
```

### Arguments

- `` - List of tool identifiers with optional version.

- `` - Command to execute within the environment. Must be passed after a `--` separator.

### Options

- `--tools-from-config` - Inherit tools to initialize from `.prototools` configs, instead of passing an explicit list.

- `--raw` - Execute the command as-is without quoting or escaping when using `--shell`.

- `--shell` - Shell to execute the command with (e.g. `bash` or `pwsh`).

## /docs/proto/commands/install

Source: https://moonrepo.dev/docs/proto/commands/install

# install

The `proto install` (or `proto i`) command can be used to install one or many tools.

### Installing all toolsv0.39.0

The `proto install` command (without arguments) will download and install all tools and plugins
from all parent [`.prototools`](/docs/proto/config) configuration files, and any
[versions detected](/docs/proto/detection) in the current working directory (if not defined in
`.prototools`).

```
$ proto install
```

By default, this command does not install tools for versions pinned in the global
`~/.proto/.prototools` file. Pass `--config-mode all` to include them.

### Installing one tool

The `proto install  [version]` command will download and install a single tool by unpacking
their archive to `~/.proto/tools/`. If the tool has already been installed, the command will
exit early.

The command is also smart enough to resolve partial versions, so 1, 1.2, and 1.2.3 are all
acceptable. It even supports aliases when applicable, like `latest`, `next`, `beta`, etc. To install
a canary release, use `canary`.

```
$ proto install deno$ proto install deno 1.31$ proto install deno canary
```

#### Pinning the version

By default this command will only install the tool into `~/.proto/tools` but will not make the
binary available. If you would like to also pin the resolved version to a `.prototools` file, use
the `--pin` option.

```
# ./.prototools$ proto install bun --pin$ proto install bun --pin local# ~/.proto/.prototools$ proto install bun --pin global# ~/.prototools$ proto install bun --pin user
```

### Handling plugin hooks

Some tools run [post-install hooks](/docs/proto/tools) that support arbitrary arguments that can be passed
after `--`.

```
$ proto install go -- --no-gobin
```

### Arguments

- One tool `[tool]` - Type of tool.

- `[version]` - Version of tool. Defaults to a pinned version in `.prototools` or "latest".

- `[-- ]` - Additional arguments to pass to post-install hooks.

### Options

- `--force` - Force install, even if already installed.

- `--update-lockfile` - Don't inherit a version from the lockfile and update the existing record. v0.51.0

- One tool `--build` - Build from source if available. v0.45.0

- `--no-build` - Download a pre-built if available. v0.45.0

- `--pin` - Pin the resolved version and create a symlink in `~/.proto/bin`. Accepts a boolean (pins locally by default), or the string "global", or the string "local".

## /docs/proto/commands/list

Source: https://moonrepo.dev/docs/proto/commands/list

# list

danger

This command was removed in v0.44, use [`proto versions`](/docs/proto/commands/versions) instead!

The `proto list ` (or `proto ls`) command will list installed versions by scanning the
manifest at `~/.proto/tools//manifest.json` for possible versions.

```
$ proto list node16.16.018.2.019.4.0
```

### Arguments

- `` - Type of tool.

### Options

- `--aliases` - Include aliases in the list.

## /docs/proto/commands/list-remote

Source: https://moonrepo.dev/docs/proto/commands/list-remote

# list-remote

danger

This command was removed in v0.44, use [`proto versions`](/docs/proto/commands/versions) instead!

The `proto list-remote ` (or `proto lsr`) command will list available versions by resolving
versions from the tool's remote release manifest.

```
$ proto list-remote node...18.10.018.11.018.12.018.12.118.13.018.14.018.14.118.14.219.0.019.0.119.1.019.2.019.3.019.4.019.5.019.6.019.6.119.7.0
```

### Arguments

- `` - Type of tool.

### Options

- `--aliases` - Include aliases in the list.

## /docs/proto/commands/outdated

Source: https://moonrepo.dev/docs/proto/commands/outdated

# outdated

v0.19.0

The `proto outdated` command will load all [`.prototools`](/docs/proto/config) files and check for newer
(matching configured range) and latest versions of each configured tool. Will also include the
configuration file in which the version has been configured.

```
$ proto outdated╭───────────────────────────────────────────────────────────────────────╮│ Tool      Current Newest  Latest  Config                              ││───────────────────────────────────────────────────────────────────────││ bun       1.1.42  1.1.42  1.1.42  /Users/name/.proto/.prototools      ││ node      23.5.0  23.5.0  23.5.0  /Users/name/.proto/.prototools      ││ npm       10.7.0  10.7.0  11.0.0  /Users/name/.proto/.prototools      ││ rust      1.83.0  1.83.0  1.83.0  /Users/name/.proto/.prototools      ││ yarn      3.6.3   3.8.7   4.5.1   /Users/name/.proto/.prototools      │╰───────────────────────────────────────────────────────────────────────╯
```

By default, this command does not check tools for versions pinned in the global
`~/.proto/.prototools` file. Pass `--config-mode all` to include them.

### Options

- `--json` - Print the list in JSON format.

- `--latest` - When updating versions with `--update`, use the latest version instead of newest.

- `--update` - Update and write newest/latest versions to their respective configuration.

- `--yes` - Avoid and confirm all prompts. v0.44.0

## /docs/proto/commands/pin

Source: https://moonrepo.dev/docs/proto/commands/pin

# pin

v0.19.0

The `proto pin  ` command will pin a version (or alias) of a tool. This version will
be used when attempting to [detect a version](/docs/proto/detection).

```
$ proto pin go 1.20$ proto pin python 3.14 --to=global$ proto pin node lts --resolve$ proto pin npm latest --resolve --tool-native
```

By default this will update the local [`./.prototools`](/docs/proto/config) file. Pass `--to` to customize
the location, or use the `--tool-native` option to use a location unique to the tool.

### Arguments

- `` - Type of tool.

- `` - Version of tool.

### Options

- `--resolve` - Resolve the version to a fully-qualified semantic version before pinning.

- `--to` - [Location of `.prototools`](/docs/proto/config#locations) to update. Supports `global`, `local`, and `user`. v0.41.0

- `--tool-native` - Pins the version in a tool specific location. Examples: JavaScript tooling (Node, Bun, Deno, npm, pnpm, Yarn, etc) Pins version in the `devEngines` field in the `package.json` file.

- v0.55.0

## /docs/proto/commands/plugin

Source: https://moonrepo.dev/docs/proto/commands/plugin

- [Home](/)
- Commands
- [plugin](/docs/proto/commands/plugin)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# plugin

Operations for managing tools and plugins.

[📄️ addThe proto plugin add command will add the provided ID and plugin locator string to](/docs/proto/commands/plugin/add)

[📄️ infoThe proto plugin info command will display information about a tool and its plugin.](/docs/proto/commands/plugin/info)

[📄️ listThe proto plugin list [...id] command will list all available and configured plugins, for both](/docs/proto/commands/plugin/list)

[📄️ removeThe proto plugin remove command will remove the provided tool ID from the [plugins] section](/docs/proto/commands/plugin/remove)

[📄️ searchThe proto plugin search command will search for plugins provided by the community, based](/docs/proto/commands/plugin/search)

[pin](/docs/proto/commands/pin)

[add](/docs/proto/commands/plugin/add)

## /docs/proto/commands/plugin/add

Source: https://moonrepo.dev/docs/proto/commands/plugin/add

# plugin add

v0.23.0

The `proto plugin add
` command will add the provided ID and plugin locator string to
the `[plugins]` section of a chosen `.prototools`.

```
$ proto plugin add node "https://github.com/moonrepo/node-plugin/releases/latest/download/node_plugin.wasm"
```

Learn more about [plugin locator strings](/docs/proto/plugins#enabling-plugins).

### Arguments

- `` - ID of the tool.

- `` - How to locate the plugin.

### Options

- `--to` - [Location of `.prototools`](/docs/proto/config#locations) to update. v0.41.0

- `--type` - Type of plugin to add, either `tool` (default) or `backend`. v0.52.0

## /docs/proto/commands/plugin/info

Source: https://moonrepo.dev/docs/proto/commands/plugin/info

# plugin info

v0.23.0

The `proto plugin info ` command will display information about a tool and its plugin.

```
$ proto plugin info nodePlugin ─────────────────────────────────  ID: node  Name: Node.js  Type: Language  Version: 0.13.0  Source URL: https://github.com/moonrepo/plugins/releases/download/node_tool-v0.13.0/node_tool.wasmInventory ──────────────────────────────  Detected version: 23.5.0  Fallback version: 23.5.0  Store directory: /Users/name/.proto/tools/node  Executable file: /Users/name/.proto/tools/node/23.5.0/bin/node  Executables directory: /Users/name/.proto/tools/node/23.5.0/bin  Global packages directory: /Users/name/.proto/tools/node/globals/bin  Shims:    - /Users/name/.proto/shims/node  Binaries:    - /Users/name/.proto/bin/node    - /Users/name/.proto/bin/node-20    - /Users/name/.proto/bin/node-20.15    - /Users/name/.proto/bin/node-20.8    - /Users/name/.proto/bin/node-23    - /Users/name/.proto/bin/node-23.4    - /Users/name/.proto/bin/node-23.5  Installed versions:    20.8.0 - installed 12/19/24, last used 12/19/24    20.15.0 - installed 12/25/24, last used 12/25/24    23.4.0 - installed 12/19/24, last used 12/19/24    23.5.0 - installed 12/25/24, last used 12/25/24, fallback version  Remote aliases:    argon = 4.9.1    boron = 6.17.1    carbon = 8.17.0    dubnium = 10.24.1    erbium = 12.22.12    fermium = 14.21.3    gallium = 16.20.2    hydrogen = 18.20.5    iron = 20.18.1    jod = 22.12.0    latest = 23.5.0    stable = 22.12.0Configuration ──────────────────────────  Local aliases:    example = 19.0.0  Environment variables: —  Settings: —
```

### Arguments

- `` - ID of tool.

### Options

- `--json` - Print the info in JSON format.

## /docs/proto/commands/plugin/list

Source: https://moonrepo.dev/docs/proto/commands/plugin/list

# plugin list

v0.23.0

The `proto plugin list [...id]` command will list all available and configured plugins, for both
third-party and built-in tools. Will load all `./.prototools` traversing upwards, and the
`~/.proto/.prototools` file.

Furthermore, it can list tool information, along with their installed versions, relevant timestamps,
available aliases, and store location.

```
$ proto plugin list --versionsBun ────────────────────────────────────  ID: bun  Source URL: https://github.com/moonrepo/plugins/releases/download/bun_tool-v0.14.0/bun_tool.wasm  Store directory: /Users/miles/.proto/tools/bun  Versions:    1.1.42 - installed 12/25/24, fallback versionDeno ───────────────────────────────────  ID: deno  Source URL: https://github.com/moonrepo/plugins/releases/download/deno_tool-v0.13.0/deno_tool.wasm  Store directory: /Users/miles/.proto/tools/deno  Versions:    1.30.0 - installed 02/01/24, last used 11/28/24    1.40.0 - installed 02/01/24, last used 12/09/24    1.43.1 - installed 12/25/24, fallback versionGo ─────────────────────────────────────  ID: go  Source URL: https://github.com/moonrepo/plugins/releases/download/go_tool-v0.14.0/go_tool.wasm  Store directory: /Users/miles/.proto/tools/go  Versions:    1.18.0 - installed 12/25/24, fallback version    1.19.0 - installed 12/22/24    1.20.12 - installed 12/09/23    1.23.4 - installed 12/24/24
```

A list of tool IDs can be provided to filter the output list.

```
$ proto plugin list node npm
```

### Arguments

- `[id...]` - IDs of tools.

### Options

- `--aliases` - Print the list with resolved aliases.

- `--versions` - Print the list with installed versions.

- `--json` - Print the list in JSON format.

## /docs/proto/commands/plugin/remove

Source: https://moonrepo.dev/docs/proto/commands/plugin/remove

# plugin remove

v0.23.0

The `proto plugin remove ` command will remove the provided tool ID from the `[plugins]` section
of the chosen (`.prototools`).

```
$ proto plugin remove node
```

Built-in plugins cannot be removed!

### Arguments

- `` - ID of the tool.

### Options

- `--from` - [Location of `.prototools`](/docs/proto/config#locations) to update. v0.41.0

- `--type` - Type of plugin to remove, either `tool` (default) or `backend`. v0.52.0

## /docs/proto/commands/plugin/search

Source: https://moonrepo.dev/docs/proto/commands/plugin/search

# plugin search

v0.36.0

The `proto plugin search ` command will search for plugins provided by the community, based
on the provided query string. Built-in plugins are not searchable.

```
$ proto plugin search moonSearch results for: moonLearn more about plugins: https://moonrepo.dev/docs/proto/plugins╭──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮│ Plugin      Author    Format Description             Locator                                                             ││──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────││ moon        moonrepo  TOML   moon is a multi-        https://raw.githubusercontent.com/moonrepo/moon/master/proto-       ││                              language build system   plugin.toml                                                         ││                              and codebase management                                                                     ││                              tool.                                                                                       │╰──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

### Arguments

- `` - Query string to match against.

### Options

- `--json` - Print the results in JSON format.

## /docs/proto/commands/regen

Source: https://moonrepo.dev/docs/proto/commands/regen

# regen

v0.27.0

The `proto regen` command can be used to regenerate all shims in the `~/.proto/shims` directory.
This command will also clean the shims directory before regenerating, in an effort to remove
unexpected or broken shims.

```
$ proto regen
```

By default this will only regenerate shims. If you want to regenerate bins in `~/.proto/bin` as
well, pass the `--bin` flag. This will also clean the bins directory before regenerating.

```
$ proto regen --bin
```

Only versions pinned in `~/.proto/.prototools` will be linked as bins.

## Options

- `--bin` - Also recreate `~/.proto/bin` symlinks.

## /docs/proto/commands/run

Source: https://moonrepo.dev/docs/proto/commands/run

# run

The `proto run  [version]` (or `proto r`) command will run a tool after
[detecting a version](/docs/proto/detection) from the environment.

```
# Run and detect version from environment$ proto run bun# Run with explicit version$ proto run bun 0.5.3# Run with version from environment variable$ PROTO_BUN_VERSION=0.5.3 proto run bun
```

Arguments can be passed to the underlying tool binary by providing additional arguments after `--`.

```
$ proto run bun -- run ./script.ts# When using the binary on PATH$ bun run ./script.ts
```

### Arguments

- `` - Type of tool.

- `[version]` - Version of tool. If not provided, will attempt to detect the version from the environment.

## /docs/proto/commands/setup

Source: https://moonrepo.dev/docs/proto/commands/setup

# setup

The `proto setup` command will setup proto in your current shell by modifying an applicable profile
file and appending proto's bin directory to `PATH`. If a shell could not be detected, you'll be
prompted to select one.

```
$ proto setup
```

During setup, the following profiles will be searched or prompted for.

- Bash `~/.bash_profile`

- `~/.bashrc`

- `~/.profile`

- Elvish `~/.elvish/rc.elv`

- `~/.config/elvish/rc.elv`

- Fish `~/.config/fish/config.fish`

- Ion `~/.config/ion/initrc`

- Murex `~/.murex_preload`

- `~/.murex_profile`

- Nu `~/.config/nushell/env.nu`

- `~/.config/nushell/config.nu`

- PowerShell Windows `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`

- `~\Documents\PowerShell\Profile.ps1`

- Unix `~/.config/powershell/Microsoft.PowerShell_profile.ps1`

- `~/.config/powershell/profile.ps1`

- Xonsh `~/.config/xonsh/rc.xsh`

- `~/.xonshrc`

- Zsh `~/.zprofile`

- `~/.zshenv`

- `~/.zshrc`

### Windows support

In addition to updating a shell profile file (most likely PowerShell), we'll also modify the `PATH`
(or `Path`) system environment variable, by prepending the `~/.proto/shims` and `~/.proto/bin`
paths.

If you would like to opt-out of this behavior, pass the `--no-modify-path` flag.

### Options

- `--shell` - Shell to explicitly setup for.

- `--no-modify-profile` / `PROTO_NO_MODIFY_PROFILE` - Don't update a shell profile file.

- `--no-modify-path` / `PROTO_NO_MODIFY_PATH` - Don't update the system `PATH` environment variable (Windows only).

- `--yes` - Avoid interactive prompts and use defaults.

## /docs/proto/commands/status

Source: https://moonrepo.dev/docs/proto/commands/status

# status

v0.34.0

The `proto status` command will list all tools that are currently active for a target directory,
what versions of those tools are resolved to, and the configuration file in which they are defined.

```
$ proto status╭───────────────────────────────────────────────────────────────────────────────────────────────────────╮│ Tool      Configured Resolved  Installed                           Config                             ││───────────────────────────────────────────────────────────────────────────────────────────────────────││ bun       1.1.42     1.1.42    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                bun/1.1.42                                                             ││ deno      1.43.1     1.43.1    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                deno/1.43.1                                                            ││ node      23.5.0     23.5.0    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                node/23.5.0                                                            ││ npm       ~10.7      10.7.0    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                npm/10.7.0                                                             ││ python    3.12.0     3.12.0    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                python/3.12.0                                                          ││ yarn      3.6.3      3.6.3     /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                yarn/3.6.3                                                             │╰───────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

By default, this command does not check tools for versions pinned in the global
`~/.proto/.prototools` file. Pass `--config-mode all` to include them.

### Options

- `--json` - Print the list in JSON format.

## /docs/proto/commands/unalias

Source: https://moonrepo.dev/docs/proto/commands/unalias

# unalias

The `proto unalias  ` (or `proto ua`) command will remove a custom alias for the
provided tool.

```
$ proto unalias node work
```

By default this will update the local [`./.prototools`](/docs/proto/config) file. Pass `--from` to customize
the location.

### Arguments

- `` - Type of tool.

- `` - Name of the alias. Supports alphanumeric chars.

## Options

- `--from` - [Location of `.prototools`](/docs/proto/config#locations) to update. Supports `global`, `local`, and `user`. v0.41.0

## /docs/proto/commands/uninstall

Source: https://moonrepo.dev/docs/proto/commands/uninstall

# uninstall

The `proto uninstall  [version]` (or `proto ui`) command will uninstall and remove a tool from
`~/.proto/tools/`. If the tool has not been installed, the command will exit early.

```
# Remove a specific version$ proto uninstall deno 1.31# Remove all versions$ proto uninstall deno
```

### Arguments

- `` - Type of tool.

- `[version]` - Version of tool.

## /docs/proto/commands/unpin

Source: https://moonrepo.dev/docs/proto/commands/unpin

# unpin

v0.36.0

The `proto unpin ` command will unpin a version of a tool.

```
$ proto unpin go$ proto unpin node --tool-native
```

By default this will update the local [`./.prototools`](/docs/proto/config) file. Pass `--from` to customize
the location.

### Arguments

- `` - Type of tool.

### Options

- `--from` - [Location of `.prototools`](/docs/proto/config#locations) to update. Supports `global`, `local`, and `user`. v0.41.0

- `--tool-native` - Use a tool specific location, like the `devEngines` field in the `package.json` for JavaScript tools. v0.55.0

## /docs/proto/commands/upgrade

Source: https://moonrepo.dev/docs/proto/commands/upgrade

# upgrade

The `proto upgrade` (or `proto up`) command can be used to upgrade your current proto binary to the
latest version, or check if you're currently outdated.

```
$ proto upgrade# Up/downgrade to a specific version$ proto upgrade 0.39.0
```

info

The previous binary will be moved to `~/.proto/tools/proto/`, while the new binary will be
installed to `~/.proto/bin`.

### Arguments

- `` - The version of proto to explicitly upgrade or downgrade to. v0.39.3

### Options

- `--check` - Check if there's a new version without executing the upgrade.

- `--json` - Print the upgrade information as JSON.

## /docs/proto/commands/use

Source: https://moonrepo.dev/docs/proto/commands/use

# use

danger

This command has been deprecated and its functionality was merged into [`proto install`](/docs/proto/commands/install)
in v0.39. Use that command instead!

The `proto use` (or `proto u`) command will download and install all tools and plugins from all
parent [`.prototools`](/docs/proto/config) configuration files, and any [versions detected](/docs/proto/detection) in
the current working directory (if not defined in `.prototools`).

```
$ proto use
```

This command does not install tools for versions pinned in the global `~/.proto/.prototools`
file.

## /docs/proto/commands/versions

Source: https://moonrepo.dev/docs/proto/commands/versions

# versions

v0.44.0

The `proto versions ` command will list available versions by resolving versions from the
tool's remote release manifest. Furthermore, if a version has been installed locally, it will be
denoted with a timestamp.

```
$ proto versions node...22.0.022.1.022.2.022.3.022.4.022.4.122.5.0 - installed 12/25/2422.5.122.6.022.7.022.8.022.9.022.10.022.11.022.12.023.0.023.1.023.2.023.3.023.4.0 - installed 12/19/2423.5.0 - installed 12/25/24
```

### Arguments

- `` - Type of tool.

### Options

- `--aliases` - Include aliases in the list.

- `--installed` - Only display installed versions.

- `--json` - Print the versions and aliases in JSON format.

## /docs/proto/config

Source: https://moonrepo.dev/docs/proto/config

# Configuration

We support configuration at both the project-level and user-level using a
[TOML](https://toml.io/en/) based `.prototools` file. This file can be used to pin versions of
tools, provide tool specific configuration, enable new tools via plugins, define proto settings, and
more.

## Locationsv0.41.0

proto supports 3 locations in which a `.prototools` file can exist. These locations are used
throughout the command line and proto's own settings.

- `local` -> `./.prototools`, `.\.prototools` (current directory)

- `global` -> `~/.proto/.prototools`, `%USERPROFILE%\.proto\.prototools`

- `user` -> `~/.prototools`, `%USERPROFILE%\.prototools`

Local is a bit of a misnomer as a `.prototools` file can theoretically exist in any directory, but
when reading/writing to a file, `local` refers to the current working directory.

### Where to configure?

With so many locations to store proto configuration, the question of where to store certain
configurations become blurred, especially when [resolution](#resolution-mode) comes into play. We
suggest the following locations:

- Default/fallback [versions](#pinning-versions) of tools -> `global`

- Project specific [versions](#pinning-versions) of tools -> `local`

- Project specific [settings](#settings) -> `local`

- Shared/developer [settings](#settings) -> `user`

- Non-project related -> `user`

## Resolution modev0.40.0

When a `proto` command or shim is ran, we must find and load all applicable `.prototools` files. We
then deeply merge all of these configuration files into a final configuration object, with the
current directory taking highest precedence.

The order in which to resolve configuration can be defined using the `--config-mode` (`-c`) command
line option, or the `PROTO_CONFIG_MODE` environment variable. The following 4 modes are supported:

### `global`

In this mode, proto will only load the `~/.proto/.prototools` file. This "global" file acts as
configuration at the user-level and allows for fallback settings.

```
~/.proto/.prototools
```

### `local`

In this mode, proto will only load the `.prototools` file in the current directory.

```
./.prototools
```

### `upwards`

In this mode, proto will traverse upwards starting from the current directory, and load
`.prototools` within each directory, until we reach the system root or the user directory (`~`),
whichever comes first.

```
~/Projects/app/.prototools (cwd)~/Projects/.prototools~/.prototools
```

This is the default mode for the [`activate`](/docs/proto/commands/activate),
[`install`](/docs/proto/commands/install), [`outdated`](/docs/proto/commands/outdated), and
[`status`](/docs/proto/commands/status) commands.

### `upwards-global` / `all`

This mode works exactly like [`upwards`](#upwards) but with the functionality of [`global`](#global)
as well. The global `~/.proto/.prototools` file is appended as the final entry.

```
~/Projects/app/.prototools (cwd)~/Projects/.prototools~/.prototools~/.proto/.prototools
```

This is the default mode for all other commands not listed above in `upwards`.

## Environment modev0.29.0

We also support environment specific configuration, such as `.prototools.production` or
`.prototools.development`, when the `PROTO_ENV` environment variable is set. This is useful for
defining environment specific aliases, or tool specific configuration.

These environment aware settings take precedence over the default `.prototools` file, for the
directory it's located in, and are merged in the same way as the default configuration. For example,
the lookup order would be the following when `PROTO_ENV=production`:

```
~/Projects/.prototools.production~/Projects/.prototools~/.prototools.production~/.prototools~/.proto/.prototools
```

The global `~/.proto/.prototools` file does not support environment modes.

## Pinning versions

proto supports pinning versions of tools on a per-directory basis through our `.prototools`
configuration file. This file takes precedence during [version detection](/docs/proto/detection) and can be
created/updated with [`proto pin`](/docs/proto/commands/pin).

At its most basic level, you can map tools to specific versions, for the directory the file is
located in. A [version](/docs/proto/tool-spec) can either be a fully-qualified version, a partial version, a
range or requirement, or an alias.

.prototools

```
node = "16.16.0"npm = "9"go = "~1.20"rust = "stable"
```

### Lock `proto` versionv0.39.0

You can also pin the version of proto that you want all tools to execute with by adding a `proto`
version entry. This approach uses shims and dynamic version detection like other tools.

.prototools

```
proto = "0.38.0"
```

## Available settings

### `[env]`v0.29.0

This setting is a map of environment variables that will be applied to all tools when they are
executed, or when [`proto activate`](/docs/proto/commands/activate) is ran in a shell profile. Variables
defined here will not override existing environment variables (either passed on the command line,
or inherited from the shell).

.prototools

```
[env]DEBUG = "*"
```

Additionally, `false` can be provided as a value, which will remove the environment variable. This
is useful for removing inherited shell variables.

.prototools

```
[env]DEBUG = false
```

Variables also support substitution using the syntax `${VAR_NAME}`. When using substitution,
variables in the current process and merged `[env]` can be referenced. Recursive substitution is not
supported!

This functionality enables per-directory environment variables!

#### `file`v0.43.0

This is a special field that points to a dotenv file, relative from the current configuration file,
that will be loaded into the environment variables mapping. Variables defined in a dotenv file will
be loaded before variables manually defined within `[env]`.

This feature utilizes the [dotenvy](https://github.com/allan2/dotenvy) crate for parsing dotfiles.

.prototools

```
[env]file = ".env"
```

### `[settings]`

#### `auto-install`

When enabled, will automatically install a missing version of a tool when
[`proto run`](/docs/proto/commands/run) is ran, instead of erroring. Defaults to `false` or
`PROTO_AUTO_INSTALL`.

.prototools

```
[settings]auto-install = true
```

warning

This functionality requires shims (not activation) and will only work after a tool has been
installed at least once. This is because the shim executable handles the interception and the shim
is created after a tool is installed.

#### `auto-clean`

When enabled, will automatically clean up the proto store in the background, by removing unused
tools and outdated plugins. Defaults to `false` or `PROTO_AUTO_CLEAN`.

.prototools

```
[settings]auto-clean = true
```

#### `builtin-plugins`v0.39.0

Can be used to customize the [built-in plugins](/docs/proto/tools#built-in) within proto. Can disable all
built-ins by passing `false`, or enabling a select few by name. Defaults to `true`, which enables
all.

.prototools

```
[settings]# Disable allbuiltin-plugins = false# Enable somebuiltin-plugins = ["node", "bun"]
```

#### `cache-duration`v0.50.1

The duration in seconds in which to cache downloaded plugins. Defaults to 30 days.

.prototools

```
[settings]cache-duration = 3600
```

#### `detect-strategy`

The strategy to use when [detecting versions](/docs/proto/detection). Defaults to `first-available` or
`PROTO_DETECT_STRATEGY`.

- `first-available` - Will use the first available version that is found. Either from `.prototools` or a tool specific file (`.nvmrc`, etc).

- `prefer-prototools` - Prefer a `.prototools` version, even if found in a parent directory. If none found, falls back to tool specific file.

- `only-prototools` - Only use a version defined in `.prototools`. v0.34.0

.prototools

```
[settings]detect-strategy = "prefer-prototools"
```

#### `pin-latest`

When defined and a tool is installed with the "latest" alias, will automatically pin the resolved
version to the configured location. Defaults to disabled or `PROTO_PIN_LATEST`.

- `global` - Pins globally to `~/.proto/.prototools`.

- `local` - Pins locally to `./.prototools` in current directory.

- `user` - Pins to the user's `~/.prototools` in their home directory. v0.41.0

.prototools

```
[settings]pin-latest = "local"
```

#### `telemetry`

When enabled, we collect anonymous usage statistics for tool installs and uninstalls. This helps us
prioritize which tools to support, what tools or their versions may be broken, the plugins currently
in use, and more. Defaults to `true`.

.prototools

```
[settings]telemetry = false
```

The data we track is publicly available and
[can be found here](https://github.com/moonrepo/proto/blob/master/crates/cli/src/telemetry.rs).

#### `unstable-lockfile`v0.51.0

When enabled, will create a `.protolock` file relative to this configuration file. The lockfile will
record and lock all tools, their versions, and checksums from the configuration file, ensuring
consistency across machines, and reliability.

.prototools

```
[settings]unstable-lockfile = true
```

#### `unstable-registries`v0.51.0

A list of OCI registries to query for plugins by
[reference](https://oras.land/docs/concepts/reference). Registries will be queried in the order they
are configured. Each registry object supports the following fields:

- `registry` - The registry host, e.g. `ghcr.io`.

- `namespace` - The namespace (or organization) that the plugin belongs to.

.prototools

```
[settings]unstable-registries = [  { registry: "ghcr.io", namespace: "moonrepo" }]
```

#### `url-rewrites`v0.50.0

Provides a mechanism for rewriting most URLs used by proto, such as those used for downloading
tools. This setting accepts a map of [Rust regular expressions](https://docs.rs/regex/latest/regex/)
to [replacement strings](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace). When
a URL is rewritten, all entries in the map are applied in order, and all matches are replaced.

.prototools

```
[settings.url-rewrites]"github.com/(\\w+)/(\\w+)" = "gh-mirror.corp.com/$1/$2""mo+n" = "lunar"
```

The following types of URLs are rewritten:

- Tool download/checksum URLs (even from third-party plugins)

- Plugin download URLs

- Build script URLs

- Archive URLs

The following are not rewritten:

- Git repository URLs

- proto version check/telemetry URLs

### `[settings.build]`v0.46.0

Can be used to customize the build from source flow.

#### `exclude-packages`

Configures a list of packages that should be excluded during installation.

.prototools

```
[settings.build]exclude-packages = ["git", "python3", "libssl-dev"]
```

#### `install-system-packages`

When enabled, will install packages required for building using the system package manager. Defaults
to `true`.

.prototools

```
[settings.build]install-system-packages = false
```

#### `system-package-manager`

Customize the system package manager to use when installing system packages and their dependencies.
By default we attempt to detect the package manager to use from the environment.

This setting accepts a map, where the key is the name of the
[operating system](https://doc.rust-lang.org/std/env/consts/constant.OS.html), and the value is the
[package manager](https://docs.rs/system_env/latest/system_env/enum.SystemPackageManager.html) to
use. Both the key and value are in kebab-case.

.prototools

```
[settings.build.system-package-manager]windows = "choco"
```

#### `write-log-file`

When a build has completed, write a log file to the current directory. This is always `true` when a
build fails, but `false` otherwise.

.prototools

```
[settings.build]write-log-file = true
```

### `[settings.http]`

Can be used to customize the HTTP client used by proto, primarily for requesting files to download,
available versions, and more.

#### `allow-invalid-certs`

When enabled, will allow invalid certificates instead of failing. This is an escape hatch and
should only be used if other settings have failed. Be sure you know what you're doing! Defaults to
`false`.

.prototools

```
[settings.http]allow-invalid-certs = true
```

#### `proxies`

A list of proxy URLs to use for requests. As an alternative, the `HTTP_PROXY` and `HTTPS_PROXY`
environment variables can be set. URLs that start with `http://` will be considered insecure, while
`https://` will be secure.

.prototools

```
[settings.http]proxies = ["https://internal.proxy", "https://corp.net/proxy"]
```

#### `secure-proxies`v0.40.3

A list of proxy URLs that will be considered secure, regardless of the HTTP protocol.

.prototools

```
[settings.http]secure-proxies = ["http://internal.proxy", "http://corp.net/proxy"]
```

#### `root-cert`

The path to a root certificate to use for requests. This is useful for overriding the native
certificate, or for using a self-signed certificate, especially when in a corporate/internal
environment. Supports `pem` and `der` files.

.prototools

```
[settings.http]root-cert = "/path/to/root/cert.pem"
```

### `[settings.offline]`v0.41.0

Can be used to customize how we detect an internet connection for offline based logic. These
settings are useful if you're behind a VPN or corporate proxy.

#### `custom-hosts`

A list of custom hosts to ping. Will be appended to our
[default list of hosts](#override-default-hosts) and will be ran last.

.prototools

```
[settings.offline]custom-hosts = ["proxy.corp.domain.com:80"]
```

#### `override-default-hosts`

If our default hosts are blocked or are too slow, you can disable pinging them by setting this
option to true. Our default hosts are Google DNS, Cloudflare DNS, and then Google and Mozilla hosts.

This should be used in parallel with [`custom-hosts`](#custom-hosts).

.prototools

```
[settings.offline]override-default-hosts = true
```

#### `timeout`

The timeout in milliseconds to wait for a ping against a host to resolve. Default timeout is 750ms.

.prototools

```
[settings.offline]timeout = 500
```

### `[plugins]`

This setting was renamed to [`[plugins.tools]`](#pluginstools) in v0.52 but exists for backwards
compatibility.

## Backend specific settings

### `[plugins.backends]`v0.52.0

Custom [backend plugins](/docs/proto/plugins) can be configured with the `[plugins.backends]` section.
[Learn more about this syntax](/docs/proto/plugins#enabling-plugins).

.prototools

```
[plugins.backends]my-backend = "https://raw.githubusercontent.com/my/backend/master/proto-plugin.toml"
```

Once configured, you can manage a tool plugin using your custom backend:

```
$ proto install my-backend:tool-id
```

### `[backends.*]`v0.53.0

Backends support custom configuration that will be passed to their WASM plugin, which can be used to
control the behavior for all tools managed by the backend. Please refer to the
[official documentation](/docs/proto/tool-spec#backends) around backends.

.prototools

```
[backends.example]setting = true
```

### `[backends.*.env]`v0.53.0

This setting is a map of environment variables for a specific backend, and will be applied when that
backend is executed through a managed tool, or when [`proto activate`](/docs/proto/commands/activate) is ran
in a shell profile. These variables will override those defined in `[env]`. Refer to [`[env]`](#env)
for usage examples.

.prototools

```
[backends.example.env]KEY = "value"
```

#### `file`v0.53.0

Like [`[env].file`](#file), this is a path to a dotenv file, relative from the current configuration
file, that will be loaded into the environment variables mapping for this specific backend.

.prototools

```
[backends.example.env]file = "backend/.env"
```

## Tool specific settings

### `[plugins.tools]`v0.52.0

Custom [tool plugins](/docs/proto/plugins) can be configured with the `[plugins.tools]` section.
[Learn more about this syntax](/docs/proto/plugins#enabling-plugins).

.prototools

```
[plugins.tools]my-tool = "https://raw.githubusercontent.com/my/tool/master/proto-plugin.toml"
```

Once configured, you can manage a tool plugin:

```
$ proto install my-tool
```

### `[tools.*]`

Tools support custom configuration that will be passed to their WASM plugin, which can be used to
control the business logic within the plugin. Please refer to the [official documentation](/docs/proto/tools)
of each tool (typically on their repository) for a list of available settings.

As an example, let's configure [Node.js](https://github.com/moonrepo/node-plugin) (using the `node`
identifier).

.prototools

```
npm = "bundled" # use bundled npm instead of specific version[tools.node]bundled-npm = true[tools.npm]shared-globals-dir = true
```

### `[tools.*.aliases]`

Aliases are custom and unique labels that map to a specific version, and can be configured manually
within `.prototools`, or by calling the [`proto alias`](/docs/proto/commands/alias) command.

.prototools

```
[tools.node.aliases]work = "18"oss = "20"
```

### `[tools.*.env]`v0.29.0

This setting is a map of environment variables for a specific tool, and will be applied when that
tool is executed, or when [`proto activate`](/docs/proto/commands/activate) is ran in a shell profile. These
variables will override those defined in `[env]`. Refer to [`[env]`](#env) for usage examples.

.prototools

```
[tools.node.env]NODE_ENV = "production"
```

#### `file`v0.43.0

Like [`[env].file`](#file), this is a path to a dotenv file, relative from the current configuration
file, that will be loaded into the environment variables mapping for this specific tool.

.prototools

```
[tools.node.env]file = "frontend/.env"
```

## GitHub Action

To streamline GitHub CI workflows, we provide the
[`moonrepo/setup-toolchain`](https://github.com/moonrepo/setup-toolchain) action, which can be used
to install `proto` globally, and cache the toolchain found at `~/.proto`.

.github/workflows/ci.yml

```
# ...jobs:  ci:    name: 'CI'    runs-on: 'ubuntu-latest'    steps:      - uses: 'actions/checkout@v4'      - uses: 'moonrepo/setup-toolchain@v0'        with:          auto-install: true
```

## /docs/proto/detection

Source: https://moonrepo.dev/docs/proto/detection

# Version detection

2 min

The most powerful feature in proto is its contextual version detection, that is triggered with
[`proto run`](/docs/proto/commands/run), [`proto bin`](/docs/proto/commands/bin), or when a shim is executed. So what
does this mean exactly? Before a tool in proto's toolchain can be executed, we need to determine the
version of the tool to execute with. If a detected version exists locally, we proceed using that
binary, otherwise we fail with a missing installation error.

When detecting a version, the following steps are checked, in the order as listed:

#### 1. Version is explicitly passed as a command line argument

```
$ proto run node 24.0.0
```

#### 2. Version is provided with the `PROTO_*_VERSION` environment variable

```
$ PROTO_NODE_VERSION=24.0.0 proto run node
```

#### 3. Version is located by traversing the file system

This step will attempt to find a configuration or manifest file in the current working directory,
and traverse upwards through parent directories (stops at the user's home directory) until a file is
found.

##### 3.1. Version is defined locally in `.prototools`

A `.prototools` file was found and a version entry exists for the current tool. This is also known
as a "local version" and can be created with [`proto pin`](/docs/proto/commands/pin).

.prototools

```
node = "24.0.0"
```

##### 3.2. Version is defined in the tool's ecosystem

Depending on the tool, a version is extracted from a found file unique to that tool's ecosystem.
This includes version manager configs (`.nvmrc`, etc), manifest files (`package.json`, etc), and
more.

.nvmrc

```
24.0.0
```

package.json

```
{  "devEngines": {    "runtime": {      "name": "node",      "version": "24.0.0"    },    "packageManager": {      "name": "npm",      "version": "11.0.0"    }  }}
```

#### 4. Version is defined globally

As the last check, we look for a "global version" that was pinned with
[`proto pin --global`](/docs/proto/commands/pin) or [`proto install --pin`](/docs/proto/commands/install). This version
is stored at `~/.proto/.prototools` (`%USERPROFILE%\.proto\.prototools` on Windows).

#### 5. Version could not be detected

If all the previous steps have failed, then we could not detect an applicable version, and the
process will fail.

## /docs/proto/faq

Source: https://moonrepo.dev/docs/proto/faq

# FAQ

## General

### Where did the name "proto" come from?

We wanted to keep with the space theme, and spent quite some time digging through Wikipedia and
ultimately landed on the page for [protostar](https://en.wikipedia.org/wiki/Protostar) (this is why
our logo's a star). We really liked the definition of protostar, as it basically means "the
beginning phase of a star". Even the the prefix proto means "first" or "earliest form of".

This was great as that's the impression we had in mind for our tool. proto is the first piece
required for setting up your developer environment. The toolchain is the first layer in the
foundation.

From an aesthetic standpoint, proto's typography works well with moon, as most of the letters are
circle shaped. Double points for proto having two o's like the other products!

### Are you worried about confusion with other tools like protobufs?

Nah.

### What is a tool?

A tool in the context of proto is either a language, dependency/package manager (typically for a
language), or third-party CLI. The tool is something that can be downloaded and installed by
version onto a machine.

Furthermore, a tool should have a primary executable file that can be executed with `proto run` or
through proto's shims. Additionally, a tool can also provide secondary executable files. For
example, `npm` (the primary) also provides `npx` and `node-gyp` (secondaries).

### What is a backend?

A backend is a special type of tool that provides additional integration with 3rd-party plugins,
greatly expanding what can be installed and managed with proto.

### What is a plugin?

A plugin is a WASM (or JSON, TOML, YAML) file for a tool or backend.

The terms tool and plugin are often used interchangeably, but plugin primarily refers to the WASM
portion of a tool, while tool refers to the entire package: metadata, business logic, branding, so
on an so forth.

### Will you support more languages?

Yes! We'd love to support as many as possible, and if you'd like to help, join our Discord
community! Feel free to create a [plugin](/docs/proto/plugins) in the mean time.

### Will you support other kinds of tools?

No, we will only support languages, dependency managers, and CLIs, which should be enough. However,
you can create a [plugin](/docs/proto/plugins) to support other kinds of tools.

### Do you support "build from source"?

As of version 0.45, we do! Simple pass `--build` to `proto install`. However, building from source
is a complicated process and is unique per tool, so not all tools support it.

### How to run a canary release after installing it?

Once a tool has been installed with `canary`, the canary version can be explicitly referenced using
our [version detection rules](/docs/proto/detection). The easiest approach is to prefix the shim with an
environment variable:

```
$ PROTO_BUN_VERSION=canary bun ./index.ts
```

Or to explicitly configure the version in [`.prototools`](/docs/proto/config):

```
bun = "canary"
```

### What kind of features are supported for HTTP requests?

proto makes a lot of HTTP requests, for information such as available versions/releases, and for
downloading the blobs/archives themselves. Because of this, we do our best to support all kinds of
internet connections, proxy and intranet usage, and more, through the following:

- All GET and HEAD requests are cached to `~/.proto/cache/requests` based on the [HTTP cache semantics](https://github.com/kornelski/rusty-http-cache-semantics) and relevant RFCs.

- We support the [netrc file format](https://www.gnu.org/software/inetutils/manual/html_node/The-_002enetrc-file.html) and will automatically load `~/.netrc` if it exists.

- We support an offline mode that will short-circuit certain workflows if there's no internet connection. We check for a connection by pinging DNS endpoints, but this can be configured with [`[settings.offline]`](/docs/proto/config#settingsoffline).

- We attempt to automatically load root and system certifications so that secure connections work correctly. This can be configured with [`[settings.http]`](/docs/proto/config#settingshttp).

## Troubleshooting

### Network requests keep failing, how can I bypass?

When a tool is executed, we validate the version to ensure it's correct. We achieve this by making
network requests to a remote service to gather the list of valid versions. If you're having network
issues, or the request is timing out, you can bypass these checks with the following:

- Pass a fully-qualified version as an environment variable. The version must be installed for this to work. ``` PROTO_NODE_VERSION=20.0.0 node --version ``` If executing a Node.js package manager, you'll need to set versions for both Node.js and the manager. This is required since manager's execute `node` processes under the hood. ``` PROTO_NODE_VERSION=20.0.0 PROTO_NPM_VERSION=10.0.0 npm --version ```

- Pass the `PROTO_BYPASS_VERSION_CHECK` environment variable. This will bypass the network request to load versions, but does not bypass other requests. However, this is typically enough. ``` PROTO_BYPASS_VERSION_CHECK=1 node --version ```

## /docs/proto/install

Source: https://moonrepo.dev/docs/proto/install

# Install proto

1 min

The following guide can be used to install proto into your environment.

## Requirements

- Git - for fetching available versions/tags

- tar, unzip, gz, xz - for unpacking archives

```
# macOSbrew install git unzip gzip xz# Ubuntu / Debianapt-get install git unzip gzip xz-utils# RHEL-based / Fedoradnf install git unzip gzip xz
```

## Installing

The entirety of proto is packaged and shipped as 2 binaries. It works on most operating systems,
and does not require any external dependencies. For convenience, we provide the following scripts to
download and install proto.

info

The install location can be customized with the `PROTO_HOME` environment variable. If not provided,
the default location is `~/.proto`.

### Linux, macOS, WSL

In a terminal that supports Bash, run the following command. This will download and install proto,
then open an interactive prompt to complete the installation.

```
bash administrator Powershell or Windows Terminal, run the following command. This will download
and install proto, then open an interactive prompt to complete the installation.

```
irm https://moonrepo.dev/install/proto.ps1 | iex
```

You may also need to run the following command for shims to be executable:

```
Set-ExecutionPolicy RemoteSigned# Without admin privilegesSet-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

### Other

proto can also be downloaded and installed manually, by downloading an asset from
[https://github.com/moonrepo/proto/releases](https://github.com/moonrepo/proto/releases). Be sure to
rename the file after downloading, and apply the executable bit (`chmod +x`) on macOS and Linux.

## Upgrading

To upgrade proto, run the [`proto upgrade`](/docs/proto/commands/upgrade) command, or re-run the install
scripts above.

## Uninstalling

To uninstall proto, delete the `~/.proto` directory, and remove any `PROTO_HOME` references from
your shell profile.

## Canary releases

proto supports canary releases, which are built and published for every commit to our development
branches. These releases will include features and functionality that have not yet landed on master.
Canary releases are available as a
[GitHub prerelease](https://github.com/moonrepo/proto/releases/tag/canary) using the `canary` tag.

## Nightly releases

proto supports nightly releases, which are built and published once a day from the latest commit on
master. Nightly releases are available as a
[GitHub prerelease](https://github.com/moonrepo/proto/releases/tag/nightly) using the `nightly` tag.

## Next steps

[Choose a workflow](/docs/proto/workflows)[Learn about `.prototools`](/docs/proto/config)

## /docs/proto/non-wasm-plugin

Source: https://moonrepo.dev/docs/proto/non-wasm-plugin

# Non-WASM plugin

The non-WASM plugin is by design, very simple. It's a JSON, TOML, or YAML file that describes a
schema for the tool, how it should be installed, and how it should be invoked. Since this is a
static configuration file, it does not support any logic or complex behavior, and is merely for
simple and common use cases, like CLIs.

info

JSON and YAML support was added in proto v0.42.

## Create a plugin

Let's start by creating a new plugin, and defining the `name` and `type` fields. The type can either
be `language`, `dependency-manager`, `package-manager`, or `cli`. For this example, we'll create a
plugin for our fake product called Protostar, a CLI tool.

- JSON
- TOML
- YAML

protostar.json

```
{  "name": "Protostar",  "type": "cli"}
```

protostar.toml

```
name = "Protostar"type = "cli"
```

protostar.yaml

```
name: 'Protostar'type: 'cli'
```

### Platform variations

Native tools are often platform specific, and proto supports this by allowing you to define
variations based on operating system using the `[platform]` section. For non-native tools, this
section can typically be skipped.

This section requires a mapping of Rust
[`OS` strings](https://doc.rust-lang.org/std/env/consts/constant.OS.html) to platform settings. The
following settings are available:

- `archs` - A list of architectures supported for this platform. If not provided, supports all archs.

- `archive-prefix` - If the tool is distributed as an archive (zip, tar, etc), this is the name of the direct folder within the archive that contains the tool, and will be removed when unpacking the archive. If there is no prefix folder within the archive, this setting can be omitted.

- `exes-dir` - A relative path to a directory that contains pre-installed executables.

- `exe-path` - The path to the main executable binary within the archive (without the prefix). If the tool is distributed as a single binary, this setting can be typically omitted.

- `checksum-file` - Name of the checksum file to verify the downloaded file with. If the tool does not support checksum verification, this setting can be omitted.

- `download-file` (required) - Name of the file to download. [Learn more about downloading](#downloading-and-installing).

- JSON
- TOML
- YAML

protostar.json

```
{  "platform": {    "linux": {      "archivePrefix": "protostar-linux",      "exePath": "bin/protostar",      "checksumFile": "protostar-{arch}-unknown-linux-{libc}.sha256",      "downloadFile": "protostar-{arch}-unknown-linux-{libc}.tar.gz"    },    "macos": {      "archivePrefix": "protostar-macos",      "exePath": "bin/protostar",      "checksumFile": "protostar-{arch}-apple-darwin.sha256",      "downloadFile": "protostar-{arch}-apple-darwin.tar.xz"    },    "windows": {      "archivePrefix": "protostar-windows",      "exePath": "bin/protostar.exe",      "checksumFile": "protostar-{arch}-pc-windows-msvc.sha256",      "downloadFile": "protostar-{arch}-pc-windows-msvc.zip"    }  }}
```

protostar.toml

```
[platform][platform.linux]archive-prefix = "protostar-linux"exe-path = "bin/protostar"checksum-file = "protostar-{arch}-unknown-linux-{libc}.sha256"download-file = "protostar-{arch}-unknown-linux-{libc}.tar.gz"[platform.macos]archive-prefix = "protostar-macos"exe-path = "bin/protostar"checksum-file = "protostar-{arch}-apple-darwin.sha256"download-file = "protostar-{arch}-apple-darwin.tar.xz"[platform.windows]archive-prefix = "protostar-windows"exe-path = "bin/protostar.exe"checksum-file = "protostar-{arch}-pc-windows-msvc.sha256"download-file = "protostar-{arch}-pc-windows-msvc.zip"
```

protostar.yaml

```
platform:  linux:    archivePrefix: 'protostar-linux'    exePath: 'bin/protostar'    checksumFile: 'protostar-{arch}-unknown-linux-{libc}.sha256'    downloadFile: 'protostar-{arch}-unknown-linux-{libc}.tar.gz'  macos:    archivePrefix: 'protostar-macos'    exePath: 'bin/protostar'    checksumFile: 'protostar-{arch}-apple-darwin.sha256'    downloadFile: 'protostar-{arch}-apple-darwin.tar.xz'  windows:    archivePrefix: 'protostar-windows'    exePath: 'bin/protostar.exe'    checksumFile: 'protostar-{arch}-pc-windows-msvc.sha256'    downloadFile: 'protostar-{arch}-pc-windows-msvc.zip'
```

You may have noticed tokens above, like `{arch}`. These are special tokens that are replaced with a
dynamic value at runtime, based on the current host machine executing the code. The following tokens
are available:

- `{version}` - The currently resolved version, as a fully-qualified semantic or calendar version.

- `{versionMajor}` / `{versionYear}` - Only the major version. v0.41.4

- `{versionMinor}` / `{versionMonth}` - Only the minor version. v0.45.2

- `{versionPatch}` / `{versionDay}` - Only the patch version. v0.45.2

- `{versionPrerelease}` - The prerelease identifier, if applicable. Returns an empty string otherwise. v0.41.4

- `{versionBuild}` - The build identifier, if applicable. Returns an empty string otherwise. v0.41.4

- `{arch}` - The architecture of the host machine, like `x86_64`. These values map to Rust's [`ARCH` constant](https://doc.rust-lang.org/std/env/consts/constant.ARCH.html), but can be customized with [`install.arch`](#downloading-and-installing).

- `{os}` - The operating system of the host machine, like `windows`. These values map to Rust's [`OS` constant](https://doc.rust-lang.org/std/env/consts/constant.OS.html).

- `{libc}` - For Linux machines, this is the current libc implementation, either `gnu` or `musl`. v0.31.2

### Downloading and installing

A non-WASM plugin only supports downloading pre-built tools, typically as an archive, and does
not support building from source. The `[install]` section can be used to configure how the tool
should be downloaded and installed into the toolchain. The following settings are available:

- `arch` - A mapping of Rust [`ARCH` strings](https://doc.rust-lang.org/std/env/consts/constant.ARCH.html) to custom values for the `{arch}` token. This is useful if the tool has different terminology.

- `libc` - A mapping of custom values for the `{libc}` token.

- `checksum-url` - A secure URL to download the checksum file for verification. If the tool does not support checksum verification, this setting can be omitted.

- `checksum-url-canary` - A URL for canary releases.

- `checksum-public-key` - Public key used for verifying checksums. Only used for `.minisig` files.

- `download-url` (required) - A secure URL to download the tool/archive.

- `download-url-canary` - A URL for canary releases.

- `primary` - Configures the primary executable.

- `secondary` - Configures secondary executables.

The URL settings support `{checksum_file}` and `{download_file}` tokens, which will be replaced with
the values from the `[platform]` section.

- JSON
- TOML
- YAML

protostar.json

```
{  "install": {    "checksumUrl": "https://github.com/moonrepo/protostar/releases/download/v{version}/{checksum_file}",    "downloadUrl": "https://github.com/moonrepo/protostar/releases/download/v{version}/{download_file}",    "arch": {      "aarch64": "arm64",      "x86_64": "x64"    }  }}
```

protostar.toml

```
[install]checksum-url = "https://github.com/moonrepo/protostar/releases/download/v{version}/{checksum_file}"download-url = "https://github.com/moonrepo/protostar/releases/download/v{version}/{download_file}"[install.arch]aarch64 = "arm64"x86_64 = "x64"
```

protostar.yaml

```
install:  checksumUrl: 'https://github.com/moonrepo/protostar/releases/download/v{version}/{checksum_file}'  downloadUrl: 'https://github.com/moonrepo/protostar/releases/download/v{version}/{download_file}'  arch:    aarch64: 'arm64'    x86_64: 'x64'
```

#### Executables

The available executables (bins and shims) can be customized with the `[install.exes]` section,
which is required. This setting requires a map, where the key is the executable file name, and the
value is an object of the following options:

- `exe-path` - The file to execute, relative from the tool directory. On Windows, the `.exe` extension will automatically be appended. If you need more control over platform variance, use `[platform.*.exe-path]` instead.

- `no-bin` - Do not symlink a binary in `~/.proto/bin`.

- `no-shim`- Do not generate a shim in `~/.proto/shims`.

- `parent-exe-name` - Name of a parent executable required to execute the executable path. For example, `node` is required for `.js` files.

- `primary` - Is the main executable in the tool. There can only be 1 primary! v0.42.0

- `shim-before-args` - Custom args to prepend to user-provided args within the generated shim.

- `shim-after-args` - Custom args to append to user-provided args within the generated shim.

- `shim-env-vars` - Custom environment variables to set when executing the shim.

This field supports both the required primary executable, and optional secondary executables. The
primary executable must be marked with `primary = true`.

- JSON
- TOML
- YAML

protostar.json

```
{  "install": {    "exes": {      "protostar": {        "exePath": "bins/protostar",        "primary": true,        "shimBeforeArgs": [          "--verbose"        ]      },      "protostar-debug": {        "exePath": "bins/protostar-debug",        "noShim": true      }    }  }}
```

protostar.toml

```
[install][install.exes][install.exes.protostar]exe-path = "bins/protostar"primary = trueshim-before-args = [ "--verbose" ][install.exes.protostar-debug]exe-path = "bins/protostar-debug"no-shim = true
```

protostar.yaml

```
install:  exes:    protostar:      exePath: 'bins/protostar'      primary: true      shimBeforeArgs:        - '--verbose'    protostar-debug:      exePath: 'bins/protostar-debug'      noShim: true
```

#### Global packages

The `[packages]` sections can be configured that provides information about where global packages
are stored.

- `globals-lookup-dirs` - A list of directories where global binaries are stored. This setting supports interpolating environment variables via the syntax `$ENV_VAR`.

- `globals-prefix` - A string that all package names are prefixed with. For example, Cargo/Rust binaries are prefixed with `cargo-`.

- JSON
- TOML
- YAML

protostar.json

```
{  "packages": {    "globalsLookupDirs": [      "$PROTOSTAR_HOME/bin",      "$HOME/.protostar/bin"    ]  }}
```

protostar.toml

```
[packages]globals-lookup-dirs = [ "$PROTOSTAR_HOME/bin", "$HOME/.protostar/bin" ]
```

protostar.yaml

```
packages:  globalsLookupDirs:    - '$PROTOSTAR_HOME/bin'    - '$HOME/.protostar/bin'
```

### Resolving versions

Now that the tool can be downloaded and installed, we must configure how to resolve available
versions. Resolving is configured through the `[resolve]` section, which supports 2 patterns to
resolve with: Git tags or a JSON manifest.

#### Git tags

To resolve a list of available versions using Git tags, the following settings are available:

- `git-url` (required) - The remote URL to fetch tags from.

- JSON
- TOML
- YAML

protostar.json

```
{  "resolve": {    "gitUrl": "https://github.com/moonrepo/protostar"  }}
```

protostar.toml

```
[resolve]git-url = "https://github.com/moonrepo/protostar"
```

protostar.yaml

```
resolve:  gitUrl: 'https://github.com/moonrepo/protostar'
```

#### JSON manifest

To resolve a list of available versions using a JSON manifest, the following settings are available:

- `manifest-url` (required) - A URL that returns a JSON response of all versions. This response must be an array of strings, or an array of objects.

- `manifest-version-key` - If the response is an array of objects, this is the key to extract the version from. If the response is an array of strings, this setting can be omitted. Defaults to `version`.

- JSON
- TOML
- YAML

protostar.json

```
{  "resolve": {    "manifestUrl": "https://someregistry.com/protostar/versions.json",    "manifestVersionKey": "latest_version"  }}
```

protostar.toml

```
[resolve]manifest-url = "https://someregistry.com/protostar/versions.json"manifest-version-key = "latest_version"
```

protostar.yaml

```
resolve:  manifestUrl: 'https://someregistry.com/protostar/versions.json'  manifestVersionKey: 'latest_version'
```

#### Versions and aliasesv0.36.0

As an alternative, we also support a static configuration of explicit versions and aliases. This is
useful if you have an internal tool that is relatively stable, or does not provide a means in which
to extract version information.

- `versions` - A list of versions.

- `aliases` - A mapping of alias names to versions.

- JSON
- TOML
- YAML

protostar.json

```
{  "resolve": {    "versions": [      "1.2.3",      "1.2.4",      "1.2.5"    ],    "aliases": {      "stable": "1.2.4"    }  }}
```

protostar.toml

```
[resolve]versions = [ "1.2.3", "1.2.4", "1.2.5" ][resolve.aliases]stable = "1.2.4"
```

protostar.yaml

```
resolve:  versions:    - '1.2.3'    - '1.2.4'    - '1.2.5'  aliases:    stable: '1.2.4'
```

#### Version patterns

When a version is found, either from a git tag or manifest key, we attempt to parse it into a
[valid version](/docs/proto/tool-spec) using a Rust based regex pattern and the `version-pattern` setting.

This pattern uses named regex capture groups (`(?...)`) to build the version, and to support
found versions that are not fully-qualified (they may be missing patch or minor versions). The
following groups are supported:

- `major` / `year` - The major version number. Defaults to `0` if missing.

- `minor` / `month` - The minor version number. Defaults to `0` if missing.

- `patch` / `day` - The patch version number. Defaults to `0` if missing.

- `pre` - The pre-release identifier, like "rc.0" or "alpha.0". Supports an optional leading `-`. Does nothing if missing.

- `build` - The build metadata, like a timestamp. Supports an optional leading `+`. Does nothing if missing.

- JSON
- TOML
- YAML

protostar.json

```
{  "resolve": {    "versionPattern": "^@protostar/cli@((?\\d+)\\.(?\\d+)\\.(?
\\d+))"  }}
```

protostar.toml

```
[resolve]version-pattern = "^@protostar/cli@((?\\d+)\\.(?\\d+)\\.(?
\\d+))"
```

protostar.yaml

```
resolve:  versionPattern: '^@protostar/cli@((?\d+)\.(?\d+)\.(?
\d+))'
```

If no named capture groups are found, the match at index `1` is used as the version.

### Detecting versions

And lastly, we can configure how to [detect a version](/docs/proto/detection) contextually at runtime, using
the `[detect]` setting. At this time, we only support 1 setting:

- `version-files` - A list of version files to extract from. The contents of these files can only be the version string itself.

- JSON
- TOML
- YAML

protostar.json

```
{  "detect": {    "versionFiles": [      ".protostar-version",      ".protostarrc"    ]  }}
```

protostar.toml

```
[detect]version-files = [ ".protostar-version", ".protostarrc" ]
```

protostar.yaml

```
detect:  versionFiles:    - '.protostar-version'    - '.protostarrc'
```

## /docs/proto/plugins

Source: https://moonrepo.dev/docs/proto/plugins

# Plugins

proto supports a pluggable architecture as a means for consumers to integrate and manage custom
tools (languages, CLIs, etc) within proto's toolchain. It's not possible for proto to support
everything in core directly, so plugins are a way for the community to extend the toolchain to
their needs.

## Enabling plugins

Plugins can be enabled by configuring them in [`.prototools`](/docs/proto/config#plugins) files, within the
`[plugins]` section. The map key is the plugin name in kebab-case, which is used as the
binary/tool name in proto, and also the name for configuration and cache purposes. The map value is
a [plugin locator string](/docs/guides/wasm-plugins#configuring-plugin-locations) that defines a
protocol and source location.

.prototools

```
[plugins.tools] = "
://"
```

## Creating plugins

To ease the plugin development process, proto supports 2 types of plugins, a
[non-WASM configuration based plugin](/docs/proto/non-wasm-plugin) for basic use cases, and a
[WASM based plugin](/docs/proto/wasm-plugin) for advanced use cases.

## Publish a plugin

proto's registry is currently powered by static JSON files located in our official
[proto repository](https://github.com/moonrepo/proto/tree/master/registry). View that link for
information on how to publish a plugin.

## /docs/proto/tool-spec

Source: https://moonrepo.dev/docs/proto/tool-spec

# Tool specification

3 min

Since proto is a toolchain for multiple tools, each with differing version formats, we must align
them on a standard specification that can resolve and store safely. To handle this, we've
implemented our own solution called the tool and version specification. This specification currently
supports semantic and calendar based versions, each with their own guidelines and caveats.

info

If you're implementing a plugin for a specific tool that has a different version format, you'll need
to re-format it into one of the specifications below.

## Backendsv0.47.0

A backend is an internal system that allows proto to use plugins from 3rd-party package/version
managers within proto, greatly expanding the amount of tools that proto can install and support.
This functionality is achieved through special WASM plugins under the hood.

To make use of a backend, prefix the tool identifier in `.prototools` with the backend's unique
identifier. For example, we can install Zig via asdf.

.prototools

```
# >= v0.52"asdf:zig" = "0.13.0"# does
not use the `asdf` binary itself, and instead emulates the environment as best we can. Because of
this, some tools may not be usable through proto.

.prototools

```
"asdf:" = "20"
```

By default, the ID pinned in `.prototools` is the
[asdf shortname](https://asdf-vm.com/plugins/create.html#plugin-shortname-index) used when cloning a
repository. If the ID is different than the shortname (`node` vs `nodejs`), you can configure the
`asdf-shortname` setting.

.prototools

```
"asdf:node" = "20"[tools."asdf.node"]asdf-shortname = "nodejs"
```

The following settings are supported:

- `asdf-shortname` (string) - The name of the [asdf plugin](https://github.com/asdf-vm/asdf-plugins) if different than the configured ID.

- `asdf-repository` (string) - The Git repository URL in which to locate [scripts](https://asdf-vm.com/plugins/create.html#scripts-overview). If not defined, is extracted from the shortname plugin index.

- `exes` (string[]) - List of executable file names (relative from `bin`) to be linked as a shim/bin. If not defined, we'll automatically scan the `bin` directory.

## Semantic versions

The most common format is [semver](https://semver.org/), also known as a semantic version. This
format requires major, minor, and patch numbers, with optional pre-release and build metadata.

.prototools

```
tool = "1.2.3"
```

### Syntax

- `..` - 1.2.3

- `..-` - 1.2.3-alpha.0

- `..-+` - 1.2.3-alpha.0+nightly456

- `..+` - 1.2.3+nightly456

### Guidelines

- major, minor, patch - `0-9` of any length

- pre, build - `a-z`, `0-9`, `-`, `.`

[Learn more about this format!](https://semver.org/#backusnaur-form-grammar-for-valid-semver-versions)

## Calendar versionsv0.37.0

Another popular format is [calver](https://calver.org/), also known as a calendar version, which
uses the calendar year, month, and day as version numbers. This format also supports pre-release and
build metadata, but with different syntax than semver.

.prototools

```
tool = "2025-02-26"
```

### Syntax

- `-` - 2024-02

- `--` - 2024-02-26

- `--.` - 2024-02-26.123

- `--_` - 2024-02-26_123

- `--.-` - 2024-02-26.123-alpha.0

- `--_-` - 2024-02-26_123-alpha.0

- `---` - 2024-02-26-alpha.0

### Guidelines

- year - `0-9` of 1-4 length If the year is not YYYY format, it will use the year 2000 as the base. For example, `24` becomes `2024`, and `124` becomes `2124`.

- month - `0-9` of 1-2 length Supports with and without a leading zero (`02` vs `2`).

- Does not support invalid months (`0` or `13`).

- day - `0-9` of 1-2 length Can be omitted, even with build/pre.

- Supports with and without a leading zero (`02` vs `2`).

- Does not support invalid days (`0` or `32`).

- build - `0-9` of any length Also known as a "micro" number.

- The leading dot `.` format is preferred.

- pre - `a-z`, `0-9`, `-`, `.`

[Learn more about this format!](https://calver.org/#scheme)

## Requirements and ranges

Besides an explicit version, we also support partial versions known as version requirements or
version ranges. These are quite complex as we need to support both semver and calver in unison, as
well as support partial/incomplete numbers (missing patch/day, missing minor/month, etc). We do our
best to support as many combinations as possible.

.prototools

```
tool-a = "^1"tool-b = "~2.1"tool-c = ">=2000-10"
```

### Syntax

- Requirement - `[]` - `1.2.3`, `>4.5`, `~3`, `^2000-10`, etc

- AND range - `[,] ...` - `>=1, `, `>=`, ` If omitted, defaults to `~` when a partial version is provided (i.e. allow the minimum specified version component to increase)

- If a full version is provided, defaults to `^` (i.e. allow any newer version that is nominally compatible)

- To specify an exact version, use the `=` operator explicitly.

- pattern Dot-separated semver, with optional major and patch numbers.

- Dash-separated calver, with optional month and day numbers.

- Pre-release and build metadata are only supported when suffixed to full versions.

For example, if you want to use `npm` 11.6.2 but 11.6.3 has an issue, use:
`npm = "^11.6.4 || =11.6.2"`

## /docs/proto/tools

Source: https://moonrepo.dev/docs/proto/tools

# Supported tools

## Built-in

The following tools are supported natively in proto's toolchain.

## Third-party

[Add tool](https://github.com/moonrepo/proto/tree/master/registry)

Additional tools can be supported through [plugins](/docs/proto/plugins).

## /docs/proto/wasm-plugin

Source: https://moonrepo.dev/docs/proto/wasm-plugin

# WASM plugin

If you want more control over how your tool works, a WASM plugin is the way to go.

success

Refer to our [official WASM guide](/docs/guides/wasm-plugins) for more information on how our WASM
plugins work, critical concepts to know, and more. Once you have a good understanding, you may
continue this proto specific guide.

## Concepts

The following concepts are unique to proto, but be sure to also read about the general concepts in
our [WASM plugins guide](/docs/guides/wasm-plugins#concepts).

### Tool context

For plugin functions, we provide what we call the tool context, which is information that is
constantly changing depending on the current step or state of proto's execution. The context cannot
be accessed with a stand-alone function, and is instead passed as a `context` field in the input of
many plugin functions.

```
#[plugin_fn]pub fn download_prebuilt(Json(input): Json) -> FnResult> {    let version = input.context.version;    // ...}
```

The following fields are available on the
[context object](https://docs.rs/proto_pdk/latest/proto_pdk/struct.ToolContext.html):

- `proto_version` - The version of proto executing the plugin. Note that this version may be for the [`proto_core` crate](https://crates.io/crates/proto_core), and not the CLI. Patch numbers will drift, but major and minor numbers should be in sync.

- `temp_dir` - A virtual path to a temporary directory unique to this tool. v0.45.0

- `tool_dir` - A virtual path to the tool's directory for the current version.

- `version` - The current version or alias. If not resolved, will be "latest".

caution

The `version` field is either a fully-qualified version (1.2.3), an alias ("latest", "stable"), or
canary ("canary"). Be sure to account for all these variations when implementing plugin functions!

### Tool configuration

Users can configure tools through the [`[tools.*]`](/docs/proto/config#tools) section of their `.prototools`,
which can then be accessed within the WASM plugin using the
[`get_tool_config`](https://docs.rs/proto_pdk/latest/proto_pdk/fn.get_tool_config.html) function.

This function requires a struct to deserialize into. It should implement `Default`, enable serde
defaults, and map keys from `kebab-case`. If you want to error on unknown settings, also enable
`deny_unknown_fields`.

```
#[derive(Debug, Default, serde::Deserialize)]#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]struct NodeToolConfig {    pub bundled_npm: bool,}let config = get_tool_config::()?;config.bundled_npm;
```

### Backend configurationv0.53.0

Like tool configuration, users can configure backends through the
[`[backends.*]`](/docs/proto/config#backends) section of their `.prototools`, which can then be accessed
within the WASM plugin using the
[`get_backend_config`](https://docs.rs/proto_pdk/latest/proto_pdk/fn.get_backend_config.html)
function.

```
#[derive(Debug, Default, serde::Deserialize)]#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]struct ExampleBackendConfig {    pub setting: bool,}let config = get_backend_config::()?;
```

## Creating a plugin

success

Refer to our [official WASM guide](/docs/guides/wasm-plugins#creating-a-plugin) for steps on how to
create a Rust based plugin.

## Implementing tool functions

Plugins are powered by a set of functions that are called from the host, and are annotated with
`#[plugin_fn]`. These are known as plugin functions, or guest functions.

### Registering the tool

The first step in a plugin's life-cycle is to register metadata about the plugin with the
`register_tool` function. This function is called immediately after a plugin is loaded at runtime,
and must return a human-readable name and plugin type.

```
#[plugin_fn]pub fn register_tool(_: ()) -> FnResult> {    Ok(Json(RegisterToolOutput {        name: "Node.js".into(),        type_of: PluginType::Language,        minimum_proto_version: Some(Version::new(0, 42, 0)),        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),        ..RegisterToolOutput::default()    }))}
```

This function also receives the plugin ID as input, allowing for conditional logic based on the ID.
The ID is the [key the plugin was configured with](/docs/proto/plugins#enabling-plugins), and what is passed
to `proto` commands (e.g. `proto install `).

```
#[plugin_fn]pub fn register_tool(Json(input): Json) -> FnResult> {  input.id  // ...}
```

### Configuration schemav0.53.0

If you are using [tool configuration](#tool-configuration), you can register the shape of the
configuration using the [`schematic`](https://crates.io/crates/schematic) crate and the
`define_tool_config` function. This shape will be used to generate outputs such as JSON schemas, or
TypeScript types.

```
#[plugin_fn]pub fn define_tool_config(_: ()) -> FnResult> {    Ok(Json(DefineToolConfigOutput {        schema: Some(schematic::SchemaBuilder::generate::()),    }))}
```

Schematic is a heavy library, so we suggest adding the dependency like so:

```
[dependencies]schematic = { version = "", default-features = false, features = ["schema"] }
```

### Installation strategy

#### Downloading pre-builts

Our plugin layer only supports downloading pre-built tools, typically as an archive, and does
not support building from source. The `download_prebuilt` function must be defined, whichs
configures how the tool should be downloaded and installed.

The following fields are available:

- `archive_prefix` - If the tool is distributed as an archive (zip, tar, etc), this is the name of the direct folder within the archive that contains the tool, and will be removed when unpacking the archive. If there is no prefix folder within the archive, this setting can be omitted.

- `download_url` (required) - A secure URL to download the tool/archive.

- `download_name` - File name of the archive to download. If not provided, will attempt to extract it from the URL.

- `checksum` - The checksum hash itself. Will be used if no other option was provided. v0.47.0

- `checksum_url` - A secure URL to download the checksum file for verification. If the tool does not support checksum verification, this setting can be omitted.

- `checksum_public_key` - Public key used for verifying checksums. Only used for `.minisig` files.

```
#[plugin_fn]pub fn download_prebuilt(Json(input): Json) -> FnResult> {     let env = get_host_environment()?;    check_supported_os_and_arch(        NAME,        &env,        permutations! [            HostOS::Linux => [HostArch::X64, HostArch::Arm64, HostArch::Arm, HostArch::Powerpc64, HostArch::S390x],            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],            HostOS::Windows => [HostArch::X64, HostArch::X86, HostArch::Arm64],        ],    )?;    let version = input.context.version;    let arch = env.arch;    let os = env.os;    let prefix = match os {        HostOS::Linux => format!("node-v{version}-linux-{arch}"),        HostOS::MacOS => format!("node-v{version}-darwin-{arch}"),        HostOS::Windows => format!("node-v{version}-win-{arch}"),        other => {            return Err(PluginError::UnsupportedPlatform("Node.js".into(), other.into()))?;        }    };    let filename = if os == HostOS::Windows {        format!("{prefix}.zip")    } else {        format!("{prefix}.tar.xz")    };    Ok(Json(DownloadPrebuiltOutput {        archive_prefix: Some(prefix),        download_url: format!("https://nodejs.org/dist/v{version}/{filename}"),        download_name: Some(filename),        checksum_url: Some(format!("https://nodejs.org/dist/v{version}/SHASUMS256.txt")),        ..DownloadPrebuiltOutput::default()    }))}
```

##### Unpacking an archive

Our plugin layer will do its best to detect file extensions, unpack the downloaded file (if an
archive), and install the tool to the correct directory. However, we're unable to account for all
edge cases, so for situations where the install params above are not sufficient, you may define an
`unpack_archive` function.

This function receives an input with the following fields:

- `input_file` - Virtual path to the downloaded file. Maps to `~/.proto/temp//`.

- `output_dir` - Virtual directory to unpack the archive into, or copy the binary to. Maps to `~/.proto/tools//`.

```
#[plugin_fn]pub fn unpack_archive(Json(input): Json) -> FnResult {    untar(input.input_file, input.output_dir)?;    Ok(())}
```

#### Build from source

Coming soon!

#### Native installation

For tools that can't be installed through downloading pre-builts or building from source, and
require a more unique/native approach (say running a script), you may define the `native_install`
function.

This function allows you to execute host commands, make HTTP requests, and more, to install the
tool. All that's required to return whether the installation was successful or not, and an optional
error message.

```
#[plugin_fn]pub fn native_install(Json(input): Json) -> FnResult> {    let result = exec_captured("command", ["args"])?;    Ok(Json(NativeInstallOutput {        installed: result.exit_code == 0,        error: if result.stderr.is_empty() {            None        } else {            Some(result.stderr)        },        ..Default::default()    }))}
```

### Locating executables

Even though a tool has been installed, we must inform proto of where to find executables. This can
be achieved with the required `locate_executables` function. The `exes` field defines the location
of the executables, relative from the installation directory.

```
#[plugin_fn]pub fn locate_executables(    Json(_): Json,) -> FnResult> {    let env = get_host_environment()?;    Ok(Json(LocateExecutablesOutput {        exes: HashMap::from_iter([            // Primary            (                "node".into(),                ExecutableConfig::new_primary(                    // Helper that chooses between distinct Unix or Windows values                    env.os.for_native("bin/node", "node.exe"),                    // Or the same value with optional Windows extension                    // env.os.get_file_name("node", "exe")                )            ),            // Secondary            (                "corepack".into(),                ExecutableConfig::new(env.os.for_native("bin/corepack", "corepack.exe"))            ),        ]),        ..LocateExecutablesOutput::default()    }))}
```

The main executable of the tool must be marked as primary, either with
`ExecutableConfig::new_primary`, or setting the `ExecutableConfig.primary` field to true.

Furthermore, the `locate_executables` function can define a list of lookups for the globals
installation directory. proto will loop through each lookup, and return the first directory that
exists on the current file system. proto will also expand environment variables in the format of
`$VAR_NAME`. If a variable is not defined, or has an empty value, the lookup will be skipped. To
demonstrate this, we'll use [Deno](https://deno.land/manual@v1.35.0/tools/script_installer).

```
#[plugin_fn]pub fn locate_executables(    Json(_): Json,) -> FnResult> {    let env = get_host_environment()?;    Ok(Json(LocateExecutablesOutput {        globals_lookup_dirs: vec!["$DENO_INSTALL_ROOT/bin".into(), "$HOME/.deno/bin".into()],        // ...        ..LocateExecutablesOutput::default()    }))}
```

### Loading and resolving versions

Now that the tool can be downloaded and installed, we must configure how to resolve available
versions to actually be installed. To provide a list of versions and language specific aliases, the
`load_versions` function must be defined.

```
#[plugin_fn]pub fn load_versions(Json(_): Json) -> FnResult> {    let mut output = LoadVersionsOutput::default();    let response: Vec = fetch_json("https://nodejs.org/dist/index.json")?;    for (index, item) in response.iter().enumerate() {        let version = VersionSpec::parse(&item.version[1..])?; // Starts with v        if index == 0 {            output.latest = Some(version.clone());        }        output.versions.push(version);    }    Ok(Json(output))}
```

Furthermore, we support an optional function named `resolve_version`, that can be defined to
intercept the version resolution process. This function receives an input with an initial candidate,
either an alias or version, and can replace it with a new candidate. The candidate must be a valid
alias or version as defined in `load_versions`.

```
#[plugin_fn]pub fn resolve_version(    Json(input): Json,) -> FnResult> {    let mut output = ResolveVersionOutput::default();    if let UnresolvedVersionSpec::Alias(alias) = input.initial {        let candidate = if alias == "node" {            "latest"        } else if alias == "lts-*" || alias == "lts/*" {            "stable"        } else if alias.starts_with("lts-") || alias.starts_with("lts/") {            &alias[4..]        } else {            return Ok(Json(output));        };        output.candidate = Some(UnresolvedVersionSpec::Alias(candidate.to_owned()));    }    Ok(Json(output))}
```

### Detecting versions

And lastly, we can configure how to [detect a version](/docs/proto/detection) contextually at runtime, using
the `detect_version_files` function and optional `parse_version_file` function. The
`detect_version_files` function can return a list of files to locate within a directory.

```
#[plugin_fn]pub fn detect_version_files(input: Json) -> FnResult> {    Ok(Json(DetectVersionOutput {        files: vec![            ".nvmrc".into(),            ".node-version".into(),            "package.json".into(),        ],        ignore: vec!["node_modules".into()],    }))}
```

By default our plugin layer will assume the version file's contents contain the literal version, and
nothing else, like "1.2.3". If any of the files in the `detect_version_files` list require custom
parsing (for example, `package.json` above), you can define the `parse_version_file` function.

This function receives the file name and contents as input, and must return the parsed version (if
applicable).

```
#[plugin_fn]pub fn parse_version_file(Json(input): Json
) -> FnResult {    let mut version = None;    if input.file == "package.json" {        let json: PackageJson = serde_json::from_str(&input.content)?;        if let Some(engines) = json.engines {            if let Some(constraint) = engines.get("node") {                version = Some(UnresolvedVersionSpec::parse(constraint)?);            }        }    } else {        version = Some(UnresolvedVersionSpec::parse(input.content.trim())?);    }    Ok(Json(ParseVersionFileOutput { version }))}
```

## Implementing backend functions

Plugins are powered by a set of functions that are called from the host, and are annotated with
`#[plugin_fn]`. These are known as plugin functions, or guest functions.

### Registering the backendv0.52.0

Backends will also need to implement [`register_tool`](#registering-the-tool) for the managed tool
within the backend.

The first step in a plugin's life-cycle is to register metadata about the plugin with the
`register_backend` function. This function requires a unique backend identifier, which is typically
the identifier of the current tool being managed.

```
#[plugin_fn]pub fn register_backend(    Json(input): Json,) -> FnResult> {    Ok(Json(RegisterBackendOutput {        backend_id: get_plugin_id()?,        ..Default::default()    }))}
```

#### Referencing install scripts

Some tools rely on command line scripts for installation, like `asdf`. To make these scripts
available (at `~/.proto/backends`), you can define the `source` field in the
`RegisterBackendOutput`, which accepts a Git repository URL, or an HTTP URL to an archive to unpack.
Additionally, the `exes` field must be defined with a list of scripts that will be executed within
the backend.

As an example, here's how the `asdf` backend is implemented:

```
#[plugin_fn]pub fn register_backend(    Json(input): Json,) -> FnResult> {    Ok(Json(RegisterBackendOutput {        backend_id: get_plugin_id()?,        exes: vec![            "bin/download".into(),            "bin/exec-env".into(),            "bin/install".into(),            "bin/latest-stable".into(),            "bin/list-all".into(),            "bin/list-bin-paths".into(),            "bin/list-legacy-filenames".into(),            "bin/parse-legacy-file".into(),            "bin/uninstall".into(),        ],        source: Some(SourceLocation::Git(GitSource {            url: "https://github.com/asdf-vm/asdf-nodejs.git".into(),            ..Default::default()        })),    }))}
```

These scripts can then be accessed on the file system using the following virtual path.

```
PathBuf::from("/proto/backends///bin/install")
```

### Configuration schemav0.53.0

If you are using [backend configuration](#backend-configuration), you can register the shape of the
configuration using the [`schematic`](https://crates.io/crates/schematic) crate and the
`define_backend_config` function. This shape will be used to generate outputs such as JSON schemas,
or TypeScript types.

```
#[plugin_fn]pub fn define_backend_config(_: ()) -> FnResult> {    Ok(Json(DefineBackendConfigOutput {        schema: Some(schematic::SchemaBuilder::generate::()),    }))}
```

Schematic is a heavy library, so we suggest adding the dependency like so:

```
[dependencies]schematic = { version = "", default-features = false, features = ["schema"] }
```

## Testing

The best way to test the plugin is to execute it through `proto` directly. To do this, you'll need
to configure a `.prototools` file at the root of your plugin's repository that maps the plugin to a
debug build:

```
[plugins.tools] = "file://./target/wasm32-wasip1/debug/.wasm"
```

And everytime you make a change to the plugin, you'll need to rebuild it with:

```
cargo build --target wasm32-wasip1
```

With these 2 pieces in place, you can now execute `proto` commands. Be sure you're running them from
the directory with the `.prototools` file, and that you're passing `--log trace`. Logs are extremely
helpful for figuring out what's going on.

```
proto --log trace install proto --log trace list-remote ...
```

### Unit tests

Testing WASM plugins is a bit tricky, but we've taken it upon ourselves to streamline this process
as much as possible with built-in test utilities, and Rust macros for generating common test cases.
To begin, install all necessary development dependencies:

```
cargo add --dev proto_pdk_test_utils starbase_sandbox tokio
```

And as mentioned above, everytime you make a change to the plugin, you'll need to rebuild it with:

```
cargo build --target wasm32-wasip1
```

#### Testing plugin functions

The common test case is simply calling plugin functions with a provided input and asserting the
output is correct. This can be achieved by creating a plugin instance with `create_plugin` and
calling the appropriate method.

```
use proto_pdk_test_utils::*;use starbase_sandbox::create_empty_sandbox;#[tokio::test(flavor = "multi_thread")]async fn registers_metadata() {    let sandbox = create_empty_proto_sandbox();    let plugin = sandbox.create_plugin("id").await;    assert_eq!(        plugin.register_tool(RegisterToolInput::default()).await,        RegisterToolOutput {            name: "Name".into(),            ..RegisterToolOutput::default()        }    );}
```

info

We suggest using this pattern for static functions that return a deterministic output from a
provided input, and not for dynamic functions that make HTTP requests or execute host commands.

#### Generating cases from macros

To reduce the burden of writing custom tests for common flows, like downloading a pre-built,
resolving versions, and generating shims, we provide a set of Rust decl macros that will generate
the tests for you.

To test downloading and installing, use `generate_download_install_tests!`. This macro requires a
plugin ID and a real version to test with.

```
use proto_pdk_test_utils::*;generate_download_install_tests!("id", "1.2.3");
```

To test version resolving, use `generate_resolve_versions_tests!`. This macro requires a plugin ID,
and a mapping of version/aliases assertions to expectations.

```
generate_resolve_versions_tests!("id", {    "0.4" => "0.4.12",    "0.5.1" => "0.5.1",    "stable" => "1.0.0",});
```

To test installing and uninstalling globals, use `generate_globals_test!`. This macro requires a
plugin ID, the dependency to install, and an optional environment variable to the globals directory.

```
// Doesn't support all use cases! If this doesn't work, implement a test case manually.generate_globals_test!("id", "dependency", "GLOBAL_INSTALL_ROOT");
```

And lastly, to test shims, use `generate_shims_test!`. This requires a plugin ID and a list of shim
file names. This macro generates snapshots using [Insta](https://insta.rs/).

```
generate_shims_test!("id", ["primary", "secondary"]);
```

## Building and publishing

success

Refer to our [official WASM guide](/docs/guides/wasm-plugins#building-and-publishing) for steps on how
to build and publish your plugin.

## Resources

Some helpful resources for learning about and building plugins.

- [Official proto WASM plugins](https://github.com/moonrepo/plugins)

- Plugin development kit [`proto_pdk` docs](https://docs.rs/proto_pdk/)

- [`proto_pdk_test_utils` docs](https://docs.rs/proto_pdk_test_utils/)

## /docs/proto/workflows

Source: https://moonrepo.dev/docs/proto/workflows

# Workflows

With proto, we provide multiple workflows for everyday use for you to choose from. They can be used
individually, or together, it's up to you!

## Shims

proto is primarily powered by the industry standard concept of shims. For each tool installed in
proto, a shim file will exist at `~/.proto/shims` for the primary executable, and some secondary
executables. Shims are not symlinks to the tool's binary, but are thin wrappers around
[`proto run`](/docs/proto/commands/run), enabling [runtime version detection](/docs/proto/detection) on every
invocation! For example, these are equivalent:

```
$ proto run node -- --version20.0.0$ node --version20.0.0$ which node~/.proto/shims/node
```

### Setup

To make use of shims, prepend the `~/.proto/shims` directory to `PATH` in your shell profile. This
must come before the [bin directory](#binary-linking) if using both.

If you're using or plan to use [shell activation](#shell-activation), the `PATH` configuration
happens automatically, but shell activation will only work if the `proto` command is accessible,
which requires `~/.proto/bin` to be in your `PATH`.

## Binary linking

Alternatively, we also support a non-shim based approach, which creates symlinks to a versioned
tool's primary and secondary executables. For each tool installed in proto, a symlink will exist at
`~/.proto/bin`.

```
$ node --version23.1.0$ which node~/.proto/bin/node -> ~/.proto/tools/node/23.1.0/bin/node
```

When a tool is installed into proto, we symlink many binaries based on all the versions that are
installed in the toolchain. The primary binary will always point to the highest installed version,
while we also create binaries for the highest major, and highest major + minor combinations. For
example:

- `~/.proto/bin/node` - Points to the highest version.

- `~/.proto/bin/node-` - Points to the highest version within that major range (`~major`). Is created for each separate major version, for example: `node-20`, `node-22`.

- `~/.proto/bin/node-.` - Points to the highest version within that major + minor range (`~major.minor`). Is created for each separate major + minor version, for example: `node-20.1`, `node-22.4`.

- `~/.proto/bin/node-canary` - Points to a canary install, if it exists.

```
$ node-22 --version22.5.1$ which node-22~/.proto/bin/node-22 -> ~/.proto/tools/node/22.5.1/bin/node
```

info

Not all tools support symlinking a binary, as not all files are executable. For example, most
Node.js package managers currently do not support this, as JavaScript files are not executable
(especially on Windows). Shims are required for these tools.

### Setup

To make use of bins, prepend the `~/.proto/bin` directory to `PATH` in your shell profile. This
must come after the [shim directory](#shims) if using shims.

If you're using or plan to use [shell activation](#shell-activation), the `PATH` configuration
happens automatically, but shell activation will only work if the `proto` command is accessible,
which requires `~/.proto/bin` to be in your `PATH`.

warning

This directory must always exist in `PATH`, as the official proto binaries `~/.proto/bin/proto` and
`~/.proto/bin/proto-shim` are located here. If you move those binaries to another location, you can
omit `~/.proto/bin` from `PATH` if you like.

## Shell activationv0.38.0

Our last workflow is what we call shell activation (or shell hooks), and it's where the proto
environment is setup/reset every time you change directories. If you're coming from another version
manager, you may be familiar with this kind of workflow.

So how does this work exactly? In your shell profile, you'll evaluate a call to
[`proto activate `](/docs/proto/commands/activate), which generates a bunch of shell specific syntax
that registers a hook for "run this code when the current directory or prompt line changes". Once
this hook is registered and you run `cd` (for example), proto will...

- Load all `.prototools` files

- Extract tools with a [configured version](/docs/proto/config#pinning-versions)

- For each tool: Load associated WASM plugin

- Export environment variables based on [`[env]`](/docs/proto/config#env) and [`[tools.*.env]`](/docs/proto/config#toolsenv)

- Prepend `PATH` with tool-specific directories (like local and global executables) for the detected version

```
$ cd /some/path && node --version20.0.0$ cd /another/path && node --version18.0.0
```

### Setup

View the [`proto activate`](/docs/proto/commands/activate#setup) documentation for information on how to setup
your shell profile for this workflow.

## Comparison

The workflows above may come across as information overload, so we've provided the following
comparison table outlining the features each workflow supports.

Shims Bins Activate

Runtime version detection 🟢 🔴 🟠 only when the hook triggers

Supports multiple versions 🟢 🟢 🟢

Fixed to a single version 🟠 with arg or env var 🟢 🟠 if not using shims

Includes all tool executables 🔴 🔴 🟢

Includes tool globals/packages 🔴 🔴 🟢

Exports environment variables 🔴 🔴 🟢

Prepends `PATH` 🔴 🔴 🟢

Can pin proto's version 🔴 🔴 🟢

## /docs/run-task

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

## /docs/setup-toolchain

Source: https://moonrepo.dev/docs/setup-toolchain

# Setup toolchain

5 min

One of moon's most powerful features is the [toolchain](/docs/concepts/toolchain), which automatically
manages, downloads, and installs Node.js and other languages behind the scenes using
[proto](/proto). It also enables [advanced functionality](/docs/how-it-works/languages#tier-2--platform)
for task running based on the platform (language and environment combination) it runs in.

The toolchain is configured with [`.moon/toolchains.yml`](/docs/config/toolchain).

tip

Change the language dropdown at the top right to switch the examples!

## How it works

For more information on the toolchain, our tier based support, and how languages integrate into
moon, refer to the official ["how it works" language guide](/docs/how-it-works/languages) and the
[toolchain concept](/docs/concepts/toolchain) documentation!

info

The toolchain is optional but helps to solve an array of issues that developers face in their
day-to-day.

## Enabling a toolchain

By default all tasks run through the
[system toolchain](/docs/how-it-works/languages#system-language-and-toolchain) and inherit no special
functionality. If you want to take advantage of this functionality, like dependency hashing, package
shorthand execution, and lockfile management, you'll need to enable the toolchain in
[`.moon/toolchains.yml`](/docs/config/toolchain). Otherwise, you can skip to the
[create a task](/docs/create-task) guide.

Begin by declaring the necessary configuration block, even if an empty object! This configuration
can also be injected using the [`moon toolchain add `](/docs/commands/toolchain/add) command
(doesn't support all languages).

.moon/toolchains.yml

```
javascript:  packageManager: 'yarn'node: {}yarn: {}
```

Although we've enabled the toolchain, language binaries must exist on `PATH` for task execution to
function correctly. Continue reading to learn how to automate this flow using tier 3 support.

## Automatically installing a tool

One of the best features of moon is its integrated toolchain and automatic download and installation
of programming languages (when supported), for all developers and machines that moon runs on. This
feature solves the following pain points:

- Developers running tasks using different versions of languages.

- Version drift of languages between machines.

- Languages being installed through different version managers or install scripts.

- Language binaries not existing on `PATH`.

- How shell profiles should be configured.

If you have dealt with any of these pain points before and would like to eliminate them for you and
all your developers, you can try enabling moon's tier 3 support for supported tools. This is easily
done by defining the `version` field for each toolchain.

.moon/toolchains.yml

```
javascript:  packageManager: 'yarn'node:  version: '20.0.0'yarn:  version: '4.0.0'
```

When the `version` field is configured, moon will download and install the tool when a related task
is executed for the first time! It will also set the correct `PATH` lookups and environment
variables automatically. Amazing right?

## Next steps

[Create a task](/docs/create-task)[Configure `.moon/toolchains.yml` further](/docs/config/toolchain)[Learn about the toolchain](/docs/concepts/toolchain)

## /docs/setup-workspace

Source: https://moonrepo.dev/docs/setup-workspace

# Setup workspace

2 min

Once moon has been [installed](/docs/install), we must setup the [workspace](/docs/concepts/workspace),
which is denoted by the `.moon` folder — this is known as the workspace root. The workspace is in
charge of:

- Integrating with a version control system.

- Defining configuration that applies to its entire tree.

- Housing [projects](/docs/concepts/project) to build a [project graph](/docs/how-it-works/project-graph).

- Running tasks with the [action graph](/docs/how-it-works/action-graph).

## Initializing the repository

Let's scaffold and initialize moon in a repository with the [`moon init`](/docs/commands/init) command.
This should typically be ran at the root, but can be nested within a directory.

```
$ moon init
```

When executed, the following operations will be applied.

- Creates a `.moon` folder with a [`.moon/workspace.yml`](/docs/config/workspace) configuration file.

- Appends necessary ignore patterns to the relative `.gitignore`.

- Infers the version control system from the environment.

info

If you're investigating moon, or merely want to prototype, you can use `moon init --minimal` to
quickly initialize and create minimal configuration files.

## Migrate from an existing build system

Looking to migrate from Nx or Turborepo to moon? Use our
[`moon ext migrate-nx`](/docs/guides/extensions#migrate-nx) or
[`moon ext migrate-turborepo`](/docs/guides/extensions#migrate-turborepo) commands for a (somewhat)
seamless migration!

These extensions will convert your existing configuration files to moon's format as best as
possible, but is not a requirement.

## Configuring a version control system

moon requires a version control system (VCS) to be present for functionality like file diffing,
hashing, and revision comparison. The VCS and its default branch can be configured through the
[`vcs`](/docs/config/workspace#vcs) setting.

.moon/workspace.yml

```
vcs:  manager: 'git'  defaultBranch: 'master'
```

moon defaults to `git` and the settings above, so feel free to skip this.

## Next steps

[Create a project](/docs/create-project)[Configure `.moon/workspace.yml` further](/docs/config/workspace)[Learn about the workspace](/docs/concepts/workspace)

## /docs/terminology

Source: https://moonrepo.dev/docs/terminology

# Terminology

Term Description

Action A node within the dependency graph that gets executed by the action pipeline.

Action pipeline Executes actions from our dependency graph in topological order using a thread pool.

Affected Touched by an explicit set of inputs or sources.

Cache Files and outputs that are stored on the file system to provide incremental builds and increased performance.

CI Continuous integration. An environment where tests, builds, lints, etc, are continuously ran on every pull/merge request.

Dependency graph A directed acyclic graph (DAG) of targets to run and their dependencies.

Downstream Dependents or consumers of the item in question.

[Generator](/docs/guides/codegen) Generates code from pre-defined templates.

Hash A unique SHA256 identifier that represents the result of a ran task.

Hashing The mechanism of generating a hash based on multiple sources: inputs, dependencies, configs, etc.

LTS Long-term support.

Dependency manager Installs and manages dependencies for a specific tool (`npm`), using a manifest file (`package.json`).

Platform An internal concept representing the integration of a programming language (tool) within moon, and also the environment + language that a task runs in.

Primary target The target that was explicitly ran, and is the dependee of transitive targets.

[Project](/docs/concepts/project) An collection of source and test files, configurations, a manifest and dependencies, and much more. Exists within a [workspace](/docs/concepts/workspace)

Revision In the context of a VCS: a branch, revision, commit, hash, or point in history.

Runtime An internal concept representing the platform + version of a tool.

[Target](/docs/concepts/target) A label and reference to a task within the project, in the format of `project:task`.

[Task](/docs/concepts/task) A command to run within the context of and configured in a [project](/docs/concepts/project).

Template A collection of files that get scaffolded by a generator.

Template file An individual file within a template.

Template variable A value that is interpolated within a template file and its file system path.

[Token](/docs/concepts/token) A value within task configuration that is substituted at runtime.

Tool A programming language or dependency manager within the [toolchain](/docs/concepts/toolchain).

[Toolchain](/docs/concepts/toolchain) Installs and manages tools within the [workspace](/docs/concepts/workspace).

Transitive target A target that is the dependency of the primary target, and must be ran before the primary.

Touched A file that has been created, modified, deleted, or changed in any way.

Upstream Dependencies or producers of the item in question.

VCS Version control system (like Git or SVN).

[Workspace](/docs/concepts/workspace) Root of the moon installation, and houses one or many [projects](/docs/concepts/project). Also refers to package manager workspaces (like Yarn).


----
## Notes / Comments / Lessons

- Collection method: sitemap URL discovery + markdown conversion (r.jina.ai primary, direct HTML fallback parser secondary).
- Most docs pages are content-rich and command-oriented; command reference and config schemas are the highest-density sections.
- For ongoing updates, re-run this script to refresh snapshots and compare diffs over time.
- Capture results: success=171, failed=0.
----
