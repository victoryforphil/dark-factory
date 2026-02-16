---
description: Run project-wide cleanup audit and optional low-risk fixes
agent: build
---

Route execution through `@sweeper`.

Goal:

- Keep repository quality high with repeatable cleanup cycles.
- Enforce `STYLE.md`, `AGENTS.md`, and `docs/lessons/*.lessons.md` guidance.
- Detect dead code, stale docs/READMEs, and process drift.
- Produce a focused report under `docs/reports/*.report.md`.

Modes:

- `audit` (default): analyze and write report; no risky edits.
- `fix`: apply low-risk, high-confidence cleanup edits and write report.

Depth:

- `quick` (default): high-signal checks over core files and modules.
- `deep`: broader scan across docs, scripts, frontends, and config.

Process:

1. Build project intent context from:
   - `README.md`
   - `AGENTS.md`
   - `STYLE.md`
   - `docs/lessons/*.lessons.md`
   - component READMEs under `dark_core/`, `frontends/`, and `docker/`
2. Run hygiene checks:
   - style and rule conformance
   - dead code and unused exports/imports/files (high confidence only)
   - stale docs and README drift (commands, paths, endpoints, workflows)
   - DRY violations via duplicated constants/logic/config
   - stale or contradictory guidance in `.opencode/commands/*` and `.opencode/agents/*`
3. In `fix` mode, apply only low-risk cleanup:
   - remove clearly unused imports/locals
   - fix obvious docs drift and broken internal references
   - de-duplicate repeated bullets in guidance docs
4. Save report to `docs/reports/sweeper_<YYYY-MM-DD_HHMM>.report.md`.
5. Include prioritized follow-up actions for deferred items.

Constraints:

- Evidence-backed findings only; always reference concrete file paths.
- No destructive commands, broad rewrites, or speculative refactors.
- Do not auto-commit or push.

Return:

- Report path
- Findings counts by severity/category
- Auto-fixes applied (if any)
- Top follow-up actions
