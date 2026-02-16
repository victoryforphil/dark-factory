----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/regen
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, regen
- Summary: The `proto regen` command can be used to regenerate all shims in the `~/.proto/shims` directory.
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
