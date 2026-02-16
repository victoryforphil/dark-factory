----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/concepts/token
- Keywords: moon, moonrepo, docs, monorepo, build, concepts, token
- Summary: Tokens are variables and functions that can be used by [`command`](/docs/config/project#command),
----

Source: https://moonrepo.dev/docs/concepts/token

# Tokens

Tokens are variables and functions that can be used by [`command`](/docs/config/project#command),
[`args`](/docs/config/project#args), [`env`](/docs/config/project#env) (>= v1.12),
[`inputs`](/docs/config/project#inputs), and [`outputs`](/docs/config/project#outputs) when configuring a
task. They provide a way of accessing file group paths, referencing values from other task fields,
and referencing metadata about the project and task itself.

## Functions

A token function is labeled as such as it takes a single argument, starts with an `@`, and is
formatted as `@name(arg)`. The following token functions are available, grouped by their
functionality.

caution

Token functions must be the only content within a value, as they expand to multiple files. When
used in an `env` value, multiple files are joined with a comma (`,`).

### File groups

These functions reference file groups by name, where the name is passed as the argument.

### `@group`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@group(file_group)` token is a standard token that will be replaced with the file group items
as-is, for both file paths and globs. This token merely exists for reusability purposes.

```
fileGroups:  storybook:    - '.storybook/**/*'    - 'src/**/*'    - '**/*.stories.*'# Configured astasks:  build:    command: 'build-storybook'    inputs:      - '@group(storybook)'  start:    command: 'start-storybook'    inputs:      - '@group(storybook)'# Resolves totasks:  build:    command: 'build-storybook'    inputs:      - '/path/to/project/.storybook/**/*'      - '/path/to/project/src/**/*'      - '/path/to/project/**/*.stories.*'  start:    command: 'start-storybook'    inputs:      - '/path/to/project/.storybook/**/*'      - '/path/to/project/src/**/*'      - '/path/to/project/**/*.stories.*'
```

### `@dirs`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@dirs(file_group)` token will be replaced with an expanded list of directory paths, derived
from the file group of the same name. If a glob pattern is detected within the file group, it will
aggregate all directories found.

warning

This token walks the file system to verify each directory exists, and filters out those that don't.
If using within `outputs`, you're better off using [`@group`](#group) instead.

```
fileGroups:  lintable:    - 'src'    - 'tests'    - 'scripts'    - '*.config.js'# Configured astasks:  lint:    command: 'eslint @dirs(lintable) --color'    inputs:      - '@dirs(lintable)'# Resolves totasks:  lint:    command:      - 'eslint'      - 'src'      - 'tests'      - 'scripts'      - '--color'    inputs:      - '/path/to/project/src'      - '/path/to/project/tests'      - '/path/to/project/scripts'
```

### `@files`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@files(file_group)` token will be replaced with an expanded list of file paths, derived from
the file group of the same name. If a glob pattern is detected within the file group, it will
aggregate all files found.

warning

This token walks the file system to verify each file exists, and filters out those that don't. If
using within `outputs`, you're better off using [`@group`](#group) instead.

```
fileGroups:  config:    - '*.config.js'    - 'package.json'# Configured astasks:  build:    command: 'webpack build @files(config)'    inputs:      - '@files(config)'# Resolves totasks:  build:    command:      - 'webpack'      - 'build'      - 'babel.config.js'      - 'webpack.config.js'      - 'package.json'    inputs:      - '/path/to/project/babel.config.js'      - '/path/to/project/webpack.config.js'      - '/path/to/project/package.json'
```

### `@globs`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@globs(file_group)` token will be replaced with the list of glob patterns as-is, derived from
the file group of the same name. If a non-glob pattern is detected within the file group, it will be
ignored.

```
fileGroups:  tests:    - 'tests/**/*'    - '**/__tests__/**/*'# Configured astasks:  test:    command: 'jest --testMatch @globs(tests)'    inputs:      - '@globs(tests)'# Resolves totasks:  test:    command:      - 'jest'      - '--testMatch'      - 'tests/**/*'      - '**/__tests__/**/*'    inputs:      - '/path/to/project/tests/**/*'      - '/path/to/project/**/__tests__/**/*'
```

### `@root`

Usable in `args`, `env`, `inputs`, and `outputs`.

The `@root(file_group)` token will be replaced with the lowest common directory, derived from the
file group of the same name. If a glob pattern is detected within the file group, it will walk the
file system and aggregate all directories found before reducing.

```
fileGroups:  sources:    - 'src/app'    - 'src/packages'    - 'src/scripts'# Configured astasks:  format:    command: 'prettier --write @root(sources)'    inputs:      - '@root(sources)'# Resolves totasks:  format:    command:      - 'prettier'      - '--write'      - 'src'    inputs:      - '/path/to/project/src'
```

When there's no directies, or too many directories, this function will return the project root
using `.`.

### `@envs`v1.21.0

Usable in `inputs`.

The `@envs(file_group)` token will be replaced with all environment variables that have been
configured in the group of the provided name.

```
fileGroups:  sources:    - 'src/**/*'    - '$NODE_ENV'# Configured astasks:  build:    command: 'vite build'    inputs:      - '@envs(sources)'# Resolves totasks:  build:    command: 'vite build'    inputs:      - '$NODE_ENV'
```

### Inputs & outputs

### `@in`

Usable in `script` and `args` only.

The `@in(index)` token will be replaced with a single path, derived from
[`inputs`](/docs/config/project#inputs) by numerical index. If a glob pattern is referenced by index,
the glob will be used as-is, instead of returning the expanded list of files.

```
# Configured astasks:  build:    command:      - 'babel'      - '--copy-files'      - '--config-file'      - '@in(1)'      - '@in(0)'    inputs:      - 'src'      - 'babel.config.js'# Resolves totasks:  build:    command:      - 'babel'      - '--copy-files'      - '--config-file'      - 'babel.config.js'      - 'src'    inputs:      - '/path/to/project/src'      - '/path/to/project/babel.config.js'
```

### `@out`

Usable in `script` and `args` only.

The `@out(index)` token will be replaced with a single path, derived from
[`outputs`](/docs/config/project#outputs) by numerical index.

```
# Configured astasks:  build:    command:      - 'babel'      - '.'      - '--out-dir'      - '@out(0)'    outputs:      - 'lib'# Resolves totasks:  build:    command:      - 'babel'      - '.'      - '--out-dir'      - 'lib'    outputs:      - '/path/to/project/lib'
```

### Miscellaneous

### `@meta`v1.28.0

Usable in `command`, `script`, `args`, `env`, `inputs`, and `outputs` only.

The `@meta(key)` token can be used to access project metadata and will be replaced with a value
derived from [`project`](/docs/config/project#project) in [`moon.yml`](/docs/config/project).

The top-level fields (like `name` and `description`) will be used as-is (no quotes). If the setting
is not defined, it will default to nothing or an empty string. For lists of values, they will be
joined with `,`.

Custom metadata defined in [`project`](/docs/config/project#project) can also be accessed by key, but
will return a JSON stringified value. For example, a custom string value of `example` will be
stringified to `"example"` (with quotes).

```
project:  title: 'example'  index: 123# Configured astasks:  build:    script: 'build --name @meta(title) --index @meta(index)'# Resolves totasks:  build:    script: 'build --name example --index 123'
```

## Variables

A token variable is a value that starts with `$` and is substituted to a value derived from the
current workspace, project, and task. And unlike token functions, token variables can be placed
within content when necessary, and supports multiple variables within the same content.

### Environmentv1.30.0

- `$arch` - The current host architecture, derived from the Rust [`ARCH` constant](https://doc.rust-lang.org/std/env/consts/constant.ARCH.html).

- `$os` - The current operating system, derived from the Rust [`OS` constant](https://doc.rust-lang.org/std/env/consts/constant.OS.html).

- `$osFamily` - The current operating system family, either `unix` or `windows`.

```
# Configured astasks:  build:    command: 'example --arch $arch'# Resolves totasks:  build:    command:      - 'example'      - '--arch'      - 'aarch64'
```

### Workspace

- `$workingDir` - The current working directory.

- `$workspaceRoot` - Absolute file path to the workspace root.

```
# Configured astasks:  build:    command:      - 'example'      - '--cwd'      - '$workspaceRoot'# Resolves totasks:  build:    command:      - 'example'      - '--cwd'      - '/path/to/repo'
```

### Project

Most values are derived from settings in [`moon.yml`](/docs/config/project). When a setting is not
defined, or does not have a config, the variable defaults to "unknown" (for enums) or an empty
string.

- `$language` Programming language the project is written in, as defined with [`language`](/docs/config/project#language).

- `$project` - ID of the project that owns the currently running task, as defined in [`.moon/workspace.yml`](/docs/config/workspace).

- `$projectAlias` - Alias of the project that owns the currently running task.

- `$projectChannel` - The discussion channel for the project, as defined with [`project.channel`](/docs/config/project#channel). v1.28.0

- `$projectLayer` - The project layer, as defined with [`layer`](/docs/config/project#layer). v1.39.0

- `$projectTitle` - The human-readable name of the project, as defined with [`project.title`](/docs/config/project#title). v1.28.0

- `$projectOwner` - The owner of the project, as defined with [`project.owner`](/docs/config/project#name). v1.28.0

- `$projectRoot` - Absolute file path to the project root.

- `$projectSource` - Relative file path from the workspace root to the project root, as defined in [`.moon/workspace.yml`](/docs/config/workspace).

- `$projectStack` - The stack of the project, as defined with [`stack`](/docs/config/project#stack). v1.22.0

```
# Configured astasks:  build:    command: 'example debug $language'# Resolves totasks:  build:    command:      - 'example'      - 'debug'      - 'node'
```

### Task

- `$target` - Fully-qualified target that is currently running.

- `$task` - ID of the task that is currently running. Does not include the project ID.

- `$taskToolchain` - The toolchain that task will run against, as defined in [`moon.yml`](/docs/config/project). v1.31.0

- `$taskType` - The [type of task](/docs/concepts/task#types), based on its configured settings.

```
# Configured astasks:  build:    command: 'example $target'# Resolves totasks:  build:    command:      - 'example'      - 'web:build'
```

### Date/Time

- `$date` - The current date in the format of `YYYY-MM-DD`.

- `$datetime` - The current date and time in the format of `YYYY-MM-DD_HH:MM:SS`.

- `$time` - The current time in the format of `HH:MM:SS`.

- `$timestamp` - The current date and time as a UNIX timestamp in seconds.

```
# Configured astasks:  build:    command: 'example --date $date'# Resolves totasks:  build:    command:      - 'example'      - '--date'      - '2023-03-17'
```

### VCSv1.30.0

- `$vcsBranch` - The current branch.

- `$vcsRepository` - The repository slug, in the format of `owner/repo`.

- `$vcsRevision` - The current revision (commit, etc).

```
# Configured astasks:  build:    command: 'example --branch $vcsBranch'# Resolves totasks:  build:    command:      - 'example'      - '--branch'      - 'master'
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
