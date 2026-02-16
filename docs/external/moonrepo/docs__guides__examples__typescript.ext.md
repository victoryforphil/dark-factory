----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/typescript
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, typescript
- Summary: In this guide, you'll learn how to integrate [TypeScript](https://www.typescriptlang.org/) into
----

Source: https://moonrepo.dev/docs/guides/examples/typescript

# TypeScript example

In this guide, you'll learn how to integrate [TypeScript](https://www.typescriptlang.org/) into
moon. We'll be using [project references](/docs/guides/javascript/typescript-project-refs), as it ensures that
only affected projects are built, and not the entire repository.

Begin by installing `typescript` and any pre-configured tsconfig packages in your root. We suggest
using the same version across the entire repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev typescript tsconfig-moon
```

```
yarn add --dev typescript tsconfig-moon# If using workspacesyarn add --dev -W typescript tsconfig-moon
```

```
npm install --save-dev typescript tsconfig-moon
```

```
pnpm add --save-dev typescript tsconfig-moon# If using workspacespnpm add --save-dev -w typescript tsconfig-moon
```

```
bun install --dev typescript tsconfig-moon
```

## Setup

Since typechecking is a universal workflow, add a `typecheck` task to
[`.moon/tasks/node.yml`](/docs/config/tasks) with the following parameters.

.moon/tasks/node.yml

```
tasks:  typecheck:    command:      - 'tsc'      # Use incremental builds with project references      - '--build'      # Always use pretty output      - '--pretty'      # Use verbose logging to see affected projects      - '--verbose'    inputs:      # Source and test files      - 'src/**/*'      - 'tests/**/*'      # Type declarations      - 'types/**/*'      # Project configs      - 'tsconfig.json'      - 'tsconfig.*.json'      # Root configs (extended from only)      - '/tsconfig.options.json'    outputs:      # Matches `compilerOptions.outDir`      - 'lib'
```

Projects can extend this task and provide additional parameters if need be, for example.

/moon.yml

```
tasks:  typecheck:    args:      # Force build every time      - '--force'
```

## Configuration

### Root-level

Multiple root-level TypeScript configs are required, as we need to define compiler options that
are shared across the repository, and we need to house a list of all project references.

To start, let's create a `tsconfig.options.json` that will contain our compiler options. In our
example, we'll extend [tsconfig-moon](https://www.npmjs.com/package/tsconfig-moon) for convenience.
Specifically, the `tsconfig.workspaces.json` config, which enables ECMAScript modules, composite
mode, declaration emitting, and incremental builds.

tsconfig.options.json

```
{  "extends": "tsconfig-moon/tsconfig.projects.json",  "compilerOptions": {    // Your custom options    "moduleResolution": "nodenext",    "target": "es2022"  }}
```

We'll also need the standard `tsconfig.json` to house our project references. This is used by
editors and tooling for deep integrations.

tsconfig.json

```
{  "extends": "./tsconfig.options.json",  "files": [],  // All project references in the repo  "references": []}
```

The [`typescript.rootConfigFileName`](/docs/config/toolchain#rootconfigfilename) setting can be
used to change the root-level config name and the
[`typescript.syncProjectReferences`](/docs/config/toolchain#syncprojectreferences) setting will
automatically keep project references in sync!

### Project-level

Every project will require a `tsconfig.json`, as TypeScript itself requires it. The following
`tsconfig.json` will typecheck the entire project, including source and test files.

/tsconfig.json

```
{  // Extend the root compiler options  "extends": "../../tsconfig.options.json",  "compilerOptions": {    // Declarations are written here    "outDir": "lib"  },  // Include files in the project  "include": ["src/**/*", "tests/**/*"],  // Depends on other projects  "references": []}
```

The [`typescript.projectConfigFileName`](/docs/config/toolchain#projectconfigfilename) setting can
be used to change the project-level config name.

### Sharing

To share configuration across projects, you have 3 options:

- Define settings in a [root-level config](#root-level). This only applies to the parent repository.

- Create and publish an [`tsconfig base`](https://www.typescriptlang.org/docs/handbook/tsconfig-json.html#tsconfig-bases) npm package. This can be used in any repository.

- A combination of 1 and 2.

For options 2 and 3, if you're utilizing package workspaces, create a local package with the
following content.

packages/tsconfig-company/tsconfig.json

```
{  "compilerOptions": {    // ...    "lib": ["esnext"]  }}
```

Within another `tsconfig.json`, you can extend this package to inherit the settings.

tsconfig.json

```
{  "extends": "tsconfig-company/tsconfig.json"}
```

## FAQ

### How to preserve pretty output?

TypeScript supports a pretty format where it includes codeframes and color highlighting for
failures. However, when `tsc` is piped or the terminal is not a TTY, the pretty format is lost. To
preserve and always display the pretty format, be sure to pass the `--pretty` argument!

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
