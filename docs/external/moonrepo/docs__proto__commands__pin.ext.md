----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/pin
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, pin
- Summary: The `proto pin  ` command will pin a version (or alias) of a tool. This version will
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
