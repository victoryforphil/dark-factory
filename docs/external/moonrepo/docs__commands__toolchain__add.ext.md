----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/toolchain/add
- Keywords: moon, moonrepo, docs, monorepo, build, commands, toolchain, add
- Summary: The `moon toolchain add  [plugin]` command will add a toolchain to the workspace by injecting a
----

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

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
