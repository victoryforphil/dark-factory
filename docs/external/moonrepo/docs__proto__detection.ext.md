----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/detection
- Keywords: moon, moonrepo, docs, monorepo, build, proto, detection
- Summary: The most powerful feature in proto is its contextual version detection, that is triggered with
----

Source: https://moonrepo.dev/docs/proto/detection

# Version detection

2 min

The most powerful feature in proto is its contextual version detection, that is triggered with
[`proto run`](/docs/proto/commands/run), [`proto bin`](/docs/proto/commands/bin), or when a shim is executed. So what
does this mean exactly? Before a tool in proto's toolchain can be executed, we need to determine the
version of the tool to execute with. If a detected version exists locally, we proceed using that
binary, otherwise we fail with a missing installation error.

When detecting a version, the following steps are checked, in the order as listed:

#### 1. Version is explicitly passed as a command line argument

```
$ proto run node 24.0.0
```

#### 2. Version is provided with the `PROTO_*_VERSION` environment variable

```
$ PROTO_NODE_VERSION=24.0.0 proto run node
```

#### 3. Version is located by traversing the file system

This step will attempt to find a configuration or manifest file in the current working directory,
and traverse upwards through parent directories (stops at the user's home directory) until a file is
found.

##### 3.1. Version is defined locally in `.prototools`

A `.prototools` file was found and a version entry exists for the current tool. This is also known
as a "local version" and can be created with [`proto pin`](/docs/proto/commands/pin).

.prototools

```
node = "24.0.0"
```

##### 3.2. Version is defined in the tool's ecosystem

Depending on the tool, a version is extracted from a found file unique to that tool's ecosystem.
This includes version manager configs (`.nvmrc`, etc), manifest files (`package.json`, etc), and
more.

.nvmrc

```
24.0.0
```

package.json

```
{  "devEngines": {    "runtime": {      "name": "node",      "version": "24.0.0"    },    "packageManager": {      "name": "npm",      "version": "11.0.0"    }  }}
```

#### 4. Version is defined globally

As the last check, we look for a "global version" that was pinned with
[`proto pin --global`](/docs/proto/commands/pin) or [`proto install --pin`](/docs/proto/commands/install). This version
is stored at `~/.proto/.prototools` (`%USERPROFILE%\.proto\.prototools` on Windows).

#### 5. Version could not be detected

If all the previous steps have failed, then we could not detect an applicable version, and the
process will fail.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
