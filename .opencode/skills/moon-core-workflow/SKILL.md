---
name: moon-core-workflow
description: Run dark-factory core setup/codegen/start through moon with Bun scripts.
---

## What I do

- Standardize moon usage for the `core` Elysia API project.
- Ensure protobuf code generation runs before `core:start` and `core:dev`.
- Keep setup repeatable with repository scripts.

## When to use me

Use this skill when:

- You need to bootstrap tools and run core via moon.
- You need to regenerate TypeScript files from `schemas/core/**/*.proto`.
- You need a single, readable command flow for local development.

## Commands

- Install toolchain and prerequisites:
  - `bun scripts/install.sh.ts`
- Regenerate protobuf artifacts:
  - `bun scripts/proto_codegen_core.sh.ts`
- Run core through moon:
  - `bun scripts/moon_core.sh.ts start`
  - `bun scripts/moon_core.sh.ts dev`

## Notes

- `core:codegen-proto` is configured in `core/moon.yml` and depends on `core:install`.
- Generated files are written to `core/src/gen/proto/`.
- Descriptor output is written to `schemas/core/core.pb`.
