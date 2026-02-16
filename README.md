# Dark Factory - Autonomous Agentic Development At Scale

# Concept

- A central `dark-factory` "core" is used to track `worlds`
- A `world` is an `env` + `actor`(s)
- `env` is like a project and represents where an `actor` is spawned.
  - Similar to `projets` in OpenCode or other agent orchestrators
  - `env` may be a directory on host (simplest and most common) or later expanded to:
    - Remote directory on a SSH-connected machine
    - Dockershell w/ virutal FS
    - Git Worktree
- `world` can link multiple `envs` - which assume to all be multiple versions of the same root project
  - we call this concept `dimensions`
  - Examples:
    - Multiple cloned directories of the same root project
    - Git worktree supportds
    - Instanced Docker shells
- When an agent is spawned - it is linked to a given env
  - Linking can just be spawning the agent at the directory (IE: `opencode .`)
  - Link in the future can be custom tools/mcps to overwrite default shell and FS tools to ones that understand multi-verses
- `actor` is a spawned agent that operates on an env.
  - Can be various agent backends (currently scoped in OpenCode, Codex and a custom agent) - but first iteration is JUST opencode
  - Currently is 1-1 agent to env but future work will allow multiple agents per world or even env if operations allow.

# Stack + Tools

- Agentic Coding: `opencode`
- Main languages: `rust` and `bun` / `typscript`
- Optional build system: `moon` / `proto`
- Common Schema Defintion: `prisma`
  - [ ] TODO: Investigate use of protobuff

# Components

## Core

- Central service that manages tracking of all worlds/envs/settigns etc.
  - TBD: will either be stateless w/ DB and route all commands just to the agents
    - OR: a background API service (Rest even) we can query from frontends
- Currently looking lile will be Bun + Elysia JS

## Frontends

- Various frontends that invoke / connect to Core
- Main way of iterating with the system
- Uses the Core API to communicate (REST / WS or GRPC in the future)
- Some Frontends:
  - Rust TUI (Ratatui.rs) - First One
  - Web Client (Bun + Vite + Shadcn + React)
  - Pure CLI (Rust)

## Agents

- Abstracted communication with actual agents doing work
- First is OpenCode
