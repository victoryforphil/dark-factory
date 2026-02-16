----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/comparison
- Keywords: moon, moonrepo, docs, monorepo, build, comparison
- Summary: The following comparisons are not an exhaustive list of features, and may be inaccurate or out of
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
