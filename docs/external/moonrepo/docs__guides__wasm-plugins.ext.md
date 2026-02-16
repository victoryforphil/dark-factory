----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/wasm-plugins
- Keywords: moon, moonrepo, docs, monorepo, build, guides, wasm plugins
- Summary: [moon](/moon) and [proto](/proto) plugins can be written in
----

Source: https://moonrepo.dev/docs/guides/wasm-plugins

# WASM plugins

[moon](/moon) and [proto](/proto) plugins can be written in
[WebAssembly (WASM)](https://webassembly.org/), a portable binary format. This means that plugins
can be written in any language that compiles to WASM, like Rust, C, C++, Go, TypeScript, and more.
Because WASM based plugins are powered by a programming language, they implicitly support complex
business logic and behavior, have access to a sandboxed file system (via WASI), can execute child
processes, and much more.

danger

Since our WASM plugin implementations are still experimental, expect breaking changes to occur in
non-major releases.

## Powered by Extism

Our WASM plugin system is powered by [Extism](https://extism.org/), a Rust-based cross-language
framework for building WASM plugins under a unified guest and host API. Under the hood, Extism uses
[wasmtime](https://wasmtime.dev/) as its WASM runtime.

For the most part, you do not need to know about Extism's host SDK, as we have implemented the
bulk of it within moon and proto directly. However, you should be familiar with the guest PDKs, as
this is what you'll be using to implement Rust-based plugins. We suggest reading the following
material:

- [Plugin development kits](https://extism.org/docs/concepts/pdk) (PDKs)

- The [extism-pdk](https://github.com/extism/rust-pdk) Rust crate

- [Host functions](https://extism.org/docs/concepts/host-functions) (how they work)

## Concepts

Before we begin, let's talk about a few concepts that are critical to WASM and our plugin systems.

### Plugin identifier

When implementing plugin functions, you'll need to access information about the current plugin. To
get the current plugin identifier (the key the plugin was configured with), use the
[`get_plugin_id`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.get_plugin_id.html) function.

```
let id = get_plugin_id();
```

### Virtual paths

WASM by default does not have access to the host file system, but through [WASI](https://wasi.dev/),
we can provide sandboxed access to a pre-defined list of allowed directories. We call these
[virtual paths](https://docs.rs/warpgate_api/latest/warpgate_api/enum.VirtualPath.html), and all
paths provided via function input or context use them.

Virtual paths are implemented by mapping a real path (host machine) to a virtual path (guest
runtime) using file path prefixes. The following prefixes are currently supported:

Real path Virtual path Only for

`~` `/userhome` ~

`~/.proto` `/proto` ~

`~/.moon` `/moon` moon

moon workspace `/workspace` moon

For example, from the context of WASM, you may have a virtual path of `/proto/tools/node/1.2.3`,
which simply maps back to `~/.proto/tools/node/1.2.3` on the host machine. However, this should
almost always be transparent to you, the developer, and to end users.

However, there may be a few cases where you need access to the real path from WASM, for example,
logging or executing commands. For this, the real path can be accessed with the
[`real_path`](https://docs.rs/warpgate_api/latest/warpgate_api/enum.VirtualPath.html#method.real_path)
function on the `VirtualPath` enum (this is a Rust only feature).

```
virtual_path.real_path();
```

#### File system caveats

When working with the file system from the context of WASM, there are a few caveats to be aware of.

- All `fs` calls must use the virtual path. Real paths will error.

- Paths not white listed (using prefixes above) will error.

- Changing file permissions is not supported (on Unix and Windows). This is because WASI does not support this.

- This also means operations like unpacking archives is not possible.

### Host environment

Since WASM executes in its own runtime, it does not have access to the current host operating
system, architecture, so on and so forth. To bridge this gap, we provide the
[`get_host_environment`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.get_host_environment.html)
function.
[Learn more about this type](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/struct.HostEnvironment.html).

```
let env = get_host_environment()?;
```

The host operating system and architecture can be accessed with `os` and `arch` fields respectively.
Both fields are an enum in Rust, or a string in other languages.

```
if env.os == HostOS::Windows {    // Windows only}if env.arch == HostArch::Arm64 {    // aarch64 only}
```

Furthermore, the user's home directory (`~`) can be accessed with the `home_dir` field, which is a
[virtual path](#virtual-paths).

```
if env.home_dir.join(some_path).exists() {    // Do something}
```

### Host functions & macros

WASM is pretty powerful but it can't do everything since it's sandboxed. To work around this, we
provide a mechanism known as host functions, which are functions that are implemented on the host
(in Rust), and can be executed from WASM. The following host functions are currently available:

- [`exec_command`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.exec_command.html) - Execute a system command on the host machine, with a provided list of arguments or environment variables.

- [`from_virtual_path`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.real_path.html) - Converts a virtual path into a real path.

- [`get_env_var`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.host_env.html) - Get an environment variable value from the host environment.

- [`host_log`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.host_log.html) - Log an stdout, stderr, or tracing message to the host's terminal.

- [`send_request`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.send_request.html) - Requests a URL on the host machine using a Rust-based HTTP client (not WASM).

- [`set_env_var`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.host_env.html) - Set an environment variable to the host environment.

- [`to_virtual_path`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.virtual_path.html) - Converts a real path into a virtual path.

To use host functions, you'll need to make them available by registering them at the top of your
Rust file (only add the functions you want to use) using the
[extism-pdk](https://crates.io/crates/extism-pdk) crate.

```
use extism_pdk::*;#[host_fn]extern "ExtismHost" {    fn exec_command(input: Json) -> Json;    fn from_virtual_path(path: String) -> String;    fn get_env_var(key: String) -> String;    fn host_log(input: Json);    fn send_request(input: Json) -> Json;    fn set_env_var(key: String, value: String);    fn to_virtual_path(path: String) -> Json;}
```

info

To simplify development, we provide built-in functions and macros for the host functions above.
Continue reading for more information on these macros.

#### Converting paths

When working with virtual paths, you may need to convert them to real paths, and vice versa. The
[`into_virtual_path`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.into_virtual_path.html)
and [`into_real_path`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.into_real_path.html)
functions can be used for such situations, which use the `to_virtual_path` and `from_virtual_path`
host functions respectively.

```
// Supports strings or pathslet virt = into_virtual_path("/some/real/path")?;let real = into_real_path(PathBuf::from("/some/virtual/path"))?;
```

#### Environment variables

The [`get_host_env_var`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.get_host_env_var.html)
and [`set_host_env_var`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.set_host_env_var.html)
functions can be used to read and write environment variables on the host, using the `set_env_var`
and `get_env_var` host functions respectively.

```
// Set a valueset_host_env_var("ENV_VAR", "value")?;// Get a value (returns an `Option`)let value = get_host_env_var("ENV_VAR")?;
```

Additionally, the
[`add_host_paths`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.add_host_paths.html) function
can be used to append paths to the `PATH` environment variable.

```
// Append to pathadd_host_paths(["/userhome/some/virtual/path"])?;
```

#### Executing commands

The [`exec_command!`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.exec_command.html)
macro can be used to execute a command on the host, using the `exec_command` host function. If the
command does not exist on `PATH`, an error is thrown. This macros supports three modes: pipe,
inherit, and raw (returns `Result`).

```
let result = exec_command!(raw, "which", ["node"]);
```

If you want a simpler API, the
[`exec`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.exec.html),
[`exec_captured`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.exec_captured.html) (pipe),
and [`exec_streamed`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.exec_streamed.html)
(inherit) functions can be used.

```
// Pipe stdout/stderrlet output = exec_captured("which", ["node"])?;// Inherit stdout/stderrexec_streamed("npm", ["install"])?;// Full controlexec(ExecCommandInput {    command: "npm".into(),    args: vec!["install".into()],    ..ExecCommandInput::default()})?;
```

#### Sending requests

The [`send_request`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.send_request.html) macro
can be used to request a URL on the host, instead of from WASM, allowing it to use the same HTTP
client as the host CLI. This macro returns a response object, with the raw body in bytes, and the
status code.

```
let response = send_request!("https://some.com/url/to/fetch");if response.status == 200 {  let json = response.json::()?;  let text = response.text()?;} else {  // Error!}
```

To simplify the handling of requests -> responses, we also provide the
[`fetch_bytes`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.fetch_bytes.html),
[`fetch_json`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.fetch_json.html), and
[`fetch_text`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/fn.fetch_text.html) functions.

```
let json: T = fetch_json("https://some.com/url/to/fetch.json")?;
```

Only GET requests are supported.

#### Logging

The [`host_log!`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/macro.host_log.html) macro can be
used to write stdout or stderr messages to the host's terminal, using the `host_log` host function.
It supports the same argument patterns as `format!`.

If you want full control, like providing data/fields, use the input mode and provide
[`HostLogInput`](https://docs.rs/warpgate_pdk/latest/warpgate_pdk/struct.HostLogInput.html).

```
host_log!(stdout, "Some message");host_log!(stderr, "Some message with {}", "args");// With datahost_log!(input, HostLogInput {    message: "Some message with data".into(),    data: HashMap::from_iter([        ("data".into(), serde_json::to_value(data)?),    ]),    target: HostLogTarget::Stderr,});
```

Furthermore, the [extism-pdk](https://crates.io/crates/extism-pdk) crate provides a handful of
macros for writing level-based messages that'll appear in the host's terminal when `--log` is
enabled in the CLI. These also support arguments.

```
debug!("This is a debug message");info!("Something informational happened");warn!("Proceed with caution");error!("Oh no, something went wrong");
```

## Configuring plugin locations

To use a WASM plugin, it'll need to be configured in both moon and proto. Luckily both tools use a
similar approach for configuring plugins called the
[plugin locator](https://docs.rs/warpgate/latest/warpgate/enum.PluginLocator.html). A locator string
is composed of 2 parts separated by `://`, the former is the protocol, and the latter is the
location.

```
"
://"
```

The following locator patterns are supported:

### `file`

The `file://` protocol represents a file path, either absolute or relative (from the current
configuration file).

```
# Relative"file://./path/to/example.wasm"# Absolute"file:///root/path/to/example.wasm"
```

### `github`

The `github://` protocol can be used to target and download an asset from a specific GitHub release.
The location must be an organization + repository slug (owner/repo), and the release must have a
`.wasm` asset available to download.

```
"github://moonrepo/example-repo"
```

If you are targeting releases in a monorepo, you can append the project name after the repository.
The project name will be used as a prefix for tags, and will match `@v?` or
`-v?` based tags.

```
"github://moonrepo/example-repo/project-name"
```

By default, the latest release will be used and cached for 7 days. If you'd prefer to target a
specific release (preferred), append the release tag to the end of the location.

```
"github://moonrepo/example-repo@v1.2.3"
```

This strategy is powered by the [GitHub API](https://api.github.com/) and is subject to rate
limiting. If running in a CI environment, we suggesting setting a `GITHUB_TOKEN` environment
variable to authorize API requests with. If using GitHub Actions, it's as simple as:

```
# In some job or step...env:  GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}'
```

### `https`

The `https://` protocol is your standard URL, and must point to an absolute file path. Files will be
downloaded to `~/.moon/plugins` or `~/.proto/plugins`. Non-secure URLs are not supported!

```
"https://domain.com/path/to/plugins/example.wasm"
```

## Creating a plugin

info

Although plugins can be written in any language that compiles to WASM, we've only tested Rust. The
rest of this article assume you're using Rust and Cargo! Refer to [Extism](https://extism.org/)'s
documentation for other examples.

To start, create a new crate with Cargo:

```
cargo new plugin --libcd plugin
```

Set the lib type to `cdylib`, and provide other required settings.

Cargo.toml

```
[package]name = "example_plugin"version = "0.0.1"edition = "2024"publish = false[lib]crate-type = ['cdylib'][profile.release]codegen-units = 1debug = falselto = trueopt-level = "s"panic = "abort"
```

Our Rust plugins are powered by [Extism](https://extism.org/), so lets add their PDK and ours as a
dependency.

```
cargo add extism-pdk# For protocargo add proto_pdk# For mooncargo add moon_pdk
```

In all Rust files, we can import all the PDKs with the following:

src/lib.rs

```
use extism_pdk::*;
```

We can then build the WASM binary. The file will be available at
`target/wasm32-wasip1/debug/.wasm`.

```
cargo build --target wasm32-wasip1
```

## Building and publishing

At this point, you should have a fully working WASM plugin, but to make it available to the
community, you'll still need to build and make the `.wasm` file available. The easiest solution is
to publish a GitHub release and include the `.wasm` file as an asset.

### Building, optimizing, and stripping

WASM files are pretty fat, even when compiling in release mode. To reduce the size of these files,
we can use `wasm-opt` and `wasm-strip`, both of which are provided by the
[WebAssembly](https://github.com/WebAssembly) group. The following script is what we use to build
our own plugins.

info

This functionality is natively supported in our
[moonrepo/build-wasm-plugin](https://github.com/moonrepo/build-wasm-plugin) GitHub Action!

build-wasm

```
#!/usr/bin/env bashtarget="${CARGO_TARGET_DIR:-target}"input="$target/wasm32-wasip1/release/$1.wasm"output="$target/wasm32-wasip1/$1.wasm"echo "Building"cargo build --target wasm32-wasip1 --releaseecho "Optimizing"# https://github.com/WebAssembly/binaryen~/binaryen/bin/wasm-opt -Os "$input" --output "$output"echo "Stripping"# https://github.com/WebAssembly/wabt~/wabt/bin/wasm-strip "$output"
```

### Manually create releases

When your plugin is ready to be published, you can create a release on GitHub using the following
steps.

- Tag the release and push to GitHub.

```
git tag v0.0.1git push --tags
```

- Build a release version of the plugin using the `build-wasm` script above. The file will be available at `target/wasm32-wasip1/.wasm`.

```
build-wasm
```

- In GitHub, navigate to the tags page, find the new tag, create a new release, and attach the built file as an asset.

### Automate releases

If you're using GitHub Actions, you can automate the release process with our official
[moonrepo/build-wasm-plugin](https://github.com/moonrepo/build-wasm-plugin) action.

- Create a new workflow file at `.github/workflows/release.yml`. Refer to the link above for a working example.

- Tag the release and push to GitHub.

```
# In a polyrepogit tag v0.0.1# In a monorepogit tag example_plugin-v0.0.1# Push the tagsgit push --tags
```

- The action will automatically build the plugin, create a release, and attach the built file as an asset.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
