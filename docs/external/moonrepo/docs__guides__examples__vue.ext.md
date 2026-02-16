----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/vue
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, vue
- Summary: Vue is an application or library concern, and not a build system one, since the bundling of Vue is
----

Source: https://moonrepo.dev/docs/guides/examples/vue

# Vue example

Vue is an application or library concern, and not a build system one, since the bundling of Vue is
abstracted away through other tools. Because of this, moon has no guidelines around utilizing Vue
directly. You can use Vue however you wish!

However, with that being said, Vue is typically coupled with [Vite](https://vitejs.dev/). To
scaffold a new Vue project with Vite, run the following command in a project root.

```
npm init vue@latest
```

We highly suggest reading our documentation on [using Vite (and Vitest) with moon](/docs/guides/examples/vite) for a
more holistic view.

## Setup

This section assumes Vue is being used with Vite.

### ESLint integration

When linting with [ESLint](/docs/guides/examples/eslint) and the
[`eslint-plugin-vue`](https://eslint.vuejs.org/user-guide/#installation) library, you'll need to
include the `.vue` extension within the `lint` task. This can be done by extending the top-level
task within the project (below), or by adding it to the top-level entirely.

/moon.yml

```
tasks:  lint:    args:      - '--ext'      - '.js,.ts,.vue'
```

Furthermore, when using TypeScript within ESLint, we need to make a few additional changes to the
`.eslintrc.js` config found in the root (if the entire repo is Vue), or within the project (if only
the project is Vue).

```
module.exports = {  parser: 'vue-eslint-parser',  parserOptions: {    extraFileExtensions: ['.vue'],    parser: '@typescript-eslint/parser',    project: 'tsconfig.json', // Or another config    tsconfigRootDir: __dirname,  },};
```

### TypeScript integration

Vue does not use [TypeScript](/docs/guides/examples/typescript)'s `tsc` binary directly, but instead uses
[`vue-tsc`](https://vuejs.org/guide/typescript/overview.html), which is a thin wrapper around `tsc`
to support Vue components. Because of this, we should update the `typecheck` task in the project to
utilize this command instead.

/moon.yml

```
workspace:  inheritedTasks:    exclude: ['typecheck']tasks:  typecheck:    command:      - 'vue-tsc'      - '--noEmit'      # Always use pretty output      - '--pretty'    inputs:      - 'env.d.ts'      # Source and test files      - 'src/**/*'      - 'tests/**/*'      # Project configs      - 'tsconfig.json'      - 'tsconfig.*.json'      # Root configs (extended from only)      - '/tsconfig.options.json'
```

Be sure `tsconfig.json` compiler options are based on
[`@vue/tsconfig`](https://vuejs.org/guide/typescript/overview.html#configuring-tsconfig-json).

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
