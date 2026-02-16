----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/concepts/toolchain
- Keywords: moon, moonrepo, docs, monorepo, build, concepts, toolchain
- Summary: The toolchain is an internal layer for downloading, installing, and managing tools (languages,
----

Source: https://moonrepo.dev/docs/concepts/toolchain

# Toolchain

The toolchain is an internal layer for downloading, installing, and managing tools (languages,
dependency managers, libraries, and binaries) that are required at runtime. We embrace this approach
over relying on these tools "existing" in the current environment, as it ensures the following
across any environment or machine:

- The version and enabled features of a tool are identical.

- Tools are isolated and unaffected by external sources.

- Builds are consistent, reproducible, and hopefully deterministic.

Furthermore, this avoids a developer, pipeline, machine, etc, having to pre-install all the
necessary tools, and to keep them in sync as time passes.

## How it works

The toolchain is built around [proto](/proto), our stand-alone multi-language version manager. moon
will piggyback of proto's toolchain found at `~/.proto` and reuse any tools available, or download
and install them if they're missing.

### Force disabling

The `MOON_TOOLCHAIN_FORCE_GLOBALS` environment variable can be set to `true` to force moon to use
tool binaries available on `PATH`, instead of downloading and installing them. This is useful for
pre-configured environments, like CI and Docker.

```
MOON_TOOLCHAIN_FORCE_GLOBALS=true
```

Additionally, the name of one or many tools can be passed to this variable to only force globals for
those tools, and use the toolchain for the remaining tools.

```
MOON_TOOLCHAIN_FORCE_GLOBALS=node,yarn
```

## Configuration

The tools that are managed by the toolchain are configured through the
[`.moon/toolchains.yml`](/docs/config/toolchain) file, but can be overridden in each project with
[`moon.yml`](/docs/config/project#toolchain).

### Version specification

As mentioned above, tools within the toolchain are managed by version for consistency across
machines. These versions are configured on a per-tool basis in
[`.moon/toolchains.yml`](/docs/config/toolchain). So what kinds of versions are allowed?

- Full versions - A full version is a semantic version that is fully specified, such as `1.2.3` or `2.0.0-rc.1`. This is the most common way to specify a version, and is preferred to avoid subtle deviations.

- Partial versions - A partial version is a version that is either missing a patch number, minor number, or both, such as `1.2` or `1`. These can also be represented with requirement syntax, such as `^1.2` or `~1`. If using partials, we suggest having a major and minor number to reduce the deviation of versions across machines.

- Aliases - An alias is a human-readable word that maps to a specific version. For example, `latest` or `stable` maps to the latest version of a tool, or `canary` which maps to applicable canary release, or even a completely custom alias like `berry`. Aliases are language specific, are not managed by moon, and are not suggested for use since they can change at any time (or even daily!).

This sounds great, but how exactly does this work? For full versions and aliases, it's straight
forward, as the resolved version is used as-is (assuming it's a legitimate version), and can be
found at `~/.proto/tools//`.

For partial versions, we first check locally installed versions for a match, by scanning
`~/.proto/tools/`. For example, if the requested version is `1.2` and we have `1.2.10`
installed locally, we'll use that version instead of downloading the latest `1.2.*` version.
Otherwise, we'll download the latest version that matches the partial version, and install it
locally.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
