----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /tutorial/patterns/standalone-schema.md
- Keywords: elysiajs, docs, bun, typescript, tutorial, patterns, standalone schema md
- Summary: url: 'https://elysiajs.com/tutorial/patterns/standalone-schema.md'
----

Source: https://elysiajs.com/tutorial/patterns/standalone-schema.md

---
url: 'https://elysiajs.com/tutorial/patterns/standalone-schema.md'
---

# Standalone Schema

When we define a schema using Guard, the schema will be added to a route. But it will be **override** if the route provide a schema:

```typescript
import { Elysia, t } from 'elysia'

new Elysia()
	.guard({
		body: t.Object({
			age: t.Number()
		})
	})
	.post(
		'/user',
		({ body }) => body,
		{
			// This will override the guard schema
			body: t.Object({
				name: t.String()
			})
		}
	)
	.listen(3000)
```

If we want a schema to **co-exist** with route schema, we can define it as **standalone schema**:

```typescript
import { Elysia, t } from 'elysia'

new Elysia()
	.guard({
		schema: 'standalone', // [!code ++]
		body: t.Object({
			age: t.Number()
		})
	})
	.post(
		'/user',
		// body will have both age and name property
		({ body }) => body,
		{
			body: t.Object({
				name: t.String()
			})
		}
	)
	.listen(3000)
```

## Schema Library Interoperability

Schema between standalone schema can be from a different validation library.

For example you can define a standalone schema using **zod**, and a local schema using **Elysia.t**, and both will works interchangeably.

## Assignment

Let's make both `age` and `name` property required in the request body by using standalone schema.

\

We can define a standalone schema by adding `schema: 'standalone'` in the guard options.

```typescript
import { Elysia, t } from 'elysia'
import { z } from 'zod'

new Elysia()
	.guard({
		schema: 'standalone', // [!code ++]
		body: z.object({
			age: z.number()
		})
	})
	.post(
		'/user',
		({ body }) => body,
		{
			body: t.Object({
				name: t.String()
			})
		}
	)
	.listen(3000)
```

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
