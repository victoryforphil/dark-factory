----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /eden/test.md
- Keywords: elysiajs, docs, bun, typescript, eden, test md
- Summary: url: 'https://elysiajs.com/eden/test.md'
----

Source: https://elysiajs.com/eden/test.md

---
url: 'https://elysiajs.com/eden/test.md'
---

# Eden Test

Using Eden, we can create an integration test with end-to-end type safety and auto-completion.

## Setup

We can use [Bun test](https://bun.sh/guides/test/watch-mode) to create tests.

Create **test/index.test.ts** in the root of project directory with the following:

```typescript
// test/index.test.ts
import { describe, expect, it } from 'bun:test'

import { edenTreaty } from '@elysiajs/eden'

const app = new Elysia()
    .get('/', () => 'hi')
    .listen(3000)

const api = edenTreaty('http://localhost:3000') describe('Elysia', () => { it('return a response', async () => { const { data } = await api.get() expect(data).toBe('hi') }) }) ``` Then we can perform tests by running **bun test** ```bash bun test ``` This allows us to perform integration tests programmatically instead of manual fetch while supporting type checking automatically.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
