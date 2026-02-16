----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /eden/treaty/unit-test.md
- Keywords: elysiajs, docs, bun, typescript, eden, treaty, unit test md
- Summary: url: 'https://elysiajs.com/eden/treaty/unit-test.md'
----

Source: https://elysiajs.com/eden/treaty/unit-test.md

---
url: 'https://elysiajs.com/eden/treaty/unit-test.md'
---

# Unit Test

According to [Eden Treaty config](/eden/treaty/config.html#urlorinstance) and [Unit Test](/patterns/unit-test), we may pass an Elysia instance to Eden Treaty directly to interact with Elysia server directly without sending a network request.

We may use this pattern to create a unit test with end-to-end type safety and type-level test all at once.

```typescript twoslash
// test/index.test.ts
import { describe, expect, it } from 'bun:test'
import { Elysia } from 'elysia'
import { treaty } from '@elysiajs/eden'

const app = new Elysia().get('/hello', 'hi')
const api = treaty(app)

describe('Elysia', () => {
    it('returns a response', async () => {
        const { data } = await api.hello.get()

        expect(data).toBe('hi')
              // ^?

    })
})
```

## Type safety test

To perform a type safety test, simply run **tsc** on test folders.

```bash
tsc --noEmit test/**/*.ts
```

This is useful to ensure type integrity for both client and server, especially during migrations.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
