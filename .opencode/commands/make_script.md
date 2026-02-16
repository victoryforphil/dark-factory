---
description: Create a new Bun shebang script from prompt/example/bash/context
agent: build
---

Use the `script-authoring` skill from `.opencode/skills/script-authoring/SKILL.md`.

Goal:

- Create a reusable script under `scripts/` based on one of these input types:
  - user prompt
  - provided example script
  - provided bash script/command sequence
  - agent-summarized successful context from the current conversation

Process:

1. Identify the source input type and extract required behavior.
2. Implement a Bun shebang script in `scripts/` using `*.sh.ts` naming.
3. Reuse or add helpers in `scripts/helpers/` when utility logic is needed.
4. If input includes troubleshooting history, filter out failed/diagnostic steps and keep only confirmed working steps.
5. Keep scripts explicit, readable, and safe (no embedded secrets).
6. If conventions or entrypoints change, update `README.md` and/or `AGENTS.md`.

Return:

- New/updated script paths
- Helper paths (if any)
- What source input was used (prompt/example/bash/context)
- Any excluded failed/diagnostic steps (if applicable)
- Verification command(s) run or exact manual verify steps
