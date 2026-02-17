---
name: rust_lint
description: Run Rust build/test, gather diagnostics, and fix safe items with multiple parallel @developer_jr workers.
---

## What I do

- Keep the parent agent as the orchestrator for lint/build discovery and final validation.
- Parse and categorize Rust diagnostics by severity, risk, and file proximity.
- Dispatch many low-risk fixes to multiple parallel `@developer_jr` subagents.
- Batch nearby related diagnostics into one subagent task when that is safer/faster.
- Escalate risky or design-sensitive issues to the parent agent for approval.

## Workflow

1. **Parent-first discovery**: Parent agent runs `cargo build` and/or `cargo test` (and optionally clippy) and collects full diagnostics.
2. **Normalize diagnostics**: Extract `file:line`, code (`E...`, lint name), severity, and suggested fix when available.
3. **Categorize risk**:
   - **Safe/easy**: unused imports/variables, dead code cleanup, trivial doc or naming cleanup, obvious clippy suggestions, formatting.
   - **Risky**: behavior changes, API signature changes, architecture refactors, ambiguous type/lifetime fixes.
4. **Create parallel fix batches**:
   - Prefer **one `@developer_jr` task per warning/error** when items are independent.
   - If there are many nearby diagnostics in the same file/module, batch them into a single `@developer_jr` task.
   - Keep each batch low-risk and scoped (ideally 1-5 edits in one area).
5. **Dispatch in parallel**: Launch multiple `@developer_jr` subagents at once for independent safe batches.
6. **Integrate and validate**: Parent agent re-runs build/test and repeats until clean or only risky items remain.

## Parallel dispatch rules

- Run `@developer_jr` subagents in parallel whenever batches do not touch the same lines/files.
- If two batches overlap in one file, merge to one `@developer_jr` task to avoid conflicts.
- Cap each `@developer_jr` task to a tight objective with explicit file/line references.
- Parent agent owns merge conflict handling, reruns, and final pass/fail reporting.

## Fallback when routed directly to a developer

- If work lands directly on `@developer_jr` **without parent-prepared diagnostics**, first call `@explore` to gather the current Rust diagnostics and propose safe batches.
- `@developer_jr` should not guess lint state from stale context; use fresh evidence from `@explore` (or fresh build output) before editing.
- After fallback discovery, continue using the same safe-fix and parallel-batching policy.

## Safe-fix policy

Apply only low-risk fixes automatically via @developer_jr:

- Remove unused imports, variables, and dead code.
- Add `#[allow(dead_code)]` or `#[allow(unused)]` where removal would break APIs.
- Fix obvious typos in code and comments.
- Add trivial type annotations where inference is clear.
- Apply simple clippy suggestions (renames, trivial optimizations).
- Fix formatting issues via `cargo fmt`.
- Add or fix doc comments on public items where intent is clear.
- Resolve simple shadowing or naming conflicts.

Do NOT auto-apply without approval:

- Behavioral changes or logic modifications.
- API signature changes (renaming public items, changing return types).
- Large refactors across multiple modules.
- Dependency version changes.
- Config changes (Cargo.toml, rust-toolchain.toml).
- Workarounds for complex type errors or lifetime issues.

## Escalation rule

If warnings/errors require risky changes:

1. List remaining items with file:line context.
2. Mark each as "requires approval" with brief risk note.
3. Ask parent agent to decide: fix now, defer, or skip.

## Iteration loop

1. Run `cargo build 2>&1` or `cargo test 2>&1`.
2. Extract warnings/errors (use `RUSTFLAGS="-Awarnings"` to test error-only baseline).
3. Group by risk + file proximity and create independent batches.
4. Dispatch multiple `@developer_jr` tasks in parallel.
5. Re-run to verify fixes and detect newly surfaced diagnostics.
6. Repeat until clean or only risky items remain.

## Final report contract

- Build/test command run and toolchain version.
- Total warnings/errors before/after.
- Fixes applied (file, change category, subagent batch id/summary).
- Remaining issues (risky items requiring approval).
- Re-run validation status (pass/fail).
