# Dark Factory - Agents MD

# Project Context

- TODO / See README.md

# Git

## Commmiting

- Commit reguarly as features are developed
- Usually try and verify functionality - or at least existing build and/or tests pass before committing.
- Format: `{Component/Meta} // {Optional Addition section} // {Description, Short} (Optional,Tags)`
- Examples:
  - `Docs // Added converted PDF docs`
  - `Pipelines // ODM // Added initial ODM pipeline (WIP)`
- Note the "addition section" is optional. Usually not needed as much, Id say 80/20 -> 60/40 split as the repo grows
- In the longer commit summary - sign - `// Chappie/Model (your model if you know)`

## Branchs and Pushing

- If working with user, commit on current branch, else ask about what branch to use if working auto.
- Don't push unless the user has asked you to for the session. At the very least, ask first.
  - So the user can review, reword or revert commits before they end up on the interwebs.
- `gh` CLI is avalible.

# OpenCode Agents

- `.opencode/agents/gitter.md` handles git status/diff review and commits using repo conventions.
- `.opencode/skills/gitter-commit/SKILL.md` tells parent agents when to invoke `@gitter`.
