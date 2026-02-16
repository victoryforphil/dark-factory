----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /tutorial/features/unit-test.md
- Keywords: elysiajs, docs, bun, typescript, tutorial, features, unit test md
- Summary: url: 'https://elysiajs.com/tutorial/features/unit-test.md'
----

Source: https://elysiajs.com/tutorial/features/unit-test.md

---
url: 'https://elysiajs.com/tutorial/features/unit-test.md'
---

# Unit Test

Elysia provides a **Elysia.fetch** function to easily test your application.

**Elysia.fetch** takes a Web Standard Request, and returns a Response similar to the browser's fetch API.

```typescript
import { Elysia } from 'elysia'

const app = new Elysia()
	.get('/', 'Hello World')

app.fetch(new Request('http://localhost/'))
	.then((res) => res.text())
	.then(console.log)
```

This will run a request like an **actual request** (not simulated).

### Test

This allows us to easily test our application without running a server.

::: code-group

```typescript [Bun Test]
import { describe, it, expect } from 'bun:test'

import { Elysia } from 'elysia'

describe('Elysia', () => {
	it('should return Hello World', async () => {
		const app = new Elysia().get('/', 'Hello World')

		const text = await app.fetch(new Request('http://localhost/'))
			.then(res => res.text())

		expect(text).toBe('Hello World')
	})
})
```

```typescript [Vitest]
import { describe, it, expect } from 'vitest'

import { Elysia } from 'elysia'

describe('Elysia', () => {
	it('should return Hello World', async () => {
		const app = new Elysia().get('/', 'Hello World')

		const text = await app.fetch(new Request('http://localhost/'))
			.then(res => res.text())

		expect(text).toBe('Hello World')
	})
})
```

```typescript [Jest]
import { describe, it, test } from '@jest/globals'

import { Elysia } from 'elysia'

describe('Elysia', () => {
	test('should return Hello World', async () => {
		const app = new Elysia().get('/', 'Hello World')

		const text = await app.fetch(new Request('http://localhost/'))
			.then(res => res.text())

		expect(text).toBe('Hello World')
	})
})
```

:::

See Unit Test.

## Assignment

Let's tab the  icon in the preview to see how's the request is logged.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
