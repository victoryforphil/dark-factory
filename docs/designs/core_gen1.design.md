# Notes

- Use moon and proto to manage tools and deps
- Use a shared schemas language to define our types / APIs
    - This will allow us to port to a new language easier
    - Prisma was first choice but lacks solid rust support
- Protobuf using elysia-protobuf seems like a good option
        - Library is a bit immature, so we forked it to allow patches
- Bun + Elysia JS based server for Gen 1 API for the following reasons:
    - Faster iteration
    - Nice OpenAPI support which is useful for testing and open source development
    - Out-of-the-box plugins for things like llms.txt, useful for the spirit
    - In theory, can do protobuf -> OpenAPI -> routes / docs all out of the box
- High level architecture is a core service that handles product / variant / actor tracking and provides an API for agentic comms. The core service will be used by agents to interact with product state, variant state, and each other. The core service will also handle logging and other helpers.
- Core acts as a spawnable API server that reads from a local database of stored product / variant / actor state. It exposes HTTP/WebSocket APIs to query / manipulate this state, as well as an API for agentic comms. The core service is designed to be lightweight and fast, allowing for quick iteration and development.
- Frontend (first one being a simple CLI, followed by a TUI) will interact with the core service to provide a user interface for monitoring and interacting with products, variants, and agents. The frontend will also provide tools for debugging and visualizing this state.


# Scope - Stage 0
- [ ] Shared Schema REST API w/ Build System
- [ ] Exposed REST API Docs
- [ ] Schemas for:
    - [ ] Product Definitions
        - [ ] Product locator key using `@local://{abs_path}`
        - [ ] Optional product `display_name`
    - [ ] Variant Definitions
        - [ ] Variant locator key using `@local://{abs_path}#default`
    - [ ] Actor State (Spawned Agents and their info)
- [ ] REST API for creating products from local-path locators
    - [ ] Product creation immediately creates default variant
- [ ] REST API for spawning OpenCode-based session servers in a defined variant
    - [ ] OpenCode over its Server API based manager
- [ ] REST API for querying last known state of spawned agents
- [ ] Stage 0 invariant: single product - single default variant - single OpenCode actor
- [ ] Basic Rust-based CLI for interacting with the core service and querying state


# ID Generation Spec (Deterministic Hash IDs)

Stage 0 uses deterministic IDs for `Product` and `Variant` derived from canonical locator strings.

## Goals
- Stable IDs across restarts.
- No raw absolute path exposure in primary keys.
- Cross-language deterministic behavior (TS + Rust).

## Algorithms
- Hash function: `SHA-256`
- Encoding: lowercase hex
- Prefixes:
    - `product_id`: `prd_`
    - `variant_id`: `var_`

## Canonical Locator Rules

### Product locator input
- External format: `@local://{abs_path}`

### Canonicalization steps (must be applied in this order)
1. Ensure prefix is exactly `@local://`.
2. Extract `{abs_path}`.
3. Normalize path separators to `/`.
4. Resolve `.` and `..` path segments.
5. Remove trailing `/` unless path is filesystem root.
6. On Windows only: lowercase drive letter (`C:` -> `c:`).
7. Rebuild canonical locator as: `@local://{canonical_abs_path}`.

> Note: Do not percent-decode/re-encode during canonicalization. Treat locator text as authoritative after separator and segment normalization.

## Product ID
- `product_id = "prd_" + sha256_hex(canonical_product_locator)`

## Variant Rules

### Variant name
- Stage 0 invariant: only `default` is valid.

### Canonical variant locator
- `canonical_variant_locator = canonical_product_locator + "#default"`

### Variant ID
- `variant_id = "var_" + sha256_hex(canonical_variant_locator)`

## Persistence and Validation
- Store both `id` and `locator`.
- `id` is the primary key; `locator` is indexed for lookup/debugging.
- On create:
    - Canonicalize locator.
    - Recompute deterministic `id`.
    - If `id` already exists, treat as idempotent create and return existing record.

## Collision Policy
- SHA-256 collisions are treated as practically impossible.
- If an `id` conflict is detected with mismatched locator, return a server error with code `ID_COLLISION_DETECTED`.

## Versioning
- This spec is `id_algo_version = 1`.
- If canonicalization or hash strategy changes later, bump version and migrate explicitly.
