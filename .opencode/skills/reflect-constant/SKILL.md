---
name: reflect_constant
description: Periodically run background reflector reviews and apply low-risk maintenance updates.
---

## What I do

- Start a background PTY loop that invokes `@reflector` every so often.
- Keep lessons and agent guidance fresh with small maintenance updates.
- Prioritize optimization and accuracy over broad process changes.
- Prefer using `scripts/reflect_constant.sh.ts` for launch/status/stop ergonomics.

## When to use me

Use on long or multi-agent sessions where periodic self-review is useful.

Use shorter intervals while developing reflection tooling (for example 1-5 minutes).

## Background PTY pattern

Use `pty_spawn` to run a bounded loop in the background. Each cycle should:

1. Detect latest session id (`opencode session list --max-count 1 --format json`).
2. Export session context (`opencode export <sessionID>`).
3. Invoke `@reflector` with the export attached.
4. Save each reflector result as a timestamped file in `docs/reflections/`.
5. Apply only low-risk updates:
   - `docs/lessons/*.lessons.md`
   - `AGENTS.md`
   - `.opencode/skills/*`
   - `.opencode/agents/*`
6. Sleep until next interval.

Prefer finite loops (for example 3-8 cycles) instead of infinite background daemons.

## Guardrails

- Never store secrets or sensitive transcript content in lessons/docs.
- Reflection may edit previous lessons and refine existing guidance files.
- Avoid design-direction changes and one-off human preference/style edits unless explicitly requested.
- Skip edits if no durable improvements are found.
