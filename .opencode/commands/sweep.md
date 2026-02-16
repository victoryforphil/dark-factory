---
description: Alias for /sweeper cleanup workflow
agent: build
---

Alias command for `/sweeper`.

Use the same workflow defined in `.opencode/commands/sweeper.md`:

- run project-wide cleanup in `audit` (default) or `fix` mode
- enforce `STYLE.md`, `AGENTS.md`, and `docs/lessons/*.lessons.md`
- write findings to `docs/reports/sweeper_<timestamp>.report.md`
