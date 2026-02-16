---
description: Capture quick post-task lessons for future agents
agent: build
---

Use the `reflect` skill from `.opencode/skills/reflect/SKILL.md`.

Goal:

- Add a small number of durable lessons to `docs/lessons/*.lessons.md` files.
- Keep reflection fast and lightweight.

Modes:

- `quick` (default): reflect directly from the current task context.
- `deep`: run transcript/session-aware reflection via `@reflector`.

Quick process:

1. Review the just-completed task.
2. Identify up to 3 lessons that are generic and reusable.
3. Include relevant behavior/rule learnings from `.opencode/commands/rule.md`.
4. Add concise bullet points to a relevant `docs/lessons/*.lessons.md` file.
5. Keep each lessons file tidy, de-duplicated, and under 100 lines.
6. If needed, apply maintenance-only refinements to `AGENTS.md`, `.opencode/skills/*`, or `.opencode/agents/*` to keep guidance current.

Deep process (`@reflector`):

1. Gather input from one of:
   - a short parent-agent task summary
   - pasted transcript snippets
   - an exported session (`opencode export <sessionID>`)
2. Route analysis to `@reflector` and request: wins, misses, and 0-3 durable lesson bullets.
3. Save a timestamped reflection artifact to `docs/reflections/`.
4. Store only reusable lesson bullets in a relevant `docs/lessons/*.lessons.md` file.
5. Skip lessons storage if `@reflector` reports no durable lessons.

Constraints:

- Do not add secrets or sensitive details.
- Skip adding lessons when nothing durable was learned.
- Do not over-structure the file; bullets only.
- Avoid design-direction edits and one-off human preference/style changes unless explicitly requested.
