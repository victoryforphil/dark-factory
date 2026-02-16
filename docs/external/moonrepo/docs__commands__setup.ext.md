----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/setup
- Keywords: moon, moonrepo, docs, monorepo, build, commands, setup
- Summary: The `moon setup` command can be used to setup the developer and pipeline environments. It achieves
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
