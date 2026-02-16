----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /plugins/bearer.md
- Keywords: elysiajs, docs, bun, typescript, plugins, bearer md
- Summary: url: 'https://elysiajs.com/plugins/bearer.md'
----

Source: https://elysiajs.com/plugins/bearer.md

---
url: 'https://elysiajs.com/plugins/bearer.md'
---

# Bearer Plugin

Plugin for [elysia](https://github.com/elysiajs/elysia) for retrieving the Bearer token.

Install with:

```bash
bun add @elysiajs/bearer
```

Then use it:

```typescript twoslash
import { Elysia } from 'elysia'
import { bearer } from '@elysiajs/bearer'

const app = new Elysia()
    .use(bearer())
    .get('/sign', ({ bearer }) => bearer, {
        beforeHandle({ bearer, set, status }) {
            if (!bearer) {
                set.headers[
                    'WWW-Authenticate'
                ] = `Bearer realm='sign', error="invalid_request"`

                return status(400, 'Unauthorized')
            }
        }
    })
    .listen(3000)
```

This plugin is for retrieving a Bearer token specified in [RFC6750](https://www.rfc-editor.org/rfc/rfc6750#section-2).

This plugin DOES NOT handle authentication validation for your server. Instead, the plugin leaves the decision to developers to apply logic for handling validation check themselves.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
