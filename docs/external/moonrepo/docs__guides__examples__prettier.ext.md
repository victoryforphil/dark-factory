----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/prettier
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, prettier
- Summary: In this guide, you'll learn how to integrate [Prettier](https://prettier.io/) into moon.
----

Source: https://moonrepo.dev/docs/guides/examples/prettier

# Prettier example

In this guide, you'll learn how to integrate [Prettier](https://prettier.io/) into moon.

Begin by installing `prettier` in your root. We suggest using the same version across the entire
repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev prettier
```

```
yarn add --dev prettier# If using workspacesyarn add --dev -W prettier
```

```
npm install --save-dev prettier
```

```
pnpm add --save-dev prettier# If using workspacespnpm add --save-dev -w prettier
```

```
bun install --dev prettier
```

## Setup

Since code formatting is a universal workflow, add a `format` task to
[`.moon/tasks/node.yml`](/docs/config/tasks) with the following parameters.

.moon/tasks/node.yml

```
tasks:  format:    command:      - 'prettier'      # Use the same config for the entire repo      - '--config'      - '@in(4)'      # Use the same ignore patterns as well      - '--ignore-path'      - '@in(3)'      # Fail for unformatted code      - '--check'      # Run in current dir      - '.'    inputs:      # Source and test files      - 'src/**/*'      - 'tests/**/*'      # Config and other files      - '**/*.{md,mdx,yml,yaml,json}'      # Root configs, any format      - '/.prettierignore'      - '/.prettierrc.*'
```

## Configuration

### Root-level

The root-level Prettier config is required, as it defines conventions and standards to apply to
the entire repository.

.prettierrc.js

```
module.exports = {  arrowParens: 'always',  semi: true,  singleQuote: true,  tabWidth: 2,  trailingComma: 'all',  useTabs: true,};
```

The `.prettierignore` file must also be defined at the root, as
[only 1 ignore file](https://prettier.io/docs/en/ignore.html#ignoring-files-prettierignore) can
exist in a repository. We ensure this ignore file is used by passing `--ignore-path` above.

.prettierignore

```
node_modules/*.min.js*.map*.snap
```

### Project-level

We suggest against project-level configurations, as the entire repository should be formatted
using the same standards. However, if you're migrating code and need an escape hatch,
[overrides in the root](https://prettier.io/docs/en/configuration.html#configuration-overrides) will
work.

## FAQ

### How to use `--write`?

Unfortunately, this isn't currently possible, as the `prettier` binary itself requires either the
`--check` or `--write` options, and since we're configuring `--check` in the task above, that takes
precedence. This is also the preferred pattern as checks will run (and fail) in CI.

To work around this limitation, we suggest the following alternatives:

- Configure your editor to run Prettier on save.

- Define another task to write the formatted code, like `format-write`.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
