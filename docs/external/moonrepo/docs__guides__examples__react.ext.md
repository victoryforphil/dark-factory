----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/react
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, react
- Summary: React is an application or library concern, and not a build system one, since the bundling of React
----

Source: https://moonrepo.dev/docs/guides/examples/react

# React example

React is an application or library concern, and not a build system one, since the bundling of React
is abstracted away through another tool like webpack. Because of this, moon has no guidelines around
utilizing React directly. You can use React however you wish!

However, with that being said, we do suggest the following:

- Add `react` and related dependencies to each project, not the root. This includes `@types/react` as well. This will ensure accurate [hashing](/docs/concepts/cache#hashing).

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add react
```

```
yarn workspace
 add react
```

```
npm install --workspace
 react
```

```
pnpm add --filter
 react
```

```
bun install react
```

- Configure Babel with the `@babel/preset-react` preset.

- Configure [TypeScript](/docs/guides/examples/typescript) compiler options with `"jsx": "react-jsx"`.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
