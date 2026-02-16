----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/vite
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, vite
- Summary: In this guide, you'll learn how to integrate [Vite](https://vitejs.dev/) and
----

Source: https://moonrepo.dev/docs/guides/examples/vite

# Vite & Vitest example

In this guide, you'll learn how to integrate [Vite](https://vitejs.dev/) and
[Vitest](https://vitest.dev/) into moon.

Begin by creating a new Vite project in the root of an existing moon project (this should not be
created in the workspace root, unless a polyrepo).

- Yarn
- Yarn (classic)
- npm
- pnpm

```
yarn create vite
```

```
yarn create vite
```

```
npm create vite
```

```
pnpm create vite
```

If you plan on using Vitest, run the following command to add the `vitest` dependency to a project,
otherwise skip to the setup section.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add --dev vitest
```

```
yarn workspace
 add --dev vitest
```

```
npm install --save-dev --workspace
 vitest
```

```
pnpm add --save-dev --filter
 vitest
```

```
bun install --dev vitest
```

## Setup

Since Vite is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Vite tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/vite).

/moon.yml

```
# Inherit tasks from the `vite` and `vitest` presets# https://github.com/moonrepo/moon-configstags: ['vite', 'vitest']
```

## Configuration

### Root-level

We suggest against root-level configuration, as Vite should be installed per-project, and the
`vite` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Vite project, a [`vite.config.`](https://vitejs.dev/config) is created,
and must exist in the project root.

/vite.config.js

```
import { defineConfig } from 'vite';export default defineConfig({  // ...  build: {    // These must be `outputs` in the `build` task    outDir: 'dist',  },  test: {    // Vitest settings  },});
```

If you'd prefer to configure Vitest in a
[separate configuration file](https://vitest.dev/guide/#configuring-vitest), create a
`vitest.config.` file.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
