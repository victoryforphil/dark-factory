---
name: gitter-commit
description: Use the gitter subagent to handle git status, diffs, and commits regularly.
---

## What I do

- Remind parent agents to invoke `@gitter` when commits are requested or changes should be captured.
- Provide a short template for the handoff context (scope, intent, and any constraints).

## When to use me

Use this whenever the user asks for commits, when a task introduces meaningful changes, or when the working tree should be cleaned up into commits.

## Handoff template

Send `@gitter` a short summary like:

- Goal: <what the user asked for>
- Scope: <files or areas touched>
- Notes: <any constraints, tests run, or commit grouping preferences>
