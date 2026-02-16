----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/astro
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, astro
- Summary: In this guide, you'll learn how to integrate [Astro](https://docs.astro.build).
----

Source: https://moonrepo.dev/docs/guides/examples/astro

# Astro example

In this guide, you'll learn how to integrate [Astro](https://docs.astro.build).

Begin by creating a new Astro project in the root of an existing moon project (this should not be
created in the workspace root, unless a polyrepo).

```
cd apps && npm create astro@latest
```

## Setup

Since Astro is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Astro tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/astro).

/moon.yml

```
# Inherit tasks from the `astro` preset# https://github.com/moonrepo/moon-configstags: ['astro']# Disable project referencestoolchain:  typescript:    syncProjectReferences: false
```

### ESLint integration

When using a [`lint`](/docs/guides/examples/eslint) task, the
[`eslint-plugin-astro`](https://ota-meshi.github.io/eslint-plugin-astro/user-guide/) package must be
installed to lint `.astro` files.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace  add --dev eslint-plugin-astro
```

```
yarn workspace  add --dev eslint-plugin-astro
```

```
npm install --save-dev --workspace  eslint-plugin-astro
```

```
pnpm add --save-dev --filter  eslint-plugin-astro
```

```
bun install --dev eslint-plugin-astro
```

Once the dependency has been installed in the application's `package.json`. We can then enable this
configuration by creating an `.eslintrc.js` file in the project root. Be sure this file is listed in
your lint task's inputs!

/.eslintrc.js

```
module.exports = {  extends: ['plugin:astro/recommended'],  overrides: [    {      files: ['*.astro'],      parser: 'astro-eslint-parser',      // If using TypeScript      parserOptions: {        parser: '@typescript-eslint/parser',        extraFileExtensions: ['.astro'],        project: 'tsconfig.json',        tsconfigRootDir: __dirname,      },    },  ],};
```

And lastly, when linting through moon's command line, you'll need to include the `.astro` extension
within the `lint` task. This can be done by extending the top-level task within the project (below),
or by adding it to the top-level entirely.

/moon.yml

```
tasks:  lint:    args:      - '--ext'      - '.ts,.tsx,.astro'
```

### Prettier integration

When using a [`format`](/docs/guides/examples/prettier) task, the `prettier-plugin-astro` package must be installed to
format `.astro` files. View the official
[Astro docs](https://docs.astro.build/en/editor-setup/#prettier) for more information.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace  add --dev prettier-plugin-astro
```

```
yarn workspace  add --dev prettier-plugin-astro
```

```
npm install --save-dev --workspace  prettier-plugin-astro
```

```
pnpm add --save-dev --filter  prettier-plugin-astro
```

```
bun install --dev prettier-plugin-astro
```

### TypeScript integration

Since Astro utilizes custom `.astro` files, it requires a specialized TypeScript integration, and
luckily Astro provides an [in-depth guide](https://docs.astro.build/en/guides/typescript/). With
that being said, we do have a few requirements and pointers!

- Use the official [Astro `tsconfig.json`](https://docs.astro.build/en/guides/typescript/#setup) as a basis.

- From our internal testing, the `astro check` command (that typechecks `.astro` files) does not support project references. If the `composite` compiler option is enabled, the checker will fail to find `.astro` files. To work around this, we disable `workspace.typescript` in our moon config above.

- Since typechecking requires 2 commands, one for `.astro` files, and the other for `.ts`, `.tsx` files, we've added the [`typecheck`](/docs/guides/examples/typescript) task as a dependency for the `check` task. This will run both commands through a single task!

## Configuration

### Root-level

We suggest against root-level configuration, as Astro should be installed per-project, and the
`astro` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Astro project, a
[`astro.config.mjs`](https://docs.astro.build/en/reference/configuration-reference/) is created, and
must exist in the project root. This allows each project to configure Astro for their needs.

/astro.config.mjs

```
import { defineConfig } from 'astro/config';// https://astro.build/configexport default defineConfig({});
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
