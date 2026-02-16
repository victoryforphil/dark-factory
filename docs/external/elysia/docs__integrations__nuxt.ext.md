----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /integrations/nuxt.md
- Keywords: elysiajs, docs, bun, typescript, integrations, nuxt md
- Summary: url: 'https://elysiajs.com/integrations/nuxt.md'
----

Source: https://elysiajs.com/integrations/nuxt.md

---
url: 'https://elysiajs.com/integrations/nuxt.md'
---

# Integration with Nuxt

We can use [nuxt-elysia](https://github.com/tkesgar/nuxt-elysia), a community plugin for Nuxt, to setup Elysia on Nuxt API route with Eden Treaty.

1. Install the plugin with the following command:

```bash
bun add elysia @elysiajs/eden
bun add -d nuxt-elysia
```

2. Add `nuxt-elysia` to your Nuxt config:

```ts
export default defineNuxtConfig({
    modules: [ // [!code ++]
        'nuxt-elysia' // [!code ++]
    ], // [!code ++]
    nitro: { // [!code ++]
        preset: 'Bun' // [!code ++]
    } // [!code ++]
})
```

::: tip
The `nitro.preset: 'Bun'` configuration is required because Elysia runs on Bun runtime. This tells Nuxt's Nitro to use Bun as the server runtime instead of the default Node.js runtime.
:::

3. Create `api.ts` in the project root:

```typescript [api.ts]
export default () => new Elysia() // [!code ++]
  .get('/hello', () => ({ message: 'Hello world!' })) // [!code ++]
```

4. Use Eden Treaty in your Nuxt app:

```vue

{{ data.message }}

```

This will automatically setup Elysia to run on Nuxt API route automatically.

### pnpm

If you use pnpm, [pnpm doesn't auto install peer dependencies by default](https://github.com/orgs/pnpm/discussions/3995#discussioncomment-1893230) forcing you to install additional dependencies manually.

```bash
pnpm add @sinclair/typebox openapi-types
```

## Prefix

By default, Elysia will be mounted on **/\_api** but we can customize it with `nuxt-elysia` config.

```ts
export default defineNuxtConfig({
	nuxtElysia: {
		path: '/api' // [!code ++]
	}
})
```

This will mount Elysia on **/api** instead of **/\_api**.

For more configuration, please refer to [nuxt-elysia](https://github.com/tkesgar/nuxt-elysia)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
