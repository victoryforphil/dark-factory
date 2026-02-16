----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/next
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, next
- Summary: In this guide, you'll learn how to integrate [Next.js](https://nextjs.org) into moon.
----

Source: https://moonrepo.dev/docs/guides/examples/next

# Next example

In this guide, you'll learn how to integrate [Next.js](https://nextjs.org) into moon.

Begin by creating a new Next.js project at a specified folder path (this should not be created in
the workspace root, unless a polyrepo).

```
cd apps && npx create-next-app
 --typescript
```

View the [official Next.js docs](https://nextjs.org/learn/basics/create-nextjs-app/setup) for a
more in-depth guide to getting started!

## Setup

Since Next.js is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Next.js tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/next).

/moon.yml

```
# Inherit tasks from the `next` preset# https://github.com/moonrepo/moon-configstags: ['next']
```

### ESLint integration

Next.js has [built-in support for ESLint](https://nextjs.org/docs/basic-features/eslint), which is
great, but complicates things a bit. Because of this, you have two options for moving forward:

- Use a [global `lint` task](/docs/guides/examples/eslint) and bypass Next.js's solution (preferred).

- Use Next.js's solution only.

Regardless of which option is chosen, the following changes are applicable to all options and should
be made. Begin be installing the
[`eslint-config-next`](https://nextjs.org/docs/basic-features/eslint#eslint-config) dependency in
the application's `package.json`.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add --dev eslint-config-next
```

```
yarn workspace
 add --dev eslint-config-next
```

```
npm install --save-dev --workspace
 eslint-config-next
```

```
pnpm add --save-dev --filter
 eslint-config-next
```

```
bun install --dev eslint-config-next
```

Since the Next.js app is located within a subfolder, we'll need to tell the ESLint plugin where to
locate it. This can be achieved with a project-level `.eslintrc.js` file.

/.eslintrc.js

```
module.exports = {  extends: 'next', // or 'next/core-web-vitals'  settings: {    next: {      rootDir: __dirname,    },  },};
```

With the basics now setup, choose the option that works best for you.

- Global lint
- Next.js lint

We encourage using the global `lint` task for consistency across all projects within the repository.
With this approach, the `eslint` command itself will be ran and the `next lint` command will be
ignored, but the `eslint-config-next` rules will still be used.

Additionally, we suggest disabling the linter during the build process, but is not a requirement. As
a potential alternative, add the `lint` task as a dependency for the `build` task.

/next.config.js

```
module.exports = {  eslint: {    ignoreDuringBuilds: true,  },};
```

If you'd prefer to use the `next lint` command, add it as a task to the project's
[`moon.yml`](/docs/config/project).

/moon.yml

```
tasks:  lint:    command: 'next lint'    inputs:      - '@group(next)'
```

Furthermore, if a global `lint` task exists, be sure to exclude it from being inherited.

/moon.yml

```
workspace:  inheritedTasks:    exclude: ['lint']
```

### TypeScript integration

Next.js also has
[built-in support for TypeScript](https://nextjs.org/docs/basic-features/typescript), but has
similar caveats to the [ESLint integration](#eslint-integration). TypeScript itself is a bit
involved, so we suggest reading the official Next.js documentation before continuing.

At this point we'll assume that a `tsconfig.json` has been created in the application, and
typechecking works. From here we suggest utilizing a [global `typecheck` task](/docs/guides/examples/typescript) for
consistency across all projects within the repository.

Additionally, we suggest disabling the typechecker during the build process, but is not a
requirement. As a potential alternative, add the `typecheck` task as a dependency for the `build`
task.

/next.config.js

```
module.exports = {  typescript: {    ignoreBuildErrors: true,  },};
```

## Configuration

### Root-level

We suggest against root-level configuration, as Next.js should be installed per-project, and the
`next` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Next.js project, a
[`next.config.`](https://nextjs.org/docs/api-reference/next.config.js/introduction) is
created, and must exist in the project root. This allows each project to configure Next.js for
their needs.

/next.config.js

```
module.exports = {  compress: true,};
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
