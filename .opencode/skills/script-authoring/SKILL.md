---
name: script-authoring
description: Create Bun shebang scripts from prompts, examples, bash snippets, or conversation context.
---

## What I do

- Turn requested workflows into reusable scripts under `scripts/`.
- Normalize script implementation to project conventions (`*.sh.ts` + shebang + helpers).
- Capture successful ad-hoc command sequences as reusable script entrypoints.

## Accepted inputs

Use this skill when script requirements come from any of these sources:

- User prompt only (goal and constraints)
- Provided example script
- Provided bash script or shell command sequence
- Agent-summarized conversation context (for example, previously successful setup steps)

## Output conventions

- Primary scripts go in `scripts/` and use `*.sh.ts` naming.
- Include shebang: `#!/usr/bin/env bun`.
- Prefer shared helpers from `scripts/helpers/` instead of duplicating utility logic.
- Default to explicit, readable command flow and fail fast on command errors.
- For external docs scraping workflows, default output to split per-page `.ext.md` files and include an `index.ext.md` in the target directory.

## Implementation workflow

1. Identify script intent, inputs, and expected outputs from provided context.
2. If input comes from troubleshooting history, separate successful steps from failed or diagnostic attempts.
3. Exclude failed retries, dead-end commands, and one-off debugging probes from final reusable script logic.
4. Choose script name and location under `scripts/`.
5. Reuse or add helper utilities in `scripts/helpers/` when needed.
6. Implement Bun-based script and run it once when safe to verify behavior.
7. Update docs (`README.md` or `AGENTS.md`) when conventions or entrypoints change.

## Context capture rules

- When converting conversation history into a script, include only the final successful sequence.
- Keep troubleshooting commands as optional notes in output, not in executable script flow.
- If multiple working paths exist, prefer the shortest deterministic path.

## Notes

- Keep scripts idempotent where practical.
- Avoid embedding secrets or environment-specific absolute paths.
- If verification cannot be run safely, state exactly what should be executed by the user.
