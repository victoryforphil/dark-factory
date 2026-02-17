---
description: Plan interactively, then execute with parallel senior/junior build tracks
agent: build
---

Use the `builder` skill from `.opencode/skills/builder/SKILL.md`.

Primary flow has two phases.

## Phase 1: Plan

Start with `@planner` to collect and shape work before coding.

Goals:

- Gather project prompts, feedback items, and constraints.
- Ask targeted follow-up questions until scope is actionable.
- Keep planning interactive for fast-moving, feedback-heavy sessions.

Planner loop:

1. Ask one high-impact clarifying question at a time.
2. Capture answers into a prioritized task list.
3. Re-check for missing constraints (scope, risk, validation, deadlines).
4. Continue until either:
   - user says planning is complete, or
   - requirements are sufficiently concrete to start implementation.

## Phase 2: Build

Switch to execution using parallel tracks when possible.

Routing:

- `@developer_senior`: free-form, tricky, logic-sensitive work.
- `@developer_jr`: simpler, bounded, low-risk cleanup/refactor tasks.

Execution rules:

1. Split independent items into parallel tracks.
2. Keep parent agent as orchestrator and integration owner.
3. Merge outputs, run focused verification, and report progress.
4. If new feedback arrives mid-build, optionally return to Phase 1 briefly and re-split.

Return:

- Planning summary (goals, assumptions, priorities).
- Senior/Jr task split and status.
- Changes made, validations run, and remaining follow-ups.
