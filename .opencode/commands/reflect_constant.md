---
description: Run periodic background reflection using the reflector subagent
agent: build
---

Use the `reflect_constant` skill from `.opencode/skills/reflect-constant/SKILL.md` and route review through `@reflector`.

Goal:

- Periodically review recent session context in the background.
- Keep lessons and agent context docs continuously improved with low-risk edits.
- Provide a single helper command for launch/status/stop workflow.

Helper script:

- `scripts/reflect_constant.sh.ts start --interval-minutes 2 --cycles 3`
- `scripts/reflect_constant.sh.ts status`
- `scripts/reflect_constant.sh.ts stop`

Process:

1. Start a background PTY loop with a short interval while testing (default 5 minutes).
2. For each cycle:
   - get latest session id with `opencode session list --max-count 1 --format json`
   - export context with `opencode export <sessionID>`
   - invoke `@reflector` with focused prompt for wins/misses/lessons/maintenance updates
   - write a timestamped reflection artifact under `docs/reflections/`
3. Apply only maintenance-safe edits to:
   - `docs/lessons/*.lessons.md`
   - `AGENTS.md`
   - `.opencode/skills/*`
   - `.opencode/agents/*`
4. Stop after a finite number of cycles unless explicitly asked to run continuously.

Constraints:

- No secrets or sensitive transcript content in saved docs.
- Reflection can modify existing lessons and existing context docs.
- Avoid design-direction changes and one-off human preference/style edits unless explicitly requested.
- Skip edits when no durable improvement is found.
