---
name: reflect
description: Do a quick end-of-task reflection and capture durable lessons.
---

## What I do

- Run a lightweight reflection after meaningful task work is complete.
- Capture short, reusable lessons in `docs/lessons/*.lessons.md` files.
- Prefer practical guidance over exhaustive retrospectives.
- Support `quick` reflection directly and `deep` reflection via `@reflector`.

## When to use me

Use this near the end of tasks that involved troubleshooting, trial-and-error, new workflows, or notable user feedback.

Skip heavy analysis. It is fine to add nothing when no durable lesson emerged.

Use `deep` mode when you want one agent to judge another agent's session quality.

## Reflection checklist

1. Scan what just happened in the task.
2. Extract at most 1-3 generic lessons worth reusing.
3. Focus on:
   - successful command sequences or debugging flows
   - recurring pitfalls and reliable fixes
   - rule changes from `.opencode/commands/rule.md`
   - maintenance updates needed in `AGENTS.md` or `.opencode/*` context files
4. Write concise bullet points only (no long sections).
5. Keep each lessons file short (target 10-50 lines, always under 100).

## Deep review option (`@reflector`)

When context quality needs extra review, hand off to `@reflector` with either:

- a task summary
- transcript snippets
- exported session context from `opencode export <sessionID>`

Then store only `Lessons` bullets returned by `@reflector`.

When useful, keep a timestamped deep-review artifact in `docs/reflections/`.

Reflection is also allowed to refine existing lessons and agent context docs (`AGENTS.md`, `.opencode/skills/*`, `.opencode/agents/*`) when updates are maintenance-oriented.

## Guardrails

- Never write secrets, tokens, credentials, or private one-off data.
- Prefer durable, generic phrasing over task-specific details.
- De-duplicate near-identical bullets instead of growing noise.
- Avoid design-direction edits and one-off human preference/style changes unless explicitly requested.
