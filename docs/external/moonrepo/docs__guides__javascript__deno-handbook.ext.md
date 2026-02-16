----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/javascript/deno-handbook
- Keywords: moon, moonrepo, docs, monorepo, build, guides, javascript, deno handbook
- Summary: Utilizing Deno in a TypeScript based monorepo can be a non-trivial task. With this handbook, we'll
----

Source: https://moonrepo.dev/docs/guides/javascript/deno-handbook

# Deno handbook

Utilizing Deno in a TypeScript based monorepo can be a non-trivial task. With this handbook, we'll
help guide you through this process.

info

This guide is a living document and will continue to be updated over time!

## moon setup

For this part of the handbook, we'll be focusing on [moon](/moon), our task runner. To start,
languages in moon act like plugins, where their functionality and support is not enabled unless
explicitly configured. We follow this approach to avoid unnecessary overhead.

### Enabling the language

To enable TypeScript support via Deno, define the [`deno`](/docs/config/toolchain#deno) setting in
[`.moon/toolchains.yml`](/docs/config/toolchain), even if an empty object.

.moon/toolchains.yml

```
# Enable Denodeno: {}# Enable Deno and override default settingsdeno:  lockfile: true
```

Or by pinning a `deno` version in [`.prototools`](/docs/proto/config) in the workspace root.

.prototools

```
deno = "1.31.0"
```

This will enable the Deno toolchain and provide the following automations around its ecosystem:

- Automatic handling and caching of lockfiles (when the setting is enabled).

- Relationships between projects will automatically be discovered based on `imports`, `importMap`, and `deps.ts` (currently experimental).

- And more to come!

### Work in progress

caution

Deno support is currently experimental while we finalize the implementation.

The following features are not supported:

- `deno.jsonc` files (use `deno.json` instead).

- `files.exclude` are currently considered an input. These will be filtered in a future release.

## Coming soon!

The handbook is currently being written while we finalize our Deno integration support!

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
