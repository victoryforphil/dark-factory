---
description: Apply user critique, then codify durable rule updates
agent: build
---

Use this command when the user gives corrective feedback about agent behavior or output quality.

Goal:

- First fix the concrete issue in code/content for the current task.
- Then prevent repeat misses by codifying durable guidance.

Process:

1. Parse the critique into two parts:
   - immediate fix for current work
   - durable guidance for future tasks
2. Apply the immediate fix first (code, docs, scripts, or output artifacts as requested).
3. Update rule guidance for future cases:
   - add or refine `AGENTS.md` rules directly, or
   - update `.opencode/commands/rule.md` when the rule-authoring workflow should change.
4. Save a concise, reusable lesson bullet in `docs/lessons/*.lessons.md`.
5. Keep updates practical, de-duplicated, and free of one-off personal style nits unless explicitly requested.

Constraints:

- Do not log secrets or sensitive details in rules or lessons.
- Prefer small, targeted edits over broad rewrites.
- Avoid adding rules for single-use preferences that are unlikely to recur.

Return:

- What was fixed now
- What durable rule/guidance was added or updated
- Which lessons file was updated
