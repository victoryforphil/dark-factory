----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/plugins
- Keywords: moon, moonrepo, docs, monorepo, build, proto, plugins
- Summary: proto supports a pluggable architecture as a means for consumers to integrate and manage custom
----

Source: https://moonrepo.dev/docs/proto/plugins

# Plugins

proto supports a pluggable architecture as a means for consumers to integrate and manage custom
tools (languages, CLIs, etc) within proto's toolchain. It's not possible for proto to support
everything in core directly, so plugins are a way for the community to extend the toolchain to
their needs.

## Enabling plugins

Plugins can be enabled by configuring them in [`.prototools`](/docs/proto/config#plugins) files, within the
`[plugins]` section. The map key is the plugin name in kebab-case, which is used as the
binary/tool name in proto, and also the name for configuration and cache purposes. The map value is
a [plugin locator string](/docs/guides/wasm-plugins#configuring-plugin-locations) that defines a
protocol and source location.

.prototools

```
[plugins.tools] = "
://"
```

## Creating plugins

To ease the plugin development process, proto supports 2 types of plugins, a
[non-WASM configuration based plugin](/docs/proto/non-wasm-plugin) for basic use cases, and a
[WASM based plugin](/docs/proto/wasm-plugin) for advanced use cases.

## Publish a plugin

proto's registry is currently powered by static JSON files located in our official
[proto repository](https://github.com/moonrepo/proto/tree/master/registry). View that link for
information on how to publish a plugin.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
