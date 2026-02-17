---
name: builder
description: Split larger or multi-feedback tasks across parallel implementation subagents.
---

## What I do

- Encourage divide-and-conquer execution for large tasks and feedback-heavy sessions.
- Route tricky, logic-heavy work to `@developer_senior`.
- Route straightforward cleanup/refactor work to `@developer_jr`.
- Help parent agents run both in parallel when workstreams are independent.

## When to use me

Use this skill when:

- The user adds multiple follow-up items while a task is in progress.
- The task naturally splits into independent tracks.
- A mixed workload has both complex reasoning and simple mechanical edits.

## Delegation policy

- `@developer_senior`: free-form, ambiguous, cross-cutting, logic-sensitive tasks.
- `@developer_jr`: simpler, bounded, low-risk implementation tasks (cleanup, light refactor, script polish).
- Parent agent remains orchestrator and owns final integration decisions.

## Parallel handoff template

- Goal: <overall user outcome>
- Track A (`@developer_senior`): <complex/problem-solving portion>
- Track B (`@developer_jr`): <simple/mechanical portion>
- Constraints: <tests, safety, file scope, deadlines>
- Return format: <what each subagent should report back>
