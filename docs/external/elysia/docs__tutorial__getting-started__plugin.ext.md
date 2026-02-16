----
## External Docs Snapshot // elysia

- Captured: 2026-02-16T05:53:41.821Z
- Source root: https://elysiajs.com/
- Source page: /tutorial/getting-started/plugin.md
- Keywords: elysiajs, docs, bun, typescript, tutorial, getting started, plugin md
- Summary: url: 'https://elysiajs.com/tutorial/getting-started/plugin.md'
----

Source: https://elysiajs.com/tutorial/getting-started/plugin.md

---
url: 'https://elysiajs.com/tutorial/getting-started/plugin.md'
---

# Plugin

Every Elysia instance can be plug-and-play with other instances by `use` method.

```typescript
import { Elysia } from 'elysia'

const user = new Elysia()
	.get('/profile', 'User Profile')
	.get('/settings', 'User Settings')

new Elysia()
	.use(user) // [!code ++]
	.get('/', 'Home')
	.listen(3000)
```

Once applied, all routes from `user` instance will be available in `app` instance.

### Plugin Config

You can also create a plugin that takes argument, and returns an Elysia instance to make a more dynamic plugin.

```typescript
import { Elysia } from 'elysia'

const user = ({ log = false }) => new Elysia() // [!code ++]
	.onBeforeHandle(({ request }) => {
		if (log) console.log(request)
	})
	.get('/profile', 'User Profile')
	.get('/settings', 'User Settings')

new Elysia()
	.use(user({ log: true })) // [!code ++]
	.get('/', 'Home')
	.listen(3000)
```

It's also recommended that you should also read about [Key Concept: Dependency](/key-concept#dependency) to understand how Elysia handles dependencies between plugins.

## Assignment

Let's apply the `user` instance to the `app` instance.

\

Similar to the above example, we can use the `use` method to plug the `user` instance into the `app` instance.

```typescript
import { Elysia } from 'elysia'

const user = new Elysia()
	.get('/profile', 'User Profile')
	.get('/settings', 'User Settings')

const app = new Elysia()
	.use(user) // [!code ++]
	.get('/', 'Home')
	.listen(3000)
```

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with llms fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
