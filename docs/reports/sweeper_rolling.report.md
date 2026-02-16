----
---
entry_id: 2026-02-16_0327_dtui-sys-install-execbits
category: process
severity: high
risk: small-adjustment
effort: S
location: scripts/dtui.sh.ts:1
---
offense snippet:
"#!/usr/bin/env bun" (shebang present) but file mode is non-executable; `./scripts/dtui.sh.ts --help` returns `permission denied`.

why it matters:
Documented direct invocation in `frontends/dark_tui/README.md:71` is currently broken. This creates immediate setup friction and violates the repo's runnable script convention.

suggested fix(es):
- Restore executable bit on runnable shebang files: `scripts/dtui.sh.ts`, `scripts/sys_install.sh.ts`.
- Re-run direct command checks used in docs.

notes/constraints:
- Keep helper-only modules (non-entrypoint utilities) non-executable.
- No behavior change required; permission-only correction.

----
---
entry_id: 2026-02-16_0327_bun-env-drift
category: style
severity: medium
risk: small-adjustment
effort: M
location: dark_core/src/modules/prisma/prisma.client.ts:49
---
offense snippet:
`env: { ...process.env, RUST_LOG: 'info' }`

why it matters:
`STYLE.md` sets Bun-first runtime conventions (`Bun.env`). Mixed env APIs in Bun-first paths make runtime behavior less predictable and can reduce confidence in documented standards.

suggested fix(es):
- Prefer `Bun.env` where runtime/tooling compatibility permits.
- If `process.env` is required by dependency contracts, add a short rationale note at the call site.

notes/constraints:
- `prisma.config.ts` may require special handling depending on Prisma loader behavior.

----
---
entry_id: 2026-02-16_0327_prisma-db-push-dup
category: dry
severity: medium
risk: bigger-design-change
effort: M
location: dark_core/src/modules/prisma/prisma.client.ts:43
---
offense snippet:
Two separate `runPrismaDbPush` implementations exist in app and test helpers.

why it matters:
Operational duplication around schema push can drift over time (spawn args, env handling, logging/error surfaces), creating inconsistent startup vs test behavior.

suggested fix(es):
- Extract one shared helper for Prisma db push process execution.
- Keep ownership clear (core module helper vs test helper wrapper).

notes/constraints:
- Avoid over-abstraction; helper should stay narrow and command-specific.

----
---
entry_id: 2026-02-16_0327_readme-moon-list-drift
category: docs
severity: low
risk: small-adjustment
effort: S
location: README.md:65
---
offense snippet:
Moon config list omits `frontends/dark_tui/moon.yml`.

why it matters:
Hardcoded infra inventories become stale quickly and create contradictory guidance versus canonical docs (`AGENTS.md`).

suggested fix(es):
- Add missing `frontends/dark_tui/moon.yml` entry in `README.md`.
- Optionally replace exhaustive list with a pointer to canonical source-of-truth doc.

notes/constraints:
- Keep README current-state oriented; avoid volatile exhaustive lists unless actively maintained.
