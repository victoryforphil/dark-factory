---
description: Safely and iteratively fix TypeScript compiler errors
agent: build
---

Use the `tsc-fix` skill from `.opencode/skills/tsc-fix/SKILL.md` and route execution through `@tsc-fixer`.

Goal:

- Autonomously fix TypeScript errors with low-risk changes first.
- Iterate until safe workspace errors are exhausted.
- Leave risky-only issues clearly documented instead of forcing workarounds.

Process:

1. Discover the best typecheck command for the target project:
   - `moon run <target-project>:typecheck`
2. Run iterative fix loop:
   - summarize errors by file/code
   - apply smallest safe change set
   - re-run typecheck
   - repeat
3. Stop conditions:
   - no workspace errors left, or
   - remaining issues are high-risk / external / require `any` approval
4. If `any` is the best remaining path, ask once before applying.

Constraints:

- Do not change `tsconfig` or similar config files as a workaround.
- Do not auto-commit or push.
- Keep an explicit ledger of any `any`/workaround usage.
- Ignore `scripts/` diagnostics by default (unless user asks to include script typing fixes).

Return:

- Selected command + TS version
- Before/after error totals
- Files changed
- Remaining blocked issues
- `any`/workaround report
