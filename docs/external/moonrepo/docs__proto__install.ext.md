----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/install
- Keywords: moon, moonrepo, docs, monorepo, build, proto, install
- Summary: The following guide can be used to install proto into your environment.
----

Source: https://moonrepo.dev/docs/proto/install

# Install proto

1 min

The following guide can be used to install proto into your environment.

## Requirements

- Git - for fetching available versions/tags

- tar, unzip, gz, xz - for unpacking archives

```
# macOSbrew install git unzip gzip xz# Ubuntu / Debianapt-get install git unzip gzip xz-utils# RHEL-based / Fedoradnf install git unzip gzip xz
```

## Installing

The entirety of proto is packaged and shipped as 2 binaries. It works on most operating systems,
and does not require any external dependencies. For convenience, we provide the following scripts to
download and install proto.

info

The install location can be customized with the `PROTO_HOME` environment variable. If not provided,
the default location is `~/.proto`.

### Linux, macOS, WSL

In a terminal that supports Bash, run the following command. This will download and install proto,
then open an interactive prompt to complete the installation.

```
bash administrator Powershell or Windows Terminal, run the following command. This will download
and install proto, then open an interactive prompt to complete the installation.

```
irm https://moonrepo.dev/install/proto.ps1 | iex
```

You may also need to run the following command for shims to be executable:

```
Set-ExecutionPolicy RemoteSigned# Without admin privilegesSet-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

### Other

proto can also be downloaded and installed manually, by downloading an asset from
[https://github.com/moonrepo/proto/releases](https://github.com/moonrepo/proto/releases). Be sure to
rename the file after downloading, and apply the executable bit (`chmod +x`) on macOS and Linux.

## Upgrading

To upgrade proto, run the [`proto upgrade`](/docs/proto/commands/upgrade) command, or re-run the install
scripts above.

## Uninstalling

To uninstall proto, delete the `~/.proto` directory, and remove any `PROTO_HOME` references from
your shell profile.

## Canary releases

proto supports canary releases, which are built and published for every commit to our development
branches. These releases will include features and functionality that have not yet landed on master.
Canary releases are available as a
[GitHub prerelease](https://github.com/moonrepo/proto/releases/tag/canary) using the `canary` tag.

## Nightly releases

proto supports nightly releases, which are built and published once a day from the latest commit on
master. Nightly releases are available as a
[GitHub prerelease](https://github.com/moonrepo/proto/releases/tag/nightly) using the `nightly` tag.

## Next steps

[Choose a workflow](/docs/proto/workflows)[Learn about `.prototools`](/docs/proto/config)

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
