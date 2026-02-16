----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/jest
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, jest
- Summary: In this guide, you'll learn how to integrate [Jest](https://jestjs.io/) into moon.
----

Source: https://moonrepo.dev/docs/guides/examples/jest

# Jest example

In this guide, you'll learn how to integrate [Jest](https://jestjs.io/) into moon.

Begin by installing `jest` in your root. We suggest using the same version across the entire
repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev jest
```

```
yarn add --dev jest# If using workspacesyarn add --dev -W jest
```

```
npm install --save-dev jest
```

```
pnpm add --save-dev jest# If using workspacespnpm add --save-dev -w jest
```

```
bun install --dev jest
```

## Setup

Since testing is a universal workflow, add a `test` task to
[`.moon/tasks/node.yml`](/docs/config/tasks) with the following parameters.

.moon/tasks/node.yml

```
tasks:  test:    command:      - 'jest'      # Always run code coverage      - '--coverage'      # Dont fail if a project has no tests      - '--passWithNoTests'    inputs:      # Source and test files      - 'src/**/*'      - 'tests/**/*'      # Project configs, any format      - 'jest.config.*'
```

Projects can extend this task and provide additional parameters if need be, for example.

/moon.yml

```
tasks:  test:    args:      # Disable caching for this project      - '--no-cache'
```

## Configuration

### Root-level

A root-level Jest config is not required and should be avoided, instead, use a [preset](#sharing) to
share configuration.

### Project-level

A project-level Jest config can be utilized by creating a `jest.config.` in the
project root. This is optional, but necessary when defining project specific settings.

/jest.config.js

```
module.exports = {  // Project specific settings  testEnvironment: 'node',};
```

### Sharing

To share configuration across projects, you can utilize Jest's built-in
[`preset`](https://jestjs.io/docs/configuration#preset-string) functionality. If you're utilizing
package workspaces, create a local package with the following content, otherwise publish the npm
package for consumption.

packages/company-jest-preset/jest-preset.js

```
module.exports = {  testEnvironment: 'jsdom',  watchman: true,};
```

Within your project-level Jest config, you can extend the preset to inherit the settings.

/jest.config.js

```
module.exports = {  preset: 'company-jest-preset',};
```

You can take this a step further by passing the `--preset` option in the [task above](#setup), so
that all projects inherit the preset by default.

## FAQ

### How to test a single file or folder?

You can filter tests by passing a file name, folder name, glob, or regex pattern after `--`. Any
passed files are relative from the project's root, regardless of where the `moon` command is being
ran.

```
$ moon run
:test -- filename
```

### How to use `projects`?

With moon, there's no reason to use
[`projects`](https://jestjs.io/docs/configuration#projects-arraystring--projectconfig) as the `test`
task is ran per project. If you'd like to test multiple projects, use
[`moon run :test`](/docs/commands/run).

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
