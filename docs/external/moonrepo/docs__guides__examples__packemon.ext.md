----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/packemon
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, packemon
- Summary: In this guide, you'll learn how to integrate [Packemon](https://packemon.dev/) into moon. Packemon
----

Source: https://moonrepo.dev/docs/guides/examples/packemon

# Packemon example

In this guide, you'll learn how to integrate [Packemon](https://packemon.dev/) into moon. Packemon
is a tool for properly building npm packages for distribution, it does this by providing the
following functionality:

- Compiles source code to popular formats: CJS, MJS, ESM, UMD, etc.

- Validates the `package.json` for incorrect fields or values.

- Generates `exports` mappings for `package.json` based on the define configuration.

- And many more [optimizations and features](https://packemon.dev/docs/features)!

Begin by installing `packemon` in your root. We suggest using the same version across the entire
repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev packemon
```

```
yarn add --dev packemon# If using workspacesyarn add --dev -W packemon
```

```
npm install --save-dev packemon
```

```
pnpm add --save-dev packemon# If using workspacespnpm add --save-dev -w packemon
```

```
bun install --dev packemon
```

## Setup

Since Packemon is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting Packemon tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/packemon).

/moon.yml

```
# Inherit tasks from the `packemon` preset# https://github.com/moonrepo/moon-configstags: ['packemon']# Set the output formatstasks:  build:    outputs:      - 'cjs'
```

### TypeScript integration

Packemon has built-in support for TypeScript, but to not conflict with a
[typecheck task](/docs/guides/examples/typescript), a separate `tsconfig.json` file is required, which is named
`tsconfig..json`.

This config is necessary to only compile source files, and to not include unwanted files in the
declaration output directory.

tsconfig.esm.json

```
{  "extends": "../../tsconfig.options.json",  "compilerOptions": {    "outDir": "esm",    "rootDir": "src"  },  "include": ["src/**/*"],  "references": []}
```

### Build targets

To configure the target platform(s) and format(s), you must define a
[`packemon` block](https://packemon.dev/docs/config) in the project's `package.json`. The chosen
formats must also be listed as `outputs` in the task.

package.json

```
{  "name": "package",  // ...  "packemon": {    "format": "esm",    "platform": "browser"  }}
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
