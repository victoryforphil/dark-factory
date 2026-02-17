---
name: research
description: Run inspection-first, evidence-backed codebase research with parallel @explore subagents before implementation.
---

## What I do

- Convert ambiguous or architecture-heavy requests into a concrete, file-backed implementation plan.
- Use `@explore` subagents to inspect code paths, contracts, and current behavior.
- Return actionable findings with exact files/functions and clear next edits.

## When to use me

Use this skill when:

- The user asks for ideas, architecture direction, or tradeoff analysis.
- The task spans multiple modules and needs investigation before coding.
- You need to confirm current behavior before proposing schema/API/UI changes.
- There are unclear integration points and you need evidence before editing.

## Inspection workflow

1. Split the problem into 2-4 focused inspection tracks.
2. Launch `@explore` subagents in parallel for independent tracks.
3. Require each track to return:
   - exact file paths
   - key structs/functions/routes
   - constraints/edge cases
4. Synthesize into one plan:
   - minimal viable change path
   - optional extensions
   - test/update checklist
5. Ask at most one blocking clarification if required.

## Output contract

- Findings: concrete file/function evidence (not guesses).
- Plan: ordered implementation steps with affected files.
- Risks: migration/compatibility/runtime caveats.
- Validation: specific commands/tests to run.
- Open questions: only if materially blocking.

## Ratatui component debugging note

- When research covers `dark_chat` TUI component behavior, include snapshot-backed checks in the validation plan.
- Use `cargo test -p dark_chat tui::panels::chat_panel::tests` to validate render behavior.
- Use `INSTA_UPDATE=always cargo test -p dark_chat tui::panels::chat_panel::tests` only when intentionally accepting new snapshot output.

## Guardrails

- Prefer read-only research during inspection.
- Do not propose destructive actions without explicit user request.
- Keep recommendations incremental and compatible with current contracts.
- Highlight uncertainty explicitly when evidence is incomplete.
