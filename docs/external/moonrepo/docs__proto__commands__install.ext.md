----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/install
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, install
- Summary: The `proto install` (or `proto i`) command can be used to install one or many tools.
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
