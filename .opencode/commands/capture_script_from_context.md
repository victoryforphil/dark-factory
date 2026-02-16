---
description: Convert successful conversation steps into a reusable script
agent: build
---

Use the `script-authoring` skill from `.opencode/skills/script-authoring/SKILL.md`.

Goal:

- Turn a successful sequence from this conversation into a reusable Bun script in `scripts/`.
- Include only the final working flow; do not include troubleshooting attempts.

Process:

1. Summarize candidate steps from conversation history.
2. Mark each as one of: `successful`, `failed`, `diagnostic`, or `optional`.
3. Build the script using only `successful` steps required for deterministic execution.
4. Keep `failed` and `diagnostic` steps out of executable script logic.
5. Reuse helpers from `scripts/helpers/` and follow `*.sh.ts` + shebang conventions.
6. Add short usage notes and explicit verification steps in the response.
7. For external docs scraping workflows, emit split per-page `.ext.md` files and include an `index.ext.md` file.

Return:

- Script and helper paths added/updated
- Included successful steps
- Excluded failed/diagnostic steps and why
- Verification command(s) run (or manual verification instructions)
