----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/config/extensions
- Keywords: moon, moonrepo, docs, monorepo, build, config, extensions
- Summary: The `.moon/extensions.yml` file configures extensions that can hook into pipeline events, or be
----

Source: https://moonrepo.dev/docs/config/extensions

# .moon/extensions

v2.0.0

The `.moon/extensions.yml` file configures extensions that can hook into pipeline events, or be
executed directly. This file is optional.

## `extends`

Defines one or many external `.moon/extensions.yml`'s to extend and inherit settings from. Perfect
for reusability and sharing configuration across repositories and projects. When defined, this
setting must be an HTTPS URL or relative file system path that points to a valid YAML document!

.moon/extensions.yml

```
extends: 'https://raw.githubusercontent.com/organization/repository/master/.moon/extensions.yml'
```

caution

Settings will be merged recursively for blocks, with values defined in the local configuration
taking precedence over those defined in the extended configuration.

## How it works

A mapping of extensions that can be downloaded and executed with the [`moon ext`](/docs/commands/ext)
command. An extension is a WASM plugin, and the location of the WASM file must be defined with the
`plugin` field, which requires a
[plugin locator string](/docs/guides/wasm-plugins#configuring-plugin-locations).

.moon/extensions.yml

```
example:  plugin: 'file://./path/to/example.wasm'  # or  plugin: 'https://example.com/path/to/example.wasm'
```

Additionally, extensions support custom configuration that is passed to the WASM runtime when the
plugin is instantiated. This configuration is defined by inserting additional fields under the
extension name, relative to the `plugin` field. Each extension may have its own settings, so refer
to their documentation for more information.

.moon/extensions.yml

```
example:  plugin: 'file://./path/to/example.wasm'  setting1: true  setting2: 'abc'
```

## Supported extensions

View the [official guide](/docs/guides/extensions) for all built-in extensions.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
