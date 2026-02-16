----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/rust/handbook
- Keywords: moon, moonrepo, docs, monorepo, build, guides, rust, handbook
- Summary: Utilizing Rust in a monorepo is a trivial task, thanks to Cargo, and also moon. With this handbook,
----

Source: https://moonrepo.dev/docs/guides/rust/handbook

# Rust handbook

Utilizing Rust in a monorepo is a trivial task, thanks to Cargo, and also moon. With this handbook,
we'll help guide you through this process.

info

moon is not a build system and does not replace Cargo. Instead, moon runs `cargo` commands, and
efficiently orchestrates those tasks within the workspace.

## moon setup

For this part of the handbook, we'll be focusing on [moon](/moon), our task runner. To start,
languages in moon act like plugins, where their functionality and support is not enabled unless
explicitly configured. We follow this approach to avoid unnecessary overhead.

### Enabling the language

To enable Rust, define the [`rust`](/docs/config/toolchain#rust) setting in
[`.moon/toolchains.yml`](/docs/config/toolchain), even if an empty object.

.moon/toolchains.yml

```
# Enable Rustrust: {}# Enable Rust and override default settingsrust:  syncToolchainConfig: true
```

Or by pinning a `rust` version in [`.prototools`](/docs/proto/config) in the workspace root.

.prototools

```
rust = "1.69.0"
```

This will enable the Rust toolchain and provide the following automations around its ecosystem:

- Manifests and lockfiles are parsed for accurate dependency versions for hashing purposes.

- Cargo binaries (in `~/.cargo/bin`) are properly located and executed.

- Automatically sync `rust-toolchain.toml` configuration files.

- For non-workspaces, will inherit `package.name` from `Cargo.toml` as a project alias.

- And more to come!

### Utilizing the toolchain

When a language is enabled, moon by default will assume that the language's binary is available
within the current environment (typically on `PATH`). This has the downside of requiring all
developers and machines to manually install the correct version of the language, and to stay in
sync.

Instead, you can utilize [moon's toolchain](/docs/concepts/toolchain), which will download and
install the language in the background, and ensure every task is executed using the exact version
across all machines.

Enabling the toolchain is as simple as defining the
[`rust.version`](/docs/config/toolchain#version-2) setting.

.moon/toolchains.yml

```
# Enable Rust toolchain with an explicit versionrust:  version: '1.69.0'
```

Versions can also be defined with [`.prototools`](/docs/proto/config).

caution

moon requires `rustup` to exist in the environment, and will use this to install the necessary Rust
toolchains. This requires Rust to be manually installed on the machine, as moon does not
auto-install the language, just the toolchains.

## Repository structure

Rust/Cargo repositories come in two flavors: a single crate with one `Cargo.toml`, or multiple
crates with many `Cargo.toml`s using
[Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). The latter is
highly preferred as it enables Cargo incremental caching.

Regardless of which flavor your repository uses, in moon, both flavors are a single
[moon project](/docs/concepts/project). This means that all Rust crates are grouped together into a
single moon project, and the [`moon.yml`](/docs/config/project) file is located at the root relative
to `Cargo.lock` and the `target` folder.

An example of this layout is demonstrated below:

- Workspaces
- Non-workspaces

```
/├── .moon/├── crates/│   ├── client/|   │   ├── ...│   │   └── Cargo.toml│   ├── server/|   │   ├── ...│   │   └── Cargo.toml│   └── utils/|       ├── ...│       └── Cargo.toml├── target/├── Cargo.lock├── Cargo.toml└── moon.yml
```

```
/├── .moon/├── src/│   └── lib.rs├── tests/│   └── ...├── target/├── Cargo.lock├── Cargo.toml└── moon.yml
```

### Example `moon.yml`

The following configuration represents a base that covers most Rust projects.

- Workspaces
- Non-workspaces

/moon.yml

```
language: 'rust'layer: 'application'env:  CARGO_TERM_COLOR: 'always'fileGroups:  sources:    - 'crates/*/src/**/*'    - 'crates/*/Cargo.toml'    - 'Cargo.toml'  tests:    - 'crates/*/benches/**/*'    - 'crates/*/tests/**/*'tasks:  build:    command: 'cargo build'    inputs:      - '@globs(sources)'  check:    command: 'cargo check --workspace'    inputs:      - '@globs(sources)'  format:    command: 'cargo fmt --all --check'    inputs:      - '@globs(sources)'      - '@globs(tests)'  lint:    command: 'cargo clippy --workspace'    inputs:      - '@globs(sources)'      - '@globs(tests)'  test:    command: 'cargo test --workspace'    inputs:      - '@globs(sources)'      - '@globs(tests)'
```

/moon.yml

```
language: 'rust'layer: 'application'env:  CARGO_TERM_COLOR: 'always'fileGroups:  sources:    - 'src/**/*'    - 'Cargo.toml'  tests:    - 'benches/**/*'    - 'tests/**/*'tasks:  build:    command: 'cargo build'    inputs:      - '@globs(sources)'  check:    command: 'cargo check'    inputs:      - '@globs(sources)'  format:    command: 'cargo fmt --check'    inputs:      - '@globs(sources)'      - '@globs(tests)'  lint:    command: 'cargo clippy'    inputs:      - '@globs(sources)'      - '@globs(tests)'  test:    command: 'cargo test'    inputs:      - '@globs(sources)'      - '@globs(tests)'
```

## Cargo integration

You can't use Rust without Cargo -- well you could but why would you do that? With moon, we're doing
our best to integrate with Cargo as much as possible. Here's a few of the benefits we currently
provide.

### Global binaries

Cargo supports global binaries through the
[`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html) command, which
installs a crate to `~/.cargo/bin`, or makes it available through the `cargo ` command. These
are extremely beneficial for development, but they do require every developer to manually install
the crate (and appropriate version) to their machine.

With moon, this is no longer an issue with the [`rust.bins`](/docs/config/toolchain#bins) setting.
This setting requires a list of crates (with optional versions) to install, and moon will install
them as part of the task runner install dependencies action. Furthermore, binaries will be installed
with [`cargo-binstall`](https://crates.io/crates/cargo-binstall) in an effort to reduce build and
compilation times.

.moon/toolchains.yml

```
rust:  bins:    - 'cargo-make@0.35.0'    - 'cargo-nextest'
```

At this point, tasks can be configured to run this binary as a command. The `cargo` prefix is
optional, as we'll inject it when necessary.

/moon.yml

```
tasks:  test:    command: 'nextest run --workspace'    toolchain: 'rust'
```

tip

The `cargo-binstall` crate may require a `GITHUB_TOKEN` environment variable to make GitHub Releases
API requests, especially in CI. If you're being rate limited, or fail to find a download, try
creating a token with necessary permissions.

### Lockfile handling

To expand our integration even further, we also take `Cargo.lock` into account, and apply the
following automations when a target is being ran:

- If the lockfile does not exist, we generate one with [`cargo generate-lockfile`](https://doc.rust-lang.org/cargo/commands/cargo-generate-lockfile.html).

- We parse and extract the resolved checksums and versions for more accurate hashing.

## FAQ

### Should we cache the `target` directory as an output?

No, we don't believe so. Both moon and Cargo support incremental caching, but they're not entirely
compatible, and will most likely cause problems when used together.

The biggest factor is that moon's caching and hydration uses a tarball strategy, where each task
would unpack a tarball on cache hit, and archive a tarball on cache miss. The Cargo target directory
is extremely large (moon's is around 50gb), and coupling this with our tarball strategy is not
viable. This would cause massive performance degradation.

However, at maximum, you could cache the compiled binary itself as an output, instead of the
entire target directory. Example:

moon.yml

```
tasks:  build:    command: 'cargo build --release'    outputs: ['target/release/moon']
```

### How can we improve CI times?

Rust is known for slow build times and CI is no exception. With that being said, there are a few
patterns to help alleviate this, both on the moon side and outside of it.

To start, you can cache Rust builds in CI. This is a non-moon solution to the `target` directory
problem above.

- If you use GitHub Actions, feel free to use our [moonrepo/setup-rust](https://github.com/moonrepo/setup-rust) action, which has built-in caching.

- A more integrated solution is [sccache](https://crates.io/crates/sccache), which stores build artifacts in a cloud storage provider.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
