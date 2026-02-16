----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/javascript/node-handbook
- Keywords: moon, moonrepo, docs, monorepo, build, guides, javascript, node handbook
- Summary: Utilizing JavaScript (and TypeScript) in a monorepo can be a daunting task, especially when using
----

Source: https://moonrepo.dev/docs/guides/javascript/node-handbook

# Node.js handbook

Utilizing JavaScript (and TypeScript) in a monorepo can be a daunting task, especially when using
Node.js, as there are many ways to structure your code and to configure your tools. With this
handbook, we'll help guide you through this process.

info

This guide is a living document and will continue to be updated over time!

## moon setup

For this part of the handbook, we'll be focusing on [moon](/moon), our task runner. To start,
languages in moon act like plugins, where their functionality and support is not enabled unless
explicitly configured. We follow this approach to avoid unnecessary overhead.

### Enabling the language

To enable JavaScript support via Node.js, define the [`node`](/docs/config/toolchain#node) setting
in [`.moon/toolchains.yml`](/docs/config/toolchain), even if an empty object.

.moon/toolchains.yml

```
# Enable Node.jsnode: {}# Enable Node.js and override default settingsnode:  packageManager: 'pnpm'
```

info

In moon v1.40+, use `javascript` and `node` instead of `node` to enable the new WASM powered Node.js
toolchain, which is far more accurate and efficient. The non-WASM toolchain will be deprecated in
the future.

Or by pinning a `node` version in [`.prototools`](/docs/proto/config) in the workspace root.

.prototools

```
node = "18.0.0"pnpm = "7.29.0"
```

This will enable the Node.js toolchain and provide the following automations around its ecosystem:

- Node modules will automatically be installed if dependencies in `package.json` have changed, or the lockfile has changed, since the last time a task has ran. We'll also take `package.json` workspaces into account and install modules in the correct location; either the workspace root, in a project, or both.

- Relationships between projects will automatically be discovered based on `dependencies`, `devDependencies`, and `peerDependencies` in `package.json`. The versions of these packages will also be automatically synced when changed.

- Tasks can be [automatically inferred](/docs/config/toolchain#infertasksfromscripts) from `package.json` scripts.

- And much more!

### Utilizing the toolchain

When a language is enabled, moon by default will assume that the language's binary is available
within the current environment (typically on `PATH`). This has the downside of requiring all
developers and machines to manually install the correct version of the language, and to stay in
sync.

Instead, you can utilize [moon's toolchain](/docs/concepts/toolchain), which will download and
install the language in the background, and ensure every task is executed using the exact version
across all machines.

Enabling the toolchain is as simple as defining the [`node.version`](/docs/config/toolchain#version)
setting.

.moon/toolchains.yml

```
# Enable Node.js toolchain with an explicit versionnode:  version: '18.0.0'
```

Versions can also be defined with [`.prototools`](/docs/proto/config).

### Using `package.json` scripts

If you're looking to prototype moon, or reduce the migration effort to moon tasks, you can configure
moon to inherit `package.json` scripts, and internally convert them to moon tasks. This can be
achieved with the [`node.inferTasksFromScripts`](/docs/config/toolchain#infertasksfromscripts)
setting.

.moon/toolchains.yml

```
node:  inferTasksFromScripts: true
```

Or you can run scripts through `npm run` (or `pnpm`, `yarn`) calls.

moon.yml

```
tasks:  build:    command: 'npm run build'
```

## Repository structure

JavaScript monorepo's work best when projects are split into applications and packages, with each
project containing its own `package.json` and dependencies. A root `package.json` must also exist
that pieces all projects together through workspaces.

For small repositories, the following structure typically works well:

```
/├── .moon/├── package.json├── apps/│   ├── client/|   |   ├── ...│   |   └── package.json│   └── server/|       ├── ...│       └── package.json└── packages/    ├── components/    |   ├── ...    │   └── package.json    ├── theme/    |   ├── ...    │   └── package.json    └── utils/        ├── ...        └── package.json
```

For large repositories, grouping projects by team or department helps with ownership and
organization. With this structure, applications and libraries can be nested at any depth.

```
/├── .moon/├── package.json├── infra/│   └── ...├── internal/│   └── ...├── payments/│   └── ...└── shared/    └── ...
```

### Applications

Applications are runnable or executable, like an HTTP server, and are pieced together with packages
and its own encapsulated code. They represent the whole, while packages are the pieces. Applications
can import and depend on packages, but they must not import and depend on other applications.

In moon, you can denote a project as an application using the [`layer`](/docs/config/project#layer)
setting in [`moon.yml`](/docs/config/project).

moon.yml

```
layer: 'application'
```

### Packages

Packages (also known as a libraries) are self-contained reusable pieces of code, and are the
suggested pattern for [code sharing](#code-sharing). Packages can import and depend on other
packages, but they must not import and depend on applications!

In moon, you can denote a project as a library using the [`layer`](/docs/config/project#layer)
setting in [`moon.yml`](/docs/config/project).

moon.yml

```
layer: 'library'
```

### Configuration

Every tool that you'll utilize in a repository will have its own configuration file. This will be a
lot of config files, but regardless of what tool it is, where the config file should go will fall
into 1 of these categories:

- Settings are inherited by all projects. These are known as universal tools, and enforce code consistency and quality across the entire repository. Their config file must exist in the repository root, but may support overrides in each project. Examples: Babel, [ESLint](/docs/guides/examples/eslint), [Prettier](/docs/guides/examples/prettier), [TypeScript](/docs/guides/examples/typescript)

- Settings are unique per project. These are developers tools that must be configured separately for each project, as they'll have different concerns. Their config file must exist in each project, but a shared configuration may exist as a base (for example, Jest presets). Examples: [Jest](/docs/guides/examples/jest), [TypeScript](/docs/guides/examples/typescript) (with project references)

- Settings are one-offs. These are typically for applications or tools that require their own config, but aren't prevalent throughout the entire repository. Examples: [Astro](/docs/guides/examples/astro), [Next](/docs/guides/examples/next), [Nuxt](/docs/guides/examples/nuxt), [Remix](/docs/guides/examples/remix), Tailwind

## Dependency management

Dependencies, also known as node modules, are required by all projects, and are installed through a
package manager like npm, pnpm, or yarn. It doesn't matter which package manager you choose, but we
highly suggest choosing one that has proper workspaces support. If you're unfamiliar with
workspaces, they will:

- Resolve all `package.json`'s in a repository using glob patterns.

- Install dependencies from all `package.json`'s at once, in the required locations.

- Create symlinks of local packages in `node_modules` (to emulate an installed package).

- Deduplicate and hoist `node_modules` when applicable.

All of this functionality enables robust monorepo support, and can be enabled with the following:

- npm
- pnpm
- Yarn
- Yarn (classic)

package.json

```
{  // ...  "workspaces": ["apps/*", "packages/*"]}
```

.yarnrc.yml

```
# ...nodeLinker: 'node-modules'
```

- [Documentation](https://yarnpkg.com/features/workspaces)

package.json

```
{  // ...  "workspaces": ["apps/*", "packages/*"]}
```

- [Documentation](https://classic.yarnpkg.com/en/docs/workspaces)

package.json

```
{  // ...  "workspaces": ["apps/*", "packages/*"]}
```

- [Documentation](https://docs.npmjs.com/cli/v8/using-npm/workspaces)

pnpm-workspace.yaml

```
packages:  - 'apps/*'  - 'packages/*'
```

- [Documentation](https://pnpm.io/workspaces)

caution

Package workspaces are not a requirement for monorepos, but they do solve an array of problems
around module resolution, avoiding duplicate packages in bundles, and general interoperability.
Proceed with caution for non-workspaces setups!

### Workspace commands

The following common commands can be used for adding, removing, or managing dependencies in a
workspace. View the package manager's official documentation for a thorough list of commands.

- npm
- pnpm
- Yarn
- Yarn (classic)

Install dependencies:

```
npm install
```

Add a package:

```
# At the rootnpm install # In a projectnpm install  --workspace

```

Remove a package:

```
# At the rootnpm install # In a projectnpm install  --workspace

```

Update packages:

```
npx npm-check-updates --interactive
```

Install dependencies:

```
pnpm install
```

Add a package:

```
# At the rootpnpm add # In a projectpnpm add  --filter

```

Remove a package:

```
# At the rootpnpm remove # In a projectpnpm remove  --filter

```

Update packages:

```
pnpm update -i -r --latest
```

Install dependencies:

```
yarn install
```

Add a package:

```
# At the rootyarn add # In a projectyarn workspace
 add
```

Remove a package:

```
# At the rootyarn remove # In a projectyarn workspace
 remove
```

Update packages:

```
yarn upgrade-interactive
```

Install dependencies:

```
yarn install
```

Add a package:

```
# At the rootyarn add  -w# In a projectyarn workspace
 add
```

Remove a package:

```
# At the rootyarn remove  -w# In a projectyarn workspace
 remove
```

Update packages:

```
yarn upgrade-interactive --latest
```

### Developer tools at the root

While not a strict guideline to follow, we've found that installing universal developer tool related
dependencies (Babel, ESLint, Jest, TypeScript, etc) in the root `package.json` as `devDependencies`
to be a good pattern for consistency, quality, and the health of the repository. It provides the
following benefits:

- It ensures all projects are utilizing the same version (and sometimes configuration) of a tool.

- It allows the tool to easily be upgraded. Upgrade once, applied everywhere.

- It avoids conflicting or outdated versions of the same package.

With that being said, this does not include development dependencies that are unique to a project!

### Product libraries in a project

Product, application, and or framework specific packages should be installed as production
`dependencies` in a project's `package.json`. We've found this pattern to work well for the
following reasons:

- Application dependencies are pinned per project, avoiding accidental regressions.

- Applications can upgrade their dependencies and avoid breaking neighbor applications.

## Code sharing

One of the primary reasons to use a monorepo is to easily share code between projects. When code is
co-located within the same repository, it avoids the overhead of the "build -> version -> publish to
registry -> upgrade in consumer" workflow (when the code is located in an external repository).

Co-locating code also provides the benefit of fast iteration, fast adoption, and easier migration
(when making breaking changes for example).

With [package workspaces](#dependency-management), code sharing is a breeze. As mentioned above,
every project that contains a `package.json` that is part of the workspace, will be symlinked into
`node_modules`. Because of this, these packages can easily be imported using their `package.json`
name.

```
// Imports from /packages/utils/package.jsonimport utils from '@company/utils';
```

### Depending on packages

Because packages are symlinked into `node_modules`, we can depend on them as if they were normal npm
packages, but with 1 key difference. Since these packages aren't published, they do not have a
version to reference, and instead, we can use the special `workspace:^` version (yarn and pnpm only,
use `*` for npm).

```
{  "name": "@company/consumer",  "dependencies": {    "@company/provider": "workspace:^"  }}
```

The `workspace:` version basically means "use the package found in the current workspace". The `:^`
determines the version range to substitute with when publishing. For example, the `workspace:^`
above would be replaced with version of `@company/provider` as `^` when the
`@company/consumer` package is published.

There's also `workspace:~` and `workspace:*` which substitutes to `~` and ``
respectively. We suggest using `:^` so that version ranges can be deduped.

### Types of packages

When sharing packages in a monorepo, there's typically 3 different kinds of packages:

#### Local only

A local only package is just that, it's only available locally to the repository and is not
published to a registry, and is not available to external repositories. For teams and companies
that utilize a single repository, this will be the most common type of package.

A benefit of local packages is that they do not require a build step, as source files can be
imported directly ([when configured correctly](#bundler-integration)). This avoids a lot of
`package.json` overhead, especially in regards to `exports`, `imports`, and other import patterns.

#### Internally published

An internal package is published to a private registry, and is not available to the public.
Published packages are far more strict than local packages, as the `package.json` structure plays a
much larger role for downstream consumers, as it dictates how files are imported, where they can be
found, what type of formats are supported (CJS, ESM), so on and so forth.

Published packages require a build step, for both source code and TypeScript types (when
applicable). We suggest using [esbuild](https://esbuild.github.io/) or
[Packemon](/docs/guides/examples/packemon) to handle this entire flow. With that being said, local projects
can still [import their source files](#bundler-integration).

#### Externally published

An external package is structured similarly to an internal package, but instead of publishing to a
private registry, it's published to the npm public registry.

External packages are primarily for open source projects, and require the repository to also be
public.

### Bundler integration

Co-locating packages is great, but how do you import and use them effectively? The easiest solution
is to configure resolver aliases within your bundler (Webpack, Vite, etc). By doing so, you enable
the following functionality:

- Avoids having to build (and rebuild) the package everytime its code changes.

- Enables file system watching of the package, not just the application.

- Allows for hot module reloading (HMR) to work.

- Package code is transpiled and bundled alongside application code.

- Vite
- Webpack

vite.config.ts

```
import path from 'path';import { defineConfig } from 'vite';export default defineConfig({  // ...  resolve: {    alias: {      '@company/utils': path.join(__dirname, '../packages/utils/src'),    },  },});
```

webpack.config.js

```
const path = require('path');module.exports = {  // ...  resolve: {    alias: {      '@company/utils': path.join(__dirname, '../packages/utils/src'),    },  },};
```

info

When configuring aliases, we suggest using the `package.json` name as the alias! This ensures that
on the consuming side, you're using the package as if it's a normal node module, and avoids
deviating from the ecosystem.

### TypeScript integration

We suggest using TypeScript project references. Luckily, we have an
[in-depth guide on how to properly and efficiently integrate them](/docs/guides/javascript/typescript-project-refs)!

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
