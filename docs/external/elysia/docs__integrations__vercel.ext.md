----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /integrations/vercel.md
- Keywords: elysiajs, docs, bun, typescript, integrations, vercel md
- Summary: url: 'https://elysiajs.com/integrations/vercel.md'
----

Source: https://elysiajs.com/integrations/vercel.md

---
url: 'https://elysiajs.com/integrations/vercel.md'
---

# Deploy Elysia on Vercel

Elysia can deploys on Vercel with zero configuration using either Bun or Node runtime.

1. In **src/index.ts**, create or import an existing Elysia server
2. Export the Elysia server as default export

```typescript
import { Elysia, t } from 'elysia'

export default new Elysia() // [!code ++]
    .get('/', () => 'Hello Vercel Function')
    .post('/', ({ body }) => body, {
        body: t.Object({
            name: t.String()
        })
    })
```

3. Develop locally with Vercel CLI

```bash
vc dev
```

4. Deploy to Vercel

```bash
vc deploy
```

That's it. Your Elysia app is now running on Vercel.

### pnpm

If you use pnpm, [pnpm doesn't auto install peer dependencies by default](https://github.com/orgs/pnpm/discussions/3995#discussioncomment-1893230) forcing you to install additional dependencies manually.

```bash
pnpm add @sinclair/typebox openapi-types
```

### Using Node.js

To deploy with Node.js, make sure to set `type: module` in your `package.json`

::: code-group

```ts [package.json]
{
  "name": "elysia-app",
  "type": "module" // [!code ++]
}
```

:::

### Using Bun

To deploy with Bun, make sure to set the runtime to Bun in your `vercel.json`

::: code-group

```ts [vercel.json]
{
  "$schema": "https://openapi.vercel.sh/vercel.json",
  "bunVersion": "1.x" // [!code ++]
}
```

## If this doesn't work

Vercel has zero configuration for Elysia, for additional configuration, please refers to [Vercel documentation](https://vercel.com/docs/frameworks/backend/elysia)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
