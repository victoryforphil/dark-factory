----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/setup-toolchain
- Keywords: moon, moonrepo, docs, monorepo, build, setup toolchain
- Summary: One of moon's most powerful features is the [toolchain](/docs/concepts/toolchain), which automatically
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
