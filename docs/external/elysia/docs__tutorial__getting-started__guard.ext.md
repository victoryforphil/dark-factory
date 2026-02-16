----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /tutorial/getting-started/guard.md
- Keywords: elysiajs, docs, bun, typescript, tutorial, getting started, guard md
- Summary: url: 'https://elysiajs.com/tutorial/getting-started/guard.md'
----

Source: https://elysiajs.com/tutorial/getting-started/guard.md

---
url: 'https://elysiajs.com/tutorial/getting-started/guard.md'
---

# Guard

When you need to apply multiple hook to your application, instead of repeating hook multiple time, you can use `guard` to bulk add hooks to your application.

```typescript
import { Elysia, t } from 'elysia'

new Elysia()
	.onBeforeHandle(({ query: { name }, status }) => { // [!code --]
		if(!name) return status(401) // [!code --]
	}) // [!code --]
	.onBeforeHandle(({ query: { name } }) => { // [!code --]
		console.log(name) // [!code --]
	}) // [!code --]
	.onAfterResponse(({ responseValue }) => { // [!code --]
		console.log(responseValue) // [!code --]
	}) // [!code --]
	.guard({ // [!code ++]
		beforeHandle: [ // [!code ++]
			({ query: { name }, status }) => { // [!code ++]
				if(!name) return status(401) // [!code ++]
			}, // [!code ++]
			({ query: { name } }) => { // [!code ++]
				console.log(name) // [!code ++]
			} // [!code ++]
		], // [!code ++]
		afterResponse({ responseValue }) { // [!code ++]
			console.log(responseValue) // [!code ++]
		} // [!code ++]
	}) // [!code ++]
	.get(
		'/auth',
		({ query: { name = 'anon' } }) => `Hello ${name}!`,
		{
			query: t.Object({
				name: t.String()
			})
		}
	)
	.get(
		'/profile',
		({ query: { name = 'anon' } }) => `Hello ${name}!`,
		{
			query: t.Object({
				name: t.String()
			})
		}
	)
	.listen(3000)
```

Not only that, you can also apply **schema** to multiple routes using `guard`.

```typescript
import { Elysia, t } from 'elysia'

new Elysia()
	.guard({
		beforeHandle: [
			({ query: { name }, status }) => {
				if(!name) return status(401)
			},
			({ query: { name } }) => {
				console.log(name)
			}
		],
		afterResponse({ responseValue }) {
			console.log(responseValue)
		},
		query: t.Object({ // [!code ++]
			name: t.String() // [!code ++]
		}) // [!code ++]
	})
	.get(
		'/auth',
		({ query: { name = 'anon' } }) => `Hello ${name}!`,
		{ // [!code --]
			query: t.Object({ // [!code --]
				name: t.String() // [!code --]
			}) // [!code --]
		} // [!code --]
	)
	.get(
		'/profile',
		({ query: { name = 'anon' } }) => `Hello ${name}!`,
		{ // [!code --]
			query: t.Object({ // [!code --]
				name: t.String() // [!code --]
			}) // [!code --]
		} // [!code --]
	)
	.listen(3000)
```

This will apply hooks and schema to every routes **after .guard** is called in the same instance.

See Guard for more information.

## Assignment

Let's put 2 types of hooks into practice.

\

We can use `beforeHandle` to intercept the request before it reaches the handler, and return a response with `status` method.

```typescript
import { Elysia } from 'elysia'

new Elysia()
	.onBeforeHandle(({ query: { name }, status }) => {
		if(!name) return status(401)
	})
	.get('/auth', ({ query: { name = 'anon' } }) => {
		return `Hello ${name}!`
	})
	.get('/profile', ({ query: { name = 'anon' } }) => {
		return `Hello ${name}!`
	})
	.listen(3000)
```

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
