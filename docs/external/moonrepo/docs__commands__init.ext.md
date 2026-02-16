----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/init
- Keywords: moon, moonrepo, docs, monorepo, build, commands, init
- Summary: The `moon init` command will initialize moon into a repository and scaffold necessary config files
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
