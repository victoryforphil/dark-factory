----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/nuxt
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, nuxt
- Summary: In this guide, you'll learn how to integrate [Nuxt v3](https://nuxt.com), a [Vue](/docs/guides/examples/vue) framework,
----

Source: https://moonrepo.dev/docs/guides/examples/nuxt

# Nuxt example

In this guide, you'll learn how to integrate [Nuxt v3](https://nuxt.com), a [Vue](/docs/guides/examples/vue) framework,
into moon.

Begin by creating a new Nuxt project at a specified folder path (this should not be created in the
workspace root, unless a polyrepo).

```
cd apps && npx nuxi init

```

View the [official Nuxt docs](https://nuxt.com/docs/getting-started/installation) for a more
in-depth guide to getting started!

## Setup

Since Nuxt is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

/moon.yml

```
fileGroups:  nuxt:    - 'assets/**/*'    - 'components/**/*'    - 'composables/**/*'    - 'content/**/*'    - 'layouts/**/*'    - 'middleware/**/*'    - 'pages/**/*'    - 'plugins/**/*'    - 'public/**/*'    - 'server/**/*'    - 'utils/**/*'    - '.nuxtignore'    - 'app.config.*'    - 'app.vue'    - 'nuxt.config.*'tasks:  nuxt:    command: 'nuxt'    preset: 'server'  # Production build  build:    command: 'nuxt build'    inputs:      - '@group(nuxt)'    outputs:      - '.nuxt'      - '.output'  # Development server  dev:    command: 'nuxt dev'    preset: 'server'  # Preview production build locally  preview:    command: 'nuxt preview'    deps:      - '~:build'    preset: 'server'
```

Be sure to keep the `postinstall` script in your project's `package.json`.

/package.json

```
{  // ...  "scripts": {    "postinstall": "nuxt prepare"  }}
```

### ESLint integration

Refer to our [Vue documentation](/docs/guides/examples/vue#eslint-integration) for more information on linting.

### TypeScript integration

Nuxt requires `vue-tsc` for typechecking, so refer to our
[Vue documentation](/docs/guides/examples/vue#typescript-integration) for more information.

## Configuration

### Root-level

We suggest against root-level configuration, as Nuxt should be installed per-project, and the
`nuxt` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Nuxt project, a
[`nuxt.config.ts`](https://v3.nuxtjs.org/api/configuration/nuxt-config) is created, and must exist
in the project root. This allows each project to configure Next.js for their needs.

/nuxt.config.ts

```
export default defineNuxtConfig({});
```

## Testing

Nuxt supports testing through [Jest](https://jestjs.io/) or [Vitest](https://vitest.dev/). Refer to
our [Jest documentation](/docs/guides/examples/jest) or [Vitest documentation](/docs/guides/examples/vite) for more information on testing.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
