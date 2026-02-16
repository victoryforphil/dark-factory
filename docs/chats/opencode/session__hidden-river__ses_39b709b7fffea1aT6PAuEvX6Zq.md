----
## OpenCode Chat Export

- Session ID: ses_39b709b7fffea1aT6PAuEvX6Zq
- Title: Rename terminology in README.md for dark-factory core concepts
- Slug: hidden-river
- Project directory: /Users/alex/repos/vfp/dark-factory
- OpenCode version: 1.2.5
- Created: 2026-02-16T03:47:24.929Z
- Updated: 2026-02-16T04:06:19.684Z
- Message count: 48
----

## Conversation

### 1. user (2026-02-16T03:47:24.940Z)

I want to change our termonogly we oriignally defined in @README.md.

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/README.md"}

<path>/Users/alex/repos/vfp/dark-factory/README.md</path>
<type>file</type>
<content>1: # Dark Factory - Autonomous Agentic Development At Scale
2: 
3: # Concept
4: 
5: - A central `dark-factory` "core" is used to track `worlds`
6: - A `world` is an `env` + `actor`(s)
7: - `env` is like a project and represents where an `actor` is spawned.
8:   - Similar to `projets` in OpenCode or other agent orchestrators
9:   - `env` may be a directory on host (simplest and most common) or later expanded to:
10:     - Remote directory on a SSH-connected machine
11:     - Dockershell w/ virutal FS
12:     - Git Worktree
13: - `world` can link multiple `envs` - which assume to all be multiple versions of the same root project
14:   - we call this concept `dimensions`
15:   - Examples:
16:     - Multiple cloned directories of the same root project
17:     - Git worktree supportds
18:     - Instanced Docker shells
19: - When an agent is spawned - it is linked to a given env
20:   - Linking can just be spawning the agent at the directory (IE: `opencode .`)
21:   - Link in the future can be custom tools/mcps to overwrite default shell and FS tools to ones that understand multi-verses
22: - `actor` is a spawned agent that operates on an env.
23:   - Can be various agent backends (currently scoped in OpenCode, Codex and a custom agent) - but first iteration is JUST opencode
24:   - Currently is 1-1 agent to env but future work will allow multiple agents per world or even env if operations allow.
25: 
26: # Stack + Tools
27: 
28: - Agentic Coding: `opencode`
29: - Main languages: `rust` and `bun` / `typscript`
30: - Scripting: shebanged Bun TypeScript scripts under `scripts/` (`*.sh.ts`)
31: - Optional build system: `moon` / `proto`
32: - Common Schema Defintion: `prisma`
33:   - [ ] TODO: Investigate use of protobuff
34: 
35: # Scripts
36: 
37: - Project shell-style scripts are Bun TypeScript files with shebangs: `#!/usr/bin/env bun`.
38: - Shared helpers live in `scripts/helpers/`.
39: - `scripts/install.sh.ts` runs an ordered array of install steps from repository root.
40: - `scripts/proto_install.sh.ts` runs `proto install` from repository root.
41: - `scripts/scrape_moon_docs.sh.ts` captures moonrepo docs as split per-page `docs/external/moonrepo/*.ext.md` files plus `docs/external/moonrepo/index.ext.md`.
42: - OpenCode skill reference: `.opencode/skills/proto-install/SKILL.md`.
43: - OpenCode skill reference: `.opencode/skills/script-authoring/SKILL.md`.
44: 
45: # Components
46: 
47: ## Core
48: 
49: - Central service that manages tracking of all worlds/envs/settigns etc.
50:   - TBD: will either be stateless w/ DB and route all commands just to the agents
51:     - OR: a background API service (Rest even) we can query from frontends
52: - Currently looking lile will be Bun + Elysia JS
53: 
54: ## Frontends
55: 
56: - Various frontends that invoke / connect to Core
57: - Main way of iterating with the system
58: - Uses the Core API to communicate (REST / WS or GRPC in the future)
59: - Some Frontends:
60:   - Rust TUI (Ratatui.rs) - First One
61:   - Web Client (Bun + Vite + Shadcn + React)
62:   - Pure CLI (Rust)
63: 
64: ## Agents
65: 
66: - Abstracted communication with actual agents doing work
67: - First is OpenCode
68: 

(End of file - total 68 lines)
</content>

### 2. assistant (2026-02-16T03:47:24.953Z)

_No text content captured for this message._

### 3. user (2026-02-16T03:51:43.574Z)

I want to replace world, env, actor with the next concept: we have "products" which where like worlds. These represent a single project / repository or similar code product to work on. This is most often linked to a git repository 1-1. These ~technicaly/conceptually~ are virutal / a tempalte defintion. So for like a git repo this will be the git url + metadata. We then spawn "variants" which are the spawned instances of these products. Such as checked out directories on file system / git worktrees etc. The idea being we add / define products and the tool (dark-factory / core) can manage / spawn / despawn new variants (instances) when an new task comes in and can assign aggnets to that. This replaces the env/dimensions concept. Actors is fine for now as is. --- With this new context - firs help me refine that raw thought - then update - in similar depth/woridng/style our @README.md section (this acts as source human input of truth) - then prep a plan to refactor ALL instances across @docs/ @.opencode/ @AGENTS.md - @docs/designs/core_gen1.design.md etc. (repo wide) with new terms and context.

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/README.md"}

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/AGENTS.md"}

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/docs/designs/core_gen1.design.md"}

Called the Read tool with the following input: {"filePath":"/Users/alex/repos/vfp/dark-factory/.opencode/"}

<path>/Users/alex/repos/vfp/dark-factory/.opencode/</path>
<type>directory</type>
<entries>
.gitignore
agents/
bun.lock
commands/
node_modules/
package.json
skills/

(7 entries)
</entries>

<path>/Users/alex/repos/vfp/dark-factory/README.md</path>
<type>file</type>
<content>1: # Dark Factory - Autonomous Agentic Development At Scale
2: 
3: # Concept
4: 
5: - A central `dark-factory` "core" is used to track `worlds`
6: - A `world` is an `env` + `actor`(s)
7: - `env` is like a project and represents where an `actor` is spawned.
8:   - Similar to `projets` in OpenCode or other agent orchestrators
9:   - `env` may be a directory on host (simplest and most common) or later expanded to:
10:     - Remote directory on a SSH-connected machine
11:     - Dockershell w/ virutal FS
12:     - Git Worktree
13: - `world` can link multiple `envs` - which assume to all be multiple versions of the same root project
14:   - we call this concept `dimensions`
15:   - Examples:
16:     - Multiple cloned directories of the same root project
17:     - Git worktree supportds
18:     - Instanced Docker shells
19: - When an agent is spawned - it is linked to a given env
20:   - Linking can just be spawning the agent at the directory (IE: `opencode .`)
21:   - Link in the future can be custom tools/mcps to overwrite default shell and FS tools to ones that understand multi-verses
22: - `actor` is a spawned agent that operates on an env.
23:   - Can be various agent backends (currently scoped in OpenCode, Codex and a custom agent) - but first iteration is JUST opencode
24:   - Currently is 1-1 agent to env but future work will allow multiple agents per world or even env if operations allow.
25: 
26: # Stack + Tools
27: 
28: - Agentic Coding: `opencode`
29: - Main languages: `rust` and `bun` / `typscript`
30: - Scripting: shebanged Bun TypeScript scripts under `scripts/` (`*.sh.ts`)
31: - Optional build system: `moon` / `proto`
32: - Common Schema Defintion: `prisma`
33:   - [ ] TODO: Investigate use of protobuff
34: 
35: # Scripts
36: 
37: - Project shell-style scripts are Bun TypeScript files with shebangs: `#!/usr/bin/env bun`.
38: - Shared helpers live in `scripts/helpers/`.
39: - `scripts/install.sh.ts` runs an ordered array of install steps from repository root.
40: - `scripts/proto_install.sh.ts` runs `proto install` from repository root.
41: - `scripts/scrape_moon_docs.sh.ts` captures moonrepo docs as split per-page `docs/external/moonrepo/*.ext.md` files plus `docs/external/moonrepo/index.ext.md`.
42: - OpenCode skill reference: `.opencode/skills/proto-install/SKILL.md`.
43: - OpenCode skill reference: `.opencode/skills/script-authoring/SKILL.md`.
44: 
45: # Components
46: 
47: ## Core
48: 
49: - Central service that manages tracking of all worlds/envs/settigns etc.
50:   - TBD: will either be stateless w/ DB and route all commands just to the agents
51:     - OR: a background API service (Rest even) we can query from frontends
52: - Currently looking lile will be Bun + Elysia JS
53: 
54: ## Frontends
55: 
56: - Various frontends that invoke / connect to Core
57: - Main way of iterating with the system
58: - Uses the Core API to communicate (REST / WS or GRPC in the future)
59: - Some Frontends:
60:   - Rust TUI (Ratatui.rs) - First One
61:   - Web Client (Bun + Vite + Shadcn + React)
62:   - Pure CLI (Rust)
63: 
64: ## Agents
65: 
66: - Abstracted communication with actual agents doing work
67: - First is OpenCode
68: 

(End of file - total 68 lines)
</content>

<path>/Users/alex/repos/vfp/dark-factory/AGENTS.md</path>
<type>file</type>
<content>1: # Dark Factory - Agent Operating Guide
2: 
3: This file is for coding agents working in `dark-factory`.
4: It reflects only what is currently true in this repository.
5: 
6: ## 1) Current Repository Context
7: 
8: - The repo is early-stage and mostly docs/config right now.
9: - Tracked project files currently include:
10:   - `README.md`
11:   - `TODO.md`
12:   - `AGENTS.md`
13:   - `opencode.jsonc`
14:   - `.opencode/agents/gitter.md`
15:   - `.opencode/skills/gitter-commit/SKILL.md`
16: - No application code tree is committed yet (`src/`, `apps/`, `crates/`, etc.).
17: 
18: ## 2) Build/Lint/Test Status (Current Truth)
19: 
20: - There are no validated build, lint, typecheck, or test commands yet.
21: - Do not invent or assume project commands in commits or PR notes.
22: - If new tooling is added, document only commands that are verified in this repo.
23: 
24: ## 3) Cursor and Copilot Rules
25: 
26: - Checked and currently absent:
27:   - `.cursor/rules/`
28:   - `.cursorrules`
29:   - `.github/copilot-instructions.md`
30: - If any of these files appear later, treat them as authoritative and update this guide.
31: 
32: ## 4) Coding Style (Minimal Baseline)
33: 
34: Until language/tool-specific configs exist, follow pragmatic defaults:
35: 
36: - Favor readable, explicit code over clever shortcuts.
37: - Keep functions focused and avoid hidden side effects.
38: - Avoid dead code, unused imports, and speculative abstractions.
39: - Keep naming consistent (`PascalCase` types, `camelCase` values/functions, `UPPER_SNAKE_CASE` constants).
40: - Handle errors with context; do not swallow exceptions silently.
41: - Never log secrets or credentials.
42: - Log messages should follow: `System // Optional Sub system // Message (Metadata)`.
43:   - Example: `Core // HTTP // Listening (env=development,host=127.0.0.1,port=4150)`.
44: 
45: ## 5) Git Workflow
46: 
47: - Commit meaningful units of work; avoid giant mixed commits.
48: - Do not push unless the user explicitly requests it.
49: - Commit message format:
50:   - `{Component/Meta} // {Optional Addition} // {Short Description} (Optional,Tags)`
51: - Examples:
52:   - `Docs // Added converted PDF docs`
53:   - `Pipelines // ODM // Added initial ODM pipeline (WIP)`
54: - In commit body, add a short rationale and signature when possible:
55:   - `// Chappie/Model (your model if known)`
56: 
57: ## 6) OpenCode Integration
58: 
59: - `.opencode/agents/gitter.md` is the git-focused subagent.
60: - `.opencode/skills/gitter-commit/SKILL.md` documents when to route commit tasks to `@gitter`.
61: - `.opencode/skills/proto-install/SKILL.md` documents standardized script-based `proto install` usage.
62: - `.opencode/skills/protobuf-schema-authoring/SKILL.md` documents how to author proto2/proto3 schemas and provide Rust/TypeScript codegen workflows.
63: - `.opencode/skills/script-authoring/SKILL.md` documents how to build reusable Bun scripts from prompt/example/bash/context.
64: - Use `@gitter` when the user asks for commit support or cleanup.
65: 
66: ## 7) Local Fork Workflow (`elysia-protobuf`)
67: 
68: - `elysia-protobuf` is also cloned locally at `~/repos/vfp/elysia-protobuf`.
69: - If Core hits a bug in that dependency, patch it in the local fork repo, push the fork, then update the pinned git ref in `core/package.json` and refresh `core/bun.lock`.
70: - Core currently tracks `elysia-protobuf` from the fork `master` branch in `core/package.json`; after pushing fork changes, refresh `core/bun.lock` in this repo.
71: 
72: ## 8) Script Conventions
73: 
74: - Project scripts use shebanged Bun TypeScript files (`#!/usr/bin/env bun`) under `scripts/`.
75: - Script naming uses `*.sh.ts` to indicate executable shell-style Bun scripts.
76: - Shared script helpers live under `scripts/helpers/`.
77: - `scripts/install.sh.ts` runs a reusable ordered list of install steps.
78: - `scripts/proto_install.sh.ts` runs `proto install` from repository root.
79: - `scripts/scrape_moon_docs.sh.ts` scrapes moonrepo docs into split per-page `docs/external/moonrepo/*.ext.md` files and writes `docs/external/moonrepo/index.ext.md`.
80: - After a new command sequence succeeds in a conversation, suggest capturing it into a reusable script via `.opencode/commands/capture_script_from_context.md`.
81: 
82: ## 9) Keep This File Updated
83: 
84: - Update this file when real code, tooling, or CI is added.
85: - Keep instructions tied to verified repository behavior.
86: - Prefer short, accurate guidance over aspirational process docs.
87: 

(End of file - total 87 lines)
</content>

<path>/Users/alex/repos/vfp/dark-factory/docs/designs/core_gen1.design.md</path>
<type>file</type>
<content>1: # Notes
2: 
3: - Use moon and proto to manage tools and deps
4: - Use a shared schemas language to define our types / API's
5:     - This will allow us to port to a new language eaier
6:     - Prisma was first choice but lacks solid rust support
7:     - Protobuf using elysia-protobuf seems like a good option
8:         - Libray is a bit immature - so we forked it to allow patches
9: - Bun + Elysia JS based server for Gen 1 API for the following reasons:
10:     - Faster iteration
11:     - Nice OpenAPI support which will is nice for testing and open source development
12:     - Out of the box plugins for stuff like llms.txt - userful for the spirit
13:     - In theroy - can do ProtoBuff -> OpenAPI -> Routes / Docs all out of the box.
14: - High level architecture is a core service that handles world / env / actor tracking and provides an API for agentic comms. The core service will be used by the agents to interact with the world and each other. The core service will also handle logging and other helpers.
15: - Core acts a spawnable api server that reads from a local database oof our stored world / env / actor state. It exposes HTTP/WebSocket APIs to query / manipulate this state, as well as an API for agentic comms. The core service is designed to be lightweight and fast, allowing for quick iteration and development.
16: - Frontend (first one being a simple CLI, followed by a TUI) will interact with the core service to provide a user interface for monitoring and interacting with the world and agents. The frontend will also provide tools for debugging and visualizing the state of the world and agents.
17: 
18: 
19: # Scope - Stage 0 
20: - [ ] Shared Schema REST API w/ Build System
21: - [ ] Exposed REST API Docs
22: - [ ] Schemas for:
23:     - [ ] World Definitions
24:     - [ ] Actor State (Spawned Agents and their info)
25: - [ ] REST API for Creating Worlds w/ a single directory
26: - [ ] REST API for Spawning OpenCode-based session servers in a defined world
27:     - [ ] OpenCode over its Server API based manager
28: - [ ] REST API for querying last known state of spawned agents
29: - [ ] Single world - single instance - single OpenCode agent for now
30: - [ ] Basic Rust-based CLI for interacting with the core service and querying state
31: 

(End of file - total 31 lines)
</content>

### 4. assistant (2026-02-16T03:51:43.585Z)

_No text content captured for this message._

### 5. assistant (2026-02-16T03:51:50.752Z)

_No text content captured for this message._

### 6. assistant (2026-02-16T03:51:58.324Z)

_No text content captured for this message._

### 7. assistant (2026-02-16T03:52:09.246Z)

_No text content captured for this message._

### 8. assistant (2026-02-16T03:52:15.467Z)

_No text content captured for this message._

### 9. assistant (2026-02-16T03:52:21.014Z)

_No text content captured for this message._

### 10. assistant (2026-02-16T03:52:25.857Z)

_No text content captured for this message._

### 11. assistant (2026-02-16T03:52:35.158Z)

_No text content captured for this message._

### 12. assistant (2026-02-16T03:52:38.859Z)

_No text content captured for this message._

### 13. assistant (2026-02-16T03:52:53.121Z)

Love this direction. Your new model is cleaner and more scalable than `world/env/dimensions`.

Here’s a refined version of your concept, then a ready-to-drop `README.md` rewrite, then a repo-wide refactor plan.

**Refined Model**
- `product`: canonical, mostly-virtual definition of a code product (usually a git repo + metadata), not the live working copy.
- `variant`: spawned/materialized instance of a `product` (filesystem clone, git worktree, remote directory, docker shell, etc).
- `actor`: spawned agent session bound to one `variant` at execution time.
- Relationship model: one `product` -> many `variants`; one `variant` -> one active `actor` for now (can expand later).
- Task flow: incoming task -> select/create `product` -> spawn/reuse `variant` -> assign `actor` -> run -> optionally despawn `variant`.

**README Draft (Concept + Core line update)**  
I can’t edit files while plan mode is active, but this is the exact content I’d apply.

```md
# Concept

- A central `dark-factory` "core" is used to track `products`
- A `product` is the canonical definition of a codebase we work on.
  - Usually maps 1-1 to a git repository
  - Stores source identity + metadata (repo URL, branch defaults, labels, auth/profile refs, etc.)
  - Is mostly virtual/conceptual (template-like), not the spawned runtime instance
- A `variant` is a spawned instance of a `product` where real work happens.
  - Similar to runtime project instances in other orchestrators
  - `variant` may be:
    - A checked-out directory on host (simplest and most common)
    - A Git worktree
    - A remote directory on an SSH-connected machine
    - A dockershell w/ virtual FS
- A `product` can link multiple `variants` (parallel implementations/branches of the same root code product)
  - Examples:
    - Multiple cloned directories of the same repository
    - Multiple git worktrees from one repository
    - Instanced docker shells
- When a task comes in, Core can create/reuse a `product`, spawn/reuse a `variant`, and assign an agent to it
  - This enables managed spawn/despawn lifecycle per task or workload class
- `actor` is a spawned agent that operates on a `variant`.
  - Can be various agent backends (currently scoped in OpenCode, Codex and a custom agent) - but first iteration is JUST opencode
  - Currently is 1-1 actor to variant but future work will allow multiple actors per product or variant where operations allow

## Core

- Central service that manages tracking of all products/variants/actors/settings etc.
  - TBD: will either be stateless w/ DB and route all commands just to the agents
    - OR: a background API service (Rest even) we can query from frontends
- Currently looking lile will be Bun + Elysia JS
```

**Repo-Wide Refactor Plan**
- **Terminology policy first**
  - Replace conceptual `world/env/dimensions` with `product/variant`.
  - Keep technical `.env`, env vars, shebang `#!/usr/bin/env bun`, and log metadata like `env=development` unchanged.
  - Do not edit vendored/imported docs under `docs/external/` or `node_modules`.

- **Source-of-truth + design docs**
  - Update `README.md` concept section and core description (`README.md:5`, `README.md:49`).
  - Update architecture and stage-0 scope in `docs/designs/core_gen1.design.md:14`, `docs/designs/core_gen1.design.md:15`, `docs/designs/core_gen1.design.md:16`, `docs/designs/core_gen1.design.md:23`, `docs/designs/core_gen1.design.md:25`, `docs/designs/core_gen1.design.md:26`, `docs/designs/core_gen1.design.md:29`.
  - Update planning language in `TODO.md:15`, `TODO.md:28`, `TODO.md:29`.

- **Core package docs + API surface wording**
  - Update `core/README.md:7`, `core/README.md:8` to `product`/`variant` terminology.
  - Update `core/src/index.ts:12` from `["world","env","actor"]` -> `["product","variant","actor"]`.
  - Keep `core/src/index.ts:13` and `core/src/index.ts:22` `env` usage unchanged (runtime environment).
  
- **AGENTS + .opencode audit alignment**
  - `AGENTS.md` has no conceptual `world/env/dimensions` references needing rename; current `env` mentions are technical and should stay.
  - `.opencode` markdown files also only contain technical `.env`/shebang mentions; no terminology migration needed.
  - Optional: add one short note in `AGENTS.md` clarifying conceptual vocabulary (`product/variant/actor`) vs technical `env` usage.

- **Verification pass**
  - Run targeted grep checks for conceptual terms in first-party paths:
    - root markdowns, `docs/designs/`, `core/`, `.opencode/`.
  - Confirm any remaining `world/env` instances are only technical contexts or intentionally retained.
  - Since repo has no validated test/lint commands yet, rely on static doc/code review + `bun` smoke run for `core` only if desired.

If you want, next step I can execute this plan and apply all edits in one coherent pass.

### 14. user (2026-02-16T03:54:57.282Z)

Note for first stages itll be pretty 1-1-1, aka a product will be defined by a local idenfitfier (path) instead of a git URL (define an asbtract "locator" field / type?) - will immeidately have a variant of that local path - and will be allowed one actor

### 15. assistant (2026-02-16T03:54:57.288Z)

Perfect, that constraint makes Gen 1 much cleaner.

**Stage 0 Model (Refined)**
- `product` = logical code product definition, but in Stage 0 it is sourced by a local path locator.
- `locator` = abstract typed source descriptor; Stage 0 supports only `local_path`.
- `variant` = spawned working instance of a `product`; in Stage 0 it is created immediately and points to the same local path.
- `actor` = agent session bound to the `variant`; Stage 0 allows exactly one actor.
- Operationally this is a strict 1-1-1 chain in Gen 1: `product -> variant -> actor`.

**Recommended data shape (future-safe, stage-limited)**
- `Product`
  - `id`
  - `name`
  - `locator: { type: "local_path", value: "/abs/path" }`
  - `metadata`
- `Variant`
  - `id`
  - `productId`
  - `kind: "local_path_mirror"` (or similar)
  - `workingPath`
  - `lifecycleState`
- `Actor`
  - `id`
  - `variantId`
  - `backend` (`opencode` first)
  - `state`

**README wording adjustment I’ll use**
- Keep `product` as the primary concept.
- Define `locator` as abstract, with Stage 0 constrained to local paths.
- Explicitly state:
  - Product creation immediately creates one default variant.
  - Only one variant per product in Stage 0.
  - Only one actor per variant in Stage 0.
- Keep note that future stages expand to git URL locators, worktrees, remote dirs, and multiple variants/actors.

**Refactor plan updates with this constraint**
- Update conceptual sections in `README.md` and `docs/designs/core_gen1.design.md` to:
  - replace `world/env/dimensions` with `product/variant/locator`
  - codify Stage 0 invariants (`1 product : 1 variant : 1 actor`)
- Update planning/tasks in `TODO.md`:
  - “World + Env + Actor” -> “Product + Variant + Actor (Stage 0 local-path locator)”
- Update `core/README.md` and `core/src/index.ts` concept labels:
  - `["world", "env", "actor"]` -> `["product", "variant", "actor"]`
  - keep technical runtime `env` mentions unchanged (NODE_ENV, `#!/usr/bin/env bun`, log metadata)
- Skip external/vendor docs (`docs/external/**`, `node_modules/**`) from terminology migration.

If this framing looks good, once plan mode is lifted I’ll apply the full doc/code terminology pass in one implementation sweep.

### 16. user (2026-02-16T03:58:15.736Z)

Notes on data shape: merge locator + id + name into a single locator key system. We will use path/url-like identifiers to do this. for example, local is "@local://{path}"

### 17. assistant (2026-02-16T03:58:15.742Z)

Great call - I like this a lot.

Adopting a single **locator-key system** is cleaner than carrying separate `id`/`name` fields in Gen 1.

Updated direction I’ll implement:

- `product` identity is the locator key itself (no separate product id/name in Stage 0).
- Stage 0 locator format: `@local://{path}` (local filesystem source of truth).
- Product creation is effectively “register this locator”; it immediately creates the default `variant` and allows one `actor`.
- Stage 0 invariant stays strict: `1 locator-product -> 1 default variant -> 1 actor`.
- Future-ready: later add more schemes (`@git://...`, `@ssh://...`, `@docker://...`) without changing the conceptual model.

Recommended normalization rules (so keys are stable):
- Use absolute paths only.
- Canonicalize path before key generation (realpath-style normalization where possible).
- Keep one canonical string form (no trailing slash drift).
- Treat locator string as the primary key everywhere.

Example:
- Product key: `@local:///Users/alex/repos/vfp/dark-factory`

I’ll reflect this in the planned refactor across:
- `README.md` concept section and core section
- `docs/designs/core_gen1.design.md` architecture + stage 0 scope
- `TODO.md` terminology/tasks
- `core/README.md` concept bullets
- `core/src/index.ts` concepts payload (`["product","variant","actor"]`)

Default I’ll use unless you want different:
- Keep optional human-friendly display labels as derived values (e.g. basename from path), not identity fields.

### 18. user (2026-02-16T03:59:27.674Z)

Do the same pattern for varaints as well - we can add back an display_name fields since we are using abs paths and those are hard to render nicely

### 19. assistant (2026-02-16T03:59:27.679Z)

Perfect - agreed. We should apply the same locator-key identity pattern to `variant` too.

Updated model I’ll use in the refactor plan:

- `product` identity = `product_locator` (string key), no separate `id`/`name` in Stage 0.
- `variant` identity = `variant_locator` (string key), no separate `id`/`name` in Stage 0.
- Add optional `display_name` to both `product` and `variant` for UI rendering.
- Stage 0 still enforces strict 1-1-1:
  - one product locator
  - one variant locator (default)
  - one actor

Recommended key formats:

- Product: `@local://{abs_path}`
- Variant (same source, explicit variant namespace): `@variant/local://{abs_path}#default`

Why this format:
- Avoids key collision between product and variant.
- Keeps locator parsing predictable.
- Leaves room for future variant types (`#wt-main`, `#docker-a`, etc.).

Display names (derived + overridable):

- `product.display_name`: default from path basename (e.g. `dark-factory`)
- `variant.display_name`: default like `default` or `local-default`
- Both can be overridden manually later without changing identity keys.

One targeted decision to lock before implementation:
- Do you want variant locators namespaced like `@variant/local://...#default` (recommended), or do you want both product/variant to share the exact same `@local://...` format and rely on object type to disambiguate?

### 20. user (2026-02-16T03:59:49.413Z)

only product has display-name

### 21. assistant (2026-02-16T03:59:49.416Z)

Perfect, locked in.

Updated decisions for implementation:

- `product`
  - Identity: `product_locator` (string key), e.g. `@local:///abs/path`
  - Has `display_name` (optional, UI-facing)
- `variant`
  - Identity: `variant_locator` (string key)
  - No `display_name`
  - UI label for variant should be derived at render time from locator (not stored)

Stage 0 constraints remain:

- Only `@local://` locator scheme
- Product creation immediately creates one default variant
- One actor max per variant
- Effective topology: `1 product -> 1 variant -> 1 actor`

Recommended variant key shape (still recommended):
- `@variant/local:///abs/path#default`

This keeps product and variant identities unambiguous while preserving path-based semantics.

Plan updates I’ll execute once write mode is enabled:

- Update terminology + Stage 0 invariants in `README.md`
- Update architecture/scope language in `docs/designs/core_gen1.design.md`
- Update TODO entries in `TODO.md`
- Update `core/README.md` concept bullets
- Update `core/src/index.ts` concepts payload to `["product", "variant", "actor"]`
- Keep technical env usage unchanged (`NODE_ENV`, `.env`, shebangs, log metadata)

If you want, I can also include a short “Locator Key Spec (Stage 0)” section in `README.md` so future changes stay consistent.

### 22. user (2026-02-16T04:00:14.176Z)

remove variant part of the id

### 23. assistant (2026-02-16T04:00:14.179Z)

Got it - we’ll remove the variant namespace from the key.

Updated key scheme:

- `product_locator`: `@local:///abs/path`
- `variant_locator`: `@local:///abs/path#default` (same base locator, variant suffix only)
- No `@variant/...` prefix

So identity is now:

- product identity = locator string without fragment
- variant identity = same locator + fragment (for Stage 0 default variant)

This keeps your path/url-like system clean while still allowing multiple variants later via different fragments (`#default`, `#wt-main`, etc.).

Implementation plan delta (no edits yet, just plan updates):

- Replace prior variant-key examples in planned docs with `@local:///...#default`
- Explicitly document that **entity type + locator** disambiguates records (since both keys use `@local://`)
- Keep only `product.display_name`; variant has no stored display name
- Preserve Stage 0 invariants: one product, one default variant, one actor

If you want, I’ll also add one short normalization rule in docs: “product locator must not include a fragment; variant locator must include a fragment.”

### 24. user (2026-02-16T04:00:35.047Z)

do it

### 25. assistant (2026-02-16T04:00:35.051Z)

_No text content captured for this message._

### 26. assistant (2026-02-16T04:00:43.128Z)

_No text content captured for this message._

### 27. assistant (2026-02-16T04:00:57.006Z)

_No text content captured for this message._

### 28. assistant (2026-02-16T04:01:10.772Z)

_No text content captured for this message._

### 29. assistant (2026-02-16T04:01:15.784Z)

_No text content captured for this message._

### 30. assistant (2026-02-16T04:01:26.853Z)

_No text content captured for this message._

### 31. assistant (2026-02-16T04:01:32.476Z)

_No text content captured for this message._

### 32. assistant (2026-02-16T04:01:44.406Z)

_No text content captured for this message._

### 33. assistant (2026-02-16T04:01:48.304Z)

_No text content captured for this message._

### 34. assistant (2026-02-16T04:01:52.563Z)

_No text content captured for this message._

### 35. assistant (2026-02-16T04:02:00.934Z)

_No text content captured for this message._

### 36. assistant (2026-02-16T04:02:07.751Z)

Done - I applied the terminology/model refactor with your latest constraints.

- Replaced `world/env/dimensions` with `product/variant/actor` in the core concept docs, and codified Stage 0 as strict `1 product -> 1 variant -> 1 actor`.
- Implemented locator-key identity language with your format:
  - product: `@local://{abs_path}`
  - variant: `@local://{abs_path}#default` (no `@variant/...` prefix)
- Kept only `product.display_name` (optional); no variant display name field.
- Left technical `env` usage untouched (`NODE_ENV`, `.env`, `#!/usr/bin/env bun`, runtime log metadata).

Updated files:
- `README.md:5`
- `docs/designs/core_gen1.design.md:14`
- `TODO.md:22`
- `core/README.md:7`
- `core/src/index.ts:12`

I also did a targeted terminology audit in `AGENTS.md` and `.opencode/**/*.md`; there were no conceptual `world/env/dimensions` terms there needing migration (only technical env mentions).

1. If you want, I can do a second pass to clean existing typos in the same docs while preserving meaning.
2. If you want this captured, I can prepare a commit with your repo’s commit message format.

### 37. user (2026-02-16T04:04:00.107Z)

2nd pass please

### 38. assistant (2026-02-16T04:04:00.111Z)

_No text content captured for this message._

### 39. assistant (2026-02-16T04:04:12.799Z)

_No text content captured for this message._

### 40. assistant (2026-02-16T04:04:21.048Z)

_No text content captured for this message._

### 41. assistant (2026-02-16T04:04:28.293Z)

_No text content captured for this message._

### 42. assistant (2026-02-16T04:04:33.947Z)

_No text content captured for this message._

### 43. assistant (2026-02-16T04:04:40.288Z)

_No text content captured for this message._

### 44. assistant (2026-02-16T04:04:45.505Z)

_No text content captured for this message._

### 45. assistant (2026-02-16T04:04:50.933Z)

Done - I completed a second polish pass and tightened wording/typos across the terminology changes.

- Cleaned language and spelling in `README.md` (typescript/protobuf/settings/REST phrasing + actor backend sentence polish).
- Cleaned wording and typo fixes in `docs/designs/core_gen1.design.md` (APIs/easier/library/useful/theory/protobuf casing and phrasing).
- Cleaned remaining typo/format issues in `TODO.md` (`Protobuf`, trailing spaces, naming consistency).
- Left technical `env` usage unchanged where it is runtime/shell related.
- Kept your Stage 0 model intact: product + variant locator keys, with `product.display_name` only.

Updated files:
- `README.md`
- `docs/designs/core_gen1.design.md`
- `TODO.md`
- `core/README.md`
- `core/src/index.ts`

I also noticed unrelated untracked items in your tree (`.opencode/commands/scrape_doc.md`, `.opencode/skills/external-docs-scraping/`) and left them untouched.

1. If you want, I can now prepare a clean commit using your AGENTS.md commit format.

### 46. user (2026-02-16T04:05:42.795Z)

@gitter This threads' work

Use the above message and context to generate a prompt and call the task tool with subagent: gitter

### 47. assistant (2026-02-16T04:05:42.799Z)

_No text content captured for this message._

### 48. assistant (2026-02-16T04:06:14.571Z)

Done - I handed this off to `@gitter` and it committed the thread’s work.

- Commit: `2146b6e`
- Message: `Core // Stage 0 Model // Refactor terminology to product-variant-actor`
- Included files:
  - `README.md`
  - `TODO.md`
  - `docs/designs/core_gen1.design.md`
  - `core/README.md`
  - `core/src/index.ts`
- Left unrelated untracked files untouched:
  - `.opencode/commands/scrape_doc.md`
  - `.opencode/skills/external-docs-scraping/`
- No push was performed.

----
## Notes

- Export source: `opencode export <sessionID>` JSON payload.
- This markdown keeps text parts only; use the sibling `.json` file for full structured data.
----
