----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/remix
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, remix
- Summary: In this guide, you'll learn how to integrate [Remix](https://remix.run) into moon.
----

Source: https://moonrepo.dev/docs/guides/examples/remix

# Remix example

In this guide, you'll learn how to integrate [Remix](https://remix.run) into moon.

Begin by creating a new Remix project at a specified folder path (this should not be created in the
workspace root, unless a polyrepo).

```
cd apps && npx create-remix
```

During this installation, Remix will ask a handful of questions, but be sure to answer "No" for the
"Do you want me to run `npm install`?" question. We suggest installing dependencies at the workspace
root via package workspaces!

View the [official Remix docs](https://remix.run/docs/en/v1) for a more in-depth guide to getting
started!

## Setup

Since Remix is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Remix tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/remix).

/moon.yml

```
# Inherit tasks from the `remix` preset# https://github.com/moonrepo/moon-configstags: ['remix']
```

### ESLint integration

Remix does not provide a built-in linting abstraction, and instead provides a simple ESLint
configuration package,
[`@remix-run/eslint-config`](https://www.npmjs.com/package/@remix-run/eslint-config). For the rest
of this section, we're going to assume that a [global `lint` task](/docs/guides/examples/eslint) has been configured.

Begin be installing the `@remix-run/eslint-config` dependency in the application's `package.json`.
We can then enable this configuration by creating an `.eslintrc.js` file in the project root. Be
sure this file is listed in your `lint` task's inputs!

/.eslintrc.js

```
module.exports = {  extends: ['@remix-run/eslint-config', '@remix-run/eslint-config/node'],  // If using TypeScript  parser: '@typescript-eslint/parser',  parserOptions: {    project: 'tsconfig.json',    tsconfigRootDir: __dirname,  },};
```

### TypeScript integration

Remix ships with TypeScript support (when enabled during installation), but the `tsconfig.json` it
generates is not setup for TypeScript project references, which we suggest using with a
[global `typecheck` task](/docs/guides/examples/typescript).

When using project references, we suggest the following `tsconfig.json`, which is a mix of Remix and
moon. Other compiler options, like `isolatedModules` and `esModuleInterop`, should be declared in a
shared configuration found in the workspace root (`tsconfig.projectOptions.json` in the example).

/tsconfig.json

```
{  "extends": "../../tsconfig.projectOptions.json",  "compilerOptions": {    "baseUrl": ".",    "emitDeclarationOnly": false,    "jsx": "react-jsx",    "resolveJsonModule": true,    "moduleResolution": "node",    "noEmit": true,    "paths": {      "~/*": ["./app/*"]    }  },  "include": [".eslintrc.js", "remix.env.d.ts", "**/*"],  "exclude": [".cache", "build", "public"]}
```

## Configuration

### Root-level

We suggest against root-level configuration, as Remix should be installed per-project, and the
`remix` command expects the configuration to live relative to the project root.

### Project-level

When creating a new Remix project, a
[`remix.config.js`](https://remix.run/docs/en/v1/api/conventions) is created, and must exist in
the project root. This allows each project to configure Remix for their needs.

/remix.config.js

```
module.exports = {  appDirectory: 'app',};
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
