----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs
- Keywords: moon, moonrepo, docs, monorepo, build
- Summary: moonrepo is a productivity platform that aims to eliminate pain points for both developers and companies, by automating tiresome and complex workflows, and improving the overall developer experience.
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
