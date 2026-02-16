---
name: proto-install
description: Use Bun shebang scripts to run `proto install` from repository root.
---

## What I do

- Standardize `proto install` execution through repository scripts.
- Ensure root execution by using the shared helper-based script entrypoint.

## When to use me

Use this when:

- A task needs the proto toolchain installed or refreshed.
- An agent should run project setup consistently without ad-hoc shell commands.

## Preferred command

- `bun scripts/proto_install.sh.ts`

The script resolves repository root and runs `proto install` from there.

## Notes

- Keep script usage aligned with `scripts/*.sh.ts` conventions.
- If command execution fails, report the error and next required action instead of inventing fallback steps.
