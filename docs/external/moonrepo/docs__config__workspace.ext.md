----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/config/workspace
- Keywords: moon, moonrepo, docs, monorepo, build, config, workspace
- Summary: The `.moon/workspace.yml` file configures projects and services in the workspace. This file is
----

Source: https://moonrepo.dev/docs/config/workspace

# .moon/workspace

The `.moon/workspace.yml` file configures projects and services in the workspace. This file is
required.

## `extends`

Defines one or many external `.moon/workspace.yml`'s to extend and inherit settings from. Perfect
for reusability and sharing configuration across repositories and projects. When defined, this
setting must be an HTTPS URL or relative file system path that points to a valid YAML document!

.moon/workspace.yml

```
extends: 'https://raw.githubusercontent.com/organization/repository/master/.moon/workspace.yml'
```

info

Settings will be merged recursively for blocks, with values defined in the local configuration
taking precedence over those defined in the extended configuration. However, the `projects` setting
does not merge!

## `projects`Required

Defines the location of all [projects](/docs/concepts/project) within the workspace. Supports either a
manual map of projects (default), a list of globs in which to automatically locate projects, or
both.

caution

Projects that depend on each other and form a cycle must be avoided! While we do our best to avoid
an infinite loop and disconnect nodes from each other, there's no guarantee that tasks will run in
the correct order.

### Using a map

When using a map, each project must be manually configured and requires a unique
[name](/docs/concepts/project#names) as the map key, where this name is used heavily on the command
line and within the project graph for uniquely identifying the project amongst all projects. The map
value (known as the project source) is a file system path to the project folder, relative from the
workspace root, and must be contained within the workspace boundary.

.moon/workspace.yml

```
projects:  admin: 'apps/admin'  apiClients: 'packages/api-clients'  designSystem: 'packages/design-system'  web: 'apps/web'
```

### Using globs

If manually mapping projects is too tedious or cumbersome, you may provide a list of
[globs](/docs/concepts/file-pattern#globs) to automatically locate all project folders, relative from
the workspace root.

When using this approach, the project name is derived from the project folder name, and is cleaned
to our [supported characters](/docs/concepts/project#names), but can be customized with the
[`id`](/docs/config/project#id) setting in [`moon.yml`](/docs/config/project). Furthermore, globbing does risk the
chance of collision, and when that happens, we log a warning and skip the conflicting project from
being configured in the project graph.

.moon/workspace.yml

```
projects:  - 'apps/*'  - 'packages/*'  # Only shared folders with a moon configuration  - 'shared/*/moon.yml'
```

### Using a map and globs

For those situations where you want to use both patterns, you can! The list of globs can be
defined under a `globs` field, while the map of projects under a `sources` field.

.moon/workspace.yml

```
projects:  globs:    - 'apps/*'    - 'packages/*'  sources:    www: 'www'
```

Additionally, you can customize the format of project IDs for glob discovered projects. By default
it inherits the fodler name, but this has a high chance of collision. Instead you can configure
`globFormat` to use a different format, for example, using the full workspace relative path as the
project ID.

.moon/workspace.yml

```
projects:  globFormat: 'source-path'  globs:    - 'packages/**/moon.yml'
```

## `defaultProject`v2.0.0

Defines the default project to focus on when no project scope is specified on the command line for
task targets.

.moon/workspace.yml

```
defaultProject: 'web'
```

## `codeowners`v1.8.0

Configures code owners (`CODEOWNERS`) integration across the entire workspace.

### `globalPaths`

This setting defines file patterns and their owners at the workspace-level, and are applied to any
matching path, at any depth, within the entire workspace. This is useful for defining global or
fallback owners when a granular [project-level path](/docs/config/project#paths) does not match or exist.

.moon/workspace.yml

```
codeowners:  globalPaths:    '*': ['@admins']    'config/': ['@infra']    '/.github/': ['@infra']
```

### `orderBy`

The order in which code owners, grouped by project, are listed in the `CODEOWNERS` file. Accepts
"file-source" (default) or "project-id".

.moon/workspace.yml

```
codeowners:  orderBy: 'project-id'
```

### `sync`

Will automatically generate a `CODEOWNERS` file by aggregating and syncing all project
[`owners`](/docs/config/project#owners) in the workspace when a [target is run](/docs/concepts/target). The format
and location of the `CODEOWNERS` file is based on the [`vcs.provider`](#provider) setting. Defaults
to `false`.

.moon/workspace.yml

```
codeowners:  sync: true
```

## `constraints`

Configures constraints between projects that are enforced during project graph generation. This is
also known as project boundaries.

### `enforceLayerRelationships`

Enforces allowed relationships between a project and its dependencies based on the project's
[`layer`](/docs/config/project#layer) and [`stack`](/docs/config/project#stack) settings. When a project depends on
another project of an invalid layer, a layering violation error will be thrown when attempting to
run a task.

Layers are allowed to depend on lower layers in the same stack, but not higher layers. Additionally,
layers may depend on itself, excluding automations and applications. The following layers are
stacked as such:

Layer Description

`automation` An automated testing suite, like E2E, integration, or visual tests.

`application` An application of any kind.

`tool` An internal tool, CLI, one-off script, etc.

`library` A self-contained, shareable, and publishable set of code.

`scaffolding` Templates or generators for scaffolding.

`configuration` Configuration files or infrastructure.

`unknown` When not configured.

When the project `stack` setting is defined, it alters these rules to allow these kinds of
relationships. For example, a frontend application can depend on a backend application, but not
another frontend application.

.moon/workspace.yml

```
constraints:  enforceLayerRelationships: false
```

Projects with an unconfigured or unknown layer are ignored during enforcement.

### `tagRelationships`

Enforces allowed relationships between a project and its dependencies based on the project's
[`tags`](/docs/config/project#tags) setting. This works in a similar fashion to `enforceLayerRelationships`,
but gives you far more control over what these relationships look like.

For example, let's enforce that Next.js projects using the `next` tag can only depend on React
projects using the `react` tag. If a dependency does not have one of the configured required tags,
in this case `react`, an error will occur.

.moon/workspace.yml

```
constraints:  tagRelationships:    next: ['react']
```

On the project side, we would configure [`moon.yml`](/docs/config/project#tags) like so:

app/moon.yml

```
tags: ['next']dependsOn: ['components']
```

packages/components/moon.yml

```
tags: ['react']
```

## `docker`v1.27.0

Configures Docker integration for the entire workspace.

### `prune`

Configures aspects of the Docker pruning process when
[`moon docker prune`](/docs/commands/docker/prune) is executed.

#### `deleteVendorDirectories`

Automatically delete vendor directories (package manager dependencies, build targets, etc) while
pruning. For example, `node_modules` for JavaScript, or `target` for Rust. Defaults to `true`.

.moon/workspace.yml

```
docker:  prune:    deleteVendorDirectories: false
```

This process happens before toolchain dependencies are installed.

#### `installToolchainDependencies`

Automatically install production dependencies for all required toolchain's of the focused projects
within the Docker build. For example, `node_modules` for JavaScript. Defaults to `true`.

.moon/workspace.yml

```
docker:  prune:    installToolchainDependencies: false
```

This process happens after vendor directories are deleted.

### `scaffold`

Configures aspects of the Docker scaffolding process when
[`moon docker scaffold`](/docs/commands/docker/scaffold) is executed. Only applies to the
[workspace skeleton](/docs/commands/docker/scaffold#workspace).

#### `configsPhaseGlobs`

List of globs in which to copy additional workspace-relative files into the `.moon/docker/workspace`
skeleton. When not defined, does nothing.

.moon/workspace.yml

```
docker:  scaffold:    configsPhaseGlobs:      - '**/package.json'
```

## `experiments`v1.11.0

Enable or disable experiments that alter core functionality.

warning

Experiments are a work in progress and may be buggy. Please report any issues you encounter!

### `fasterGlobWalk`v1.34.0

Utilizes a new concurrent glob walking implementation that is on average, 1.5-2x faster than the
current implementation. Additionally, common globs are now cached for the duration of the process.
Defaults to `true`.

.moon/workspace.yml

```
experiments:  fasterGlobWalk: true
```

### `gitV2`v1.34.0

Utilizes a Git implementation, that has better support for submodules, subtrees, and workspaces.
Additionally, processes are parallized when applicable. Defaults to `true`.

.moon/workspace.yml

```
experiments:  gitV2: true
```

## `generator`

Configures aspects of the template generator.

### `templates`

A list of paths in which templates can be located. Supports the following types of paths, and
defaults to `./templates`.

- File system paths, relative from the workspace root.

- Git repositories and a revision, prefixed with `git://`. v1.23.0

- npm packages and a version, prefixed with `npm://`. v1.23.0

.moon/workspace.yml

```
generator:  templates:    - './templates'    - 'file://./other/templates'    - 'git://github.com/moonrepo/templates#master'    - 'npm://@moonrepo/templates#1.2.3'
```

Learn more about this in the official
[code generation guide](/docs/guides/codegen#configuring-template-locations)!

## `hasher`

Configures aspects of the smart hashing layer.

### `ignoreMissingPatterns`v1.10.0

When [`hasher.warnOnMissingInputs`](#warnonmissinginputs) is enabled, moon will log a warning to the
terminal that an input is missing. This is useful for uncovering misconfigurations, but can be quite
noisy when inputs are truly optional.

To ignore warnings for missing inputs, a list of [glob patterns](/docs/concepts/file-pattern#globs) can
be configured to filter and ignore files. Files are matched against workspace relative paths, so
prefixing patterns with `**/` is suggested.

.moon/workspace.yml

```
hasher:  ignoreMissingPatterns:    - '**/.eslintrc.*'    - '**/*.config.*'
```

### `ignorePatterns`v1.10.0

A list of [glob patterns](/docs/concepts/file-pattern#globs) used to filter and ignore files during the
inputs hashing process. Files are matched against workspace relative paths, so prefixing patterns
with `**/` is suggested.

.moon/workspace.yml

```
hasher:  ignorePatterns:    - '**/*.png'
```

### `optimization`

Determines the optimization level to utilize when hashing content before running targets.

- `accuracy` (default) - When hashing dependency versions, utilize the resolved value in the lockfile. This requires parsing the lockfile, which may reduce performance.

- `performance` - When hashing dependency versions, utilize the value defined in the manifest. This is typically a version range or requirement.

.moon/workspace.yml

```
hasher:  optimization: 'performance'
```

### `walkStrategy`

Defines the file system walking strategy to utilize when discovering inputs to hash.

- `glob` - Walks the file system using glob patterns.

- `vcs` (default) - Calls out to the [VCS](#vcs) to extract files from its working tree.

.moon/workspace.yml

```
hasher:  walkStrategy: 'glob'
```

### `warnOnMissingInputs`

When enabled, will log warnings to the console when attempting to hash an input that does not exist.
This is useful in uncovering misconfigured tasks. Defaults to `true`.

.moon/workspace.yml

```
hasher:  warnOnMissingInputs: false
```

## `notifier`

Configures how moon notifies and interacts with a developer or an external system.

### `terminalNotifications`v1.38.0

When defined, will display OS notifications for action pipeline events when running commands from a
terminal. Supports the following values:

- `always` - Display on pipeline success and failure.

- `failure` - Display on pipeline failure only.

- `success` - Display on pipeline success only.

- `task-failure` - Display for each task failure.

.moon/workspace.yml

```
notifier:  terminalNotifications: 'always'
```

### `webhookUrl`

Defines an HTTPS URL that all pipeline events will be posted to. View the
[webhooks guide for more information](/docs/guides/webhooks) on available events.

.moon/workspace.yml

```
notifier:  webhookUrl: 'https://api.company.com/some/endpoint'
```

### `acknowledge`

When enabled, webhook notifier will wait for request result and validates the return code for 2xx.
Defaults to `false`.

warning

Activating this setting will slow down your pipeline, because every webhook request will be
evaluated!

.moon/workspace.yml

```
notifier:  webhookUrl: 'https://api.company.com/some/endpoint'  webhookAcknowledge: true
```

## `pipeline`

Configures aspects of task running and the action pipeline.

### `autoCleanCache`v1.24.0

Automatically cleans cached artifacts older than [`cacheLifetime`](#cachelifetime) from the cache
directory (`.moon/cache`) after every run. This is useful for keeping the cache directory lean.
Defaults to `true`.

.moon/workspace.yml

```
pipeline:  autoCleanCache: false
```

### `cacheLifetime`

The maximum lifetime of cached artifacts before they're marked as stale and automatically removed by
the action pipeline. Defaults to "7 days". This field requires an integer and a timeframe unit that
can be [parsed as a duration](https://docs.rs/humantime/2.1.0/humantime/fn.parse_duration.html).

.moon/workspace.yml

```
pipeline:  cacheLifetime: '24 hours'
```

### `inheritColorsForPipedTasks`

Force colors to be inherited from the current terminal for all tasks that are ran as a child process
and their output is piped to the action pipeline. Defaults to `true`.
[View more about color handling in moon](/docs/commands/overview#colors).

.moon/workspace.yml

```
pipeline:  inheritColorsForPipedTasks: true
```

### `installDependencies`v1.34.0

When enabled, runs the
[`InstallWorkspaceDeps` and `InstallProjectDeps` actions](/docs/how-it-works/action-graph#install-dependencies)
within the pipeline before running an applicable task. Installation is determined based on changed
manifests and lockfiles. Defaults to `true`.

.moon/workspace.yml

```
pipeline:  installDependencies: false
```

Instead of a boolean, a list of toolchain IDs can be provided to only allow those toolchains to
install dependencies.

.moon/workspace.yml

```
pipeline:  installDependencies: ['node']
```

### `killProcessThreshold`v1.32.1

Threshold in milliseconds in which to force kill running child processes after the pipeline receives
an external signal (like `SIGINT` or `SIGTERM`). A value of 0 will not kill the process and let them
run to completion. Defaults to `2000` (2 seconds).

.moon/workspace.yml

```
pipeline:  killProcessThreshold: 5000
```

### `logRunningCommand`

When enabled, will log the task's command, resolved arguments, and working directory when a target
is ran. Defaults to `false`.

.moon/workspace.yml

```
pipeline:  logRunningCommand: true
```

### `syncProjects`v1.34.0

When enabled, runs the [`SyncProject` action](/docs/how-it-works/action-graph#sync-project) within the
pipeline before running an applicable task. Defaults to `true`.

.moon/workspace.yml

```
pipeline:  syncProjects: false
```

Instead of a boolean, a list of project IDs can be provided to only sync those projects.

.moon/workspace.yml

```
pipeline:  syncProjects: ['app']
```

The [`moon sync projects`](/docs/commands/sync/projects) command can be executed to manually sync
projects.

### `syncWorkspace`v1.34.0

When enabled, runs the [`SyncWorkspace` action](/docs/how-it-works/action-graph#sync-workspace) within
the pipeline before all other actions. This syncing includes operations such as codeowners, VCS
hooks, and more. Defaults to `true`.

.moon/workspace.yml

```
pipeline:  syncWorkspace: false
```

The [`moon sync ...`](/docs/commands/sync) sub-commands can be executed to manually sync features.

## `remote`v1.30.0

Configures a remote service, primarily for cloud-based caching of artifacts. Learn more about this
in the [remote caching](/docs/guides/remote-cache) guide.

### `api`v1.32.0

The API format of the remote server. This format dictates which type of client moon uses for
communicating with. Supports the following:

- `grpc` (default) - Uses the gRPC API: [https://github.com/bazelbuild/remote-apis](https://github.com/bazelbuild/remote-apis)

- `http` - Uses the HTTP API: [https://bazel.build/remote/caching#http-caching](https://bazel.build/remote/caching#http-caching)

.moon/workspace.yml

```
remote:  api: 'grpc'
```

### `auth`v1.32.0

Configures authorization and authentication level features of our remote clients.

#### `headers`v1.32.0

A mapping of HTTP headers to include in all requests to the remote server. These headers are applied
to all [API formats and protocols](#api), not just HTTP.

.moon/workspace.yml

```
remote:  auth:    headers:      'X-Custom-Header': 'value'
```

#### `token`v1.32.0

The name of an environment variable in which to extract a token for
[Bearer HTTP authorization](https://swagger.io/docs/specification/v3_0/authentication/bearer-authentication/).
An `Authorization` HTTP header will be included in all requests to the remote server.

If the token does not exist, or is not enabled, remote caching will be disabled.

.moon/workspace.yml

```
remote:  auth:    token: 'ENV_VAR_NAME'
```

### `cache`

Configures aspects of the caching layer, primarily the action cache (AC) and content addressable
cache (CAS).

#### `compression`v1.31.0

The compression format to use when uploading/downloading blobs. Supports `none` and `zstd`, and
defaults to no compression (`identity` format in RE API).

.moon/workspace.yml

```
remote:  cache:    compression: 'zstd'
```

info

Compression is only applied to gRPC based APIs, not HTTP.

#### `instanceName`

A
[unique identifier](https://github.com/bazelbuild/remote-apis/blob/main/build/bazel/remote/execution/v2/remote_execution.proto#L223)
used to distinguish between the various instances on the host. This allows the same remote service
to serve and partition multiple moon repositories. Defaults to `moon-outputs`.

.moon/workspace.yml

```
remote:  cache:    instanceName: 'custom-dir-name'
```

We suggest changing the instance name to the name of your repository!

#### `localReadOnly`v1.40.0

When enabled and developing locally, existing remote blobs will only be downloaded, but new local
blobs will not be uploaded. Blobs will only be uploaded in CI environments.

.moon/workspace.yml

```
remote:  cache:    localReadOnly: true
```

#### `verifyIntegrity`v1.36.0

When downloading blobs, verify the digests/hashes in the response match the associated blob
contents. This will reduce performance but ensure partial or corrupted blobs won't cause failures.
Defaults to `false`.

.moon/workspace.yml

```
remote:  cache:    verifyIntegrity: true
```

### `host`

The host URL to communicate with when uploading and downloading artifacts. Supports both
`grpc(s)://` and `http(s)://` protocols. This field is required!

.moon/workspace.yml

```
remote:  host: 'grpcs://your-host.com:9092'
```

### `mtls`

Connect to the host using server and client authentication with mTLS. This takes precedence over
normal TLS.

.moon/workspace.yml

```
remote:  # ...  mtls:    caCert: 'certs/ca.pem'    clientCert: 'certs/client.pem'    clientKey: 'certs/client.key'    domain: 'your-host.com'
```

#### `assumeHttp2`

If true, assume that the host supports HTTP/2, even if it doesn't provide protocol negotiation via
ALPN.

#### `caCert`

A file path, relative from the workspace root, to the certificate authority PEM encoded X509
certificate (typically `ca.pem`).

#### `clientCert`

A file path, relative from the workspace root, to the client's PEM encoded X509 certificate
(typically `client.pem`).

#### `clientKey`

A file path, relative from the workspace root, to the client's PEM encoded X509 private key
(typically `client.key`).

#### `domain`

The domain name in which to verify the TLS certificate.

### `tls`

Connect to the host using server-only authentication with TLS.

.moon/workspace.yml

```
remote:  # ...  tls:    cert: 'certs/ca.pem'    domain: 'your-host.com'
```

#### `assumeHttp2`

If true, assume that the host supports HTTP/2, even if it doesn't provide protocol negotiation via
ALPN.

#### `cert`

A file path, relative from the workspace root, to the certificate authority PEM encoded X509
certificate (typically `ca.pem`).

#### `domain`

The domain name in which to verify the TLS certificate.

## `telemetry`

When enabled, will check for a newer moon version and send anonymous usage data to the moonrepo
team. This data is used to improve the quality and reliability of the tool. Defaults to `true`.

.moon/workspace.yml

```
telemetry: false
```

## `vcs`

Configures the version control system to utilize within the workspace (and repository). A VCS is
required for determining touched (added, modified, etc) files, calculating file hashes, computing
affected files, and much more.

### `defaultBranch`

Defines the default branch in the repository for comparing differences against. For git, this is
typically "master" (default) or "main".

.moon/workspace.yml

```
vcs:  defaultBranch: 'master'
```

### `hooks`v1.9.0

Defines a mapping of hooks to a list of commands to run when that event is triggered. There are no
restrictions to what commands can be run, but the binaries for each command must exist on each
machine that will be running hooks.

For Git, each [hook name](https://git-scm.com/docs/githooks#_hooks) must be a valid kebab-cased
name. [Learn more about Git hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks).

.moon/workspace.yml

```
vcs:  hooks:    pre-commit:      - 'moon run :lint :format --affected --status=staged --no-bail'      - 'another-command'
```

info

If running `moon` commands directly, the `moon` binary must be installed globally!

### `hookFormat`v1.29.0

The shell and file type in which generated hook files are formatted with. Supports the following:

- `native` (default) - The format native to the current operating system. Bash on Unix, and PowerShell on Windows.

- `bash` - Forces the format to Bash for all operating systems.

.moon/workspace.yml

```
vcs:  hookFormat: 'bash'
```

### `client`

Defines the VCS tool/binary that is being used for managing the repository. Accepts "git" (default).
Expect more version control systems in the future!

.moon/workspace.yml

```
vcs:  client: 'git'
```

### `provider`v1.8.0

Defines the service provider that the repository is hosted on. Accepts "github" (default), "gitlab",
"bitbucket", or "other".

.moon/workspace.yml

```
vcs:  provider: 'github'
```

### `remoteCandidates`

(Git only) Defines a list of remote candidates to query against to determine merge bases. Defaults
to "origin" and "upstream".

.moon/workspace.yml

```
vcs:  remoteCandidates:    - 'origin'    - 'upstream'
```

### `sync`v1.9.0

Will automatically generate [hook scripts](#hooks) to `.moon/hooks` and sync the scripts to the
local VCS checkout. The hooks format and location is based on the [`vcs.client`](#client) setting.
Defaults to `false`.

.moon/workspace.yml

```
vcs:  hooks:    # ...  sync: true
```

caution

When enabled, this will sync hooks for all users of the repository. For personal or small
projects, this may be fine, but for larger projects, this may be undesirable and disruptive!

## `versionConstraint`

Defines a version requirement for the currently running moon binary. This provides a mechanism for
enforcing that the globally installed moon on every developers machine is using an applicable
version.

.moon/workspace.yml

```
versionConstraint: '>=0.20.0'
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
