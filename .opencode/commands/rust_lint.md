---
description: Run Rust lint/build diagnostics with parallel safe-fix subagents
agent: build
---

Use the `rust_lint` skill from `.opencode/skills/rust_lint/SKILL.md`.

Goal:

- Run Rust build/test diagnostics and clear low-risk warnings/errors quickly.
- Use multiple parallel `@developer_jr` subagents for independent safe fixes.
- Keep parent agent responsible for first-run diagnostics collection and final validation.

Process:

1. Parent agent runs diagnostics first (`cargo build 2>&1` and/or `cargo test 2>&1`).
2. Parent agent parses diagnostics, marks safe vs risky, and creates scoped batches.
3. Parent agent launches multiple parallel `@developer_jr` tasks for independent safe batches.
4. If diagnostics are dense in one file/module, batch nearby items into one `@developer_jr` task.
5. Parent agent re-runs diagnostics and repeats until clean or only risky issues remain.

Fallback:

- If execution somehow starts inside `@developer_jr` without fresh diagnostics, first use `@explore` to gather current lint/build errors and prepare safe batches before editing.

Constraints:

- No behavioral or API-signature changes without explicit approval.
- No config/dependency churn as a lint workaround.
- Do not auto-commit or push.

Return:

- Commands run + toolchain version
- Before/after warning+error counts
- Parallel batch summary (what each `@developer_jr` fixed)
- Remaining risky issues requiring approval
- Final validation status
