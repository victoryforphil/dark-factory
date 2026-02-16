# Actor and Variant Linking Design

## Goals

- Introduce a provider-agnostic Actor abstraction that can be attached to existing variants.
- Support `Variant 1 -> N Actors` so each variant can spawn and manage multiple runtime actors.
- Keep product linkage implicit through variant ownership (`Actor -> Variant -> Product`).
- Standardize high-level actor lifecycle operations across providers.
- Use OpenCode as the first provider implementation and compatibility reference.

## Scope (Phase 1)

- Add Actor data model and API surface for CRUD, spawn, list, poll, attach, message, and command operations.
- Keep chat history provider-backed (no local message persistence in Prisma yet).
- Keep `Variant.locator` on current `@local://` semantics.
- Add a separate provider-aware `ActorLocator` for runtime session attachment/continuation.
- Move provider integrations into a dedicated providers module and port OpenCode integration there.
- Compatibility with the existing `/opencode/*` routes is optional and may be dropped during implementation.

## Terminology

- `VariantLocator`: existing workspace locator (`@local://...`) used by products/variants.
- `ActorLocator`: provider-aware locator (`opencode:///...#...`) used by actors.
- `Provider Session ID`: provider-native runtime identity (for OpenCode, `session.id`).

## Locator Model

### VariantLocator (existing)

- Format: `@local://<canonical_absolute_path>`
- Examples:
  - `@local:///Users/alex/repos/vfp/dark-factory`
  - `@local:///tmp/project-a`
- Canonicalization and host-path conversion stay on existing utilities:
  - `dark_core/src/utils/locator.ts`
  - `lib/dark_rust/src/locator_id.rs`

### ActorLocator (new)

- Format: `<provider>:///<canonical_absolute_path>[#<provider_ref>]`
- OpenCode examples:
  - `opencode:///Users/alex/repos/vfp/dark-factory`
  - `opencode:///Users/alex/repos/vfp/dark-factory#session_abc123`
- Rules:
  - `provider` is lowercase.
  - Triple slash is required for absolute path URI form.
  - `#<provider_ref>` is optional and stores provider-native resume identity.
  - Query params are not supported in phase 1.
- Behavior:
  - No `#provider_ref`: spawn a new provider runtime at path.
  - With `#provider_ref`: attach/continue provider runtime.

### Why Separate Locators (Now)

- Variants represent durable workspace identity.
- Actors represent runtime provider instances/sessions.
- Multiple actors per variant require actor-level session identity.

### #TODO: Unified Locator System Layer

- Build one shared locator registry/parser with typed kinds for products, variants, and actors.
- Keep backward compatibility for existing `@local://` workflows.
- Consider resource locators in a future phase:
  - `product://<productId>`
  - `variant://<variantId>`
  - `actor://<actorId>`

## Data Model (Prisma)

### Proposed Schema

```prisma
model Product {
  id          String    @id
  locator     String    @unique
  displayName String?   @map("display_name")
  gitInfo     Json?     @map("git_info")
  variants    Variant[]
  createdAt   DateTime  @default(now()) @map("created_at")
  updatedAt   DateTime  @updatedAt @map("updated_at")

  @@index([locator])
  @@map("products")
}

model Variant {
  id                  String    @id
  productId           String    @map("product_id")
  name                String    @default("default")
  locator             String
  gitInfo             Json?     @map("git_info")
  gitInfoUpdatedAt    DateTime? @map("git_info_updated_at")
  gitInfoLastPolledAt DateTime? @map("git_info_last_polled_at")
  product             Product   @relation(fields: [productId], references: [id], onDelete: Cascade)
  actors              Actor[]
  createdAt           DateTime  @default(now()) @map("created_at")
  updatedAt           DateTime  @updatedAt @map("updated_at")

  @@unique([productId, name])
  @@index([productId])
  @@index([locator])
  @@map("variants")
}

model Actor {
  id                String   @id
  variantId         String   @map("variant_id")
  provider          String
  actorLocator      String   @map("actor_locator")
  workingLocator    String   @map("working_locator")
  providerSessionId String?  @map("provider_session_id")
  status            String   @default("unknown")
  title             String?
  description       String?
  connectionInfo    Json?    @map("connection_info")
  attachCommand     String?  @map("attach_command")
  metadata          Json?
  createdAt         DateTime @default(now()) @map("created_at")
  updatedAt         DateTime @updatedAt @map("updated_at")

  variant           Variant  @relation(fields: [variantId], references: [id], onDelete: Cascade)

  @@index([variantId])
  @@index([provider])
  @@index([status])
  @@index([providerSessionId])
  @@unique([variantId, provider, providerSessionId])
  @@map("actors")
}
```

### Notes

- `workingLocator` stores the variant locator snapshot used during spawn time.
- `actorLocator` stores provider-specific attach/continue identity.
- Actor belongs to exactly one variant.
- A variant can have many actors.
- We do not add `ActorMessage` table in phase 1.

## Common Actor Status Labels

Canonical status labels used by API and DB:

- `spawning`
- `ready`
- `busy`
- `retrying`
- `stopped`
- `error`
- `unknown`

Provider adapters map native status to these labels.

## Core Interfaces (TypeScript)

```ts
type ActorProvider = "opencode" | string;

type ActorStatusLabel =
  | "spawning"
  | "ready"
  | "busy"
  | "retrying"
  | "stopped"
  | "error"
  | "unknown";

interface SpawnActorInput {
  variantId: string;
  provider: ActorProvider;
  title?: string;
  description?: string;
  metadata?: Record<string, unknown>;
}

interface ActorConnectionInfo {
  provider: ActorProvider;
  directory?: string;
  projectId?: string;
  serverUrl?: string;
  raw?: Record<string, unknown>;
}

interface ActorRecord {
  id: string;
  variantId: string;
  provider: ActorProvider;
  actorLocator: string;
  workingLocator: string;
  providerSessionId?: string;
  status: ActorStatusLabel;
  title?: string;
  description?: string;
  connectionInfo?: ActorConnectionInfo;
  attachCommand?: string;
  metadata?: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
}

interface ProviderMessage {
  id: string;
  role: "user" | "assistant" | string;
  createdAt: string;
  text?: string;
  raw?: Record<string, unknown>;
}

interface ActorProviderAdapter {
  provider: ActorProvider;
  spawn(input: {
    actorId: string;
    workingLocator: string;
    title?: string;
    description?: string;
    metadata?: Record<string, unknown>;
  }): Promise<{
    actorLocator: string;
    providerSessionId?: string;
    status: ActorStatusLabel;
    title?: string;
    description?: string;
    connectionInfo?: ActorConnectionInfo;
    attachCommand?: string;
  }>;
  poll(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
  }): Promise<{
    status: ActorStatusLabel;
    connectionInfo?: ActorConnectionInfo;
    attachCommand?: string;
  }>;
  buildAttach(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
    model?: string;
    agent?: string;
  }): Promise<{ attachCommand: string; connectionInfo?: ActorConnectionInfo }>;
  sendMessage(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
    prompt: string;
    noReply?: boolean;
    model?: string;
    agent?: string;
  }): Promise<Record<string, unknown>>;
  listMessages(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
    nLastMessages?: number;
  }): Promise<ProviderMessage[]>;
  runCommand(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
    command: string;
    args?: string;
    model?: string;
    agent?: string;
  }): Promise<Record<string, unknown>>;
  terminate?(input: {
    actorLocator: string;
    providerSessionId?: string;
    workingLocator: string;
  }): Promise<void>;
}
```

## Module Layout (Provider-First)

Provider integrations should live under a dedicated providers module, not under actors.

- `dark_core/src/modules/providers/`
  - `providers.common.ts` (shared provider contracts + shared mapping helpers)
  - `providers.registry.ts` (provider lookup/registration)
  - `opencode/`
    - `opencode.provider.ts` (adapter implementation)
    - `opencode.mapper.ts` (OpenCode -> common status/message mapping)
    - `opencode.types.ts` (provider-local helper types)

Actors module consumes providers via registry:

- `dark_core/src/modules/actors/`
  - `actors.controller.ts` (provider-agnostic actor orchestration)
  - `actors.routes.ts`
  - `actors.service.ts` (optional orchestration boundary)

### OpenCode Porting Direction

- Existing OpenCode runtime/session logic should be moved behind the providers module contract.
- Actors should call provider adapters through registry only; they should not import OpenCode module internals directly.
- Existing `/opencode/*` routes may be rewritten, simplified, or removed once `/actors/*` is working.

### #TODO: Provider Module Unification

- Extend providers module to host future provider implementations (not just actor providers).
- Evaluate migration of legacy provider-specific modules to provider-scoped folders.
- Keep one maintained hot path for actor/provider runtime management through providers registry.

## OpenCode Provider Mapping (Reference Implementation)

### Spawn

- Resolve `workingLocator` (`@local://...`) to host path via `locatorIdToHostPath`.
- Create OpenCode session in that directory.
- Store:
  - `providerSessionId = session.id`
  - `actorLocator = opencode:///<canonical_path>#<session.id>`
  - `status = ready`
  - `attachCommand` from OpenCode attach command builder.

### Poll

- Query OpenCode session status map.
- Map OpenCode status to common labels:
  - `idle -> ready`
  - `busy -> busy`
  - `retry -> retrying`
  - missing session -> `stopped`
  - unexpected failure -> `error`

### Messages and Commands

- `GET /actors/:id/messages` calls OpenCode session messages API.
- `POST /actors/:id/messages` sends provider prompt/message.
- `POST /actors/:id/commands` sends provider slash-command API call.
- `nLastMessages` maps to provider message limit if present.

## API Surface (dark_core)

### Actor CRUD + Lifecycle

- `POST /actors/`
  - Spawn a new actor attached to a variant.
  - Body: `{ variantId, provider, title?, description?, metadata? }`

- `GET /actors/`
  - List actors.
  - Query: `variantId?`, `productId?`, `provider?`, `status?`, `limit?`, `cursor?`

- `GET /actors/:id`
  - Get actor state.

- `PATCH /actors/:id`
  - Update metadata fields (`title`, `description`, `metadata`).

- `DELETE /actors/:id`
  - Delete actor record.
  - Optional query/body flag later: `terminate=true` to request provider-side runtime stop.

- `POST /actors/:id/poll`
  - Refresh status, connection info, attach command.

- `GET /actors/:id/attach`
  - Return attach command.
  - Optional query: `model?`, `agent?` for provider-specific attach tuning.

### Actor Interaction

- `POST /actors/:id/messages`
  - Body: `{ prompt, noReply?, model?, agent? }`
  - Sends a high-level message to provider session.

- `GET /actors/:id/messages`
  - Query: `nLastMessages?`
  - Returns provider-backed normalized messages.

- `POST /actors/:id/commands`
  - Body: `{ command, args?, model?, agent? }`
  - Sends provider-native command call.

## End-to-End Flow

1. Create product at local locator.
2. Default variant is created at same locator (existing behavior).
3. Spawn actor for variant via `POST /actors/`.
4. Actor stores provider session identity and attach command.
5. Client can list actors for variant and attach to any one.
6. Client may call poll/messages/commands APIs for lightweight management.

## Operational and Error Semantics

- Actor spawn is additive; it never replaces an existing actor.
- Delete actor only removes actor record by default.
- Provider failures map to clear API errors without exposing secrets.
- Status should be eventually consistent via explicit `poll` and selected write-time refreshes.

## Implementation Handoff Context

### Existing Code Anchors

- Locator canonicalization and host path conversion:
  - `dark_core/src/utils/locator.ts`
  - `lib/dark_rust/src/locator_id.rs`
- Product and variant behavior references:
  - `dark_core/src/modules/products/products.controller.ts`
  - `dark_core/src/modules/variants/variants.controller.ts`
- Existing OpenCode integration references:
  - `dark_core/src/modules/opencode/opencode.controller.ts`
  - `dark_core/src/modules/opencode/opencode.routes.ts`
  - `dark_core/src/modules/opencode/opencode.client.ts`
- OpenCode SDK API/types references:
  - `dark_core/node_modules/@opencode-ai/sdk/dist/gen/sdk.gen.d.ts`
  - `dark_core/node_modules/@opencode-ai/sdk/dist/gen/types.gen.d.ts`

### Known Constraints From Current Behavior

- `Variant.locator` is mutable (`PATCH /variants/:id` currently allows `locator` update).
- Because variant locator can change, actor records should keep `workingLocator` and `actorLocator` snapshots from spawn time.
- Existing `/opencode/*` routes already support session create/list/get/attach/command/prompt patterns and can be treated as disposable implementation scaffolding.
- Tests use isolated sqlite databases via `dark_core/src/test/helpers/sqlite-test-db.ts`.

### Useful OpenCode API Facts

- Session status is available as a map (`session.status`) keyed by session id.
- Session messages listing supports an optional `limit` argument.
- Session prompt and command APIs already map well to planned actor `messages` and `commands` operations.

### Suggested Adapter Snippets

```ts
export const mapOpenCodeSessionStatus = (
  status: { type: "idle" | "busy" | "retry" } | undefined,
): ActorStatusLabel => {
  if (!status) return "stopped";
  if (status.type === "idle") return "ready";
  if (status.type === "busy") return "busy";
  return "retrying";
};
```

```ts
const registry: Record<string, ActorProviderAdapter> = {
  opencode: opencodeProvider,
};

export const getProviderAdapter = (provider: string): ActorProviderAdapter => {
  const adapter = registry[provider];
  if (!adapter) throw new Error(`Providers // Registry // Unsupported provider (provider=${provider})`);
  return adapter;
};
```

```ts
export const spawnActor = async (input: SpawnActorInput): Promise<ActorRecord> => {
  const variant = await getVariantById(input.variantId);
  const adapter = getProviderAdapter(input.provider);
  const actorId = crypto.randomUUID();
  const spawned = await adapter.spawn({
    actorId,
    workingLocator: variant.locator,
    title: input.title,
    description: input.description,
    metadata: input.metadata,
  });
  return createActorRow({
    id: actorId,
    variantId: variant.id,
    provider: input.provider,
    workingLocator: variant.locator,
    ...spawned,
  });
};
```

## Out of Scope (Phase 1)

- Full provider event streaming through dark_core.
- Local persistence of chat transcript/messages.
- Automatic actor restart/recovery loops.
- Cross-provider actor migration.

## Stage-Based Implementation Plan

### Stage 0 - Foundation and Contracts

- Confirm this design doc contract and API names.
- Finalize provider adapter interface in `providers.common.ts`.
- Define error conventions and status mapping policy once.

Exit criteria:

- Adapter contract is stable enough that actor and provider modules can be built independently.

### Stage 1 - Providers Module Skeleton

- Add `dark_core/src/modules/providers/`:
  - `providers.common.ts`
  - `providers.registry.ts`
  - `opencode/opencode.provider.ts`
  - `opencode/opencode.mapper.ts`
  - `opencode/opencode.types.ts`
- Registry resolves provider key -> adapter.
- No actor routes yet.

Exit criteria:

- Providers module compiles and can be imported without changing existing public APIs.

### Stage 2 - Port OpenCode Behind Providers

- Move OpenCode runtime/session operations behind provider adapter implementation.
- Replace or remove old OpenCode internals as needed; backward compatibility is not required for this phase.
- Ensure attach command and status mapping are preserved.

Exit criteria:

- Provider adapter tests pass and actor-facing OpenCode operations are functionally correct.

### Stage 3 - Actor Persistence and CRUD

- Add Prisma `Actor` model and `Variant.actors` relation.
- Generate Prisma and prismabox outputs.
- Implement actor controller CRUD and spawn/list/poll/attach APIs.
- Actor module must only use providers registry for provider operations.

Exit criteria:

- Actor rows are created and linked to variants, and basic lifecycle APIs work end-to-end.

### Stage 4 - Actor Message and Command APIs

- Implement provider-backed `POST /actors/:id/messages`.
- Implement provider-backed `GET /actors/:id/messages?nLastMessages=...`.
- Implement provider-backed `POST /actors/:id/commands`.
- Keep no local chat persistence in this phase.

Exit criteria:

- Actor interaction APIs work against OpenCode provider and return normalized payloads.

### Stage 5 - Rust Client and Frontend Wiring

- Extend `lib/dark_rust` actor types and HTTP client methods.
- Add CLI/TUI surface for actor list/spawn/attach and lightweight management.
- Preserve backward compatibility for existing product/variant workflows.

Exit criteria:

- `dark_cli`/`dark_tui` can create and manage actors for variants.

### Stage 6 - Migration and Deprecation Cleanup

- Remove duplicate OpenCode-specific paths that are no longer needed.
- Keep one clear runtime management path centered on `/actors/*` + providers registry.
- Optionally keep thin legacy stubs only if they improve local developer ergonomics.

Exit criteria:

- Single maintained hot path for runtime actor management (`/actors/*` + providers registry).

## Stage Validation Checklist

Recommended per-stage validation commands:

- `moon run prisma:build`
- `moon run dark_core:typecheck`
- `moon run dark_core:test`

Optional focused checks while iterating:

- `bun test "src/modules/opencode/**/*.unit.test.ts"`
- `bun test "src/modules/actors/**/*.unit.test.ts"`
- `bun test "src/modules/actors/**/*.int.test.ts" --max-concurrency 1`

## Risks and Mitigations

- Risk: provider leakage into actor module.
  - Mitigation: enforce registry-only provider access and keep adapter contracts strict.
- Risk: locator drift after variant locator edits.
  - Mitigation: persist `workingLocator` snapshot and use actor-level `actorLocator` for resume.
- Risk: extra complexity from preserving legacy OpenCode route compatibility.
  - Mitigation: prefer direct replacement and remove legacy route behavior early.
- Risk: brittle status mapping across provider versions.
  - Mitigation: map provider status in one mapper file with unit tests per provider.

## Deterministic Mock Provider Strategy (Coverage-First)

Add a deterministic provider-native mock runtime for tests with two entry points:

- Callable TypeScript library (core state machine + typed returns).
- Elysia HTTP wrapper exposing provider-native routes for e2e/frontend tests.

### Goals

- Keep unit and integration tests independent from OpenCode runtime/network reliability.
- Reuse one behavior model for both direct function tests and HTTP-based tests.
- Cover only required provider-native operations for current actor/provider work.

### Module Shape

- `dark_core/src/modules/providers/mockagent/mockagent.types.ts`
  - Provider-native data contracts for sessions, status, and messages.
- `dark_core/src/modules/providers/mockagent/mockagent.engine.ts`
  - Deterministic in-memory runtime (session/message/status behavior).
- `dark_core/src/modules/providers/mockagent/mockagent.controller.ts`
  - Callable async functions wrapping engine methods.
- `dark_core/src/modules/providers/mockagent/mockagent.routes.ts`
  - Elysia provider-native routes with same response envelope style.
- `dark_core/src/modules/providers/mockagent/mockagent.app.ts`
  - Standalone `buildMockAgentApp()` for e2e/frontend harnesses.
- `dark_core/src/modules/providers/mockagent/mockagent.server.ts`
  - Lightweight boot helper for an HTTP test server process.

### Provider-Native Coverage (Now)

- `GET /mockagent/state`
- `GET /mockagent/sessions`
- `POST /mockagent/sessions`
- `GET /mockagent/sessions/status`
- `GET /mockagent/sessions/:id`
- `GET /mockagent/sessions/:id/messages?limit=...`
- `GET /mockagent/sessions/:id/attach`
- `POST /mockagent/sessions/:id/command`
- `POST /mockagent/sessions/:id/prompt`
- `POST /mockagent/sessions/:id/abort`
- `DELETE /mockagent/sessions/:id`

### Determinism Rules

- IDs are monotonic and fixed-width:
  - `mock_project_0001`, `mock_session_0001`, `mock_msg_0001`.
- Status model uses finite known values (`idle`, `busy`, `retry`) and command-driven transitions.
- Prompt responses are deterministic (`MockAgent reply // <prompt>`).
- Message listing honors `limit` exactly.

### Test Usage Guidance

- Unit tests: import and call `mockagent.engine.ts` directly.
- Route tests: inject dependencies into `createMockAgentRoutes(...)`.
- Integration tests for actors: register `provider: "mock"` adapter and run `/actors/*` flows without external runtime.
