----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /tutorial/features/end-to-end-type-safety.md
- Keywords: elysiajs, docs, bun, typescript, tutorial, features, end to end type safety md
- Summary: url: 'https://elysiajs.com/tutorial/features/end-to-end-type-safety.md'
----

Source: https://elysiajs.com/tutorial/features/end-to-end-type-safety.md

---
url: 'https://elysiajs.com/tutorial/features/end-to-end-type-safety.md'
---

# End-to-End Type Safety

Elysia provides an end-to-end type safety between backend and frontend **without code generation** similar to tRPC, using Eden.

```typescript
import { Elysia } from 'elysia'
import { treaty } from '@elysiajs/eden'

// Backend
export const app = new Elysia()
	.get('/', 'Hello Elysia!')
	.listen(3000)

// Frontend
const client = treaty('localhost:3000')

const { data, error } = await client.get()

console.log(data) // Hello World
```

This works by inferring the types from the Elysia instance, and use type hints to provide type safety for the client.

See Eden Treaty.

## Assignment

Let's tab the  icon in the preview to see how's the request is logged.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
