----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /tutorial/features/mount.md
- Keywords: elysiajs, docs, bun, typescript, tutorial, features, mount md
- Summary: url: 'https://elysiajs.com/tutorial/features/mount.md'
----

Source: https://elysiajs.com/tutorial/features/mount.md

---
url: 'https://elysiajs.com/tutorial/features/mount.md'
---

# Mount

Elysia provides a Elysia.mount to interlop between backend frameworks that is built on Web Standard like Hono, H3, etc.

```typescript
import { Elysia, t } from 'elysia'
import { Hono } from 'hono'

const hono = new Hono()
	.get('/', (c) => c.text('Hello from Hono')

new Elysia()
	.get('/', 'Hello from Elysia')
	.mount('/hono', hono.fetch)
	.listen(3000)
```

This allows us to gradually migrate our application to Elysia, or use multiple frameworks in a single application.

## Assignment

Let's use the preview to **GET '/hono'** to see if our Hono route is working.

Try to modify the code and see how it changes!

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
