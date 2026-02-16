----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/non-wasm-plugin
- Keywords: moon, moonrepo, docs, monorepo, build, proto, non wasm plugin
- Summary: The non-WASM plugin is by design, very simple. It's a JSON, TOML, or YAML file that describes a
----

Source: https://moonrepo.dev/docs/proto/non-wasm-plugin

# Non-WASM plugin

The non-WASM plugin is by design, very simple. It's a JSON, TOML, or YAML file that describes a
schema for the tool, how it should be installed, and how it should be invoked. Since this is a
static configuration file, it does not support any logic or complex behavior, and is merely for
simple and common use cases, like CLIs.

info

JSON and YAML support was added in proto v0.42.

## Create a plugin

Let's start by creating a new plugin, and defining the `name` and `type` fields. The type can either
be `language`, `dependency-manager`, `package-manager`, or `cli`. For this example, we'll create a
plugin for our fake product called Protostar, a CLI tool.

- JSON
- TOML
- YAML

protostar.json

```
{  "name": "Protostar",  "type": "cli"}
```

protostar.toml

```
name = "Protostar"type = "cli"
```

protostar.yaml

```
name: 'Protostar'type: 'cli'
```

### Platform variations

Native tools are often platform specific, and proto supports this by allowing you to define
variations based on operating system using the `[platform]` section. For non-native tools, this
section can typically be skipped.

This section requires a mapping of Rust
[`OS` strings](https://doc.rust-lang.org/std/env/consts/constant.OS.html) to platform settings. The
following settings are available:

- `archs` - A list of architectures supported for this platform. If not provided, supports all archs.

- `archive-prefix` - If the tool is distributed as an archive (zip, tar, etc), this is the name of the direct folder within the archive that contains the tool, and will be removed when unpacking the archive. If there is no prefix folder within the archive, this setting can be omitted.

- `exes-dir` - A relative path to a directory that contains pre-installed executables.

- `exe-path` - The path to the main executable binary within the archive (without the prefix). If the tool is distributed as a single binary, this setting can be typically omitted.

- `checksum-file` - Name of the checksum file to verify the downloaded file with. If the tool does not support checksum verification, this setting can be omitted.

- `download-file` (required) - Name of the file to download. [Learn more about downloading](#downloading-and-installing).

- JSON
- TOML
- YAML

protostar.json

```
{  "platform": {    "linux": {      "archivePrefix": "protostar-linux",      "exePath": "bin/protostar",      "checksumFile": "protostar-{arch}-unknown-linux-{libc}.sha256",      "downloadFile": "protostar-{arch}-unknown-linux-{libc}.tar.gz"    },    "macos": {      "archivePrefix": "protostar-macos",      "exePath": "bin/protostar",      "checksumFile": "protostar-{arch}-apple-darwin.sha256",      "downloadFile": "protostar-{arch}-apple-darwin.tar.xz"    },    "windows": {      "archivePrefix": "protostar-windows",      "exePath": "bin/protostar.exe",      "checksumFile": "protostar-{arch}-pc-windows-msvc.sha256",      "downloadFile": "protostar-{arch}-pc-windows-msvc.zip"    }  }}
```

protostar.toml

```
[platform][platform.linux]archive-prefix = "protostar-linux"exe-path = "bin/protostar"checksum-file = "protostar-{arch}-unknown-linux-{libc}.sha256"download-file = "protostar-{arch}-unknown-linux-{libc}.tar.gz"[platform.macos]archive-prefix = "protostar-macos"exe-path = "bin/protostar"checksum-file = "protostar-{arch}-apple-darwin.sha256"download-file = "protostar-{arch}-apple-darwin.tar.xz"[platform.windows]archive-prefix = "protostar-windows"exe-path = "bin/protostar.exe"checksum-file = "protostar-{arch}-pc-windows-msvc.sha256"download-file = "protostar-{arch}-pc-windows-msvc.zip"
```

protostar.yaml

```
platform:  linux:    archivePrefix: 'protostar-linux'    exePath: 'bin/protostar'    checksumFile: 'protostar-{arch}-unknown-linux-{libc}.sha256'    downloadFile: 'protostar-{arch}-unknown-linux-{libc}.tar.gz'  macos:    archivePrefix: 'protostar-macos'    exePath: 'bin/protostar'    checksumFile: 'protostar-{arch}-apple-darwin.sha256'    downloadFile: 'protostar-{arch}-apple-darwin.tar.xz'  windows:    archivePrefix: 'protostar-windows'    exePath: 'bin/protostar.exe'    checksumFile: 'protostar-{arch}-pc-windows-msvc.sha256'    downloadFile: 'protostar-{arch}-pc-windows-msvc.zip'
```

You may have noticed tokens above, like `{arch}`. These are special tokens that are replaced with a
dynamic value at runtime, based on the current host machine executing the code. The following tokens
are available:

- `{version}` - The currently resolved version, as a fully-qualified semantic or calendar version.

- `{versionMajor}` / `{versionYear}` - Only the major version. v0.41.4

- `{versionMinor}` / `{versionMonth}` - Only the minor version. v0.45.2

- `{versionPatch}` / `{versionDay}` - Only the patch version. v0.45.2

- `{versionPrerelease}` - The prerelease identifier, if applicable. Returns an empty string otherwise. v0.41.4

- `{versionBuild}` - The build identifier, if applicable. Returns an empty string otherwise. v0.41.4

- `{arch}` - The architecture of the host machine, like `x86_64`. These values map to Rust's [`ARCH` constant](https://doc.rust-lang.org/std/env/consts/constant.ARCH.html), but can be customized with [`install.arch`](#downloading-and-installing).

- `{os}` - The operating system of the host machine, like `windows`. These values map to Rust's [`OS` constant](https://doc.rust-lang.org/std/env/consts/constant.OS.html).

- `{libc}` - For Linux machines, this is the current libc implementation, either `gnu` or `musl`. v0.31.2

### Downloading and installing

A non-WASM plugin only supports downloading pre-built tools, typically as an archive, and does
not support building from source. The `[install]` section can be used to configure how the tool
should be downloaded and installed into the toolchain. The following settings are available:

- `arch` - A mapping of Rust [`ARCH` strings](https://doc.rust-lang.org/std/env/consts/constant.ARCH.html) to custom values for the `{arch}` token. This is useful if the tool has different terminology.

- `libc` - A mapping of custom values for the `{libc}` token.

- `checksum-url` - A secure URL to download the checksum file for verification. If the tool does not support checksum verification, this setting can be omitted.

- `checksum-url-canary` - A URL for canary releases.

- `checksum-public-key` - Public key used for verifying checksums. Only used for `.minisig` files.

- `download-url` (required) - A secure URL to download the tool/archive.

- `download-url-canary` - A URL for canary releases.

- `primary` - Configures the primary executable.

- `secondary` - Configures secondary executables.

The URL settings support `{checksum_file}` and `{download_file}` tokens, which will be replaced with
the values from the `[platform]` section.

- JSON
- TOML
- YAML

protostar.json

```
{  "install": {    "checksumUrl": "https://github.com/moonrepo/protostar/releases/download/v{version}/{checksum_file}",    "downloadUrl": "https://github.com/moonrepo/protostar/releases/download/v{version}/{download_file}",    "arch": {      "aarch64": "arm64",      "x86_64": "x64"    }  }}
```

protostar.toml

```
[install]checksum-url = "https://github.com/moonrepo/protostar/releases/download/v{version}/{checksum_file}"download-url = "https://github.com/moonrepo/protostar/releases/download/v{version}/{download_file}"[install.arch]aarch64 = "arm64"x86_64 = "x64"
```

protostar.yaml

```
install:  checksumUrl: 'https://github.com/moonrepo/protostar/releases/download/v{version}/{checksum_file}'  downloadUrl: 'https://github.com/moonrepo/protostar/releases/download/v{version}/{download_file}'  arch:    aarch64: 'arm64'    x86_64: 'x64'
```

#### Executables

The available executables (bins and shims) can be customized with the `[install.exes]` section,
which is required. This setting requires a map, where the key is the executable file name, and the
value is an object of the following options:

- `exe-path` - The file to execute, relative from the tool directory. On Windows, the `.exe` extension will automatically be appended. If you need more control over platform variance, use `[platform.*.exe-path]` instead.

- `no-bin` - Do not symlink a binary in `~/.proto/bin`.

- `no-shim`- Do not generate a shim in `~/.proto/shims`.

- `parent-exe-name` - Name of a parent executable required to execute the executable path. For example, `node` is required for `.js` files.

- `primary` - Is the main executable in the tool. There can only be 1 primary! v0.42.0

- `shim-before-args` - Custom args to prepend to user-provided args within the generated shim.

- `shim-after-args` - Custom args to append to user-provided args within the generated shim.

- `shim-env-vars` - Custom environment variables to set when executing the shim.

This field supports both the required primary executable, and optional secondary executables. The
primary executable must be marked with `primary = true`.

- JSON
- TOML
- YAML

protostar.json

```
{  "install": {    "exes": {      "protostar": {        "exePath": "bins/protostar",        "primary": true,        "shimBeforeArgs": [          "--verbose"        ]      },      "protostar-debug": {        "exePath": "bins/protostar-debug",        "noShim": true      }    }  }}
```

protostar.toml

```
[install][install.exes][install.exes.protostar]exe-path = "bins/protostar"primary = trueshim-before-args = [ "--verbose" ][install.exes.protostar-debug]exe-path = "bins/protostar-debug"no-shim = true
```

protostar.yaml

```
install:  exes:    protostar:      exePath: 'bins/protostar'      primary: true      shimBeforeArgs:        - '--verbose'    protostar-debug:      exePath: 'bins/protostar-debug'      noShim: true
```

#### Global packages

The `[packages]` sections can be configured that provides information about where global packages
are stored.

- `globals-lookup-dirs` - A list of directories where global binaries are stored. This setting supports interpolating environment variables via the syntax `$ENV_VAR`.

- `globals-prefix` - A string that all package names are prefixed with. For example, Cargo/Rust binaries are prefixed with `cargo-`.

- JSON
- TOML
- YAML

protostar.json

```
{  "packages": {    "globalsLookupDirs": [      "$PROTOSTAR_HOME/bin",      "$HOME/.protostar/bin"    ]  }}
```

protostar.toml

```
[packages]globals-lookup-dirs = [ "$PROTOSTAR_HOME/bin", "$HOME/.protostar/bin" ]
```

protostar.yaml

```
packages:  globalsLookupDirs:    - '$PROTOSTAR_HOME/bin'    - '$HOME/.protostar/bin'
```

### Resolving versions

Now that the tool can be downloaded and installed, we must configure how to resolve available
versions. Resolving is configured through the `[resolve]` section, which supports 2 patterns to
resolve with: Git tags or a JSON manifest.

#### Git tags

To resolve a list of available versions using Git tags, the following settings are available:

- `git-url` (required) - The remote URL to fetch tags from.

- JSON
- TOML
- YAML

protostar.json

```
{  "resolve": {    "gitUrl": "https://github.com/moonrepo/protostar"  }}
```

protostar.toml

```
[resolve]git-url = "https://github.com/moonrepo/protostar"
```

protostar.yaml

```
resolve:  gitUrl: 'https://github.com/moonrepo/protostar'
```

#### JSON manifest

To resolve a list of available versions using a JSON manifest, the following settings are available:

- `manifest-url` (required) - A URL that returns a JSON response of all versions. This response must be an array of strings, or an array of objects.

- `manifest-version-key` - If the response is an array of objects, this is the key to extract the version from. If the response is an array of strings, this setting can be omitted. Defaults to `version`.

- JSON
- TOML
- YAML

protostar.json

```
{  "resolve": {    "manifestUrl": "https://someregistry.com/protostar/versions.json",    "manifestVersionKey": "latest_version"  }}
```

protostar.toml

```
[resolve]manifest-url = "https://someregistry.com/protostar/versions.json"manifest-version-key = "latest_version"
```

protostar.yaml

```
resolve:  manifestUrl: 'https://someregistry.com/protostar/versions.json'  manifestVersionKey: 'latest_version'
```

#### Versions and aliasesv0.36.0

As an alternative, we also support a static configuration of explicit versions and aliases. This is
useful if you have an internal tool that is relatively stable, or does not provide a means in which
to extract version information.

- `versions` - A list of versions.

- `aliases` - A mapping of alias names to versions.

- JSON
- TOML
- YAML

protostar.json

```
{  "resolve": {    "versions": [      "1.2.3",      "1.2.4",      "1.2.5"    ],    "aliases": {      "stable": "1.2.4"    }  }}
```

protostar.toml

```
[resolve]versions = [ "1.2.3", "1.2.4", "1.2.5" ][resolve.aliases]stable = "1.2.4"
```

protostar.yaml

```
resolve:  versions:    - '1.2.3'    - '1.2.4'    - '1.2.5'  aliases:    stable: '1.2.4'
```

#### Version patterns

When a version is found, either from a git tag or manifest key, we attempt to parse it into a
[valid version](/docs/proto/tool-spec) using a Rust based regex pattern and the `version-pattern` setting.

This pattern uses named regex capture groups (`(?...)`) to build the version, and to support
found versions that are not fully-qualified (they may be missing patch or minor versions). The
following groups are supported:

- `major` / `year` - The major version number. Defaults to `0` if missing.

- `minor` / `month` - The minor version number. Defaults to `0` if missing.

- `patch` / `day` - The patch version number. Defaults to `0` if missing.

- `pre` - The pre-release identifier, like "rc.0" or "alpha.0". Supports an optional leading `-`. Does nothing if missing.

- `build` - The build metadata, like a timestamp. Supports an optional leading `+`. Does nothing if missing.

- JSON
- TOML
- YAML

protostar.json

```
{  "resolve": {    "versionPattern": "^@protostar/cli@((?\\d+)\\.(?\\d+)\\.(?
\\d+))"  }}
```

protostar.toml

```
[resolve]version-pattern = "^@protostar/cli@((?\\d+)\\.(?\\d+)\\.(?
\\d+))"
```

protostar.yaml

```
resolve:  versionPattern: '^@protostar/cli@((?\d+)\.(?\d+)\.(?
\d+))'
```

If no named capture groups are found, the match at index `1` is used as the version.

### Detecting versions

And lastly, we can configure how to [detect a version](/docs/proto/detection) contextually at runtime, using
the `[detect]` setting. At this time, we only support 1 setting:

- `version-files` - A list of version files to extract from. The contents of these files can only be the version string itself.

- JSON
- TOML
- YAML

protostar.json

```
{  "detect": {    "versionFiles": [      ".protostar-version",      ".protostarrc"    ]  }}
```

protostar.toml

```
[detect]version-files = [ ".protostar-version", ".protostarrc" ]
```

protostar.yaml

```
detect:  versionFiles:    - '.protostar-version'    - '.protostarrc'
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
