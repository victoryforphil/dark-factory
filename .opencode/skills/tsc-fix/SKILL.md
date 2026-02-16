---
name: tsc-fix
description: Safely iterate through TypeScript compiler errors with low-risk fixes first.
---

## What I do

- Standardize TypeScript error discovery for Bun/TypeScript projects.
- Run safe, iterative fixes first and re-check after each batch.
- Escalate only when changes become risky (functional behavior, large refactors, config workarounds).

## Preferred discovery workflow

1. Run `moon run <project>:typecheck`.
2. Keep `:check` for full hygiene runs, but iterate on `:typecheck` for fast error-fix loops.
3. Parse and group diagnostics by:
    - workspace file
    - external dependency file
    - TypeScript code (`TSxxxx`)

## Safe-fix policy

Apply only low-risk fixes automatically:

- Add explicit local type annotations when intent is clear.
- Correct obvious typos and symbol/import mismatches.
- Add narrow guards for `null`/`undefined` checks.
- Tighten object literal typing where schema/library APIs require explicit shape.
- Replace incorrect implicit assumptions with typed helpers.

By default, ignore `scripts/` diagnostics unless user explicitly asks to include script typing work.
Treat script files as shell-style automation where readability and reliability matter more than strict typing.

Do not auto-apply higher-risk changes unless user asked:

- broad behavioral rewrites
- large architecture refactors
- tsconfig or compiler-option workaround changes
- dependency major upgrades as a shortcut

## `any` escalation rule

- Avoid `any` by default.
- If remaining errors are blocked and best next step is `any` or broad cast workaround, ask once with a targeted question.
- If user approves, keep `any` scope minimal and track each use in the final report.

## Iteration loop

1. Run typecheck and summarize top error groups.
2. Pick the lowest-risk actionable group in workspace files.
3. Apply focused fixes.
4. Re-run typecheck.
5. Repeat until:
   - no workspace errors remain, or
   - only risky/non-safe fixes remain.

## Final report contract

- Selected typecheck command and compiler version.
- Total errors before/after.
- Files changed and safe fix categories applied.
- Remaining blocked errors (if any) and why.
- `any`/workaround ledger (explicit list, or `none`).
