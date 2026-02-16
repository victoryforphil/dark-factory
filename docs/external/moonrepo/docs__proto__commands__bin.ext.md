----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/bin
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, bin
- Summary: The `proto bin  [version]` command will return an absolute path to a tool's binary within the
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
