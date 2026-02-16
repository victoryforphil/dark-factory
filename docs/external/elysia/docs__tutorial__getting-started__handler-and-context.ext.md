----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /tutorial/getting-started/handler-and-context.md
- Keywords: elysiajs, docs, bun, typescript, tutorial, getting started, handler and context md
- Summary: url: 'https://elysiajs.com/tutorial/getting-started/handler-and-context.md'
----

Source: https://elysiajs.com/tutorial/getting-started/handler-and-context.md

---
url: 'https://elysiajs.com/tutorial/getting-started/handler-and-context.md'
---

# Handler and Context

**Handler** - a resource or a route function to send data back to client.

```ts
import { Elysia } from 'elysia'

new Elysia()
    // `() => 'hello world'` is a handler
    .get('/', () => 'hello world')
    .listen(3000)
```

A handler can also be a literal value, see Handler

```ts
import { Elysia } from 'elysia'

new Elysia()
    // `'hello world'` is a handler
    .get('/', 'hello world')
    .listen(3000)
```

Using an inline value can be useful for static resource like **file**.

## Context

Contains information about each request. It is passed as the only argument of a handler.

```typescript twoslash
import { Elysia } from 'elysia'

new Elysia()
	.get('/', (context) => context.path)
            // ^ This is a context
```

**Context** stores information about the request like:

* body - data sent by client to server like form data, JSON payload.
* query - query string as an object. (Query is extracted from a value after pathname starting from '?' question mark sign)
* params - Path parameters parsed as object
* headers - HTTP Header, additional information about the request like "Content-Type".

See Context.

## Preview

You can preview the result by looking under the **editor** section.

There should be a tiny navigator on the **top left** of the preview window.

You can use it to switch between path and method to see the response.

You can also click  to edit body, and headers.

## Assignment

Let's try extracting context parameters:

\

1. We can extract `body`, `query`, and `headers` from the first value of a callback function.
2. We can then return them like `{ body, query, headers }`.

```typescript
import { Elysia } from 'elysia'

new Elysia()
	.post('/', ({ body, query, headers }) => {
		return {
			query,
			body,
			headers
		}
	})
	.listen(3000)
```

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
