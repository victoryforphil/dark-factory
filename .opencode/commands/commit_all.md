---
description: Create clean, grouped commits for all current changes
agent: build
---

Use the `gitter-commit` skill from `.opencode/skills/gitter-commit/SKILL.md` and route commit execution through `@gitter`.

Goal:

- Turn the current working tree into a clean set of meaningful commits (not a single `git add -A` dump)
- End with a clean `git status`

Process:

1. Inspect the repository state (`git status`, staged/unstaged diffs, and recent commit titles for style).
2. Group changes into logical commit units (by feature, fix, docs, config, refactor, etc.).
3. For each commit unit:
   - Stage only the relevant files.
   - Write a commit title using this repo format:
     - `{Component/Meta} // {Optional Addition} // {Short Description} (Optional,Tags)`
   - Add a short commit body rationale and signature when possible.
4. Repeat until all intended tracked changes are committed.
5. Confirm final `git status` is clean.

Rules:

- Prefer multiple small meaningful commits over one large mixed commit.
- Do not commit likely secret files (`.env`, credentials, private keys).
- Do not push.
- If no commit-worthy changes exist, report that clearly.

Return:

- Commit list in order (hash + title + files)
- Any skipped files and why
- Final `git status` summary
